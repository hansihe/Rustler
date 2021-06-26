defmodule Rustler.Compiler do
  @moduledoc false

  alias Rustler.Compiler.{Config, Messages, Rustup}

  @doc false
  def compile_crate(module, opts) do
    otp_app = Keyword.fetch!(opts, :otp_app)
    config = Config.from(otp_app, module, opts)

    unless config.skip_compilation? do
      crate_full_path = Path.expand(config.path, File.cwd!())

      File.mkdir_p!(priv_dir())

      Mix.shell().info("Compiling crate #{config.crate} in #{config.mode} mode (#{config.path})")

      [cmd | args] =
        make_base_command(config.cargo)
        |> make_no_default_features_flag(config.default_features)
        |> make_features_flag(config.features)
        |> make_target_flag(config.target)
        |> make_build_mode_flag(config.mode)
        |> make_platform_hacks(crate_full_path, :os.type())

      compile_result =
        System.cmd(cmd, args,
          cd: crate_full_path,
          stderr_to_stdout: true,
          env: [{"CARGO_TARGET_DIR", config.target_dir} | config.env],
          into: IO.stream(:stdio, :line)
        )

      case compile_result do
        {_, 0} -> :ok
        {_, code} -> raise "Rust NIF compile error (rustc exit code #{code})"
      end

      handle_artifacts(crate_full_path, config)
      # See #326: Ensure that the libraries are copied into the correct subdirectory
      # in `_build`.
      Mix.Project.build_structure()
    end

    config
  end

  defp make_base_command(:system), do: ["cargo", "rustc"]
  defp make_base_command({:system, channel}), do: ["cargo", channel, "rustc"]
  defp make_base_command({:bin, path}), do: [path, "rustc"]
  defp make_base_command({:rustup, version}) do
    if Rustup.version() == :none do
      throw_error(:rustup_not_installed)
    end

    unless Rustup.version_installed?(version) do
      throw_error({:rust_version_not_installed, version})
    end

    ["rustup", "run", version, "cargo", "rustc"]
  end

  defp make_platform_hacks(args, crate_path, {:unix, :darwin}) do
    root = Path.join([".cargo", "config"])
    path = Path.join([crate_path, ".cargo", "config"])

    if File.exists?(root) || File.exists?(path) do
      args
    else
      IO.write([
        "\n",
        IO.ANSI.yellow(),
        """
        Compiling on macOS requires special link args in order to compile
        correctly.

        Rustler is currently working around this issue in the compiler task.
        This will be removed in v1.0.0 in favor of a user supplied .cargo/config
        file.

        To remove this warning, please create #{path}
        with the following content:

              [target.x86_64-apple-darwin]
              rustflags = [
                  "-C", "link-arg=-undefined",
                  "-C", "link-arg=dynamic_lookup",
              ]

        See https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/MachOTopics/1-Articles/executing_files.html
        for more details.

        """,
        IO.ANSI.default_color(),
        "\n"
      ])

      args ++ ["--", "-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]
    end
  end

  defp make_platform_hacks(args, _, _), do: args

  defp make_no_default_features_flag(args, true), do: args
  defp make_no_default_features_flag(args, false), do: args ++ ["--no-default-features"]

  defp make_features_flag(args, []), do: args
  defp make_features_flag(args, flags), do: args ++ ["--features", Enum.join(flags, ",")]

  defp make_target_flag(args, target) when is_binary(target), do: args ++ ["--target=#{target}"]
  defp make_target_flag(args, _), do: args

  defp make_build_mode_flag(args, :release), do: args ++ ["--release"]
  defp make_build_mode_flag(args, :debug), do: args

  defp handle_artifacts(path, config) do
    toml = toml_data(path)
    names = get_name(toml, :lib) ++ get_name(toml, :bin)

    Enum.each(names, fn {name, type} ->
      {src_file, dst_file} = make_file_names(name, type)
      compiled_lib = Path.join([config.target_dir, Atom.to_string(config.mode), src_file])
      destination_lib = Path.join(priv_dir(), dst_file)

      # If the file exists already, we delete it first. This is to ensure that another
      # process, which might have the library dynamically linked in, does not generate
      # a segfault. By deleting it first, we ensure that the copy operation below does
      # not write into the existing file.
      File.rm(destination_lib)
      File.cp!(compiled_lib, destination_lib)
    end)
  end

  defp get_name(toml, section) do
    case toml[to_string(section)] do
      nil -> []
      values when is_map(values) -> [{values["name"], section}]
      values when is_list(values) -> Enum.map(values, &{&1["name"], section})
    end
  end

  defp make_file_names(base_name, :lib) do
    case :os.type() do
      {:win32, _} -> {"#{base_name}.dll", "lib#{base_name}.dll"}
      {:unix, :darwin} -> {"lib#{base_name}.dylib", "lib#{base_name}.so"}
      {:unix, _} -> {"lib#{base_name}.so", "lib#{base_name}.so"}
    end
  end

  defp make_file_names(base_name, :bin) do
    case :os.type() do
      {:win32, _} -> {"#{base_name}.exe", "#{base_name}.exe"}
      {:unix, _} -> {base_name, base_name}
    end
  end

  defp throw_error(error_descr) do
    Mix.shell().error(Messages.message(error_descr))
    raise "Compilation error"
  end

  defp toml_data(path) do
    unless File.dir?(path) do
      throw_error({:nonexistent_crate_directory, path})
    end

    case File.read("#{path}/Cargo.toml") do
      {:error, :enoent} ->
        throw_error({:cargo_toml_not_found, path})

      {:ok, text} ->
        Toml.decode!(text)
    end
  end

  defp priv_dir, do: "priv/native"
end
