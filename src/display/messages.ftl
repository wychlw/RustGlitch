nfuzz_add_paren = try adding parentheses

nfuzz_ambiguous_range_pattern = the range pattern here has ambiguous interpretation
nfuzz_ambiguous_range_pattern_suggestion = add parentheses to clarify the precedence

nfuzz_array_brackets_instead_of_braces = this is a block expression, not an array
    .suggestion = to make an array, use square brackets instead of curly braces

nfuzz_array_index_offset_of = array indexing not supported in offset_of

nfuzz_assignment_else_not_allowed = <assignment> ... else {"{"} ... {"}"} is not allowed

nfuzz_associated_static_item_not_allowed = associated `static` items are not allowed

nfuzz_async_block_in_2015 = `async` blocks are only allowed in Rust 2018 or later

nfuzz_async_bound_modifier_in_2015 = `async` trait bounds are only allowed in Rust 2018 or later

nfuzz_async_fn_in_2015 = `async fn` is not permitted in Rust 2015
    .label = to use `async fn`, switch to Rust 2018 or later

nfuzz_async_impl = `async` trait implementations are unsupported

nfuzz_async_move_block_in_2015 = `async move` blocks are only allowed in Rust 2018 or later

nfuzz_async_move_order_incorrect = the order of `move` and `async` is incorrect
    .suggestion = try switching the order

nfuzz_at_dot_dot_in_struct_pattern = `@ ..` is not supported in struct patterns
    .suggestion = bind to each field separately or, if you don't need them, just remove `{$ident} @`

nfuzz_at_in_struct_pattern = Unexpected `@` in struct pattern
    .note = struct patterns use `field: pattern` syntax to bind to fields
    .help = consider replacing `new_name @ field_name` with `field_name: new_name` if that is what you intended

nfuzz_attr_after_generic = trailing attribute after generic parameter
    .label = attributes must go before parameters

nfuzz_attr_without_generics = attribute without generic parameters
    .label = attributes are only permitted when preceding parameters

nfuzz_attribute_on_param_type = attributes cannot be applied to a function parameter's type
    .label = attributes are not allowed here

nfuzz_bad_assoc_type_bounds = bounds on associated types do not belong here
    .label = belongs in `where` clause

nfuzz_bad_item_kind = {$descr} is not supported in {$ctx}
    .help = consider moving the {$descr} out to a nearby module scope

nfuzz_bad_return_type_notation_output =
    return type not allowed with return type notation
    .suggestion = remove the return type

nfuzz_bare_cr = {$double_quotes ->
    [true] bare CR not allowed in string, use `\r` instead
    *[false] character constant must be escaped: `\r`
    }
    .escape = escape the character

nfuzz_bare_cr_in_raw_string = bare CR not allowed in raw string

nfuzz_binder_and_polarity = `for<...>` binder not allowed with `{$polarity}` trait polarity modifier
    .label = there is not a well-defined meaning for a higher-ranked `{$polarity}` trait

nfuzz_binder_before_modifiers = `for<...>` binder should be placed before trait bound modifiers
    .label = place the `for<...>` binder before any modifiers

nfuzz_bounds_not_allowed_on_trait_aliases = bounds are not allowed on trait aliases

nfuzz_box_not_pat = expected pattern, found {$descr}
    .note = `box` is a reserved keyword
    .suggestion = escape `box` to use it as an identifier

nfuzz_box_syntax_removed = `box_syntax` has been removed
nfuzz_box_syntax_removed_suggestion = use `Box::new()` instead

nfuzz_cannot_be_raw_ident = `{$ident}` cannot be a raw identifier

nfuzz_cannot_be_raw_lifetime = `{$ident}` cannot be a raw lifetime

nfuzz_catch_after_try = keyword `catch` cannot follow a `try` block
    .help = try using `match` on the result of the `try` block instead

nfuzz_cfg_attr_bad_delim = wrong `cfg_attr` delimiters
nfuzz_colon_as_semi = statements are terminated with a semicolon
    .suggestion = use a semicolon instead

nfuzz_comma_after_base_struct = cannot use a comma after the base struct
    .note = the base struct must always be the last field
    .suggestion = remove this comma

nfuzz_comparison_interpreted_as_generic =
    `<` is interpreted as a start of generic arguments for `{$type}`, not a comparison
    .label_args = interpreted as generic arguments
    .label_comparison = not interpreted as comparison
    .suggestion = try comparing the cast value

nfuzz_comparison_operators_cannot_be_chained = comparison operators cannot be chained
    .sugg_parentheses_for_function_args = or use `(...)` if you meant to specify fn arguments
    .sugg_split_comparison = split the comparison into two
    .sugg_parenthesize = parenthesize the comparison
nfuzz_compound_assignment_expression_in_let = can't reassign to an uninitialized variable
    .suggestion = initialize the variable
    .help = if you meant to overwrite, remove the `let` binding

nfuzz_const_generic_without_braces = expressions must be enclosed in braces to be used as const generic arguments
    .suggestion = enclose the `const` expression in braces

nfuzz_const_global_cannot_be_mutable = const globals cannot be mutable
    .label = cannot be mutable
    .suggestion = you might want to declare a static instead

nfuzz_const_let_mutually_exclusive = `const` and `let` are mutually exclusive
    .suggestion = remove `let`

nfuzz_cr_doc_comment = bare CR not allowed in {$block ->
    [true] block doc-comment
    *[false] doc-comment
}

nfuzz_default_not_followed_by_item = `default` is not followed by an item
    .label = the `default` qualifier
    .note = only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`

nfuzz_do_catch_syntax_removed = found removed `do catch` syntax
    .note = following RFC #2388, the new non-placeholder syntax is `try`
    .suggestion = replace with the new syntax

nfuzz_doc_comment_does_not_document_anything = found a documentation comment that doesn't document anything
    .help = doc comments must come before what they document, if a comment was intended use `//`
    .suggestion = missing comma here

