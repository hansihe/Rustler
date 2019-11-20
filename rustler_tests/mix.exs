defmodule RustlerTest.Mixfile do
  use Mix.Project

  def project do
    [
      app: :rustler_test,
      version: "0.0.1",
      elixir: "~> 1.2",
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [applications: [:logger]]
  end

  defp deps do
    [{:rustler, path: "../rustler_mix", runtime: false}]
  end
end
