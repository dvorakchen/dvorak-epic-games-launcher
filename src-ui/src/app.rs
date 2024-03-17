use crate::components::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Transfer/>
                <Route path="/login" view=SignIn/>
                <Route path="/homepage" view=HomePage/>
            </Routes>
        </Router>
    }
}