nfuzz_doc_comment_on_param_type = documentation comments cannot be applied to a function parameter's type
    .label = doc comments are not allowed here

nfuzz_dot_dot_dot_for_remaining_fields = expected field pattern, found `{$token_str}`
    .suggestion = to omit remaining fields, use `..`

nfuzz_dot_dot_dot_range_to_pattern_not_allowed = range-to patterns with `...` are not allowed
    .suggestion = use `..=` instead

nfuzz_dot_dot_range_attribute = attributes are not allowed on range expressions starting with `..`

nfuzz_dotdotdot = unexpected token: `...`
    .suggest_exclusive_range = use `..` for an exclusive range
    .suggest_inclusive_range = or `..=` for an inclusive range

nfuzz_dotdotdot_rest_pattern = unexpected `...`
    .label = not a valid pattern
    .suggestion = for a rest pattern, use `..` instead of `...`

nfuzz_double_colon_in_bound = expected `:` followed by trait or lifetime
    .suggestion = use single colon

nfuzz_dyn_after_mut = `mut` must precede `dyn`
    .suggestion = place `mut` before `dyn`

nfuzz_empty_exponent_float = expected at least one digit in exponent

nfuzz_empty_unicode_escape = empty unicode escape
    .label = this escape must have at least 1 hex digit

nfuzz_enum_pattern_instead_of_identifier = expected identifier, found enum pattern

nfuzz_enum_struct_mutually_exclusive = `enum` and `struct` are mutually exclusive
    .suggestion = replace `enum struct` with

nfuzz_eq_field_init = expected `:`, found `=`
    .suggestion = replace equals symbol with a colon

nfuzz_escape_only_char = {$byte ->
    [true] byte
    *[false] character
    } constant must be escaped: `{$escaped_msg}`
    .escape = escape the character

nfuzz_expect_dotdot_not_dotdotdot = expected `..`, found `...`
    .suggestion = use `..` to fill in the rest of the fields

nfuzz_expect_eq_instead_of_eqeq = expected `=`, found `==`
    .suggestion = consider using `=` here

nfuzz_expect_label_found_ident = expected a label, found an identifier
    .suggestion = labels start with a tick

nfuzz_expect_path = expected a path

nfuzz_expected_binding_left_of_at = left-hand side of `@` must be a binding
    .label_lhs = interpreted as a pattern, not a binding
    .label_rhs = also a pattern
    .note = bindings are `x`, `mut x`, `ref x`, and `ref mut x`

nfuzz_expected_builtin_ident = expected identifier after `builtin #`

nfuzz_expected_comma_after_pattern_field = expected `,`

nfuzz_expected_else_block = expected `{"{"}`, found {$first_tok}
    .label = expected an `if` or a block after this `else`
    .suggestion = add an `if` if this is the condition of a chained `else if` statement

nfuzz_expected_expression_found_let = expected expression, found `let` statement
    .note = only supported directly in conditions of `if` and `while` expressions
    .not_supported_or = `||` operators are not supported in let chain expressions
    .not_supported_parentheses = `let`s wrapped in parentheses are not supported in a context with let chains

nfuzz_expected_fn_path_found_fn_keyword = expected identifier, found keyword `fn`
    .suggestion = use `Fn` to refer to the trait

nfuzz_expected_identifier = expected identifier

nfuzz_expected_identifier_found_doc_comment = expected identifier, found doc comment
nfuzz_expected_identifier_found_doc_comment_str = expected identifier, found doc comment `{$token}`
nfuzz_expected_identifier_found_keyword = expected identifier, found keyword
nfuzz_expected_identifier_found_keyword_str = expected identifier, found keyword `{$token}`
nfuzz_expected_identifier_found_metavar = expected identifier, found metavariable
# This one deliberately doesn't print a token.
nfuzz_expected_identifier_found_metavar_str = expected identifier, found metavariable
nfuzz_expected_identifier_found_reserved_identifier = expected identifier, found reserved identifier
nfuzz_expected_identifier_found_reserved_identifier_str = expected identifier, found reserved identifier `{$token}`
nfuzz_expected_identifier_found_reserved_keyword = expected identifier, found reserved keyword
nfuzz_expected_identifier_found_reserved_keyword_str = expected identifier, found reserved keyword `{$token}`
nfuzz_expected_identifier_found_str = expected identifier, found `{$token}`

nfuzz_expected_mut_or_const_in_raw_pointer_type = expected `mut` or `const` keyword in raw pointer type
    .suggestion = add `mut` or `const` here

nfuzz_expected_semi_found_doc_comment_str = expected `;`, found doc comment `{$token}`
nfuzz_expected_semi_found_keyword_str = expected `;`, found keyword `{$token}`
# This one deliberately doesn't print a token.
nfuzz_expected_semi_found_metavar_str = expected `;`, found metavariable
nfuzz_expected_semi_found_reserved_identifier_str = expected `;`, found reserved identifier `{$token}`
nfuzz_expected_semi_found_reserved_keyword_str = expected `;`, found reserved keyword `{$token}`
nfuzz_expected_semi_found_str = expected `;`, found `{$token}`

nfuzz_expected_statement_after_outer_attr = expected statement after outer attribute

nfuzz_expected_struct_field = expected one of `,`, `:`, or `{"}"}`, found `{$token}`
    .label = expected one of `,`, `:`, or `{"}"}`
    .ident_label = while parsing this struct field

nfuzz_expected_trait_in_trait_impl_found_type = expected a trait, found type

