defmodule Mix.Tasks.Rustler.New do
  use Mix.Task
  import Mix.Generator

  @shortdoc "Creates a new Rustler project."
  @moduledoc """
  Generates boilerplate for a new Rustler project.

  Usage:

  ```
  mix rustler.new [--module <Module>] [--name <Name>] [--otp-app <OTP App>]
  ```
  """

  @basic [
    {:eex, "basic/.cargo/config", ".cargo/config"},
    {:eex, "basic/README.md", "README.md"},
    {:eex, "basic/Cargo.toml.eex", "Cargo.toml"},
    {:eex, "basic/src/lib.rs", "src/lib.rs"},
    {:text, "basic/.gitignore", ".gitignore"}
  ]

  root = Path.join(:code.priv_dir(:rustler), "templates/")

  for {format, source, _} <- @basic do
    unless format == :keep do
      defp render(unquote(source)), do: unquote(File.read!(Path.join(root, source)))
    end
  end

  @switches [:module, :name, :otp_app]

  def run(argv) do
    {opts, _argv, _} = OptionParser.parse(argv, switches: @switches)

    module =
      case opts[:module] do
        nil ->
          prompt(
            "This is the name of the Elixir module the NIF module will be registered to.\n" <>
              "Module name"
          )

        module ->
          module
      end

    name =
      case opts[:name] do
        nil ->
          prompt_default(
            "This is the name used for the generated Rust crate. The default is most likely fine.\n" <>
              "Library name",
            format_module_name_as_name(module)
          )

        name ->
          name
      end

    otp_app =
      case opts[:otp_app] do
        nil -> Mix.Project.config() |> Keyword.get(:app)
        otp_app -> otp_app
      end

    check_module_name_validity!(module)

    sub_path = Path.join("native", name)

    path = Path.join([File.cwd!(), sub_path])
    new(otp_app, path, module, name, opts)

    add_to_workspace(sub_path)
  end

  defp new(otp_app, path, module, name, _opts) do
    module_elixir = "Elixir." <> module

    binding = [
      otp_app: otp_app,
      project_name: module_elixir,
      native_module: module_elixir,
      module: module,
      library_name: name,
      rustler_version: Rustler.rustler_version()
    ]

    copy_from(path, binding, @basic)

    Mix.Shell.IO.info([:green, "Ready to go! See #{path}/README.md for further instructions."])
  end

  defp add_to_workspace(sub_path) do
    Mix.Shell.IO.info([:green, "Adding #{sub_path} to workspace"])
    cargo_toml_fn = Path.join(File.cwd!(), "Cargo.toml")

    cargo_toml =
      case Toml.decode_file(cargo_toml_fn) do
        {:ok, data} ->
          data

        {:error, _} ->
          %{}
      end

    workspace = Map.get(cargo_toml, "workspace", %{})

    members =
      MapSet.new(Map.get(workspace, "members", []))
      |> MapSet.put(sub_path)
      |> MapSet.to_list()

    workspace_section = """
    [workspace]
    members = [#{members |> Enum.map(&inspect/1) |> Enum.join(", ")}]
    """

    # TODO: Only replace the existing workspace.members entry

    File.write(cargo_toml_fn, workspace_section)
  end

  defp check_module_name_validity!(name) do
    unless name =~ ~r/^[A-Z]\w*(\.[A-Z]\w*)*$/ do
      Mix.raise(
        "Module name must be a valid Elixir alias (for example: Foo.Bar), got: #{inspect(name)}"
      )
    end
  end

  defp format_module_name_as_name(module_name) do
    String.replace(String.downcase(module_name), ".", "_")
  end

  defp copy_from(target_dir, binding, mapping) when is_list(mapping) do
    for {format, source, target_path} <- mapping do
      target = Path.join(target_dir, target_path)

      case format do
        :keep ->
          File.mkdir_p!(target)

        :text ->
          create_file(target, render(source))

        :eex ->
          contents = EEx.eval_string(render(source), binding, file: source)
          create_file(target, contents)
      end
    end
  end

  defp prompt_default(message, default) do
    response = prompt([message, :white, " (", default, ")"])

    case response do
      "" -> default
      _ -> response
    end
  end

  defp prompt(message) do
    Mix.Shell.IO.print_app()
    resp = IO.gets(IO.ANSI.format([message, :white, " > "]))
    ?\n = :binary.last(resp)
    :binary.part(resp, {0, byte_size(resp) - 1})
  end
end
