use sycamore::prelude::*;

#[component(inline_props)]
pub fn Counter<'a, G: Html>(cx: Scope<'a>, state: &'a Signal<i32>) -> View<G> {
    let double = create_memo(cx, || *state.get() * 2);
    view! { cx,
        button(on:click=|_| {state.set(*state.get() + 1)}) {
                (state.get()) " double is " (double.get())
            }
    }
}
