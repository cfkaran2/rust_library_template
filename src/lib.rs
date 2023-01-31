//! Fill this in.

#![deny(absolute_paths_not_starting_with_crate)]
#![deny(ambiguous_associated_items)]
#![deny(anonymous_parameters)]
#![deny(arithmetic_overflow)]
#![deny(array_into_iter)]
#![deny(asm_sub_register)]
#![deny(bad_asm_style)]
#![deny(bare_trait_objects)]
#![deny(bindings_with_variant_name)]
#![deny(break_with_label_and_loop)]
#![deny(cenum_impl_drop_cast)]
#![deny(clashing_extern_declarations)]
#![deny(coherence_leak_check)]
#![deny(conflicting_repr_hints)]
#![deny(confusable_idents)]
#![deny(const_evaluatable_unchecked)]
#![deny(const_item_mutation)]
#![deny(dead_code)]
#![deny(deprecated)]
#![deny(deprecated_cfg_attr_crate_type_name)]
#![deny(deprecated_in_future)]
#![deny(deprecated_where_clause_location)]
#![deny(deref_into_dyn_supertrait)]
#![deny(deref_nullptr)]
#![deny(drop_bounds)]
#![deny(duplicate_macro_attributes)]
#![deny(dyn_drop)]
#![deny(elided_lifetimes_in_paths)]
#![deny(ellipsis_inclusive_range_patterns)]
#![deny(enum_intrinsics_non_enums)]
#![deny(explicit_outlives_requirements)]
#![deny(exported_private_dependencies)]
#![deny(forbidden_lint_groups)]
#![deny(function_item_references)]
#![deny(future_incompatible)]
#![deny(ill_formed_attribute_input)]
#![deny(illegal_floating_point_literal_pattern)]
#![deny(improper_ctypes)]
#![deny(improper_ctypes_definitions)]
#![deny(incomplete_features)]
#![deny(incomplete_include)]
#![deny(indirect_structural_match)]
#![deny(ineffective_unstable_trait_impl)]
#![deny(inline_no_sanitize)]
#![deny(invalid_atomic_ordering)]
#![deny(invalid_doc_attributes)]
#![deny(invalid_type_param_default)]
#![deny(invalid_value)]
#![deny(irrefutable_let_patterns)]
#![deny(keyword_idents)]
#![deny(large_assignments)]
#![deny(late_bound_lifetime_arguments)]
#![deny(legacy_derive_helpers)]
#![deny(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![deny(macro_use_extern_crate)]
#![deny(meta_variable_misuse)]
#![deny(missing_abi)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(missing_docs)]
#![deny(missing_fragment_specifier)]
#![deny(mixed_script_confusables)]
#![deny(mutable_transmutes)]
#![deny(named_arguments_used_positionally)]
#![deny(named_asm_labels)]
#![deny(no_mangle_const_items)]
#![deny(no_mangle_generic_items)]
#![deny(non_ascii_idents)]
#![deny(non_camel_case_types)]
#![deny(non_fmt_panics)]
#![deny(non_shorthand_field_patterns)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]
#![deny(nonstandard_style)]
#![deny(nontrivial_structural_match)]
#![deny(noop_method_call)]
#![deny(order_dependent_trait_objects)]
#![deny(overflowing_literals)]
#![deny(overlapping_range_endpoints)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(pointer_structural_match)]
#![deny(private_in_public)]
#![deny(proc_macro_back_compat)]
#![deny(proc_macro_derive_resolution_fallback)]
#![deny(pub_use_of_private_extern_crate)]
#![deny(redundant_semicolons)]
#![deny(renamed_and_removed_lints)]
#![deny(repr_transparent_external_private_fields)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2021_incompatible_closure_captures)]
#![deny(rust_2021_incompatible_or_patterns)]
#![deny(rust_2021_prefixes_incompatible_syntax)]
#![deny(rust_2021_prelude_collisions)]
#![deny(semicolon_in_expressions_from_macros)]
#![deny(single_use_lifetimes)]
#![deny(soft_unstable)]
#![deny(stable_features)]
#![deny(suspicious_auto_trait_impls)]
#![deny(temporary_cstring_as_ptr)]
#![deny(text_direction_codepoint_in_comment)]
#![deny(text_direction_codepoint_in_literal)]
#![deny(trivial_bounds)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(type_alias_bounds)]
#![deny(tyvar_behind_raw_pointer)]
#![deny(unaligned_references)]
#![deny(uncommon_codepoints)]
#![deny(unconditional_panic)]
#![deny(unconditional_recursion)]
#![deny(unexpected_cfgs)]
#![deny(uninhabited_static)]
#![deny(unknown_crate_types)]
#![deny(unknown_lints)]
#![deny(unnameable_test_items)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unstable_features)]
#![deny(unstable_name_collisions)]
#![deny(unsupported_calling_conventions)]
#![deny(unused)]
#![deny(unused_allocation)]
#![deny(unused_assignments)]
#![deny(unused_attributes)]
#![deny(unused_braces)]
#![deny(unused_comparisons)]
#![deny(unused_crate_dependencies)]
#![deny(unused_doc_comments)]
#![deny(unused_extern_crates)]
#![deny(unused_features)]
#![deny(unused_import_braces)]
#![deny(unused_imports)]
#![deny(unused_labels)]
#![deny(unused_lifetimes)]
#![deny(unused_macro_rules)]
#![deny(unused_macros)]
#![deny(unused_must_use)]
#![deny(unused_mut)]
#![deny(unused_parens)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_tuple_struct_fields)]
#![deny(unused_unsafe)]
#![deny(unused_variables)]
#![deny(useless_deprecated)]
#![deny(variant_size_differences)]
#![deny(warnings)]
#![deny(warnings)]
#![deny(where_clauses_object_safety)]
#![deny(while_true)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::restriction)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

