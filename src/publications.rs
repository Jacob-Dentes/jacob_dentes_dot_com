use dioxus::prelude::*;

struct PubEntry {
    pub name: String,
    pub link: String,
    pub authors: String,
    pub description: String,
}

#[component]
pub fn Publications() -> Element {
    let pub_entries = [
        PubEntry {
            name: "A Data-Driven Model for LoRaWAN Connection Quality and Coverage".to_string(),
            link: "https://par.nsf.gov/servlets/purl/10659999".to_string(),
            authors: "Alfredo A Rodriguez, Sander Aarts, Ali Amadeh, Jacob Dentes, \
                        Alex Coy, Kenneth Schlather, David Shmoys, K Max Zhang".to_string(),
            description: "LoRaWAN is a popular Long-Range Low-Power wireless communications \
                        protocol that is enabling many IoT applications worldwide, with more \
                        networks growing both in size and number around the world. To effectively \
                        plan and operate these networks, it is necessary to have tools that \
                        reliably quantify, measure, and predict the connection quality provided \
                        by LoRaWAN receivers. This paper proposes a novel data-driven approach to
                        connection quality modeling that is tailored for LoRaWAN.".to_string(),
        },
        PubEntry {
            name: "Bounding the Price-of-Fair-Sharing Using Knapsack-Cover Constraints to \
                    Guide Near-Optimal Cost-Recovery Algorithms".to_string(),
            link: "https://arxiv.org/pdf/2309.16914".to_string(),
            authors: "Sander Aarts, Jacob Dentes, Manxi Wu, David B Shmoys".to_string(),
            description: "We consider the problem of fairly allocating the cost of providing a \
                        service among a set of users, where the service cost is formulated by an \
                        NP-hard covering integer program (CIP). The central issue is to determine \
                        a cost allocation to each user that, in total, recovers as much as possible \
                        of the actual cost while satisfying a stabilizing condition known as the \
                        core property. Motivated by an application of cost allocation for network \
                        design for LPWANs, an emerging IoT technology, we investigate a general \
                        class of CIPs and give the first non-trivial price-of-fair-sharing bounds \
                        by using the natural LP relaxation strengthened with knapsack-cover inequalities".to_string(),
        },
    ];
    rsx! {
        div {
            id: "publications",
            style: "padding: 32px; flex: 1 1 auto; text-align: left; gap: 128px; overflow: scroll; justify-content: space-between;",
            for entry in pub_entries.iter() {
                div {
                    style: "margin: 32px;",
                    a {
                        class: "link",
                        style: "flex: 1 1 auto; width: 100%;",
                        target: "_blank",
                        href: entry.link.clone(),
                        "{entry.name}"
                    },
                    div {
                        style: "margin: 16px",
                        p {
                            style: "font-style: italic",
                            "{entry.authors}"
                        }
                        p {
                            "{entry.description}"
                        }
                    }
                }
            }
        }
    }
}