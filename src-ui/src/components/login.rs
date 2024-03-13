mod settings;
use settings::Settings;
use web_sys::SubmitEvent;

use crate::components::*;
use email_address::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login() -> impl IntoView {
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
                <SignInOrSignUp/>
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
fn SignInOrSignUp() -> impl IntoView {
    view! {
        <div class="w-[30rem] bg-base-200 rounded flex flex-col p-14
        transition-all 
        animate-[flash_.1s_linear] scale-100">
            <Logo/>
            <div class="mt-16 mb-6 text-center font-bold">"Sign In or Sign Up"</div>
            <EmailInput/>
            <div class="divider">"or continue with"</div>
            <ThirdPartis/>
            <Privacy/>
            <SignInLater/>
        </div>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="w-14">
                <img src="/assets/images/Epic-white.png" alt="Epic"/>
            </div>
        </div>
    }
}

#[component]
fn EmailInput() -> impl IntoView {
    let email_input = create_node_ref();
    let (email_valid, set_email_valid) = create_signal(false);

    create_effect(move |_| {
        let email_input: HtmlElement<html::Input> =
            email_input.get().expect("Email input not exist");
        email_input.focus().unwrap();
    });

    let handle_email_change = move |ev| {
        let email = event_target_value(&ev);
        if EmailAddress::is_valid(&email) {
            set_email_valid.update(|v| {
                if !*v {
                    *v = true;
                }
            });
        } else {
            set_email_valid.update(|v| {
                if *v {
                    *v = false;
                }
            });
        }
    };

    let valid_email_action = create_action(|input: &String| {
        let _input = input.clone();
        async move {
            use gloo::timers::future::TimeoutFuture;

            TimeoutFuture::new(1_000).await;
            true
        }
    });

    let disabled = move || !email_valid() || valid_email_action.pending()();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        valid_email_action.dispatch("".to_string());
    };

    view! {
        <form action="" class="flex flex-col" on:submit=handle_submit>
            <div class="input-group">
                <input
                    class="input"
                    node_ref=email_input
                    on:keyup=handle_email_change
                    type="email"
                    placeholder
                    required
                />
                <label for="">"Email Address"</label>
                <span class="error text-error">"Please input correct Email"</span>
            </div>
            <div class="flex justify-center my-4">
                <button class="btn btn-lg btn-accent" disabled=disabled>

                    {move || {
                        if valid_email_action.pending()() {
                            view! { <DashCircle class="animate-spin"/> }
                        } else {
                            view! { "CONTINUE" }.into_view()
                        }
                    }}

                </button>
            </div>
        </form>
    }
}

#[component]
fn ThirdPartis() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="flex justify-center gap-3 mt-8">
                <ThirdParty image="/assets/images/Xbox.jpg" name="Xbox"/>
                <ThirdParty image="/assets/images/PlayStation.jpg" name="PlayStation"/>
                <ThirdParty image="/assets/images/Nintendo.jpg" name="Nintendo"/>
                <ThirdParty image="/assets/images/Steam.jpg" name="Steam"/>
            </div>
            <div class="flex justify-center gap-3 mt-3">
                <ThirdParty image="/assets/images/Facebook.jpg" name="Facebook"/>
                <ThirdParty image="/assets/images/Apple.jpg" name="Apple"/>
            </div>
        </div>
    }
}

#[component]
fn ThirdParty(image: &'static str, name: &'static str) -> impl IntoView {
    view! {
        <div class="rounded-md overflow-clip w-20
        hover:brightness-110">
            <a href="/example.com" target="_blank">
                <img class="w-full h-full object-cover" src=image alt=name/>
            </a>
        </div>
    }
}

#[component]
fn Privacy() -> impl IntoView {
    view! {
        <div class="text-center text-sm text-neutral my-4">
            "By signing in or signing up. you agree with our "
            <OutLink
                class="text-primary underline
                hover:no-underline"
                link="example.com"
            >
                "Privacy Policy"
            </OutLink> "."
        </div>
    }
}

#[component]
fn SignInLater() -> impl IntoView {
    view! {
        <div class="text-center text-sm">
            <A class="underline" href="/homepage">
                "Sign In Later"
            </A>
        </div>
    }
}
