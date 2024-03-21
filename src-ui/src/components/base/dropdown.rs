use crate::components::*;
use crate::utils::is_click_outside;
use leptos::*;

pub type DropDownList = Vec<DropDownItem>;

#[derive(Debug, Clone)]
pub struct DropDownItem {
    pub key: String,
    pub value: String,
    default: bool,
}

impl DropDownItem {
    pub fn new_str(key: &'static str, value: &'static str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            default: false,
        }
    }

    pub fn new_default(key: &'static str, value: &'static str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            default: true,
        }
    }
}

#[component]
pub fn DropDown(
    #[prop(optional)] class: &'static str,
    list: DropDownList,
    #[prop(optional)] prefix: Option<IconTypes>,
    #[prop(into, optional)] on_change: Option<Callback<DropDownItem>>,
) -> impl IntoView {
    let id = format!("dvorak-dropdown-{}", rand::random::<usize>());

    let container = create_node_ref();

    use leptos_dom::helpers::window_event_listener;

    let default_display = list
        .iter()
        .find(|v| v.default)
        .map_or(DropDownItem::new_str("", ""), |v| v.clone());

    let (selected, set_selected) = create_signal(default_display);
    let (show_options, set_show_options) = create_signal(false);

    let handle_click_outside = window_event_listener(ev::mouseup, move |ev| {
        let container: HtmlElement<html::Div> = container.get().unwrap();
        if is_click_outside(ev.page_x() as f32, ev.page_y() as f32, container) {
            set_show_options(false);
        }
    });

    on_cleanup(|| {
        handle_click_outside.remove();
    });

    let unselected = move || {
        let selected_key = selected().key;
        let un = list
            .iter()
            .filter_map(|v| {
                if v.key != selected_key {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<DropDownItem>>();
        un.into_iter()
            .map(|v| {
                let cur = v.clone();
                view! {
                    <li
                        class="h-12 py-4 px-6 cursor-pointer
                        flex items-center hover:bg-neutral"
                        on:click=move |_| {
                            set_selected(cur.clone());
                            if let Some(cb) = on_change {
                                cb(cur.clone());
                            }
                            set_show_options(false);
                        }

                        key=v.key.clone()
                        data-key=v.key
                    >
                        {v.value}
                    </li>
                }
            })
            .collect_view()
    };

    let handle_click = move |_| {
        set_show_options.update(|v| *v = !*v);
    };

    view! {
        <div class=format!("relative {} w-full", class) id=id.clone() node_ref=container>
            <div
                class="flex h-12 bg-base-300 items-center
                py-4 px-6 gap-4 cursor-pointer"
                on:click=handle_click
            >

                {if let Some(icon_types) = prefix {
                    view! { <span class="fill-primary w-4 h-4">{icon_types.into_view()}</span> }
                        .into_view()
                } else {
                    view! {}.into_view()
                }}

                <span class="grow">{move || selected().value}</span>
                <span class="fill-primary">
                    <ChevronDown/>
                </span>
            </div>
            <div class="relative w-full">
                <ul
                    class="absolute dropdown-content top-0 left-0 right-0
                    h-64 overflow-y-scroll bg-base-300"
                    hidden=move || { !show_options() }
                >
                    {unselected}
                </ul>
            </div>

        </div>
    }
}
