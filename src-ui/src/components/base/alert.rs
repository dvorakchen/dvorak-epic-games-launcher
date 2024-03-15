use crate::components::*;
use leptos::*;

#[component]
pub fn Alert(message: &'static str) -> impl IntoView {
    view! {
        <div class="flex px-6 py-4 rounded-sm bg-base-300 gap-4 items-center">
            <span class="w-6 h-6 fill-error">
                <XCircle/>
            </span>
            <span class="text-error contrast-75">{message}</span>
        </div>
    }
}
