use crate::components::*;
use crate::server::games;
use leptos::*;
use leptos_router::use_params_map;
use share::{Review as ReviewModel, ReviewType};

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

            <Nav/>

            <div class="my-4">
                <Showcase/>
            </div>

            <div class="my-4">
                <Introduction/>
            </div>

            <div class="my-4">
                <AvaliableAchievements/>
            </div>

            <div class="my-4">
                <SystemRequirements/>
            </div>

            <div class="my-4">
                <EpicPlayerRatings/>
            </div>
        </div>
    }
}

#[component]
fn Nav() -> impl IntoView {
    let handle_achievements = move |_| {
        let achievements = document()
            .get_element_by_id("avaliable_achievements")
            .expect("cannot found avaliable_achievements");

        achievements.scroll_into_view();
    };

    let handle_ratings = move |_| {
        let ratings = document()
            .get_element_by_id("ratings")
            .expect("cannot found ratings");

        ratings.scroll_into_view();
    };

    view! {
        <nav class="sticky z-50 top-0 pb-8 bg-base-100">
            <div class="flex gap-8">
                <span class="cursor-pointer text-primary text-xl border-b-2 border-b-primary pb-1">
                    "Overview"
                </span>

                <span
                    class="cursor-pointer text-neutral text-xl pb-1
                    hover:border-b-2 border-b-neutral"
                    on:click=handle_achievements
                >
                    "Achievements"
                </span>

                <span
                    class="cursor-pointer text-neutral text-xl pb-1
                    hover:border-b-2 border-b-neutral"
                    on:click=handle_ratings
                >
                    "Ratings"
                </span>
            </div>
        </nav>
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
            <div class="flex items-center gap-2">
                <span class="flex shrink-0 w-10 rounded-full aspect-square bg-base-300">
                    <span class="fill-white m-auto">
                        <ChevronLeft/>
                    </span>
                </span>
                <div class="flex grow shrink gap-4 overflow-clip">
                    <div class="aspect-[16/9] bg-base-100 rounded w-1/6 overflow-clip
                    cursor-pointer border border-primary">
                        <img
                            class="w-full h-full"
                            src="/assets/images/games/black-myth-wukong-yrpgs-480x270.png"
                            alt=""
                        />
                    </div>
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
fn Introduction() -> impl IntoView {
    let (expand, set_expand) = create_signal(false);

    let handle_expand = move |_| {
        set_expand.update(|v| *v = !*v);
    };

    view! {
        <div class="flex flex-col gap-8">
            <p class="text-lg">
                "Black Myth: Wukong is an action RPG rooted in Chinese mythology. You shall set out as the Destined One to venture into the challenges and marvels ahead, to uncover the obscured truth beneath the veil of a glorious legend from the past."
            </p>

            // Genres & Features
            <div class="flex h-12">
                <div class="grow h-full border-l border-l-neutral/80 px-8">
                    <div class="text-neutral">"Genres"</div>
                    <div class="flex gap-4 text-sm">
                        <a class="underline" href="">
                            "Action-Adventure"
                        </a>
                        <a class="underline" href="">
                            "RPG"
                        </a>
                    </div>
                </div>
                <div class="grow h-full border-l border-l-neutral/80 px-8">
                    <div class="text-neutral">"Features"</div>
                    <div class="flex gap-4 text-sm">
                        <a class="underline" href="">
                            "Controller- Support"
                        </a>
                        <a class="underline" href="">
                            "Single Player"
                        </a>
                    </div>
                </div>
            </div>

            // Dedail
            <article
                class="relative text-sm space-y-4 pb-20 overflow-hidden"
                class=("max-h-[400px]", move || !expand())
            >
                <p class="text-neutral">
                    "Black Myth: Wukong is an action RPG rooted in Chinese mythology. The story is based on Journey to the West, one of the Four Great Classical Novels of Chinese literature. You shall set out as the Destined One to venture into the challenges and marvels ahead, to uncover the obscured truth beneath the veil of a glorious legend from the past."
                </p>
                <img src="/assets/images/games/black-myth-wukong-15gfy.png" alt=""/>
                <p>"• Explore a Land of Vast Wonders"</p>
                <p class="text-neutral">
                    "A world unseen, where new sights rise with every stride. Enter a fascinating realm filled with the wonders and discoveries of ancient Chinese mythology!"
                </p>
                <p class="text-neutral">
                    "As the Destined One, you shall venture through breathtaking landscapes in the classic tale of Journey to the West, creating a new epic of uncharted adventures."
                </p>
                <img src="/assets/images/games/black-myth-wukong-1e6om.png" alt=""/>
                <p>"• Confront Mighty Foes, Old and New"</p>
                <p class="text-neutral">
                    "Heroic Monkey, might and fame, adversaries rise, to test his name. One of the major highlights of Journey to the West is its diverse cast of adversaries, each with unique strengths."
                </p>
                <p class="text-neutral">
                    "As the Destined One, you shall encounter powerful foes and worthy rivals throughout your journey. Fearlessly engage them in epic battles where surrender is not an option."
                </p>
                <img src="/assets/images/games/black-myth-wukong-cbxk5.png" alt=""/>
                <p>"• Temper Your Mastery of Varied Spells"</p>
                <p class="text-neutral">
                    "Spells unbound, knowledge's flight, infinite abilities take their height. Spells, transformations, and magic vessels in all manifestations, complementary yet adversarial, have long been iconic combat elements of Chinese mythology."
                </p>
                <p class="text-neutral">
                    "As the Destined One, aside from mastering various staff techniques, you can also freely combine different spells, abilities, weapons, and equipment to find the winning strategy that best suits your combat style."
                </p>
                <img src="/assets/images/games/black-myth-wukong-1h0t4.png" alt=""/>
                <p>"• Discover Heartfelt Tales Behind Every Facade"</p>
                <p class="text-neutral">
                    "Within beings of every kind lies the story of a life. Beneath the ferocity and craftiness of your foes lies an engaging tapestry of their origins, personalities, and motivations waiting to be revealed."
                </p>
                <p class="text-neutral">
                    "As the Destined One, you will uncover the stories behind a variety of characters, delving beyond your battles with them, to taste the love and hate, greed and fury they once had and still carry with them."
                </p>

                <div class="absolute left-0 right-0 bottom-0 h-20
                flex items-end
                bg-gradient-to-t from-base-100 from-60% to-transparent">
                    <div
                        class="bg-base-300 w-full text-center text-neutral rounded py-4
                        cursor-pointer hover:brightness-125"
                        on:click=handle_expand
                    >

                        {move || { if expand() { "SHOW LESS" } else { "SHOW MORE" } }}

                    </div>
                </div>
            </article>
        </div>
    }
}

#[component]
fn AvaliableAchievements() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <h1 id="avaliable_achievements" class="text-primary text-lg">
                "Available Achievements"
            </h1>
            <div class="flex gap-2 flex-wrap">

                {move || {
                    (0..16)
                        .map(|_| {
                            view! {
                                <div class="aspect-square w-12">
                                    <img
                                        class="w-full h-full"
                                        src="/assets/images/games/achievement-1.png"
                                        alt=""
                                    />
                                </div>
                            }
                        })
                        .collect_view()
                }}
                <span class="flex items-center justify-center w-12 bg-base-300 text-neutral rounded aspect-square
                cursor-pointer text-xs
                hover:brightness-125">"MORE"</span>
            </div>
        </div>
    }
}

