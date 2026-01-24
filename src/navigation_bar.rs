use dioxus::prelude::*;

use crate::states::*;

#[component]
pub fn Navigation() -> Element {
    let navigation_options = {
        use_context::<NavigationState>().state().read().clone()
    };

    rsx! {
        div {
            id: "navigation",
            class: "h2",

            if &navigation_options == &NavigationOptions::Home {
                a { class: "highlighted", "Home" }
            } else {
                button {
                    class: "linklike",
                    onclick: move |_| consume_context::<NavigationState>()
                        .state_mut().set(NavigationOptions::Home),
                    "Home"
                }
            },

            if &navigation_options == &NavigationOptions::Publications {
                a { class: "highlighted", "Publications" }
            } else {
                button {
                    class: "linklike",
                    onclick: move |_| consume_context::<NavigationState>()
                        .state_mut().set(NavigationOptions::Publications),
                    "Publications"
                }
            },

            if &navigation_options == &NavigationOptions::CV {
                a { class: "highlighted", "CV" }
            } else {
                button {
                    class: "linklike",
                    onclick: move |_| consume_context::<NavigationState>()
                        .state_mut().set(NavigationOptions::CV),
                    "CV"
                }
            },
        }
    }
}