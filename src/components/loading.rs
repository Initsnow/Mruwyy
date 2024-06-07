use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div { id: "showbox",
        div { class: "loader",
            svg { "viewBox": "25 25 50 50", class: "circular",
                circle {
                    "r": "20",
                    "cx": "50",
                    "fill": "none",
                    "cy": "50",
                    "stroke-width": "2",
                    "stroke-miterlimit": "10",
                    class: "path"
                }
            }
        }
    }
    }
}