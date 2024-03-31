use crate::components::*;
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn StoreDetail() -> impl IntoView {
    let params = use_params_map();

    let _id = move || params.get().get("id").expect("cannot find id").clone();

    view! {
        <div class="flex relative">
            <div class="w-2/3">
                <Detail/>
            </div>
            <div class="w-1/3">
                <Operations/>
            </div>
        </div>
    }
}

#[component]
fn Detail() -> impl IntoView {
    view! {
        <div class="flex flex-col py-6 w-full">
            <div class="flex flex-col gap-3 mb-8">
                <h1 class="text-primary text-4xl">"Black Myth: Wukong"</h1>
                <div class="flex items-center gap-2 flex-wrap">
                    <span>"⭐⭐⭐⭐⭐"</span>
                    <span class="text-sm rounded bg-base-300 text-neutral px-2">"4.8"</span>
                    <span class="text-sm px-1 rounded bg-accent text-primary">
                        "Greet Bosss Battle"
                    </span>
                    <span class="text-sm px-1 rounded bg-accent text-primary">"Hot Monkey"</span>
                </div>
            </div>

            // Nav
            <nav class="sticky top-0 pb-4">
                <div class="flex gap-8">
                    <span class="cursor-pointer text-primary text-xl border-b-2 border-b-primary pb-1">
                        "Overview"
                    </span>
                    <span class="cursor-pointer text-neutral text-xl pb-1
                    hover:border-b-2 border-b-neutral">"Achievements"</span>
                </div>
            </nav>

            <Showcase />
        </div>
    }
}

#[component]
fn Showcase() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
        <div class="relative w-full aspect-[16/9] showcase">
            <img
                class="w-full h-full pointer-events-none select-none"
                src="/assets/images/games/black-myth-wukong-yrpgs-1920x1080.png"
                alt="black myth wukong"
            />
            <span class="absolute top-0 bottom-0 left-0 w-16 bg-gradient-to-r from-base-100/50 to-transparent
            -translate-x-16">
                <span class="fill-white w-8 cursor-pointer">
                    <ChevronLeft/>
                </span>
            </span>
            <span class="absolute top-0 bottom-0 right-0 w-16 bg-gradient-to-l from-base-100/50 to-transparent
            translate-x-16">
                <span class="fill-white w-8 cursor-pointer">
                    <ChevronRight/>
                </span>
            </span>
        </div>
        <div class="flex items-center gap-1">
            <span class="flex shrink-0 w-10 rounded-full aspect-square bg-base-300">
                <span class="fill-white m-auto">
                    <ChevronLeft/>
                </span>
            </span>
            <div class="grow shrink">

            </div>
            <span class="flex shrink-0 w-10 rounded-full aspect-square bg-base-300">
                <span class="fill-white m-auto">
                    <ChevronRight/>
                </span>
            </span>
        </div>
    </div>
    }
}

#[component]
fn Operations() -> impl IntoView {
    view! {}
}
