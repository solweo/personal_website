use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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

#[component]
fn Home() -> impl IntoView {
    view! {
        <h3>"Home page placeholder"</h3>
        <Outlet/>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h3>"About page placeholder"</h3>
        <Outlet/>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <h3>"Contacts page placeholder"</h3>
        <h3>"Contacts:"</h3>
        <A href="https://t.me/solweo">"Telegram: @solweo"</A>
        <A href="mailto:adrian@solweo.tech">"Mail: adrian@solweo.tech"</A>
        <Outlet/>
    }
}

#[component]
fn WorksList() -> impl IntoView {
    view! {
        <h3>"Works listing page placeholder"</h3>
        <div>
            <A href="demo1">"Demo 1"</A>
            <A href="demo2">"Demo 2"</A>
        </div>
        <Outlet/>
    }
}

#[component]
fn WorkInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "demo1" => "Demo 1",
        "demo2" => "Demo 2",
        _ => "Work not found",
    };

    view! {
        <h3>{name}</h3>
        <h3>"Work article page placeholder"</h3>
        <Outlet/>
    }
}