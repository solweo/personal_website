use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod master_page;
use master_page::*;

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
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
                <a href="/works">"Works"</a>
                <a href="/contact">"Contact"</a>
            </nav>
            
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