use sycamore::prelude::*;

#[derive(Prop)]
pub struct MyProps<'a> {
    pub state: &'a Signal<i32>,
}

#[component]
pub fn Counter<'a, G: Html>(cx: Scope<'a>, props: MyProps<'a>) -> View<G> {
    let double = create_memo(cx, || *props.state.get() * 2);
    view! { cx,
        button(on:click=|_| {props.state.set(*props.state.get() + 1)}) {
                (props.state.get()) " double is " (double.get())
            }
    }
}
