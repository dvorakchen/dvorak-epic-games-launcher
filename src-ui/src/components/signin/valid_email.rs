use std::rc::Rc;

use super::{Container, EmailContext, Logo};
use crate::components::*;
use email_address::*;
use leptos::*;
use leptos_router::*;
use web_sys::SubmitEvent;

#[component]
pub(super) fn ValidEmail() -> impl IntoView {
    view! {
        <Container>
            <Logo/>
            <div class="mt-16 mb-6 text-center font-bold">"Sign In or Sign Up"</div>
            <EmailInput/>
            <div class="divider">"or continue with"</div>
            <ThirdPartis/>
            <Privacy/>
            <SignInLater/>
        </Container>
    }
}

#[component]
fn EmailInput() -> impl IntoView {
    let email_input = create_node_ref();
    let email_input_for_effect = Rc::new(email_input);
    let email_input_for_submit = Rc::clone(&email_input_for_effect);
    let email_input_for_action = Rc::clone(&email_input_for_effect);
    let (email_valid, set_email_valid) = create_signal(false);

    let email_context =
        use_context::<RwSignal<EmailContext>>().expect("cannot found context 'EmaillContext'");

    use super::SignInProcess;
    let set_process =
        use_context::<WriteSignal<SignInProcess>>().expect("to have found Process provider");

    create_effect(move |_| {
        let email_input: HtmlElement<html::Input> =
            email_input_for_effect.get().expect("Email input not exist");
        email_input.focus().unwrap();
    });

    let handle_email_change = move |email: String| {
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
        let input = input.clone();
        async move {
            use crate::server::signin_signout::check_account_exist;

            let res = check_account_exist(input).await;
            if res.is_ok() {
                true
            } else {
                false
            }
        }
    });

    // if email is passed, redirect to 'Valid Password'
    create_effect(move |_| {
        let va = (valid_email_action.value()(), valid_email_action.pending()());
        let email = email_input_for_action
            .get()
            .expect("Email input not exist")
            .value();
        email_context.set(EmailContext(email));

        match va {
            (Some(true), false) => set_process(SignInProcess::ValidPassword),

            _ => (),
        }
    });

    let disabled = move || !email_valid() || valid_email_action.pending()();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email = email_input_for_submit
            .get()
            .expect("Email input not exist")
            .value();
        valid_email_action.dispatch(email);
    };

    view! {
        <form class="flex flex-col" on:submit=handle_submit>
            <Input
                class="input"
                node_ref=email_input
                on_change=handle_email_change
                input_type="email"
                required=true
                label="Email Address: fake@email.com"
                invalid_message="Please input correct Email"
            />

            <div class="flex justify-center my-4">
                <Button
                    class="btn-lg btn-accent"
                    disabled=Signal::derive(disabled)
                    loading=valid_email_action.pending()
                >
                    "CONTINUE"
                </Button>
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