#[component]
fn SystemRequirements() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-lg">"Black Myth: Wukong System Requirements"</h1>
            <div class="flex flex-col px-12 py-8 w-full bg-base-200 rounded-lg">
                <div class="flex border-b border-b-neutral">
                    <span class="py-6 border-b-2 border-b-primary cursor-pointer text-sm">
                        "WINDOWS"
                    </span>
                </div>

                <table class="text-sm text-left mt-8">
                    <thead>
                        <tr>
                            <th class="text-neutral">"Minimum"</th>
                            <th class="text-neutral">"Recommended"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="h-16">
                            <td class="text-left pr-4">
                                <div class="text-neutral">"Windows OS"</div>
                                <div>"Windows 10 64-bit (version 1909 or higher)"</div>
                            </td>
                            <td class="text-left">
                                <div class="text-neutral">"Windows OS"</div>
                                <div>"Windows 10 64-bit (version 1909 or higher)"</div>
                            </td>
                        </tr>
                        <tr class="h-16">
                            <td class="text-left pr-4">
                                <div class="text-neutral">"Windows Processor"</div>
                                <div>"Intel Core i3-8100 or AMD Ryzen 3 1300X"</div>
                            </td>
                            <td class="text-left">
                                <div class="text-neutral">"Windows Processor"</div>
                                <div>"Intel Core i5-8600 or AMD Ryzen 5 3600"</div>
                            </td>
                        </tr>
                        <tr class="h-16">
                            <td class="text-left pr-4">
                                <div class="text-neutral">"Windows Memory"</div>
                                <div>"16GB RAM"</div>
                            </td>
                            <td class="text-left">
                                <div class="text-neutral">"Windows Memory"</div>
                                <div>"16GB RAM"</div>
                            </td>
                        </tr>
                        <tr class="h-16">
                            <td class="text-left pr-4">
                                <div class="text-neutral">"Windows Storage"</div>
                                <div>"150GB SSD space"</div>
                            </td>
                            <td class="text-left">
                                <div class="text-neutral">"Windows Storage"</div>
                                <div>"150GB SSD space"</div>
                            </td>
                        </tr>
                        <tr class="h-16">
                            <td class="text-left pr-4">
                                <div class="text-neutral">"Windows Graphics"</div>
                                <div>"NVIDIA GeForce GTX 1650 4GB or AMD Radeon RX 5500XT 4GB"</div>
                            </td>
                            <td class="text-left">
                                <div class="text-neutral">"Windows Graphics"</div>
                                <div>"NVIDIA GeForce RTX 3060 or AMD Radeon RX 5700"</div>
                            </td>
                        </tr>

                        <tr class="h-16">
                            <td colspan="2" class="text-left">
                                <div class="text-neutral">"Login Accounts Required"</div>
                                <div>"Epic Games"</div>
                            </td>
                        </tr>

                        <tr class="h-16">
                            <td colspan="2" class="text-left">
                                <div class="text-neutral">"Languages Supported"</div>
                                <div>
                                    "AUDIO: English, French, German, Italian, Spanish - Spain, Arabic, Japanese, Polish, Portuguese, Portuguese - Brazil, Russian, Spanish - Latin America | TEXT: Arabic, English, Chinese - Simplified, Chinese - Traditional, Czech, Danish, Dutch, Finnish, French, German, Greek, Hungarian, Italian, Japanese, Korean, Norwegian, Polish, Portuguese, Portuguese - Brazil, Russian, Spanish - Spain, Spanish - Latin America, Swedish, Thai, Turkish"
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn EpicPlayerRatings() -> impl IntoView {
    let reviews = create_resource(
        || {},
        |_| async move { games::get_recently_reviews("".to_string()).await },
    );

    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-lg text-primary">"MOST HELPFUL REVIEWS"</h1>

            <Suspense fallback=move || {
                view! { "Loading..." }
            }>
                {move || match reviews.get() {
                    None => {
                        view! {
                            <div class="text-center text-neutral text-lg font-bold">
                                "Not Reviews Yet."
                            </div>
                        }
                            .into_view()
                    }
                    Some(reviews) => {
                        reviews
                            .into_iter()
                            .map(|review| view! { <Review model=review/> })
                            .collect_view()
                    }
                }}
                <div class="bg-base-300 w-full text-center text-neutral rounded py-4
                cursor-pointer hover:brightness-125">"SHOW MORE"</div>
            </Suspense>
        </div>
    }
}

