use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod master_page;
use master_page::*;

use ui_kit::widgets::Header;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // stylance::import_style!(css, "./path/to/local_style.css");

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

        <Router>

            <nav>
                <A href="/">"Home"</A>
                <A href="/about">"About"</A>
                <A href="/works">"Works"</A>
                <A href="/contact">"Contact"</A>
            </nav>
            
            <Header/>
            
            <h1>"Personal website"</h1>

            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/about" view=About/>
                    <Route path="/works" view=WorksList/>
                    <Route path="/works/:id" view=WorkInfo/>
                    <Route path="/contact" view=Contact/>
                </Routes>
            </main>
        </Router>
    }
}