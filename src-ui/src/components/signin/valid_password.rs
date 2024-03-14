use std::rc::Rc;

use super::{Container, Logo, SignInProcess};
use crate::components::*;
use leptos::*;

#[component]
pub(super) fn ValidPassword(email: String) -> impl IntoView {
    view! {
        <Container>
            <Logo/>
            <div class="mt-16 mb-6 text-center font-bold">"Sign In to your Epic Games Account"</div>
            <form action="" class="flex flex-col">
                <EmailRead email=email.clone()/>
                <PasswordInput email=email.clone()/>
            </form>
            <BackToAllOptions/>
        </Container>
    }
}

#[component]
fn EmailRead(email: String) -> impl IntoView {
    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'SignInProcess'");

    let handle_edit_email = move |_| {
        set_process(SignInProcess::ValidEmail);
    };

    view! {
        <div class="relative flex items-center">
            <div class="grow">
                <Input default=email disabled=true label="Email Address"/>
            </div>
            <button
                type="button"
                class="absolute right-4 fill-primary cursor-pointer
                hover:ring-8 hover:ring-primary/20 hover:rounded-sm"
                on:click=handle_edit_email
            >
                <PencilSquare/>
            </button>
        </div>
    }
}

#[component]
fn PasswordInput(email: String) -> impl IntoView {
    let password_input: NodeRef<html::Input> = create_node_ref();
    let password_input_for_focus = Rc::new(password_input);
    let password_input_for_visible = Rc::new(password_input);

    create_effect(move |_| {
        password_input_for_focus
            .get()
            .expect("password input not exist")
            .focus()
            .expect("focus failed");
    });

    let (password_visible, set_password_visible) = create_signal(false);

    let (password_empty, set_password_empty) = create_signal(true);

    let disabled = move || password_empty();

    let handle_password_visible = move |_| {
        set_password_visible.update(|v| *v = !*v);
        let input_type = if password_visible() {
            "text"
        } else {
            "password"
        };
        password_input_for_visible
            .get()
            .expect("password input not exist")
            .set_attribute("type", input_type)
            .expect("set password input attribute failed");
    };

    let handle_input_change = move |text: String| {
        set_password_empty(text.is_empty());
    };

    view! {
        <div class="relative flex mt-6">
            <div class="grow">
                <Input
                    input_type="password"
                    node_ref=password_input
                    label="Password"
                    required=true
                    invalid_message="Required"
                    on_change=handle_input_change
                />
            </div>
            <button
                type="button"
                class="absolute right-4 top-5 fill-primary cursor-pointer
                hover:ring-8 hover:ring-primary/20 hover:rounded-sm"
                on:click=handle_password_visible
            >

                {move || {
                    if password_visible() {
                        view! { <Eye/> }
                    } else {
                        view! { <EyeSlash/> }
                    }
                }}

            </button>
        </div>
        <Forgot email=email.clone()/>
        <button class="btn btn-accent btn-lg mt-6" disabled=disabled>
            "SIGN IN"
        </button>
    }
}

#[component]
fn Forgot(email: String) -> impl IntoView {
    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'SignInProcess'");

    let handle_forgot = move |_| {
        set_process(SignInProcess::ForgotPassword(email.clone()));
    };

    view! {
        <div class="text-center">
            <a class="text-sm underline" on:click=handle_forgot href="javascript:;">
                "Forgot Your Password"
            </a>
        </div>
    }
}

#[component]
fn BackToAllOptions() -> impl IntoView {
    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'SignInProcess'");

    let handle_back_to = move |_| {
        set_process(SignInProcess::ValidEmail);
    };

    view! {
        <div class="text-center text-sm mt-10">
            <span class="text-neutral">"Back to "</span>
            <a class="underline" on:click=handle_back_to href="javascript:;">
                "all options"
            </a>
        </div>
    }
}
