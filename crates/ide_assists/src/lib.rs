//! `assists` crate provides a bunch of code assists, also known as code actions
//! (in LSP) or intentions (in IntelliJ).
//!
//! An assist is a micro-refactoring, which is automatically activated in
//! certain context. For example, if the cursor is over `,`, a "swap `,`" assist
//! becomes available.
//!
//! ## Assists Guidelines
//!
//! Assists are the main mechanism to deliver advanced IDE features to the user,
//! so we should pay extra attention to the UX.
//!
//! The power of assists comes from their context-awareness. The main problem
//! with IDE features is that there are a lot of them, and it's hard to teach
//! the user what's available. Assists solve this problem nicely: 💡 signifies
//! that *something* is possible, and clicking on it reveals a *short* list of
//! actions. Contrast it with Emacs `M-x`, which just spits an infinite list of
//! all the features.
//!
//! Here are some considerations when creating a new assist:
//!
//! * It's good to preserve semantics, and it's good to keep the code compiling,
//!   but it isn't necessary. Example: "flip binary operation" might change
//!   semantics.
//! * Assist shouldn't necessary make the code "better". A lot of assist come in
//!   pairs: "if let <-> match".
//! * Assists should have as narrow scope as possible. Each new assists greatly
//!   improves UX for cases where the user actually invokes it, but it makes UX
//!   worse for every case where the user clicks 💡 to invoke some *other*
//!   assist. So, a rarely useful assist which is always applicable can be a net
//!   negative.
//! * Rarely useful actions are tricky. Sometimes there are features which are
//!   clearly useful to some users, but are just noise most of the time. We
//!   don't have a good solution here, our current approach is to make this
//!   functionality available only if assist is applicable to the whole
//!   selection. Example: `sort_items` sorts items alphabetically. Naively, it
//!   should be available more or less everywhere, which isn't useful. So
//!   instead we only show it if the user *selects* the items they want to sort.
//! * Consider grouping related assists together (see [`Assists::add_group`]).
//! * Make assists robust. If the assist depends on results of type-inference to
//!   much, it might only fire in fully-correct code. This makes assist less
//!   useful and (worse) less predictable. The user should have a clear
//!   intuition when each particular assist is available.
//! * Make small assists, which compose. Example: rather than auto-importing
//!   enums in `fill_match_arms`, we use fully-qualified names. There's a
//!   separate assist to shorten a fully-qualified name.
//! * Distinguish between assists and fixits for diagnostics. Internally, fixits
//!   and assists are equivalent. They have the same "show a list + invoke a
//!   single element" workflow, and both use [`Assist`] data structure. The main
//!   difference is in the UX: while 💡 looks only at the cursor position,
//!   diagnostics squigglies and fixits are calculated for the whole file and
//!   are presented to the user eagerly. So, diagnostics should be fixable
//!   errors, while assists can be just suggestions for an alternative way to do
//!   something. If something *could* be a diagnostic, it should be a
//!   diagnostic. Conversely, it might be valuable to turn a diagnostic with a
//!   lot of false errors into an assist.
//! *
//!
//! See also this post:
//! <https://husky-lang-server.github.io/blog/2020/09/28/how-to-make-a-light-bulb.html>
#![allow(unused, dead_code)]
macro_rules! eprintln {
    ($($tt:tt)*) => { stdx::eprintln!($($tt)*) };
}

mod assist_config;
mod assist_context;
pub mod utils;

use common::*;

use hir::Semantics;
use ide_db::{file_db::FileRange, IdeDatabase};

pub(crate) use crate::assist_context::{AssistContext, Assists};

pub use assist_config::AssistConfig;
pub use ide_db::assists::{
    Assist, AssistId, AssistKind, AssistResolveStrategy, GroupLabel, SingleResolve,
};

/// Return all the assists applicable at the given position.
///
// NOTE: We don't have a `Feature: ` section for assists, they are special-cased
// in the manual.
pub fn assists(
    db: &IdeDatabase,
    config: &AssistConfig,
    resolve: AssistResolveStrategy,
    range: FileRange,
) -> Vec<Assist> {
    todo!()
}

mod handlers {
    use crate::{AssistContext, Assists};

    pub(crate) type Handler = fn(&mut Assists, &AssistContext) -> Option<()>;

    mod add_explicit_type;
    mod add_missing_impl_members;
    mod add_missing_match_arms;
    mod add_return_type;
    mod add_turbo_fish;
    mod apply_demorgan;
    mod auto_import;
    mod change_visibility;
    mod convert_bool_then;
    mod convert_comment_block;
    mod convert_integer_literal;
    mod convert_into_to_from;
    mod convert_iter_for_each_to_for;
    mod convert_to_guarded_return;
    mod convert_tuple_struct_to_named_struct;
    mod convert_while_to_loop;
    mod destructure_tuple_binding;
    mod expand_glob_import;
    mod extract_function;
    mod extract_module;
    mod extract_struct_from_enum_variant;
    mod extract_type_alias;
    mod extract_variable;
    mod fix_visibility;
    mod flip_binexpr;
    mod flip_comma;
    mod inline_call;
    mod inline_local_variable;
    mod sort_items;

    pub(crate) fn all() -> &'static [Handler] {
        &[
            // These are alphabetic for the foolish consistency
            add_explicit_type::add_explicit_type,
            add_missing_match_arms::add_missing_match_arms,
            add_return_type::add_return_type,
            add_turbo_fish::add_turbo_fish,
            apply_demorgan::apply_demorgan,
            auto_import::auto_import,
            change_visibility::change_visibility,
            convert_bool_then::convert_bool_then_to_if,
            convert_bool_then::convert_if_to_bool_then,
            convert_comment_block::convert_comment_block,
            convert_integer_literal::convert_integer_literal,
            convert_into_to_from::convert_into_to_from,
            convert_iter_for_each_to_for::convert_iter_for_each_to_for,
            convert_iter_for_each_to_for::convert_for_loop_with_for_each,
            convert_to_guarded_return::convert_to_guarded_return,
            convert_tuple_struct_to_named_struct::convert_tuple_struct_to_named_struct,
            convert_while_to_loop::convert_while_to_loop,
            destructure_tuple_binding::destructure_tuple_binding,
            expand_glob_import::expand_glob_import,
            extract_struct_from_enum_variant::extract_struct_from_enum_variant,
            extract_type_alias::extract_type_alias,
            fix_visibility::fix_visibility,
            flip_binexpr::flip_binexpr,
            flip_comma::flip_comma,
            inline_call::inline_call,
            inline_call::inline_into_callers,
            inline_local_variable::inline_local_variable,
            sort_items::sort_items,
            // These are manually sorted for better priorities. By default,
            // priority is determined by the size of the target range (smaller
            // target wins). If the ranges are equal, position in this list is
            // used as a tie-breaker.
            add_missing_impl_members::add_missing_impl_members,
            add_missing_impl_members::add_missing_default_members,
            //
            extract_variable::extract_variable,
            extract_function::extract_function,
            extract_module::extract_module,
            // Are you sure you want to add new assist here, and not to the
            // sorted list above?
        ]
    }
}