nfuzz_expr_rarrow_call = `->` used for field access or method call
    .suggestion = try using `.` instead
    .help = the `.` operator will dereference the value if needed

nfuzz_extern_crate_name_with_dashes = crate name using dashes are not valid in `extern crate` statements
    .label = dash-separated idents are not valid
    .suggestion = if the original crate name uses dashes you need to use underscores in the code

nfuzz_extern_item_cannot_be_const = extern items cannot be `const`
    .suggestion = try using a static value
    .note = for more information, visit https://doc.rust-lang.org/std/keyword.extern.html

nfuzz_extra_if_in_let_else = remove the `if` if you meant to write a `let...else` statement

nfuzz_extra_impl_keyword_in_trait_impl = unexpected `impl` keyword
    .suggestion = remove the extra `impl`
    .note = this is nfuzzd as an `impl Trait` type, but a trait is expected at this position


nfuzz_field_expression_with_generic = field expressions cannot have generic arguments

nfuzz_float_literal_requires_integer_part = float literals must have an integer part
    .suggestion = must have an integer part

nfuzz_float_literal_unsupported_base = {$base} float literal is not supported

nfuzz_fn_pointer_cannot_be_async = an `fn` pointer type cannot be `async`
    .label = `async` because of this
    .suggestion = remove the `async` qualifier

nfuzz_fn_pointer_cannot_be_const = an `fn` pointer type cannot be `const`
    .label = `const` because of this
    .suggestion = remove the `const` qualifier

nfuzz_fn_ptr_with_generics = function pointer types may not have generic parameters
    .suggestion = consider moving the lifetime {$arity ->
        [one] parameter
        *[other] parameters
    } to {$for_param_list_exists ->
        [true] the
        *[false] a
    } `for` parameter list

nfuzz_fn_trait_missing_paren = `Fn` bounds require arguments in parentheses
    .add_paren = add the missing parentheses

nfuzz_forgot_paren = perhaps you forgot parentheses?

nfuzz_found_expr_would_be_stmt = expected expression, found `{$token}`
    .label = expected expression

nfuzz_function_body_equals_expr = function body cannot be `= expression;`
    .suggestion = surround the expression with `{"{"}` and `{"}"}` instead of `=` and `;`

nfuzz_generic_args_in_pat_require_turbofish_syntax = generic args in patterns require the turbofish syntax

nfuzz_generic_parameters_without_angle_brackets = generic parameters without surrounding angle brackets
    .suggestion = surround the type parameters with angle brackets

nfuzz_generics_in_path = unexpected generic arguments in path

nfuzz_help_set_edition_cargo = set `edition = "{$edition}"` in `Cargo.toml`
nfuzz_help_set_edition_standalone = pass `--edition {$edition}` to `rustc`
nfuzz_if_expression_missing_condition = missing condition for `if` expression
    .condition_label = expected condition here
    .block_label = if this block is the condition of the `if` expression, then it must be followed by another block

nfuzz_if_expression_missing_then_block = this `if` expression is missing a block after the condition
    .add_then_block = add a block here
    .condition_possibly_unfinished = this binary operation is possibly unfinished

nfuzz_in_in_typo =
    expected iterable, found keyword `in`
    .suggestion = remove the duplicated `in`

nfuzz_inappropriate_default = {$article} {$descr} cannot be `default`
    .label = `default` because of this
    .note = only associated `fn`, `const`, and `type` items can be `default`

nfuzz_inclusive_range_extra_equals = unexpected `=` after inclusive range
    .suggestion_remove_eq = use `..=` instead
    .note = inclusive ranges end with a single equals sign (`..=`)

nfuzz_inclusive_range_match_arrow = unexpected `>` after inclusive range
    .label = this is nfuzzd as an inclusive range `..=`
    .suggestion = add a space between the pattern and `=>`

nfuzz_inclusive_range_no_end = inclusive range with no end
    .suggestion_open_range = use `..` instead
    .note = inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

nfuzz_incorrect_parens_trait_bounds = incorrect parentheses around trait bounds
nfuzz_incorrect_parens_trait_bounds_sugg = fix the parentheses

nfuzz_incorrect_semicolon =
    expected item, found `;`
    .suggestion = remove this semicolon
    .help = {$name} declarations are not followed by a semicolon

nfuzz_incorrect_type_on_self = type not allowed for shorthand `self` parameter
    .suggestion = move the modifiers on `self` to the type

nfuzz_incorrect_use_of_await = incorrect use of `await`
    .parentheses_suggestion = `await` is not a method call, remove the parentheses

nfuzz_incorrect_use_of_await_postfix_suggestion = `await` is a postfix operation

nfuzz_incorrect_visibility_restriction = incorrect visibility restriction
    .help = some possible visibility restrictions are:
            `pub(crate)`: visible only on the current crate
            `pub(super)`: visible only in the current module's parent
            `pub(in path::to::module)`: visible only on the specified path
    .suggestion = make this visible only to module `{$inner_str}` with `in`

nfuzz_inner_attr_explanation = inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
nfuzz_inner_attr_not_permitted = an inner attribute is not permitted in this context
    .label_does_not_annotate_this = {nfuzz_label_inner_attr_does_not_annotate_this}
    .sugg_change_inner_to_outer = {nfuzz_sugg_change_inner_attr_to_outer}

nfuzz_inner_attr_not_permitted_after_outer_attr = an inner attribute is not permitted following an outer attribute
    .label_attr = not permitted following an outer attribute
    .label_prev_attr = previous outer attribute
    .label_does_not_annotate_this = {nfuzz_label_inner_attr_does_not_annotate_this}
    .sugg_change_inner_to_outer = {nfuzz_sugg_change_inner_attr_to_outer}

