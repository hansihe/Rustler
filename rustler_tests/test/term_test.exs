defmodule RustlerTest.TermTest do
  use ExUnit.Case, async: true

  test "term_debug" do
    assert RustlerTest.term_debug(33) == "33"
    assert RustlerTest.term_debug("hello world") == "<<\"hello world\">>"
    assert RustlerTest.term_debug("饂") == "<<233,165,130>>"
    assert RustlerTest.term_debug({:atom, :pair}) == "{atom,pair}"
    assert RustlerTest.term_debug(0..1000) == "\#{'__struct__'=>'Elixir.Range',first=>0,last=>1000}"
    assert RustlerTest.term_debug(Enum.to_list(0..5)) == "[0,1,2,3,4,5]"
    assert RustlerTest.term_debug(Enum.to_list(0..1000)) == "[#{Enum.join(0..1000, ",")}]"
    #assert RustlerTest.term_debug([[[[]],[]],[[]],[]]) == "[[[[]],[]],[[]],[]]"

    sues = Enum.map(1..500, fn i -> %{name: "Aunt Sue", id: i} end)
    sue_strs = Enum.map(1..500, fn i -> "\#{id=>#{i},name=><<\"Aunt Sue\">>}" end)
    assert RustlerTest.term_debug(sues) == "[#{Enum.join(sue_strs, ",")}]"
  end

  test "term equality" do
    assert RustlerTest.term_eq(1, 1)
    refute RustlerTest.term_eq(1.0, 1)

    assert RustlerTest.term_eq("something", "something")
    refute RustlerTest.term_eq("something", "else")

    assert RustlerTest.term_eq({1, 0}, {1, 0})
    refute RustlerTest.term_eq({0, 1}, {1, 0})
  end

  test "term compare" do
    # Number ordering
    assert RustlerTest.term_cmp(5, 10) == :less
    assert RustlerTest.term_cmp(10, 5) == :greater
    assert RustlerTest.term_cmp(5, 5) == :equal
    assert RustlerTest.term_cmp(5, 5.0) == :equal

    # Other term types
    assert RustlerTest.term_cmp(5, :test) == :less
  end

  test "term hash" do
    assert RustlerTest.term_phash2_hash(:foobar) == :erlang.phash2(:foobar)
    assert RustlerTest.term_phash2_hash("testing") == :erlang.phash2("testing")
    assert RustlerTest.term_phash2_hash(42) == :erlang.phash2(42)

    # Assume a certain distribution
    unique = 0..100
    |> Enum.map(&RustlerTest.term_phash2_hash(&1))
    |> Enum.group_by(fn n -> n end, fn n -> n end)
    |> map_size
    assert unique > 50

    unique = 0..100
    |> Enum.map(&RustlerTest.term_internal_hash(&1, 0))
    |> Enum.group_by(fn n -> n end, fn n -> n end)
    |> map_size
    assert unique > 50
  end

end
