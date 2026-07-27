use dioxus::prelude::*;

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
        Stripes {}
    }
}

#[component]
fn Stripes() -> Element {
    let mut signal = use_signal(|| false);
    rsx! {
        div { class: Styles::stripe_1, onanimationend: move |_| { signal.set(true); } }
        div { class: Styles::stripe_2 }
        div { class: Styles::stripe_3 }
    }
}