nfuzz_inner_attr_not_permitted_after_outer_doc_comment = an inner attribute is not permitted following an outer doc comment
    .label_attr = not permitted following an outer doc comment
    .label_prev_doc_comment = previous doc comment
    .label_does_not_annotate_this = {nfuzz_label_inner_attr_does_not_annotate_this}
    .sugg_change_inner_to_outer = {nfuzz_sugg_change_inner_attr_to_outer}

nfuzz_inner_doc_comment_not_permitted = expected outer doc comment
    .note = inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
    .suggestion = you might have meant to write a regular comment
    .label_does_not_annotate_this = the inner doc comment doesn't annotate this {$item}
    .sugg_change_inner_to_outer = to annotate the {$item}, change the doc comment from inner to outer style

nfuzz_invalid_attr_unsafe = `{$name}` is not an unsafe attribute
    .label = this is not an unsafe attribute
    .suggestion = remove the `unsafe(...)`
    .note = extraneous unsafe is not allowed in attributes

nfuzz_invalid_block_macro_segment = cannot use a `block` macro fragment here
    .label = the `block` fragment is within this context
    .suggestion = wrap this in another block

nfuzz_invalid_char_in_escape = {nfuzz_invalid_char_in_escape_msg}: `{$ch}`
    .label = {nfuzz_invalid_char_in_escape_msg}

nfuzz_invalid_char_in_escape_msg = invalid character in {$is_hex ->
    [true] numeric character
    *[false] unicode
    } escape


nfuzz_invalid_comparison_operator = invalid comparison operator `{$invalid}`
    .use_instead = `{$invalid}` is not a valid comparison operator, use `{$correct}`
    .spaceship_operator_invalid = `<=>` is not a valid comparison operator, use `std::cmp::Ordering`

nfuzz_invalid_curly_in_let_else = right curly brace `{"}"}` before `else` in a `let...else` statement not allowed
nfuzz_invalid_digit_literal = invalid digit for a base {$base} literal

nfuzz_invalid_dyn_keyword = invalid `dyn` keyword
    .help = `dyn` is only needed at the start of a trait `+`-separated list
    .suggestion = remove this keyword

nfuzz_invalid_expression_in_let_else = a `{$operator}` expression cannot be directly assigned in `let...else`
nfuzz_invalid_identifier_with_leading_number = identifiers cannot start with a number

nfuzz_invalid_label =
    invalid label name `{$name}`

nfuzz_invalid_literal_suffix_on_tuple_index = suffixes on a tuple index are invalid
    .label = invalid suffix `{$suffix}`
    .tuple_exception_line_1 = `{$suffix}` is *temporarily* accepted on tuple index fields as it was incorrectly accepted on stable for a few releases
    .tuple_exception_line_2 = on proc macros, you'll want to use `syn::Index::from` or `proc_macro::Literal::*_unsuffixed` for code that will desugar to tuple field access
    .tuple_exception_line_3 = see issue #60210 <https://github.com/rust-lang/rust/issues/60210> for more information

nfuzz_invalid_logical_operator = `{$incorrect}` is not a logical operator
    .note = unlike in e.g., Python and PHP, `&&` and `||` are used for logical operators
    .use_amp_amp_for_conjunction = use `&&` to perform logical conjunction
    .use_pipe_pipe_for_disjunction = use `||` to perform logical disjunction

nfuzz_invalid_meta_item = expected unsuffixed literal, found `{$token}`
    .quote_ident_sugg = surround the identifier with quotation marks to make it into a string literal

nfuzz_invalid_offset_of = offset_of expects dot-separated field and variant names

nfuzz_invalid_path_sep_in_fn_definition = invalid path separator in function definition
    .suggestion = remove invalid path separator

nfuzz_invalid_unicode_escape = invalid unicode character escape
    .label = invalid escape
    .help = unicode escape must {$surrogate ->
    [true] not be a surrogate
    *[false] be at most 10FFFF
    }

nfuzz_invalid_variable_declaration =
    invalid variable declaration

nfuzz_keyword_lifetime =
    lifetimes cannot use keyword names

nfuzz_kw_bad_case = keyword `{$kw}` is written in the wrong case
    .suggestion = write it in the correct case

nfuzz_label_inner_attr_does_not_annotate_this = the inner attribute doesn't annotate this {$item}
nfuzz_label_unexpected_token = unexpected token

nfuzz_label_while_parsing_or_pattern_here = while parsing this or-pattern starting here

nfuzz_labeled_loop_in_break = parentheses are required around this expression to avoid confusion with a labeled break expression

nfuzz_leading_plus_not_supported = leading `+` is not supported
    .label = unexpected `+`
    .suggestion_remove_plus = try removing the `+`

nfuzz_leading_underscore_unicode_escape = {nfuzz_leading_underscore_unicode_escape_label}: `_`
nfuzz_leading_underscore_unicode_escape_label = invalid start of unicode escape

nfuzz_left_arrow_operator = unexpected token: `<-`
    .suggestion = if you meant to write a comparison against a negative value, add a space in between `<` and `-`

nfuzz_lifetime_after_mut = lifetime must precede `mut`
    .suggestion = place the lifetime before `mut`

nfuzz_lifetime_in_borrow_expression = borrow expressions cannot be annotated with lifetimes
    .suggestion = remove the lifetime annotation
    .label = annotated with lifetime here

nfuzz_lifetime_in_eq_constraint = lifetimes are not permitted in this context
    .label = lifetime is not allowed here
    .context_label = this introduces an associated item binding
    .help = if you meant to specify a trait object, write `dyn /* Trait */ + {$lifetime}`
    .colon_sugg = you might have meant to write a bound here

