defmodule NifNotLoadedError do
  defexception message: "nif not loaded"
end

defmodule RustlerTest do
  use Rustler,
    otp_app: :rustler_test,
    crate: :rustler_test

  defp err do
    throw(NifNotLoadedError)
  end

  def add_u32(_, _), do: err()
  def add_i32(_, _), do: err()
  def echo_u8(_), do: err()
  def option_inc(_), do: err()
  def result_to_int(_), do: err()

  def sum_list(_), do: err()
  def make_list(), do: err()

  def term_debug(_), do: err()
  def term_eq(_, _), do: err()
  def term_cmp(_, _), do: err()

  def sum_map_values(_), do: err()
  def map_entries_sorted(_), do: err()
  def map_from_arrays(_keys, _values), do: err()
  def map_generic(_), do: err()

  def resource_make(), do: err()
  def resource_set_integer_field(_, _), do: err()
  def resource_get_integer_field(_), do: err()
  def resource_make_immutable(_), do: err()
  def resource_immutable_count(), do: err()

  def make_shorter_subbinary(_), do: err()
  def parse_integer(_), do: err()
  def binary_new(), do: err()
  def owned_binary_new(), do: err()
  def unowned_to_owned(_), do: err()
  def realloc_shrink(), do: err()
  def realloc_grow(), do: err()
  def encode_string(), do: err()
  def decode_iolist(_), do: err()

  def atom_to_string(_), do: err()
  def atom_equals_ok(_), do: err()
  def binary_to_atom(_), do: err()
  def binary_to_existing_atom(_), do: err()

  def threaded_fac(_), do: err()
  def threaded_sleep(_), do: err()

  def send_all(_, _), do: err()
  def sublists(_), do: err()

  def tuple_echo(_), do: err()
  def record_echo(_), do: err()
  def map_echo(_), do: err()
  def struct_echo(_), do: err()
  def unit_enum_echo(_), do: err()
  def untagged_enum_echo(_), do: err()
  def untagged_enum_with_truthy(_), do: err()
  def newtype_echo(_), do: err()
  def tuplestruct_echo(_), do: err()
  def newtype_record_echo(_), do: err()
  def tuplestruct_record_echo(_), do: err()
  def reserved_keywords_type_echo(_), do: err()

  def dirty_io(), do: err()
  def dirty_cpu(), do: err()

  def sum_range(_), do: err()

  def scheduled_fac(_, _ \\ nil), do: err()

  def bad_arg_error(), do: err()
  def atom_str_error(), do: err()
  def raise_atom_error(), do: err()
  def raise_term_with_string_error(), do: err()
  def raise_term_with_atom_error(), do: err()
  def term_with_tuple_error(), do: err()

  def nif_attrs_can_rename(), do: err()
end
