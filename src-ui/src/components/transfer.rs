use leptos::{leptos_dom::helpers::location, *};

#[component]
pub fn Transfer() -> impl IntoView {

    create_effect(|_| {
        location().set_href("/login").expect("redirect fail");
    });

    view! { <div class="">"TRANSFER"</div> }
}
