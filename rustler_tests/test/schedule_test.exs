defmodule RustlerTest.ScheduleTest do
  use ExUnit.Case, async: true

  test "scheduled factorial" do
    assert 24 == RustlerTest.scheduled_fac(4)
  end
end
