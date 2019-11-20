defmodule BinaryExampleTest do
  use ExUnit.Case

  test "binary is compiled" do
    bins = ~w(binary_example hello_rust hello_rust2)

    for bin <- bins do
      assert_exists(bin)
    end
  end

  defp assert_exists(name) do
    assert_exists(name, :os.type())
  end

  defp assert_exists(name, {:win32, _} = type) do
    assert File.exists?("priv/crates/#{name}.exe")
  end

  defp assert_exists(name, type) do
    assert File.exists?("priv/native/#{name}")
  end
end
