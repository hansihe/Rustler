defmodule BinaryExampleTest do
  use ExUnit.Case

  test "binary is compiled" do
    path = BinaryExample.rustler_path()
    File.exists?(path)
  end
end
