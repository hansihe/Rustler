#!/bin/bash
#
# Test if rustler_mix can be used to create a working example from the template.
#
# Note: This creates a temporary directory for the mix project. The directory is only cleaned up
# if the test works. This is done in order to allow investigating the error.
#

set -e

rustler_mix=$PWD
rustler=$(realpath $PWD/../rustler)
tmp=$(mktemp --directory)

#
# Test Steps
#
# * Create a new Elixir project
# * Add rustler as a dependency
# * Initialize a first crate from the template
# * Use rustler from the repository instead of from crates.io
# * Check that it compiles
# * Check that the module template in the generated README works
# * Check that the NIF can be loaded and used
#

echo "Creating a new mix project and rustler template in $tmp"
cd $tmp

mix new test_rustler_mix
cd test_rustler_mix
mkdir -p priv/native

cat >mix.exs <<EOF
defmodule TestRustlerMix.MixProject do
  use Mix.Project

  def project do
    [
      app: :test_rustler_mix,
      version: "0.1.0",
      elixir: "~> 1.6",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application, do: [ ]

  defp deps, do: [ {:rustler, path: "$rustler_mix"} ]
end
EOF

mix deps.get
mix deps.compile

mix rustler.new --module RustlerMixTest --name rustler_mix_test

sed -i "s|^rustler.*$|rustler = { path = \"$rustler\" }|" native/rustler_mix_test/Cargo.toml

mix compile

# Delete everything except the templated module from the generated README

delete=1
cat native/rustler_mix_test/README.md | while read line; do
    case "$line" in
      '```elixir')
        delete=0
        ;;
      '```'*)
        delete=1
        ;;
      *)
        if [ "$delete" -eq 0 ]; then
          echo "$line"
        fi
    esac
done > lib/rustler_mix_test.ex

cat >test/rustler_mix_test_test.exs <<EOF
defmodule RustlerMixTestTest do
  use ExUnit.Case

  test "can use generated nif" do
      assert RustlerMixTest.add(1, 2) == 3
  end
end
EOF

mix test

echo "Done; cleaning up"
rm -rf "$tmp"
