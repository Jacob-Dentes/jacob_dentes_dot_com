use dioxus::prelude::*;

const CV_ASSET: Asset = asset!("/assets/cv.pdf");

#[component]
pub fn CV() -> Element {
    rsx! {
        div {
            id: "cv",
            iframe {
                src: CV_ASSET,
            }
        }

    }
}