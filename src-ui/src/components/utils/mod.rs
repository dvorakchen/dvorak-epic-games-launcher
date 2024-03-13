mod checkbox;
mod dropdown;
mod icons;

pub use checkbox::*;
pub use dropdown::*;
pub use icons::*;

use leptos::*;

/// Out link
/// open the link in browser
#[component]
pub fn OutLink(
    #[prop(optional)] link: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    _ = link;

    view! {
        <a class=class href="javascript:;">
            {children()}
        </a>
    }
}
