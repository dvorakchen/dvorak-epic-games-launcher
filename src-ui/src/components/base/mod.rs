mod checkbox;
mod dropdown;
mod icons;
mod input;
mod alert;
mod button;

pub use checkbox::*;
pub use dropdown::*;
pub use icons::*;
pub use input::*;
pub use alert::*;
pub use button::Button;

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
