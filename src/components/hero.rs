use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "hero-container",

            // Background animation with particles
            div {
                class: "background-animation",

                for i in 0..50 {
                    div {
                        class: "particle",
                        style: format!(
                            "left: {}%; top: {}%; --opacity: {};",
                            (i as f32 * 2.5) % 100.0,
                            ((i as f32) * 2.0) % 100.0,
                            ((i as f32) / 10.0).min(0.8)
                        ),
                        animation: format!(
                            "float {}s linear {}s infinite",
                            5.0 + (i as f32) * 0.1,
                            (i as f32 % 4.0)
                        ),
                    }
                }
            }

            // Intro content with fade-in animation
            div {
                class: "hero-content",

                h1 {
                    class: "hero-title",
                    "Créez des expériences web extraordinaires"
                }

                p {
                    class: "hero-subtitle",
                    "Une plateforme de développement moderne pour bâtir des applications réactives et performantes."
                }

                div {
                    class: "hero-links",

                    a {
                        href: "https://dioxuslabs.com/learn/0.7/",
                        class: "btn btn-primary",
                        "📚 Learn Dioxus"
                    }

                    a {
                        href: "https://dioxuslabs.com/awesome",
                        class: "btn btn-secondary",
                        "🚀 Awesome Dioxus"
                    }

                    a {
                        href: "https://github.com/dioxus-community/",
                        class: "btn btn-outline",
                        "📡 Community Libraries"
                    }

                    a {
                        href: "https://discord.gg/XgGxMSkvUM",
                        class: "btn btn-discord",
                        "👋 Join Discord"
                    }
                }
            }
        }
    }
}
