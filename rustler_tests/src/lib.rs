//#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;

//#[macro_use]
extern crate lazy_static;

use rustler::{Env, Term};
use rustler::schedule::SchedulerFlags;

mod test_primitives;
mod test_list;
mod test_map;
mod test_resource;
mod test_binary;
mod test_atom;
mod test_thread;
mod test_env;
mod test_codegen;
mod test_term;
mod test_dirty;

rustler::rustler_export_nifs!(
    "Elixir.RustlerTest",
    [
        ("add_u32", 2, test_primitives::add_u32),
        ("add_i32", 2, test_primitives::add_i32),
        ("echo_u8", 1, test_primitives::echo_u8),
        ("option_inc", 1, test_primitives::option_inc),
        ("result_to_int", 1, test_primitives::result_to_int),

        ("sum_list", 1, test_list::sum_list),
        ("make_list", 0, test_list::make_list),

        ("term_debug", 1, test_term::term_debug),
        ("term_eq", 2, test_term::term_eq),
        ("term_cmp", 2, test_term::term_cmp),
        ("term_internal_hash", 2, test_term::term_internal_hash),
        ("term_phash2_hash", 1, test_term::term_phash2_hash),

        ("sum_map_values", 1, test_map::sum_map_values),
        ("map_entries_sorted", 1, test_map::map_entries_sorted),
        ("map_from_arrays", 2, test_map::map_from_arrays),

        ("resource_make", 0, test_resource::resource_make),
        ("resource_set_integer_field", 2, test_resource::resource_set_integer_field),
        ("resource_get_integer_field", 1, test_resource::resource_get_integer_field),
        ("resource_make_immutable", 1, test_resource::resource_make_immutable),
        ("resource_immutable_count", 0, test_resource::resource_immutable_count),

        ("atom_to_string", 1, test_atom::atom_to_string),
        ("atom_equals_ok", 1, test_atom::atom_equals_ok),
        ("binary_to_atom", 1, test_atom::binary_to_atom),
        ("binary_to_existing_atom", 1, test_atom::binary_to_existing_atom),

        ("make_shorter_subbinary", 1, test_binary::make_shorter_subbinary),
        ("parse_integer", 1, test_binary::parse_integer),
        ("binary_new", 0, test_binary::binary_new),
        ("unowned_to_owned", 1, test_binary::unowned_to_owned),
        ("realloc_shrink", 0, test_binary::realloc_shrink),
        ("realloc_grow", 0, test_binary::realloc_grow),
        ("encode_string", 0, test_binary::encode_string),

        ("threaded_fac", 1, test_thread::threaded_fac),
        ("threaded_sleep", 1, test_thread::threaded_sleep),

        ("send_all", 2, test_env::send_all),
        ("sublists", 1, test_env::sublists),

        ("tuple_echo", 1, test_codegen::tuple_echo),
        ("record_echo", 1, test_codegen::record_echo),
        ("map_echo", 1, test_codegen::map_echo),
        ("struct_echo", 1, test_codegen::struct_echo),
        ("unit_enum_echo", 1, test_codegen::unit_enum_echo),
        ("untagged_enum_echo", 1, test_codegen::untagged_enum_echo),

        ("dirty_cpu", 0, test_dirty::dirty_cpu, SchedulerFlags::DirtyCpu),
        ("dirty_io", 0, test_dirty::dirty_io, SchedulerFlags::DirtyIo),
    ],
    Some(on_load)
);

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    test_resource::on_load(env);
    test_atom::on_load(env);
    true
}
