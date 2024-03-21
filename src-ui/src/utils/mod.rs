
use leptos::leptos_dom::html::Div;
use leptos::*;

pub fn is_click_outside(x: f32, y: f32, container: HtmlElement<Div>) -> bool {
    if let Some(element) = document().element_from_point(x, y) {
        return !container.contains(Some(&element.into()));
    } else {
        false
    }
}