use leptos::html::Nav;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::test::Test;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // let location = document()
    //     .location()
    //     .expect("Couldn't get location..")
    //     .as_string()
    //     .expect("something went wrong with location");

    let nav_ref = create_node_ref::<Nav>();

    let something = nav_ref.get().unwrap();

    view! {

        // injects info into HTML tag from application code
        <Html
            lang="en"
            dir="ltr"
            attr:data-theme="light"
        />

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <base href="https://diversable.github.io/deployment-csr-gh-pages/" />

        <ErrorBoundary
            fallback=|errors| view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                        .collect_view()
                    }
                </ul>
            }
        >

        <nav _ref=nav_ref>
            <A href="/test">Go to test page</A>
        </nav>

            <Router>
                <Routes base="/deployment-csr-gh-pages".to_string()>
                    <Route path="/" view=Home />
                    <Route path="/test" view=Test />
                    // fallback
                    <Route path="/*" view=NotFound />
                </Routes>
            </Router>

        </ErrorBoundary>
    }
}
