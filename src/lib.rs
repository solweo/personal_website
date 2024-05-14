use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod master_page;
use master_page::*;

use ui_kit::widgets::{LoremImpus, Preview, Header};

pub const BRAND_KEY_MESSAGE: &str = "Embark the realms of tech artistry, art sorcery and product wizardy - thus unleashing your ingenuity!";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    // contexts are passed down through the route tree
    provide_context(ExampleContext(0));

    view! {
        <Title text="Solweo"/>

        <Link
            rel="shortcut icon"
            type_="image/ico"
            href="http://cdn.solweo.tech/solweo-logo-bold-96px.png"
        />

        <Meta
            name="description"
            content="Hands-on experience on topics ranging from design to development from Adrian Alekseev aka Solweo"
        />

        <Body/>

        <Router>   
            <Header/>
            <h1>"Personal website"</h1>
            <main>
                // <Routes>
                //     <Route path="/" view=Home/>
                //     <Route path="/about" view=About/>
                //     <Route path="/works" view=WorksList/>
                //     <Route path="/works/:id" view=WorkInfo/>
                //     <Route path="/ui-kit-preview" view=UiKitPreview/>
                // </Routes>
                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                >
                    <Route path="/" view=|| view! { <Home/> }/>
                    <Route path="/about" view=|| view! { <About/> }/>
                    <Route path="/works" view=|| view! { <WorksList/> }/>
                    <Route path="/works/:id" view=|| view! { <WorkInfo/> }/>
                    <Route path="/ui-kit-preview" view=|| view! { <UiKitPreview/> }/>
                </AnimatedRoutes>
            </main>
        </Router>
    }
}

#[component]
pub fn UiKitPreview() -> impl IntoView {
    view! {
        <h3>"UI kit preview"</h3>
        <Preview/>
        <LoremImpus/>
        // <Outlet/>
        <AnimatedOutlet
            outro="fadeOut"
            intro="fadeIn"
        />
    }
}