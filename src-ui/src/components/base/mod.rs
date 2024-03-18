mod alert;
mod button;
mod checkbox;
mod dropdown;
mod epic_loading;
mod icons;
mod input;

pub use alert::*;
pub use button::*;
pub use checkbox::*;
pub use dropdown::*;
pub use epic_loading::*;
pub use icons::*;
pub use input::*;

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
