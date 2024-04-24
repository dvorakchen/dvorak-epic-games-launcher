use std::cell::RefCell;
use std::rc::Rc;

use leptos::leptos_dom::helpers::location;
use leptos::leptos_dom::html::Div;
use leptos::*;
use leptos_router::use_navigate;

pub fn is_click_outside(x: f32, y: f32, container: HtmlElement<Div>) -> bool {
    if let Some(element) = document().element_from_point(x, y) {
        return !container.contains(Some(&element.into()));
    } else {
        false
    }
}

// pub const GAME_COVER_IMAGE_PATH: &'static str = "/assets/images/games/";

/// Navigation
/// use this navigation to redirects to a new url,
/// and stores in history independent
///
/// # examples
///
/// ```
/// let navigation = Navigation::new();
/// // navigates to a new url
/// navigation.to("".to_string);
/// // back
/// navigation.back();
/// ````
#[derive(Clone)]
pub struct Navigation {
    history: Rc<RefCell<Vec<String>>>,
    pub is_empty: ReadSignal<bool>,
    set_is_empty: WriteSignal<bool>,
}

impl Navigation {
    pub fn new() -> Self {
        let (is_empty, set_is_empty) = create_signal(true);
        Self {
            history: Rc::new(RefCell::new(Vec::new())),
            is_empty,
            set_is_empty,
        }
    }

    pub fn to(&self, to: impl AsRef<str>) {
        let to = to.as_ref();
        let cur = location().pathname().unwrap();
        if &cur == to {
            return;
        }
        let mut his = self.history.borrow_mut();
        his.push(cur);
        use_navigate()(to, Default::default());
        self.set_is_empty.set(false);
    }

    pub fn back(&mut self) {
        let mut his = self.history.borrow_mut();
        if let Some(v) = his.pop() {
            use_navigate()(&v, Default::default());
        }
        if his.is_empty() {
            self.set_is_empty.set(true);
        }
    }
}
