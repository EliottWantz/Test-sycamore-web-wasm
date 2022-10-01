use sycamore::prelude::*;

#[derive(Prop)]
pub struct MyProps {
    pub name: String,
    pub email: String,
}

#[component]
pub fn MyComponent<G: Html>(cx: Scope, props: MyProps) -> View<G> {
    view! { cx,
        p {
            "My name is " (props.name)
        }
        p {
            "Email is " (props.email)
        }
    }
}
