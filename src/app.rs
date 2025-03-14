use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

// Define a server function
#[server(GetServerMessage, "/api")]
pub async fn get_server_message() -> Result<String, ServerFnError> {
    // Simulate some server-side logic (e.g., fetching from a DB or API)
    Ok("Hello from the server!".to_string())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rusty-test.css" />
        <Title text="Welcome to Leptos" />
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    // Create a resource to call the server function
    let server_message = Resource::new(|| (), |_| async move {
        get_server_message().await
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <p>
            "Server says: "
            <Suspense fallback=move || {
                view! { "Loading..." }
            }>
                {move || {
                    server_message
                        .get()
                        .map(|result| match result {
                            Ok(msg) => msg,
                            Err(e) => format!("Error: {:?}", e),
                        })
                }}
            </Suspense>
        </p>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
