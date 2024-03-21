mod library;
mod store;
pub use library::Library;
pub use store::Store;

use crate::{components::*, server::signin_signout, storages::get_signed_in_user_info, utils::is_click_outside};
use leptos::*;
use leptos_router::*;

const STORE_ROUTE_PATH: &'static str = "/homepage";

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex min-h-screen w-screen bg-base-100 pr-8">
            <nav class="w-[18%]">
                <LeftNav/>
            </nav>

            <div class="flex flex-col grow p-2">
                <div>
                    <TopNav/>
                </div>
                <Outlet/>
            </div>
        </div>
    }
}

#[component]
fn LeftNav() -> impl IntoView {
    view! {
        <div class="flex flex-col mx-4">
            <div class="flex justify-center items-center my-8">
                <span class="fill-white w-1/2 flex place-content-center">
                    <Epic/>
                </span>
            </div>
            <ul class="flex flex-col gap-2">
                <li>
                    <LeftNavItem link=STORE_ROUTE_PATH icon_type=IconTypes::Tag>
                        "Store"
                    </LeftNavItem>
                </li>
                <li>
                    <span>
                        <LeftNavItem link="/homepage/library" icon_type=IconTypes::Grid>
                            "Library"
                        </LeftNavItem>
                    </span>
                </li>
                <li>
                    <span>
                        <LeftNavItem link="" icon_type=IconTypes::UnrealEngine>
                            "Unreal Engine"
                        </LeftNavItem>
                    </span>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn LeftNavItem(link: &'static str, children: ChildrenFn, icon_type: IconTypes) -> impl IntoView {
    let location = use_location();

    let is_current_route = move || {
        let cur = location.pathname.get();
        cur == link
    };

    view! {
        <a
            href=link
            class="flex px-4 py-3 items-center gap-4 justify-start
            rounded-lg cursor-pointer fill-neutral text-neutral
            hover:text-primary hover:fill-primary hover:bg-base-300"
            class=("bg-base-200", is_current_route)
        >
            <span class="fill-inherit">{icon_type.into_view()}</span>
            {children()}
        </a>
    }
}

#[component]
fn TopNav() -> impl IntoView {
    view! {
        <div class="flex shrink-0 py-6">
            <div class="flex items-center px-2">
                <span class="fill-neutral px-2">
                    <ChevronLeft/>
                </span>
                <Search/>
            </div>
            <div class="w-full">
                <StoreDaily/>
            </div>
            <div class="flex shrink-0 items-center">
                <div class="flex items-center gap-4">
                    <TopNavLink link="">"Wishlist"</TopNavLink>
                    <TopNavLink link="">"Cart"</TopNavLink>
                </div>
                <span class="h-4">
                    <div class="divider divider-horizontal before:bg-neutral after:bg-neutral"></div>
                </span>
                <div class="flex items-center gap-4">
                    <Friends/>
                    <Profile/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Friends() -> impl IntoView {
    use crate::components::People as PeopleIcon;

    view! {
        <span class="relative">
            <button class="btn btn-circle bg-base-200 cursor-pointer w-10 h-10
            hover:bg-base-300 focus:ring-primary focus:ring-1">
                <span class="fill-primary">
                    <PeopleIcon/>
                </span>
            </button>

            <dialog>
                <p>sssss</p>
            </dialog>
        </span>
    }
}

#[component]
fn Profile() -> impl IntoView {
    let user_info = get_signed_in_user_info().unwrap();
    let first_letter = user_info.username.chars().nth(0);

    let (show_dropdown, set_show_dropdown) = create_signal(false);

    let handle_click = move |_| {
        set_show_dropdown.update(|v| *v = !*v);
    };

    let container = create_node_ref();

    let handle_click_outside = window_event_listener(ev::mouseup, move |ev| {
        if let Some(container) = container.get() {
            if is_click_outside(ev.page_x() as f32, ev.page_y() as f32, container) {
                set_show_dropdown(false);
            }
        }
    });

    on_cleanup(|| {
        handle_click_outside.remove();
    });

    let handle_sign_out = move |_| {
        signin_signout::sign_out();
        let navigate = use_navigate();
        navigate("/sign_in", Default::default());
    };

    view! {
        <div class="relative" node_ref=container>
            <button
                class="btn btn-circle bg-base-200 cursor-pointer w-10 h-10
                hover:bg-base-300 focus:ring-primary focus:ring-1"
                on:click=handle_click
            >
                {first_letter}
            </button>
            <span class="absolute right-0 bottom-0 w-2 h-2 rounded-full bg-[green]"></span>
            <Show when=show_dropdown>
                <div class="absolute top-[120%] right-0 w-80 p-4 bg-base-200 leading-9 rounded">
                    <section class="pt-2">
                        <h1 class="font-bold text-lg">{user_info.username.clone()}</h1>
                        <ProfileItem>"My Achievements"</ProfileItem>
                    </section>
                    <div class="divider"></div>
                    <section>
                        <ProfileItem outlink="#">"Epic Rewards"</ProfileItem>
                        <ProfileItem outlink="#">"Epic Wallets"</ProfileItem>
                        <ProfileItem>"Coupons"</ProfileItem>
                    </section>
                    <div class="divider"></div>
                    <section>
                        <ProfileItem outlink="#">"Account"</ProfileItem>
                        <ProfileItem>"Redeem Code"</ProfileItem>
                        <ProfileItem>"Settings"</ProfileItem>
                        <ProfileItem outlink="#">"Terms of Service"</ProfileItem>
                        <ProfileItem outlink="#">"Privacy Policy"</ProfileItem>
                        <ProfileItem outlink="#">"Store Refund Policy"</ProfileItem>
                    </section>
                    <div class="divider"></div>
                    <ProfileItem on:click=handle_sign_out>"Sign Out"</ProfileItem>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn ProfileItem(#[prop(optional)] outlink: &'static str, children: ChildrenFn) -> impl IntoView {
    view! {
        <A
            href=outlink
            class="text-neutral text-sm cursor-pointer
            flex items-center my-3 w-full
            hover:text-primary"
        >
            {children()}

            {(!outlink.is_empty())
                .then(|| {
                    view! {
                        <span class="ml-2 fill-white w-3 h-3">
                            <BoxArrowUpRight/>
                        </span>
                    }
                })}

        </A>
    }
}

#[component]
fn StoreDaily() -> impl IntoView {
    let location = use_location();

    let is_store_route = move || {
        let cur = location.pathname.get();
        &cur == STORE_ROUTE_PATH
    };

    view! {
        <Show when=is_store_route>
            <div class="flex items-center grow shrink gap-4 px-4">
                <TopNavLink link="">"Discover"</TopNavLink>
                <TopNavLink link="">"Browser"</TopNavLink>
                <TopNavLink link="">"News"</TopNavLink>
            </div>
        </Show>
    }
}

#[component]
fn TopNavLink(link: &'static str, children: ChildrenFn) -> impl IntoView {
    let location = use_location();

    let is_current_route = move || {
        let cur = location.pathname.get();
        cur == link
    };

    view! {
        <a
            class="focus:outline-primary
            transition-all text-neutral
            outline-none outline-1 rounded p-2"
            href=link
            class=("text-primary", is_current_route)
        >
            {children()}
        </a>
    }
}

#[component]
fn Search() -> impl IntoView {
    use crate::components::Search as SearchIcon;

    view! {
        <span class="flex items-center relative">
            <label
                class="fill-neutral w-3 h-3
                absolute left-4"
                for="search_store"
            >
                <SearchIcon/>
            </label>
            <input
                id="search_store"
                type="search"
                placeholder="Search store"
                class="pl-10 pr-2 px-4 h-10 w-60 text-sm text-primary
                rounded-full outline-none bg-base-200
                focus:outline"
                autocomplete="off"
            />
        </span>
    }
}