nfuzz_lone_slash = invalid trailing slash in literal
    .label = {nfuzz_lone_slash}

nfuzz_loop_else = `{$loop_kind}...else` loops are not supported
    .note = consider moving this `else` clause to a separate `if` statement and use a `bool` variable to control if it should run
    .loop_keyword = `else` is attached to this loop

nfuzz_macro_expands_to_adt_field = macros cannot expand to {$adt_ty} fields

nfuzz_macro_expands_to_enum_variant = macros cannot expand to enum variants

nfuzz_macro_invocation_visibility = can't qualify macro invocation with `pub`
    .suggestion = remove the visibility
    .help = try adjusting the macro to put `{$vis}` inside the invocation

nfuzz_macro_invocation_with_qualified_path = macros cannot use qualified paths

nfuzz_macro_name_remove_bang = macro names aren't followed by a `!`
    .suggestion = remove the `!`

nfuzz_macro_rules_missing_bang = expected `!` after `macro_rules`
    .suggestion = add a `!`

nfuzz_macro_rules_visibility = can't qualify macro_rules invocation with `{$vis}`
    .suggestion = try exporting the macro

nfuzz_malformed_cfg_attr = malformed `cfg_attr` attribute input
    .suggestion = missing condition and attribute
    .note = for more information, visit <https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute>

nfuzz_malformed_loop_label = malformed loop label
    .suggestion = use the correct loop label format

nfuzz_match_arm_body_without_braces = `match` arm body without braces
    .label_statements = {$num_statements ->
            [one] this statement is not surrounded by a body
           *[other] these statements are not surrounded by a body
        }
    .label_arrow = while parsing the `match` arm starting here
    .suggestion_add_braces = surround the {$num_statements ->
            [one] statement
           *[other] statements
        } with a body
    .suggestion_use_comma_not_semicolon = replace `;` with `,` to end a `match` arm expression

nfuzz_maybe_comparison = you might have meant to compare for equality

nfuzz_maybe_fn_typo_with_impl = you might have meant to write `impl` instead of `fn`
    .suggestion = replace `fn` with `impl` here

nfuzz_maybe_missing_let = you might have meant to continue the let-chain

nfuzz_maybe_recover_from_bad_qpath_stage_2 =
    missing angle brackets in associated item path
    .suggestion = types that don't start with an identifier need to be surrounded with angle brackets in qualified paths

nfuzz_maybe_recover_from_bad_type_plus =
    expected a path on the left-hand side of `+`, not `{$ty}`

nfuzz_maybe_report_ambiguous_plus =
    ambiguous `+` in a type
    .suggestion = use parentheses to disambiguate

nfuzz_meta_bad_delim = wrong meta list delimiters
nfuzz_meta_bad_delim_suggestion = the delimiters should be `(` and `)`

nfuzz_mismatched_closing_delimiter = mismatched closing delimiter: `{$delimiter}`
    .label_unmatched = mismatched closing delimiter
    .label_opening_candidate = closing delimiter possibly meant for this
    .label_unclosed = unclosed delimiter

nfuzz_misplaced_return_type = place the return type after the function parameters

nfuzz_missing_comma_after_match_arm = expected `,` following `match` arm
    .suggestion = missing a comma here to end this `match` arm

nfuzz_missing_const_type = missing type for `{$kind}` item
    .suggestion = provide a type for the item

nfuzz_missing_enum_for_enum_definition = missing `enum` for enum definition
    .suggestion = add `enum` here to nfuzz `{$ident}` as an enum

nfuzz_missing_enum_or_struct_for_item_definition = missing `enum` or `struct` for enum or struct definition

nfuzz_missing_expression_in_for_loop = missing expression to iterate on in `for` loop
    .suggestion = try adding an expression to the `for` loop

nfuzz_missing_fn_for_function_definition = missing `fn` for function definition
    .suggestion = add `fn` here to nfuzz `{$ident}` as a function

nfuzz_missing_fn_for_method_definition = missing `fn` for method definition
    .suggestion = add `fn` here to nfuzz `{$ident}` as a method

nfuzz_missing_fn_or_struct_for_item_definition = missing `fn` or `struct` for function or struct definition
    .suggestion = if you meant to call a macro, try
    .help = if you meant to call a macro, remove the `pub` and add a trailing `!` after the identifier

nfuzz_missing_fn_params = missing parameters for function definition
    .suggestion = add a parameter list

nfuzz_missing_for_in_trait_impl = missing `for` in a trait impl
    .suggestion = add `for` here

nfuzz_missing_in_in_for_loop = missing `in` in `for` loop
    .use_in_not_of = try using `in` here instead
    .add_in = try adding `in` here

nfuzz_missing_let_before_mut = missing keyword
nfuzz_missing_plus_in_bounds = expected `+` between lifetime and {$sym}
    .suggestion = add `+`

nfuzz_missing_semicolon_before_array = expected `;`, found `[`
    .suggestion = consider adding `;` here

nfuzz_missing_struct_for_struct_definition = missing `struct` for struct definition
    .suggestion = add `struct` here to nfuzz `{$ident}` as a struct

nfuzz_missing_trait_in_trait_impl = missing trait in a trait impl
    .suggestion_add_trait = add a trait here
    .suggestion_remove_for = for an inherent impl, drop this `for`

nfuzz_misspelled_kw = {$is_incorrect_case ->
                    [true] write keyword `{$similar_kw}` in lowercase
                    *[false] there is a keyword `{$similar_kw}` with a similar name
}

nfuzz_modifier_lifetime = `{$modifier}` may only modify trait bounds, not lifetime bounds
    .suggestion = remove the `{$modifier}`