pub mod error;

/// One line summary
///
/// Multi-paragraph discussion of the function.  This includes the following
/// information:
///
/// - What problem the function solves.
/// - What preconditions (if any) the function expects.
/// - What postconditions (if any) the function guarantees on completion.
///
/// ## Example
///
/// Within this section, provide an example of how to use the function.  This
/// will be a doctest that can be run.  You may choose to give examples that
/// cannot be run as well, but those are optional.
///
/// ```rust
/// # use std::error::Error;
/// # use {{crate_name}}::add;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///       let output = add(5, 6);
///       assert_eq!(output, 11);
/// #
/// #     Ok(())
/// # }
/// ```
///
/// ## Safety (optional)
///
/// This section is optional, but *must* be in place if this function can lead
/// to unsafety.  All conditions that can lead to unsafety *must* be described.
/// How to avoid unsafe behavior when calling the function *must* also be
/// described.
///
/// ## Panics (optional)
///
/// If this function can panic via the 'panic!()' or 'abort!()' macros, the
/// causes *must* be described here, along with how to avoid causing a panic or
/// abort.  Note that certain types of panics and aborts are outside of the
/// control of the function and do not need to be described here.  As an
/// example, it is impossible to know if a process will run out of memory
/// during an allocation because that requires knowledge of what other
/// processes are planning on doing concurrent to this process.  As a rule of
/// thumb, if this function calls `panic!()` or `abort!()` directly, it must be
/// documented. Otherwise, use your best judgement.
///
/// ## Parameters
///
/// If a function accepts arguments, then they *must* be described here.  In
/// general, this should be in the form of a list, similar to the following:
///
/// - `arg` - A description of `arg`.  You do not need to detail what the type
///   is as the compiler will fill that in for you, but you must describe what
///   it is for.  Most importantly, if there are values that can be passed in
///   that are not allowed, you **must** describe them here.  For example, if
///   `arg` must be in the range `[0, 10]` or a panic can occur, then you
///   **must** provide a warning about that here, as well as provide the warning
///   in the `Panics` section above.
/// - `next_arg` - The next argument in the list
/// - `and_so_forth`
///
/// ## Returns
///
/// Provide a complete description of what is returned.  The type information
/// will be picked up by the compiler, but how and why a particular instance is
/// returned is not always obvious from the return type, which is why
/// you **must** explain it here.  In particular, if you return a `Result`,
/// always detail what types of errors may be returned by this function.
///
/// The only case where you don't need the `Returns` section is if nothing (AKA,
/// the `unit` type, `()`) is returned.
///
/// For the function below, we might have documentation similar to the
/// following:
///    If this function operates correctly, it will return the additive inverse
///    of `arg` in the `Ok()` variant.  The additive inverse is the value that
///    makes `(arg + x) % 256 == 0` true (where `x` is the additive inverse).
///
///    This function makes a remote procedure call (RPC) under the hood to a
///    service that may or may not be available at the time of the call.  If
///    the service isn't available (or it has some other kind of error), then
///    the appropriate \[`FooError`\] type will be returned in the
///    `Err()` variant. Read the documentation for \[`FooError`\] for more
///    information on what kinds of errors you might need to deal with.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
