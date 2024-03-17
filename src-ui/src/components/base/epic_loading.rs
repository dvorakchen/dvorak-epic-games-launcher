use crate::components::*;
use leptos::*;

#[component]
pub fn EpicLoading() -> impl IntoView {
    view! {
        <div class="relative flex w-full h-full bg-base-100">
            <span class="m-auto w-96 -translate-y-8 fill-white">
                <Epic/>
            </span>
            <div class="absolute left-1 right-1 bottom-1 h-3 overflow-clip">
                <div class="w-[120%] h-full
                animate-move-25
                bg-[url(/assets/svg/epic-loading-bar.svg)]"></div>
            </div>
        </div>
    }
}
