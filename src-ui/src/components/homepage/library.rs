use crate::server::games;
use crate::utils::GAME_COVER_IMAGE_PATH;
use crate::{components::*, utils::Navigation};
use leptos::{html::Div, *};
use leptos_router::{use_location, use_navigate, use_query_map};
use share::GameCover as GameCoverModel;
use web_sys::MouseEvent;

#[component]
pub fn Library() -> impl IntoView {
    view! {
        <div class="flex flex-col pt-0 my-4 gap-4">
            <div class="flex items-center gap-4 mb-4">
                <h1 class="text-3xl pl-4">"Library"</h1>
                <span class="fill-neutral cursor-pointer">
                    <ArrowRepeat/>
                </span>
            </div>

            <div class="pl-4">
                <Collections/>
            </div>
            <div class="flex w-full">
                <div class="w-3/4">
                    <div class="px-4 mb-2">
                        <Sort/>
                    </div>
                    <Games/>
                </div>
                <div class="w-1/4 pl-4">
                    <Filters/>
                </div>
            </div>
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
fn Sort() -> impl IntoView {
    let query = use_query_map();
    let is_layout = move |name: &'static str| {
        query.with(|q| match q.get("layout") {
            Some(a) if a == name => true,
            None if name == "grid" => true,
            _ => false,
        })
    };

    view! {
        <div class="flex items-center gap-4 sticky z-30 top-0 bg-base-100">
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
    let games = create_resource(|| {}, |_| async move { games::get_my_games().await });

    view! {
        <div class="flex flex-wrap">
            <Suspense fallback=move || {
                (0..8).map(|_| view! { <GameOverSkelecton/> }).collect_view()
            }>
                {move || match games.get() {
                    None => view! {}.into_view(),
                    Some(games) => {
                        games
                            .into_iter()
                            .map(|game_cover| view! { <GameCover model=game_cover/> })
                            .collect_view()
                    }
                }}

            </Suspense>
        </div>
    }
}

#[component]
fn GameOverSkelecton() -> impl IntoView {
    view! {
        <span class="w-1/4 aspect-[9/16] rounded-lg p-4">
            <div class="bg-base-200 opacity-50 w-full h-full
            flex items-center justify-center">
                <span class="fill-white">
                    <Epic/>
                </span>
            </div>
        </span>
    }
}

#[component]
fn GameCover(model: GameCoverModel) -> impl IntoView {
    let cover_url = format!("{}{}", GAME_COVER_IMAGE_PATH, model.cover_url);
    let (show_menu, set_show_menu) = create_signal(false);
    let menu_node: NodeRef<Div> = create_node_ref();

    let is_installed = {
        let name = model.name.clone();
        create_resource(
            || {},
            move |_| {
                let name = name.clone();
                async move {
                    let name = name.clone();
                    games::check_installed(name).await
                }
            },
        )
    };

    let handle_click = {
        let id = model.id.clone();
        let navigation =
            use_context::<Navigation>().expect("had not provided context 'Navigation'");
        move |_| {
            navigation.to(format!("/homepage/store/{}", id));
        }
    };

    let handle_contextmenu = move |ev: MouseEvent| {
        ev.prevent_default();
        set_show_menu(true);
        let left = ev.offset_x();
        let top = ev.offset_y();
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
        <span class="relative flex flex-col gap-2 w-1/4 aspect-[9/16] rounded-lg p-4
        cursor-pointer
        hover:bg-base-200">
            <div class=format!(
                "w-full h-2/3 rounded
            bg-[url('{}')] bg-cover bg-center",
                cover_url,
            )></div>
            <div class="flex flex-col gap-1">
                <div class="flex items-center gap-4">
                    <span
                        class="grow shrink whitespace-nowrap overflow-hidden"
                        title=model.name.clone()
                    >
                        {model.name.clone()}
                    </span>
                </div>
                <span class="text-neutral text-sm">
                    {model.achievements_completed} "/" {model.achievements_amount} " Achievements"
                </span>
                <span class="text-sm">
                    <span class="flex gap-2 text-sm text-neutral fill-neutral">
                        <Download/>

                        {move || {
                            match is_installed.get() {
                                None => view! { "..." }.into_view(),
                                Some(is_installed) => {
                                    if is_installed {
                                        view! { "Installed" }
                                    } else {
                                        view! { "Uninstalled" }
                                    }
                                        .into_view()
                                }
                            }
                        }}

                    </span>
                </span>
            </div>

            <span
                class="absolute inset-0"
                on:contextmenu=handle_contextmenu
                on:click=handle_click
            ></span>

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

#[component]
fn Filters() -> impl IntoView {
    view! {
        <div class="flexn flex-col">
            <div class="h-8 flex items-center">
                <span class="grow">"Filters"</span>
                <span class="text-xs cursor-pointer">"RESET"</span>
            </div>
            <div class="flex flex-col pt-6">
                <div class="relative flex rounded items-center gap-4 px-4 py-2 bg-base-200">
                    <span class="fill-white aspect-square w-3">
                        <Search/>
                    </span>
                    <input
                        class="bg-transparent w-11/12 outline-none text-primary grow shrink text-sm"
                        type="text"
                        placeholder="Title"
                    />
                    <span class="fill-white cursor-pointer">
                        <X/>
                    </span>
                </div>
            </div>
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
