use leptos::*;
use crate::typewritter::TypeWritterFX;
use stylance::import_style as get_css;

get_css!(tagline_css, "./tagline.css");

#[component]
pub fn Tagline() -> impl IntoView {
    let keywords = vec![
        "tech artistry".to_string(), 
        "art sorcery".to_string(), 
        "product wizardy".to_string()
    ];

    view! {
        <div class=tagline_css::tagline>
            <span>"Embark the realms"</span>
            <br/>
            <span>"of "</span>
            <TypeWritterFX words=keywords/>
            <br/>
            <span>" - thus unleashing"</span>
            <br/>
            <span>"your ingenuity 🌱"</span>
        </div>
    }
}