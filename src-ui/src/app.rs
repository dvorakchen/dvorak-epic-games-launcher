use crate::components::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Transfer/>
                <Route path="/login" view=Login/>
            </Routes>
        </Router>
    }
}
