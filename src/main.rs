use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/jd_logo.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");

mod home;
mod publications;
mod cv;
mod navigation_bar;
mod states;



use states::*;
use navigation_bar::Navigation;

use home::Home;
use publications::Publications;
use cv::CV;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let state_options = use_signal(|| NavigationOptions::default());
    use_context_provider(|| NavigationState::new(state_options));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    let navigation_options = {
        use_context::<NavigationState>().state().read().clone()
    };

    rsx! {
        div {
            id: "hero",
            Navigation {},
            div { id: "navigation-separation" },

            if &navigation_options == &NavigationOptions::Home {
                Home {}
            }

            if &navigation_options == &NavigationOptions::Publications {
                Publications {}
            }

            if &navigation_options == &NavigationOptions::CV {
                CV {}
            }
        }
    }
}