#[component]
fn Review(model: ReviewModel) -> impl IntoView {
    let first_letter = model.username.chars().nth(0);

    view! {
        <div class="flex flex-col rounded-lg bg-base-200 px-4 py-2">
            <div class="flex">
                <div class="flex grow items-center gap-4">
                    <span class="w-8 h-8 rounded-full bg-base-300 flex items-center justify-center">
                        {first_letter}
                    </span>
                    <div class="text-sm">{model.username}</div>
                </div>
                <div
                    class="flex gap-4 shrink
                    px-2 py-1 items-end rounded"
                    class=("bg-accent/50", move || model.review_type == ReviewType::ThumbsUp)
                    class=("bg-error/50", move || model.review_type == ReviewType::ThumbsDown)
                >
                    <span
                        class="w-6"
                        class=("fill-accent", move || model.review_type == ReviewType::ThumbsUp)
                        class=("fill-error", move || model.review_type == ReviewType::ThumbsDown)
                    >

                        {move || {
                            match model.review_type {
                                ReviewType::ThumbsUp => view! { <ThumbsUp/> },
                                ReviewType::ThumbsDown => view! { <ThumbsDown/> },
                            }
                                .into_view()
                        }}

                    </span>
                    <span class="text-primary text-lg">
                        {match model.review_type {
                            ReviewType::ThumbsUp => "RECOMMENDED",
                            ReviewType::ThumbsDown => "NOT RECOMMENDED",
                        }}

                    </span>
                    <span class="text-sm text-neutral">"POSTED: " {model.datetime}</span>
                </div>
            </div>
            <div class="my-4">
                <p class="whitespace-break-spaces">{model.content}</p>
            </div>
        </div>
    }
}

#[component]
fn Operations() -> impl IntoView {
    view! {}
}
