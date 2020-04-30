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
    is_release = Mix.env() in [:prod, :bench]

    cargo_opts = %{
      release: is_release
    }

    Mix.shell().info("Starting build in #{File.cwd!()}")

    cargo = :cargo.init(File.cwd!(), cargo_opts)
    artifacts = :cargo.build(cargo)

    # This drops the unique key in favour of the crate name
    artifacts =
      artifacts
      |> Map.new(&{:cargo_artifact.name(&1), &1})

    {:reply, artifacts, artifacts}
  end

  def handle_call(:compile, _from, artifacts) do
    {:reply, artifacts, artifacts}
  end
end
