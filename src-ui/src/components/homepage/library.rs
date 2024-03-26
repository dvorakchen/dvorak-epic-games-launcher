use crate::components::*;
use leptos::{html::Div, *};
use leptos_router::{use_location, use_navigate, use_query_map};
use web_sys::MouseEvent;

#[component]
pub fn Library() -> impl IntoView {
    view! {
        <div class="flex flex-col p-4 pt-0 my-4 gap-4">
            <div class="flex items-center gap-4 mb-4">
                <h1 class="text-3xl">"Library"</h1>
                <span class="fill-neutral cursor-pointer">
                    <ArrowRepeat/>
                </span>
            </div>

            <Collections/>
            <Filter/>
            <Games/>
        </div>
    }
}

#[component]
fn Collections() -> impl IntoView {
    let query = use_query_map();
    let is_collection = move |name: &'static str| {
        query.with(|q| match q.get("collection") {
            Some(a) if a == name => true,
            None if name == "all" => true,
            _ => false,
        })
    };

    view! {
        <div class="flex gap-8">
            <span
                class="text-lg cursor-pointer text-neutral"
                class=("text-primary", move || is_collection("all"))
                on:click=move |_| to_new_query("collection", "all")
            >

                "All"
            </span>
            <span
                class="text-lg cursor-pointer text-neutral"
                class=("text-primary", move || is_collection("favorites"))
                on:click=move |_| to_new_query("collection", "favorites")
            >

                "Favorites"
            </span>
            <span class="fill-white h-full cursor-pointer">
                <PlusCircle/>
            </span>
        </div>
    }
}

#[component]
fn Filter() -> impl IntoView {
    let query = use_query_map();
    let is_layout = move |name: &'static str| {
        query.with(|q| match q.get("layout") {
            Some(a) if a == name => true,
            None if name == "grid" => true,
            _ => false,
        })
    };

    view! {
        <div class="flex items-center gap-4 sticky top-0 bg-base-100">
            <span class="text-neutral">"Sort by:"</span>
            <span class="text-primary">"Recently Played"</span>
            <span class="grow shrink"></span>
            <div class="flex items-center gap-2 h-8">
                <span
                    class="aspect-square h-full flex items-center justify-center rounded"
                    class=("bg-base-300", move || is_layout("grid"))
                    on:click=move |_| to_new_query("layout", "grid")
                >
                    <span class="fill-white aspect-square w-5
                    cursor-pointer">
                        <Grid/>
                    </span>
                </span>
                <span
                    class="aspect-square h-full flex items-center justify-center rounded"
                    class=("bg-base-300", move || is_layout("list"))
                    on:click=move |_| to_new_query("layout", "list")
                >
                    <span class="fill-white aspect-square w-5
                    cursor-pointer">
                        <ListUl/>
                    </span>
                </span>
            </div>
        </div>
    }
}

#[component]
fn Games() -> impl IntoView {
    view! {
        <div class="flex w-full">
            <div class="w-4/5">
                <div class="flex flex-wrap justify-between">

                    {move || { (0..8).map(|_| view! { <GameCover/> }).collect_view() }}

                </div>
            </div>
            <div class="w-1/5">e</div>
        </div>
    }
}

#[component]
fn GameCover() -> impl IntoView {
    let (show_menu, set_show_menu) = create_signal(false);
    let menu_node: NodeRef<Div> = create_node_ref();

    let handle_contextmenu = move |ev: MouseEvent| {
        ev.prevent_default();
        ev.prevent_default();
        set_show_menu(true);
        let left = ev.offset_x();
        let top = ev.offset_y();
        logging::log!("left: {} - top: {}", left, top);
        let menu = menu_node.get().unwrap();
        _ = menu
            .style("left", format!("{}px", left))
            .style("top", format!("{}px", top));
    };

    let handle_click_menu = window_event_listener(ev::mouseup, move |_| {
        set_show_menu(false);
    });

    on_cleanup(|| {
        handle_click_menu.remove();
    });

    view! {
        <span
            class="relative flex flex-col gap-2 w-1/4 aspect-[9/16] rounded-lg p-4
            cursor-pointer
            hover:bg-base-200"
            on:contextmenu=handle_contextmenu
        >
            <div class="w-full h-2/3 rounded
            bg-[url('/assets/images/games/black-myth-wukong.jpg')]
            bg-cover bg-center
            "></div>
            <div class="flex flex-col gap-1">
                <div class="flex items-center gap-4">
                    <span
                        class="grow shrink whitespace-nowrap overflow-hidden"
                        title="Black Myth Wukong"
                    >
                        "Black Myth Wukong"
                    </span>
                </div>
                <span class="text-neutral text-sm">"0/10 Achievements"</span>
                <span class="text-sm">
                    <span class="flex gap-2 text-sm text-neutral fill-neutral">
                        <Download/>
                        "Uninstall"
                    </span>
                </span>
            </div>

            <Show when=show_menu>
                <div
                    class="absolute z-50 w-56 py-2 bg-base-200 rounded-lg shadow-2xl"
                    node_ref=menu_node
                >
                    <MenuItem on_click=move |_| {}>"Install"</MenuItem>
                    <MenuItem on_click=move |_| {}>"Go To Store Page"</MenuItem>
                    <MenuItem on_click=move |_| {}>"Add To Favorites"</MenuItem>
                    <MenuItem on_click=move |_| {}>"Add To Collection"</MenuItem>

                </div>
            </Show>
        </span>
    }
}

#[component]
fn MenuItem(children: ChildrenFn, #[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    let handle_click = move |ev| {
        on_click(ev);
    };

    view! {
        <div
            class="py-2 px-4 text-primary whitespace-nowrap cursor-pointer
            hover:bg-base-300"
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

fn to_new_query(key: &'static str, value: &'static str) {
    let location = use_location();
    let mut map = location.query.get_untracked();
    _ = map.insert(key.to_string(), value.to_string());
    let query_string = map.to_query_string();
    let mut path = location.pathname.get_untracked();
    path.push_str(&query_string);

    use_navigate()(&path, Default::default());
}