nfuzz_modifiers_and_polarity = `{$modifiers_concatenated}` trait not allowed with `{$polarity}` trait polarity modifier
    .label = there is not a well-defined meaning for a `{$modifiers_concatenated} {$polarity}` trait

nfuzz_more_than_one_char = character literal may only contain one codepoint
    .followed_by = this `{$chr}` is followed by the combining {$len ->
        [one] mark
        *[other] marks
        } `{$escaped_marks}`
    .non_printing = there are non-printing characters, the full sequence is `{$escaped}`
    .consider_normalized = consider using the normalized form `{$ch}` of this character
    .remove_non = consider removing the non-printing characters
    .use_double_quotes = if you meant to write a {$is_byte ->
        [true] byte string
        *[false] string
        } literal, use double quotes

nfuzz_multiple_skipped_lines = multiple lines skipped by escaped newline
    .label = skipping everything up to and including this point

nfuzz_multiple_where_clauses = cannot define duplicate `where` clauses on an item
    .label = previous `where` clause starts here
    .suggestion = consider joining the two `where` clauses into one

nfuzz_mut_on_nested_ident_pattern = `mut` must be attached to each individual binding
    .suggestion = add `mut` to each binding
nfuzz_mut_on_non_ident_pattern = `mut` must be followed by a named binding
    .suggestion = remove the `mut` prefix
nfuzz_need_plus_after_trait_object_lifetime = lifetime in trait object type must be followed by `+`

nfuzz_nested_adt = `{$kw_str}` definition cannot be nested inside `{$keyword}`
    .suggestion = consider creating a new `{$kw_str}` definition instead of nesting

nfuzz_nested_c_variadic_type = C-variadic type `...` may not be nested inside another type

nfuzz_no_brace_unicode_escape = incorrect unicode escape sequence
    .label = {nfuzz_no_brace_unicode_escape}
    .use_braces = format of unicode escape sequences uses braces
    .format_of_unicode = format of unicode escape sequences is `\u{"{...}"}`

nfuzz_no_digits_literal = no valid digits found for number

nfuzz_non_string_abi_literal = non-string ABI literal
    .suggestion = specify the ABI with a string literal

nfuzz_nonterminal_expected_ident = expected ident, found `{$token}`
nfuzz_nonterminal_expected_item_keyword = expected an item keyword
nfuzz_nonterminal_expected_lifetime = expected a lifetime, found `{$token}`

nfuzz_nonterminal_expected_statement = expected a statement

nfuzz_note_edition_guide = for more on editions, read https://doc.rust-lang.org/edition-guide

nfuzz_note_mut_pattern_usage = `mut` may be followed by `variable` and `variable @ pattern`

nfuzz_note_pattern_alternatives_use_single_vert = alternatives in or-patterns are separated with `|`, not `||`

nfuzz_nul_in_c_str = null characters in C string literals are not supported

nfuzz_or_pattern_not_allowed_in_fn_parameters = top-level or-patterns are not allowed in function parameters
nfuzz_or_pattern_not_allowed_in_let_binding = top-level or-patterns are not allowed in `let` bindings
nfuzz_out_of_range_hex_escape = out of range hex escape
    .label = must be a character in the range [\x00-\x7f]

nfuzz_outer_attr_explanation = outer attributes, like `#[test]`, annotate the item following them

nfuzz_outer_attribute_not_allowed_on_if_else = outer attributes are not allowed on `if` and `else` branches
    .branch_label = the attributes are attached to this branch
    .ctx_label = the branch belongs to this `{$ctx}`
    .suggestion = remove the attributes

nfuzz_overlong_unicode_escape = overlong unicode escape
    .label = must have at most 6 hex digits

nfuzz_parentheses_with_struct_fields = invalid `struct` delimiters or `fn` call arguments
    .suggestion_braces_for_struct = if `{$type}` is a struct, use braces as delimiters
    .suggestion_no_fields_for_fn = if `{$type}` is a function, use the arguments directly

nfuzz_parenthesized_lifetime = parenthesized lifetime bounds are not supported
nfuzz_parenthesized_lifetime_suggestion = remove the parentheses

nfuzz_path_double_colon = path separator must be a double colon
    .suggestion = use a double colon instead

nfuzz_pattern_method_param_without_body = patterns aren't allowed in methods without bodies
    .suggestion = give this argument a name or use an underscore to ignore it

nfuzz_pattern_on_wrong_side_of_at = pattern on wrong side of `@`
    .label_pattern = pattern on the left, should be on the right
    .label_binding = binding on the right, should be on the left
    .suggestion = switch the order

nfuzz_question_mark_in_type = invalid `?` in type
    .label = `?` is only allowed on expressions, not types
    .suggestion = if you meant to express that the type might not contain a value, use the `Option` wrapper type

nfuzz_recover_import_as_use = expected item, found {$token_name}
    .suggestion = items are imported using the `use` keyword

nfuzz_remove_let = expected pattern, found `let`
    .suggestion = remove the unnecessary `let` keyword

nfuzz_repeated_mut_in_pattern = `mut` on a binding may not be repeated
    .suggestion = remove the additional `mut`s

nfuzz_require_colon_after_labeled_expression = labeled expression must be followed by `:`
    .note = labels are used before loops and blocks, allowing e.g., `break 'label` to them
    .label = the label
    .suggestion = add `:` after the label

nfuzz_reserved_multihash = reserved multi-hash token is forbidden
    .note = sequences of two or more # are reserved for future use since Rust 2024
    .suggestion_whitespace = consider inserting whitespace here

nfuzz_reserved_string = invalid string literal
    .note = unprefixed guarded string literals are reserved for future use since Rust 2024
    .suggestion_whitespace = consider inserting whitespace here

nfuzz_return_types_use_thin_arrow = return types are denoted using `->`
    .suggestion = use `->` instead

