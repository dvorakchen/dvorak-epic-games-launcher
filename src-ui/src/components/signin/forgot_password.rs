use super::{Container, Logo, SignInProcess};
use crate::components::*;
use leptos::*;

#[component]
pub(super) fn ForgotPassword(email: String) -> impl IntoView {
    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'set_process'");
    let email_for_signin = email.clone();

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
    };

    let handle_signin = move |_| {
        let email_for_signin = email_for_signin.clone();
        logging::log!("{}", email_for_signin);
        set_process(SignInProcess::ValidPassword(email_for_signin));
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
                        label="Email Address"
                        required=true
                        input_type="email"
                        invalid_message="Email invalid"
                        default=email
                    />
                </form>
                <button class="btn btn-accent btn-lg">"SEND EMAIL"</button>
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
