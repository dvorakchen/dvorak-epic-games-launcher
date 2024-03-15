use super::{Container, EmailContext, Logo, SignInProcess};
use crate::components::*;
use email_address::EmailAddress;
use leptos::*;

#[component]
pub(super) fn ForgotPassword() -> impl IntoView {
    let (err_msg, set_err_msg) = create_signal("");
    let (email_valid, set_email_valid) = create_signal(true);
    let email_input: NodeRef<html::Input> = create_node_ref();

    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'set_process'");

    let email_context =
        use_context::<RwSignal<EmailContext>>().expect("cannot found context 'EmaillContext'");
    let email = String::clone(&email_context.get());

    let send_email_action = create_action(|input: &String| {
        let input = input.clone();
        async move {
            use gloo::timers::future::TimeoutFuture;
            TimeoutFuture::new(1_000).await;

            if input == "fake@email.com" {
                Ok(())
            } else {
                Err("Sorry, your account was not found.")
            }
        }
    });

    create_effect(move |_| {
        match (send_email_action.pending()(), send_email_action.value()()) {
            (false, Some(Ok(_))) => {
                set_err_msg("");
                set_process(SignInProcess::EmailSent);
            }
            (false, Some(Err(err_msg))) => {
                set_err_msg(err_msg);
            }
            _ => (),
        };
    });

    let button_disable = move || !email_valid() || send_email_action.pending()();

    let handle_email_input = move |text: String| {
        set_email_valid(EmailAddress::is_valid(&text));
    };

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let email = email_input.get().expect("email input not exist").value();
        send_email_action.dispatch(email);
    };

    let handle_signin = move |_| {
        set_process(SignInProcess::ValidPassword);
    };

    view! {
        <Container>
            <Logo/>
            <div class="flex flex-col gap-8">
                <div class="mt-16 text-center font-bold">"Forgot your Password?"</div>
                <p class="text-sm text-neutral">
                    "Please fill in the email that you used to register. You will be sent an email with instructions on how to reset your password."
                </p>
                <form class="flex flex-col" on:submit=handle_submit>
                    <Input
                        node_ref=email_input
                        label="Email Address"
                        required=true
                        input_type="email"
                        invalid_message="Email invalid"
                        default=email
                        on_change=handle_email_input
                    />

                    {move || {
                        (!err_msg().is_empty())
                            .then(|| {
                                view! {
                                    <div class="mb-4">
                                        <Alert message=err_msg()/>
                                    </div>
                                }
                            })
                    }}

                    <button class="btn btn-accent btn-lg" disabled=button_disable>
                        {move || {
                            if send_email_action.pending()() {
                                view! {
                                    <span class="animate-spin w-5 fill-primary">
                                        <ArrowRepeat/>
                                    </span>
                                }
                                    .into_view()
                            } else {
                                view! { "SEND EMAIL" }.into_view()
                            }
                        }}

                    </button>
                </form>
                <div class="text-center text-sm">
                    <span class="text-neutral">"Remember your password? "</span>
                    <a class="underline" href="javascript:;" on:click=handle_signin>
                        "Sign In"
                    </a>
                </div>
            </div>
        </Container>
    }
}
