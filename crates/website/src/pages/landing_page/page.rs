use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_primitives::context_menu;
use easer::functions::Easing;
use gloo_timers::future::sleep;
use std::time::Duration;

#[css_module("/src/pages/landing_page/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct LandingPageProps {
    #[props(default = true)]
    pub play_intro: bool,
}

#[component]
pub fn LandingPage(props: LandingPageProps) -> Element {
    rsx! {
        div {
            class: Styles::landing_page,
            canvas {
                id: "bevy-canvas",
                style: "position: absolute; inset: 0; width: 100vw; height: 100vh; z-index: -1; pointer-events: auto;",
                oncontextmenu: move |evt| { evt.prevent_default(); }
            }
            script {
                r#type: "module",
                "import init from '/bevy_bg.js'; init().catch(console.error);"
            }
            Title { text: "Lucid Games " }
        }
    }
}

#[component]
fn Title(text: &'static str) -> Element {
    let mut char_count = use_motion(0f32);
    let text_len = text.len() as f32;

    use_hook(move || {
        spawn(async move {
            sleep(Duration::from_secs(2)).await;
            char_count.animate_to(
                text_len,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs_f32(text_len * 0.2),
                    easing: easer::functions::Sine::ease_out,
                }))
                .with_loop(LoopMode::Times(1)),
            );
        });
    });

    let visible_text = text
        .chars()
        .take(char_count.get_value() as usize)
        .collect::<String>();

    rsx! {
        h1 { class: Styles::title, "{visible_text}" }
    }
}
