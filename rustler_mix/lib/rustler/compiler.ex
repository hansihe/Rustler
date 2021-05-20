defmodule Rustler.Compiler do
  @moduledoc false

  alias Rustler.Compiler.{Config}

  @doc false
  def compile_crate(module, opts) do
    shell = Mix.shell()
    otp_app = Keyword.fetch!(opts, :otp_app)

    crate = ensure_string(Keyword.fetch!(opts, :crate))
    config = Config.from(otp_app, module, opts)

    is_release = config.mode == :release
    artifacts = do_compile(crate, is_release)

    entry = artifacts[crate]

    is_lib = :cargo_artifact.kind(entry) == :cdylib
    is_bin = :cargo_artifact.kind(entry) == :bin

    if !is_lib and !is_bin do
      Mix.raise("Crate #{crate} is neither a 'bin' nor a 'cdylib' but #{entry[:kind]}")
    end

    priv_dir = priv_dir(entry, is_release)
    dest_root = :code.priv_dir(config.otp_app)

    out_paths =
      for filename <- :cargo_artifact.filenames(entry) do
        out_path = Path.join(priv_dir, Path.basename(filename))
        dest = Path.join(dest_root, out_path)
        rel_filename = Path.basename(filename)
        rel_dest = Path.relative_to_cwd(dest)

        shell.info("  Copying #{rel_filename} to #{rel_dest}")
        File.mkdir_p!(Path.dirname(dest))
        File.copy!(filename, dest)

        out_path
      end

    [load_path] =
      if is_lib do
        out_paths |> Enum.filter(&:cargo_util.is_dylib/1)
      else
        exec = :cargo_artifact.executable(entry)
        [Path.join(priv_dir, Path.basename(exec))]
      end

    %Config{config | lib: is_lib, load_path: Path.rootname(load_path)}
  end

  defp priv_dir(entry, is_release) do
    type =
      if is_release do
        "release"
      else
        "debug"
      end

    name = :cargo_artifact.name(entry)
    version = :cargo_artifact.version(entry)

    "crates/#{name}/#{version}/#{type}"
  end

  defp ensure_string(atom) when is_atom(atom) do
    Atom.to_string(atom)
  end

  defp ensure_string(list) when is_list(list) do
    IO.iodata_to_binary(list)
  end

  defp ensure_string(str) when is_binary(str) do
    str
  end

  defp do_compile(crate, is_release) do
    cargo_opts = %{
      release: is_release
    }

    Mix.shell().info("Starting build in #{File.cwd!()}")

    cargo = :cargo.init(File.cwd!(), cargo_opts)

    artifacts = :cargo.build(cargo, crate)

    # This drops the unique key in favour of the crate name
    artifacts |> Map.new(&{:cargo_artifact.name(&1), &1})
  end
end
