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
        <div class="flex min-h-screen w-screen bg-base-100">
            <nav class="w-[18%]">
                <LeftNav/>
            </nav>

            <div class="flex flex-col grow bg-neutral">
                <div>
                    <TopNav />
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
                        <LeftNavItem link="/unreal-engine" icon_type=IconTypes::UnrealEngine>
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
        logging::log!("cur: {} - link: {} - {}", cur, link, cur == link);
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
    view! {}
}
