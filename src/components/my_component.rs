use sycamore::prelude::*;

#[component]
pub fn MyComponent<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "Im the real deal"
        }
    }
}
