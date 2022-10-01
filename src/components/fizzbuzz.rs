use sycamore::prelude::*;

#[component]
pub fn FizzBuzz<G: Html>(cx: Scope) -> View<G> {
    let user_input = create_signal(cx, String::new());
    let have_input = create_selector(cx, || !user_input.get().is_empty());
    let fizz = create_selector(cx, || {
        if *have_input.get() {
            user_input.get().parse::<i32>().unwrap() % 3 == 0
        } else {
            false
        }
    });
    let buzz = create_selector(cx, || {
        if *have_input.get() {
            user_input.get().parse::<i32>().unwrap() % 5 == 0
        } else {
            false
        }
    });
    let fizz_buzz = create_selector(cx, || {
        if *have_input.get() {
            user_input.get().parse::<i32>().unwrap() % 15 == 0
        } else {
            false
        }
    });
    view! { cx,
        div {
            input(bind:value=user_input, type="number", placeholder="Enter a value")
            h1 {
                (
                    if *fizz_buzz.get() {
                        view! { cx, strong { "FizzBuzz" } }
                    } else if *buzz.get() {
                        view! { cx, strong {"Buzz"} }
                    } else if *fizz.get() {
                        view!(cx, strong{ "Fizz"})
                    } else {
                        view!(cx, p {"Nothing Crazy"})
                    }

                )
            }
        }
    }
}