nfuzz_self_argument_pointer = cannot pass `self` by raw pointer
    .label = cannot pass `self` by raw pointer

nfuzz_self_param_not_first = unexpected `self` parameter in function
    .label = must be the first parameter of an associated function

nfuzz_shift_interpreted_as_generic =
    `<<` is interpreted as a start of generic arguments for `{$type}`, not a shift
    .label_args = interpreted as generic arguments
    .label_comparison = not interpreted as shift
    .suggestion = try shifting the cast value

nfuzz_single_colon_import_path = expected `::`, found `:`
    .suggestion = use double colon
    .note = import paths are delimited using `::`

nfuzz_single_colon_struct_type = found single colon in a struct field type path
    .suggestion = write a path separator here

nfuzz_static_with_generics = static items may not have generic parameters

nfuzz_struct_literal_body_without_path =
    struct literal body without path
    .suggestion = you might have forgotten to add the struct literal inside the block

nfuzz_struct_literal_needing_parens =
    invalid struct literal
    .suggestion = you might need to surround the struct literal with parentheses

nfuzz_struct_literal_not_allowed_here = struct literals are not allowed here
    .suggestion = surround the struct literal with parentheses

nfuzz_suffixed_literal_in_attribute = suffixed literals are not allowed in attributes
    .help = instead of using a suffixed literal (`1u8`, `1.0f32`, etc.), use an unsuffixed version (`1`, `1.0`, etc.)

nfuzz_sugg_add_let_for_stmt = you might have meant to introduce a new binding

nfuzz_sugg_add_semi = add `;` here
nfuzz_sugg_change_inner_attr_to_outer = to annotate the {$item}, change the attribute from inner to outer style

nfuzz_sugg_change_this_to_semi = change this to `;`
nfuzz_sugg_escape_identifier = escape `{$ident_name}` to use it as an identifier

nfuzz_sugg_remove_comma = remove this comma
nfuzz_sugg_remove_leading_vert_in_pattern = remove the `|`
nfuzz_sugg_turbofish_syntax = use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments

nfuzz_sugg_wrap_expression_in_parentheses = wrap the expression in parentheses

nfuzz_sugg_wrap_macro_in_parentheses = use parentheses instead of braces for this macro

nfuzz_sugg_wrap_pattern_in_parens = wrap the pattern in parentheses

nfuzz_switch_mut_let_order =
    switch the order of `mut` and `let`

nfuzz_switch_ref_box_order = switch the order of `ref` and `box`
    .suggestion = swap them

nfuzz_ternary_operator = Rust has no ternary operator
    .help = use an `if-else` expression instead

nfuzz_tilde_is_not_unary_operator = `~` cannot be used as a unary operator
    .suggestion = use `!` to perform bitwise not

nfuzz_too_many_hashes = too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols, but found {$num}

nfuzz_too_short_hex_escape = numeric character escape is too short

nfuzz_trailing_vert_not_allowed = a trailing `|` is not allowed in an or-pattern
    .suggestion = remove the `{$token}`

nfuzz_trait_alias_cannot_be_auto = trait aliases cannot be `auto`
nfuzz_trait_alias_cannot_be_unsafe = trait aliases cannot be `unsafe`

nfuzz_transpose_dyn_or_impl = `for<...>` expected after `{$kw}`, not before
    .suggestion = move `{$kw}` before the `for<...>`

nfuzz_type_ascription_removed =
    if you meant to annotate an expression with a type, the type ascription syntax has been removed, see issue #101728 <https://github.com/rust-lang/rust/issues/101728>

nfuzz_unclosed_unicode_escape = unterminated unicode escape
    .label = missing a closing `{"}"}`
    .terminate = terminate the unicode escape

nfuzz_underscore_literal_suffix = underscore literal suffix is not allowed

nfuzz_unexpected_const_in_generic_param = expected lifetime, type, or constant, found keyword `const`
    .suggestion = the `const` keyword is only needed in the definition of the type

nfuzz_unexpected_const_param_declaration = unexpected `const` parameter declaration
    .label = expected a `const` expression, not a parameter declaration
    .suggestion = `const` parameters must be declared for the `impl`

nfuzz_unexpected_default_value_for_lifetime_in_generic_parameters = unexpected default lifetime parameter
    .label = lifetime parameters cannot have default values

nfuzz_unexpected_expr_in_pat =
    expected {$is_bound ->
        [true] a pattern range bound
       *[false] a pattern
    }, found an expression

    .label = not a pattern
    .note = arbitrary expressions are not allowed in patterns: <https://doc.rust-lang.org/book/ch19-00-patterns.html>

nfuzz_unexpected_expr_in_pat_const_sugg = consider extracting the expression into a `const`

nfuzz_unexpected_expr_in_pat_create_guard_sugg = consider moving the expression to a match arm guard

nfuzz_unexpected_expr_in_pat_inline_const_sugg = consider wrapping the expression in an inline `const` (requires `{"#"}![feature(inline_const_pat)]`)

nfuzz_unexpected_expr_in_pat_update_guard_sugg = consider moving the expression to the match arm guard

nfuzz_unexpected_if_with_if = unexpected `if` in the condition expression
    .suggestion = remove the `if`

nfuzz_unexpected_lifetime_in_pattern = unexpected lifetime `{$symbol}` in pattern
    .suggestion = remove the lifetime

nfuzz_unexpected_paren_in_range_pat = range pattern bounds cannot have parentheses
nfuzz_unexpected_paren_in_range_pat_sugg = remove these parentheses

nfuzz_unexpected_parentheses_in_for_head = unexpected parentheses surrounding `for` loop head
    .suggestion = remove parentheses in `for` loop

