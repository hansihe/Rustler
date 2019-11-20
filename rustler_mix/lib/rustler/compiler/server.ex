defmodule Rustler.Compiler.Server do
  use GenServer

  defp ensure_running() do
    case GenServer.start(__MODULE__, [], name: __MODULE__) do
      {:ok, _pid} -> :ok
      {:error, {:already_started, _pid}} -> :ok
    end
  end

  def build() do
    ensure_running()
    GenServer.call(__MODULE__, :compile, :infinity)
  end

  @impl true
  def init([]) do
    {:ok, nil}
  end

  @impl true
  def handle_call(:compile, _from, nil) do
    shell = Mix.shell()
    _otp_app = Mix.Project.config() |> Keyword.get(:app)
    is_release = Mix.env() in [:prod, :bench]

    cargo_opts = %{
      release: is_release
    }

    cargo = :cargo.init(File.cwd!(), cargo_opts)
    artifacts = :cargo.build_and_capture(cargo)
    {:reply, artifacts, artifacts}
  end

  def handle_call(:compile, _from, artifacts) do
    {:reply, artifacts, artifacts}
  end
end
