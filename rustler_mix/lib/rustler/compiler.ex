defmodule Rustler.Compiler do
  @moduledoc false

  alias Rustler.Compiler.{Config}
  alias Rustler.Compiler.{Server}

  @doc false
  def compile_crate(module, opts) do
    shell = Mix.shell()
    otp_app = Keyword.fetch!(opts, :otp_app)
    crate = Atom.to_string(Keyword.fetch!(opts, :crate))
    config = Config.from(otp_app, module, opts)

    artifacts = Server.build()

    is_release = Mix.env() in [:prod, :bench]
    entry = artifacts[crate]

    # Only a single file result per crate is supported right now
    [filename] = entry[:filenames]
    rel_filename = Path.basename(filename)

    out_path = Path.join(priv_dir(entry, is_release), Path.basename(filename))
    dest = Path.join(:code.priv_dir(config.otp_app), out_path)
    rel_dest = Path.relative_to_cwd(dest)

    shell.info("  Copying #{rel_filename} to #{rel_dest}")
    File.mkdir_p!(Path.dirname(dest))
    File.copy!(filename, dest)

    is_lib = entry[:kind] == ["cdylib"]

    %Config{config | lib: is_lib, load_path: Path.rootname(out_path)}
  end

  defp priv_dir(entry, is_release) do
    type =
      if is_release do
        "release"
      else
        "debug"
      end

    "crates/#{entry[:name]}/#{entry[:version]}/#{type}"
  end
end
