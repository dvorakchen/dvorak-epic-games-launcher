use std::rc::Rc;

use super::{Container, EmailContext, Logo, SignInProcess};
use crate::components::*;
use crate::server::signin_signout::sign_in;
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub(super) fn ValidPassword() -> impl IntoView {
    let (msg_err, set_msg_err) = create_signal("");

    provide_context(set_msg_err);

    let email_context =
        use_context::<RwSignal<EmailContext>>().expect("cannot found context 'EmaillContext'");
    let email = String::clone(&email_context.get());

    view! {
        <Container>
            <Logo/>
            <div class="mt-16 text-center font-bold">"Sign In to your Epic Games Account"</div>
            {move || {
                (!msg_err().is_empty())
                    .then(|| {
                        view! {
                            <div class="mt-4">
                                <Alert message=msg_err()/>
                            </div>
                        }
                    })
            }}

            <main class="flex flex-col mt-6">
                <EmailRead email=email.clone()/>
                <PasswordInput/>
            </main>
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
fn PasswordInput() -> impl IntoView {
    let password_input: NodeRef<html::Input> = create_node_ref();
    let password_input_for_focus = Rc::new(password_input);
    let password_input_for_visible = Rc::new(password_input);
    let password_input_for_submit = Rc::new(password_input);

    let email_context =
        use_context::<RwSignal<EmailContext>>().expect("cannot found context 'EmaillContext'");
    let email = String::clone(&email_context.get());

    let set_err_msg =
        use_context::<WriteSignal<&str>>().expect("cannot found Write Signal 'err_msg'");

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

    let valid_password_action = create_action(|input: &(String, String)| {
        let (email, password) = input.clone();

        async move {
            sign_in(email, password)
                .await
                .map_err(|_| "Sorry the credentials you are using are invalid.")
        }
    });

    // this handle when user sign in successed
    create_effect(move |_| {
        match (
            valid_password_action.value()(),
            valid_password_action.pending()(),
        ) {
            (Some(Ok(info)), false) => {
                //  save signned in user informations
                info.store();
                let navigate = use_navigate();
                navigate("/homepage", Default::default());
            }
            (Some(Err(err_msg)), false) => {
                set_err_msg(err_msg);
            }
            _ => (),
        }
    });

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let password = password_input_for_submit
            .get()
            .expect("password input not exist")
            .value();
        valid_password_action.dispatch((email.clone(), password));
    };

    let handle_input_change = move |text: String| {
        set_password_empty(text.is_empty());
    };

    view! {
        <form action="" class="relative flex flex-col mt-6" on:submit=handle_submit>
            <div class="grow">
                <Input
                    input_type="password"
                    node_ref=password_input
                    label="Password: 123123"
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
            <Forgot/>
            <Button
                class="btn-lg btn-accent mt-6"
                disabled=Signal::derive(disabled)
                loading=valid_password_action.pending()
            >
                "SIGN IN"
            </Button>
        </form>
    }
}

#[component]
fn Forgot() -> impl IntoView {
    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'SignInProcess'");

    let handle_forgot = move |_| {
        set_process(SignInProcess::ForgotPassword);
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
