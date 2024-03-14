mod settings;
mod valid_email;
mod valid_password;

use settings::Settings;
use valid_email::ValidEmail;
use valid_password::ValidPassword;

use crate::components::*;
use leptos::*;

#[derive(Clone, Debug)]
pub(self) enum SignInProcess {
    ValidEmail,
    ValidPassword(String),
    ForgotPassword(String),
}

#[component]
pub fn SignIn() -> impl IntoView {
    let (process, set_process) = create_signal(SignInProcess::ValidEmail);
    provide_context(set_process);

    let (show_settings, set_show_settings) = create_signal(false);

    let handle_settings = move |_| {
        set_show_settings(true);
    };

    let handle_settings_back = move || {
        set_show_settings(false);
    };

    view! {
        <div class="relative bg-base-100 flex">
            <div class="mx-auto mt-8">
                <div class="w-[30rem] min-h-[35rem]">

                    {move || {
                        match process() {
                            SignInProcess::ValidEmail => view! { <ValidEmail/> },
                            SignInProcess::ValidPassword(email) => {
                                view! { <ValidPassword email=email/> }
                            }
                            SignInProcess::ForgotPassword(email) => view! {}.into_view(),
                        }
                    }}

                // <ValidPassword email="dvorakchen@outlook.com".to_string()/>
                </div>
            </div>
            <button class="fixed left-8 bottom-8" on:click=handle_settings>
                <Gear/>
            </button>
            <Show when=show_settings fallback=|| view! {}>
                <Settings back=handle_settings_back/>
            </Show>
        </div>
    }
}

#[component]
pub(self) fn Container(children: Children) -> impl IntoView {
    view! {
        <div class="w-full bg-base-200 rounded flex flex-col p-14
        transition-all 
        animate-[flash_.1s_linear] scale-100">{children()}</div>
    }
}

#[component]
pub(self) fn Logo() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="w-14">
                <img src="/assets/images/Epic-white.png" alt="Epic"/>
            </div>
        </div>
    }
}
