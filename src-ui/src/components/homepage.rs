mod library;
mod store;
pub use library::Library;
pub use store::Store;

use crate::components::*;
use leptos::*;
use leptos_router::*;

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
                    <LeftNavItem link="/homepage" icon_type=IconTypes::Tag>
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
            <div class="flex items-center grow shrink gap-4 px-4">
                <TopNavLink link="">"Discover"</TopNavLink>
                <TopNavLink link="">"Browser"</TopNavLink>
                <TopNavLink link="">"News"</TopNavLink>
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
                    <span class="relative">
                        <button class="btn btn-circle bg-base-200 cursor-pointer w-10 h-10
                        hover:bg-base-300 focus:ring-primary focus:ring-1">
                            <span class="fill-primary">
                                <People/>
                            </span>
                        </button>
                    </span>
                    <span class="relative">
                        <button class="btn btn-circle bg-base-200 cursor-pointer w-10 h-10
                        hover:bg-base-300 focus:ring-primary focus:ring-1">"D"</button>
                        <span class="absolute right-0 bottom-0 w-2 h-2 rounded-full bg-[green]"></span>
                    </span>
                </div>
            </div>
        </div>
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
            />
        </span>
    }
}
