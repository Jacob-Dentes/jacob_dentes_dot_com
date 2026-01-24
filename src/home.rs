use dioxus::prelude::*;

const HEADSHOT: Asset = asset!("/assets/headshot.png");
const LINKEDIN: Asset = asset!("/assets/linkedin_logo.png");
const GOOGLE: Asset = asset!("/assets/scholar_logo.png");
const GITHUB: Asset = asset!("/assets/github_logo.png");

struct EducationEntry {
    university: String,
    degree: String,
}

#[component]
pub fn Home() -> Element {
    let educations = [
        EducationEntry {
        university: "Cornell University 2021-2024".to_string(),
        degree: "B.S. in Computer Science".to_string(),
        },

        EducationEntry {
            university: "MIT 2025-Present".to_string(),
            degree: "PhD Student in Operations Research".to_string(),
        },
    ];

    rsx! {
        div {
            id: "home",
            div {
                class: "sub-home",
                style: "flex: 1 1 auto;",
                img { src: HEADSHOT },
                a { class: "h1", "Jacob Dentes" },
                a { class: "h2", "PhD Student" },
                a { class: "link", target: "_blank", href: "https://orc.mit.edu/", "Operations Research Center" },
                a { class: "link", target: "_blank", href: "https://web.mit.edu/", "Massachusetts Institute of Technology" },
                div {
                    style: "display: flex; flex-direction: row; align-items: center; margin: 10px; gap: 5px",
                    a {
                        class: "link",
                        target: "_blank",
                        href: "https://scholar.google.com/citations?user=QD8UTzMAAAAJ",
                        img { class: "link-ico", src: GOOGLE }
                    },
                    a {
                        class: "link",
                        target: "_blank",
                        href: "https://github.com/Jacob-Dentes",
                        img { class: "link-ico", src: GITHUB }
                    },
                    a {
                        class: "link",
                        target: "_blank",
                        href: "https://www.linkedin.com/in/jacob-dentes-30781722a",
                        img { class: "link-ico", src: LINKEDIN }
                    },
                }
            },
            div {
                class: "sub-home",
                style: "flex: 1 1 auto; text-align: left;",
                div {
                    style: "flex: 1 1 auto; width: 100%;",
                    a {
                        class: "h1",
                        style: "flex: 1 1 auto; width: 100%;",
                        "About me"
                    },
                    p {
                        style: "flex: 1 1 auto; width: 100%; padding: 16px",
                        "My name is Jacob Dentes and I am a PhD student at the MIT Operations
                        Research Center. I am currently working on first-order methods for
                        large-scale inventory optimization on GPUs. My past work includes
                        generating congestion prices for large networks and
                        column-generation methods for sharing the cost of wireless networks."
                    },
                },
                div {
                    style: "flex: 1 1 auto; width: 100%;",
                    a {
                        class: "h1",
                        style: "flex: 1 1 auto; width: 100%;",
                        "Education"
                    },
                    div {
                        style: "padding: 16px",
                        for education in &educations {
                            p {
                                style: "font-weight: bold; vertical-align: bottom; margin-bottom: 0px",
                                "{education.university}"
                            },
                            p {
                                style: "font-style: italic; vertical-align: top; margin: 0px",
                                "{education.degree}"
                            },
                        }
                    }
                }
            }
        }
    }
}