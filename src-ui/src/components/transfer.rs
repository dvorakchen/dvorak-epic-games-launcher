use crate::components::*;
use gloo::timers::future::TimeoutFuture;
use leptos::{leptos_dom::helpers::location, *};

#[component]
pub fn Transfer() -> impl IntoView {
    create_effect(move |_| {
        spawn_local(async move {
            let logged_in = connect_and_init().await;
            if logged_in {
                location().set_href("/homepage").expect("redirect fail");
            } else {
                location().set_href("/login").expect("redirect fail");
            }
        });
    });

    view! {
        <div class="w-screen h-screen">
            <EpicLoading/>
        </div>
    }
}

async fn connect_and_init() -> bool {
    TimeoutFuture::new(2_000).await;
    false
}
