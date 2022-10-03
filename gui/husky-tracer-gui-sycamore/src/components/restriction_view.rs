use super::*;
use sycamore::render;
use web_sys::{Event, HtmlDialogElement, HtmlFormElement, HtmlInputElement, KeyboardEvent};

#[component]
pub fn RestrictionView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let dev_context = use_dev_context(scope);
    let restriction_context = &dev_context.restriction_context;
    let generic = create_signal(scope, true);
    let restriction = restriction_context.presentation;
    let last_sample_id = create_signal(scope, restriction.get_untracked().opt_sample_id());
    let toggle_restriction_kind_handler = dev_context.toggle_restriction_kind_handler();
    let restriction_kind = memo!(
        scope,
        move || match restriction.get().is_specific() {
            true => "SPECIFIC",
            false => "GENERIC",
        }
        .to_string(),
        restriction
    );
    view! {
        scope,
        div (class="RestrictionView disable-select") {
            div (
                class="RestrictionKind",
                on:click=toggle_restriction_kind_handler
            ) {
                (restriction_kind.get())
            }
             label (
                id="sample-id-name",
            ) {
                "sample id = "
            }
            label (
                id="sample-id-value",
                on:click=dev_context.set_restriction_from_dialog_handler()
            ) {
                (restriction.get().sample_id().0)
            }
        }
    }
}
