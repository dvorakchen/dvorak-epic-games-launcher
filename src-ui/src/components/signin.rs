mod email_sent;
mod forgot_password;
mod settings;
mod valid_email;
mod valid_password;

use std::ops::Deref;

use email_sent::EmailSent;
use forgot_password::ForgotPassword;
use settings::Settings;
use valid_email::ValidEmail;
use valid_password::ValidPassword;

use crate::components::*;
use leptos::*;

/// email context for passing inside current context,
/// just only used inside current module
///
/// # example
/// ```
/// # let email_context = use_context::<RwSignal<EmailContext>>().expect("cannot found context 'EmaillContext'");
///
/// ````
#[derive(Clone)]
pub(self) struct EmailContext(String);

impl Deref for EmailContext {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub(self) enum SignInProcess {
    ValidEmail,
    ValidPassword,
    ForgotPassword,
    EmailSent,
}

#[component]
pub fn SignIn() -> impl IntoView {
    let (process, set_process) = create_signal(SignInProcess::ValidEmail);
    provide_context(set_process);
    provide_context(create_rw_signal(EmailContext("".to_string())));

    let (show_settings, set_show_settings) = create_signal(false);

    let handle_settings = move |_| {
        set_show_settings(true);
    };

    let handle_settings_back = move || {
        set_show_settings(false);
    };

    view! {
        <div class="relative bg-base-100 flex min-h-screen">
            <div class="mx-auto my-8 flex">
                <div class="w-[30rem] min-h-[35rem] m-auto">

                    {move || {
                        match process() {
                            SignInProcess::ValidEmail => view! { <ValidEmail/> },
                            SignInProcess::ValidPassword => {
                                view! { <ValidPassword/> }
                            }
                            SignInProcess::ForgotPassword => view! { <ForgotPassword/> }.into_view(),
                            SignInProcess::EmailSent => view! { <EmailSent/> },
                        }
                    }}

                </div>
            </div>
            <button class="fixed left-8 bottom-8 w-4 h-4 fill-primary" on:click=handle_settings>
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
            <div class="w-14 fill-white">
                <Epic/>
            </div>
        </div>
    }
}
