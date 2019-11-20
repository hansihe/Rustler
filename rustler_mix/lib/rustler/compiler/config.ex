defmodule Rustler.Compiler.Config do
  @moduledoc false

  @type rust_version :: :stable | :beta | :nightly | binary()
  @type crate :: binary()
  @type default_features :: boolean()
  @type env :: [{binary(), binary()}]
  @type features :: [binary()]
  @type mode :: :debug | :release
  @type load_data :: term()

  defstruct crate: nil,
            lib: true,
            load_data: 0,
            load_path: nil,
            mode: :release,
            otp_app: nil,
            target: nil

  alias Rustler.Compiler.Config

  def from(otp_app, module, opts) do
    config = Application.get_env(otp_app, module, [])

    crate = config[:crate] || opts[:crate] || otp_app

    defaults = %Config{
      crate: crate,
      mode: build_mode(Mix.env()),
      otp_app: otp_app
    }

    opts =
      defaults
      |> Map.from_struct()
      |> Enum.into([])
      |> Keyword.merge(opts)
      |> Keyword.merge(config)

    struct!(Config, opts)
  end

  defp build_mode(env) when env in [:prod, :bench], do: :release
  defp build_mode(_), do: :debug
end
