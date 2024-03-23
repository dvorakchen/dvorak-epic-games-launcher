use leptos::{html::{Dialog, Div}, *};

use crate::utils::is_click_outside;
use crate::
    components::*;
    
#[component]
pub fn Friends() -> impl IntoView {
    use crate::components::People as PeopleIcon;

    #[derive(PartialEq, Clone, Copy)]
    enum FriendTab {
        Friends,
        AddFriends,
        Setting,
    }

    let dialog_node: NodeRef<Dialog> = create_node_ref();
    let container_node: NodeRef<Div> = create_node_ref();

    let (friends_tab, set_friends_tab) = create_signal(FriendTab::Friends);

    let handle_click_friends = move |_| {
        let dialog = dialog_node.get().unwrap();
        if dialog.open() {
            dialog.close();
        } else {
            dialog.show_modal().unwrap();
        }
    };

    let handle_click_outside = window_event_listener(ev::mouseup, move |ev| {
        match (container_node.get(), dialog_node.get()) {
            (Some(container), Some(dialog)) => {
                if is_click_outside(ev.page_x() as f32, ev.page_y() as f32, container) {
                    dialog.close();
                }
            }
            _ => (),
        }
    });

    on_cleanup(|| {
        handle_click_outside.remove();
    });

    view! {
        <span class="relative">
            <button
                class="btn btn-circle bg-base-200 cursor-pointer w-10 h-10
                hover:bg-base-300 focus:ring-primary focus:ring-1"
                on:click=handle_click_friends
            >
                <span class="fill-primary">
                    <PeopleIcon/>
                </span>
            </button>

            <dialog
                node_ref=dialog_node
                class="fixed right-4 left-auto top-20 bottom-auto
                rounded"
            >
                <div
                    node_ref=container_node
                    class="bg-base-200 w-[23rem] h-[600px]
                    border-neutral border rounded"
                >
                    <div class="flex flex-col h-full">
                        <div class="flex justify-center items-center shrink-0 my-6">
                            <span class="border border-neutral px-4 py-1 rounded-full flex">
                                <span
                                    class="box-content fill-white w-6 h-4 rounded-full px-4 py-2 cursor-pointer"
                                    class=(
                                        "bg-base-300",
                                        move || friends_tab() == FriendTab::Friends,
                                    )

                                    on:click=move |_| set_friends_tab(FriendTab::Friends)
                                >
                                    <PeopleIcon/>
                                </span>
                                <span
                                    class="box-content fill-white w-6 h-4 rounded-full px-4 py-2 cursor-pointer"
                                    class=(
                                        "bg-base-300",
                                        move || friends_tab() == FriendTab::AddFriends,
                                    )

                                    on:click=move |_| set_friends_tab(FriendTab::AddFriends)
                                >
                                    <PersonPlus/>
                                </span>
                                <span
                                    class="box-content fill-white w-6 h-4 rounded-full px-4 py-2 cursor-pointer"
                                    class=(
                                        "bg-base-300",
                                        move || friends_tab() == FriendTab::Setting,
                                    )

                                    on:click=move |_| set_friends_tab(FriendTab::Setting)
                                >
                                    <Gear/>
                                </span>
                            </span>
                        </div>
                        <div class="grow px-4">

                            {move || {
                                match friends_tab() {
                                    FriendTab::Friends => view! { <FriendsTabFriends/> },
                                    FriendTab::AddFriends => view! { <FriendsTabFriends/> },
                                    FriendTab::Setting => view! { <FriendsTabFriends/> },
                                }
                            }}

                        </div>
                        <div class="shrink-0 text-center text-xs py-4">
                            <a href="" class="underline">
                                "connect your social account"
                            </a>
                            " to find friends on Epic Games!"
                        </div>
                    </div>
                </div>
            </dialog>
        </span>
    }
}

#[component]
fn FriendsTabFriends() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-primary font-bold text-xl">"FRIENDS"</h1>
            <div>
                <Input input_type="text" label="Filter Friends"/>
            </div>
            <div class="flex flex-col gap-2">
                <h2 class="text-xs text-primary">"ONLINE"</h2>
                <Friend/>
                <Friend/>
            </div>
        </div>
    }
}

#[component]
fn Friend() -> impl IntoView {
    view! {
        <div class="grid grid-cols-[2rem_auto] grid-rows-[1rem_.7rem] gap-x-2
        px-2 py-3 rounded items-center cursor-pointer
        hover:bg-base-300">
            <div class="inline-block row-span-2">
                <span class="text-primary flex justify-center items-center aspect-square w-8 h-8 rounded-full bg-[green]">
                    "D"
                </span>
            </div>
            <div class="inline-block space-x-2">
                <span class="text-primary font-bold">"DvorakChen"</span>
                <span class="bg-base-300 text-xs px-1 rounded">"YOU"</span>
            </div>
            <div class="inline-block">
                <span class="text-xs text-neutral">"Online"</span>
            </div>
        </div>
    }
}