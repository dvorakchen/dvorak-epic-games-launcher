use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn StoreDetail() -> impl IntoView {

    let _params = use_params_map();
    

    view! { <div>"STORE DETAIL"</div> }
}
