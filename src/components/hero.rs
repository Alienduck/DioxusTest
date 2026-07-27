use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "hero-container",

            // Glow effect behind content
            div {
                class: "hero-glow",
            }

            // Background animation with particles
            div {
                class: "background-animation",

                for i in 0..30 {
                    div {
                        class: "particle",
                        style: format!(
                            "left: {}%; top: {}%;",
                            (i as f32 * 4.0) % 100.0,
                            ((i as f32) * 3.5) % 100.0
                        ),
                        animation: format!(
                            "float {}s linear {}s infinite",
                            6.0 + (i as f32) * 0.15,
                            (i as f32 % 5.0)
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
                    "Une plateforme de développement moderne pour bâtir des applications réactives, performantes et intuitives."
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

            // Scroll indicator
            div {
                class: "scroll-indicator",

                svg {
                    r#type: "svg",
                    dangerous_inner_html: "<path d='M12 5v14 M19 12l-6 6-6-6' stroke='currentColor' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round' fill='none'/>"
                }
            }
        }
    }
}
