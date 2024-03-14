use crate::components::*;
use leptos::*;

#[component]
pub fn Settings<F>(back: F) -> impl IntoView
where
    F: Fn() -> () + 'static,
{
    let handle_back = move |_| back();

    view! {
        <div class="fixed inset-1 z-10 bg-base-200">
            <div class="flex flex-col">
                <nav
                    class="h-16 w-full border-b border-b-neutral
                    shadow cursor-pointer brightness-75
                    hover:brightness-100 hover:bg-base-300"
                    on:click=handle_back
                >
                    <div class="flex items-center p-4 h-full text-neutral">
                        <span class="fill-primary">
                            <ChevronLeft/>
                        </span>
                        <div class="divider divider-horizontal"></div>
                        <span>"SETTINGS"</span>
                    </div>
                </nav>
                <main class="flex flex-col px-10 py-8 space-y-4">
                    <section class="flex flex-col space-y-4">
                        <h1>"LANGUAGE"</h1>
                        <div class="w-96">
                            <DropDown
                                list=vec![
                                    DropDownItem::new_default("Chinese", "中文（中国）"),
                                    DropDownItem::new_str("English", "English"),
                                    DropDownItem::new_str("Chinese1", "中文（中国）"),
                                    DropDownItem::new_str("English1", "English"),
                                    DropDownItem::new_str("Chinese2", "中文（中国）"),
                                    DropDownItem::new_str("English2", "English"),
                                ]

                                prefix=IconTypes::Earth
                            />

                        </div>
                    </section>
                    <div class="divider"></div>
                    <section class="flex flex-col space-y-8">
                        <h1>"PREFERENCES"</h1>
                        <CheckBox label="Run When My Computer Starts"/>
                        <CheckBox label="Enable Debug Logging"/>
                        <CheckBox label="Use Proxy"/>
                    </section>
                    <div class="divider"></div>
                    <section class="flex flex-col space-y-6 pt-4">
                        <Action>"Troubleshoot"</Action>
                        <Action link="https://launcherhelp.epicgames.com/?lang=en">
                            "Troubleshoot"
                        </Action>
                        <Action>"Show Logs"</Action>
                        <Action link="https://trello.com/b/GXLc34hk/epic-games-store-roadmap">
                            "Epic Games Store Roadmap"
                        </Action>
                    </section>
                </main>
            </div>
        </div>
    }
}

#[component]
fn Action(#[prop(optional)] link: &'static str, children: Children) -> impl IntoView {
    view! {
        <OutLink class="cursor-pointer w-fit" link=link>
            {children()}
        </OutLink>
    }
}
