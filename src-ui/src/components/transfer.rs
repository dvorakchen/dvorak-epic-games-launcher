use crate::server::init_connect_and_sign_in;
use crate::{components::*, server::InitStatus};
use gloo::timers::future::TimeoutFuture;
use leptos::{leptos_dom::helpers::location, *};

#[component]
pub fn Transfer() -> impl IntoView {
    create_effect(move |_| {
        spawn_local(async move {
            TimeoutFuture::new(2_000).await;
            let status = init_connect_and_sign_in().await;

            match status {
                InitStatus::ConnectFail => {}
                InitStatus::SignedOut => {
                    location().set_href("/sign_in").expect("redirect fail");
                }
                InitStatus::SignedIn(signedin_info) => {
                    signedin_info.store();
                    location().set_href("/homepage").expect("redirect fail");
                }
            }
        });
    });

    view! {
        <div class="w-screen h-screen">
            <EpicLoading/>
        </div>
    }
}
