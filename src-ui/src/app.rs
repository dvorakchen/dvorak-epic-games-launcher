use crate::components::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Transfer/>
                <Route path="/sign_in" view=SignIn/>
                <Route path="/homepage" view=HomePage>
                    <Route path="" view=Store/>
                    <Route path="library" view=Library/>
                </Route>
            </Routes>
        </Router>
    }
}
