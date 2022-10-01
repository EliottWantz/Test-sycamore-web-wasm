use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let state = create_signal(cx, 0);
        let double = create_memo(cx, || *state.get() * 2);

        create_effect(cx, || {
            println!(
                "State changed. New state value = {}, double is = {}",
                state.get(),
                *double.get()
            );
        });

        view! { cx,
            button(on:click=|_| {state.set(*state.get() + 1)}) {
                (state.get()) " double is " (double.get())
            }
        }
    });
}