nfuzz_unexpected_parentheses_in_match_arm_pattern = unexpected parentheses surrounding `match` arm pattern
    .suggestion = remove parentheses surrounding the pattern

nfuzz_unexpected_self_in_generic_parameters = unexpected keyword `Self` in generic parameters
    .note = you cannot use `Self` as a generic parameter because it is reserved for associated items

nfuzz_unexpected_token_after_dot = unexpected token: `{$actual}`

nfuzz_unexpected_token_after_label = expected `while`, `for`, `loop` or `{"{"}` after a label
    .suggestion_remove_label = consider removing the label
    .suggestion_enclose_in_block = consider enclosing expression in a block

nfuzz_unexpected_token_after_not = unexpected {$negated_desc} after identifier
nfuzz_unexpected_token_after_not_bitwise = use `!` to perform bitwise not
nfuzz_unexpected_token_after_not_default = use `!` to perform logical negation or bitwise not

nfuzz_unexpected_token_after_not_logical = use `!` to perform logical negation
nfuzz_unexpected_token_after_struct_name = expected `where`, `{"{"}`, `(`, or `;` after struct name
nfuzz_unexpected_token_after_struct_name_found_doc_comment = expected `where`, `{"{"}`, `(`, or `;` after struct name, found doc comment `{$token}`
nfuzz_unexpected_token_after_struct_name_found_keyword = expected `where`, `{"{"}`, `(`, or `;` after struct name, found keyword `{$token}`
# This one deliberately doesn't print a token.
nfuzz_unexpected_token_after_struct_name_found_metavar = expected `where`, `{"{"}`, `(`, or `;` after struct name, found metavar
nfuzz_unexpected_token_after_struct_name_found_other = expected `where`, `{"{"}`, `(`, or `;` after struct name, found `{$token}`

nfuzz_unexpected_token_after_struct_name_found_reserved_identifier = expected `where`, `{"{"}`, `(`, or `;` after struct name, found reserved identifier `{$token}`
nfuzz_unexpected_token_after_struct_name_found_reserved_keyword = expected `where`, `{"{"}`, `(`, or `;` after struct name, found reserved keyword `{$token}`
nfuzz_unexpected_vert_vert_before_function_parameter = unexpected `||` before function parameter
    .suggestion = remove the `||`

nfuzz_unexpected_vert_vert_in_pattern = unexpected token `||` in pattern
    .suggestion = use a single `|` to separate multiple alternative patterns

nfuzz_unicode_escape_in_byte = unicode escape in byte string
    .label = {nfuzz_unicode_escape_in_byte}
    .help = unicode escape sequences cannot be used as a byte or in a byte string

nfuzz_unknown_builtin_construct = unknown `builtin #` construct `{$name}`

nfuzz_unknown_prefix = prefix `{$prefix}` is unknown
    .label = unknown prefix
    .note =  prefixed identifiers and literals are reserved since Rust 2021
    .suggestion_br = use `br` for a raw byte string
    .suggestion_str = if you meant to write a string literal, use double quotes
    .suggestion_whitespace = consider inserting whitespace here

nfuzz_unknown_start_of_token = unknown start of token: {$escaped}
    .sugg_quotes = Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '{$ascii_str}' ({$ascii_name}), but are not
    .sugg_other = Unicode character '{$ch}' ({$u_name}) looks like '{$ascii_str}' ({$ascii_name}), but it is not
    .help_null = source files must contain UTF-8 encoded text, unexpected null bytes might occur when a different encoding is used
    .note_repeats = character appears {$repeats ->
        [one] once more
        *[other] {$repeats} more times
    }

nfuzz_unmatched_angle = unmatched angle {$plural ->
    [true] brackets
    *[false] bracket
    }
    .suggestion = remove extra angle {$plural ->
    [true] brackets
    *[false] bracket
    }

nfuzz_unmatched_angle_brackets = {$num_extra_brackets ->
        [one] unmatched angle bracket
       *[other] unmatched angle brackets
    }
    .suggestion = {$num_extra_brackets ->
            [one] remove extra angle bracket
           *[other] remove extra angle brackets
        }

nfuzz_unsafe_attr_outside_unsafe = unsafe attribute used without unsafe
    .label = usage of unsafe attribute
nfuzz_unsafe_attr_outside_unsafe_suggestion = wrap the attribute in `unsafe(...)`


nfuzz_unskipped_whitespace = whitespace symbol '{$ch}' is not skipped
    .label = {nfuzz_unskipped_whitespace}

nfuzz_use_empty_block_not_semi = expected { "`{}`" }, found `;`
    .suggestion = try using { "`{}`" } instead

nfuzz_use_eq_instead = unexpected `==`
    .suggestion = try using `=` instead

nfuzz_use_let_not_auto = write `let` instead of `auto` to introduce a new variable
nfuzz_use_let_not_var = write `let` instead of `var` to introduce a new variable

nfuzz_visibility_not_followed_by_item = visibility `{$vis}` is not followed by an item
    .label = the visibility
    .help = you likely meant to define an item, e.g., `{$vis} fn foo() {"{}"}`

nfuzz_where_clause_before_const_body = where clauses are not allowed before const item bodies
    .label = unexpected where clause
    .name_label = while parsing this const item
    .body_label = the item body
    .suggestion = move the body before the where clause

nfuzz_where_clause_before_tuple_struct_body = where clauses are not allowed before tuple struct bodies
    .label = unexpected where clause
    .name_label = while parsing this tuple struct
    .body_label = the struct body
    .suggestion = move the body before the where clause

nfuzz_where_generics = generic parameters on `where` clauses are reserved for future use
    .label = currently unsupported

nfuzz_zero_chars = empty character literal
    .label = {nfuzz_zero_chars}
