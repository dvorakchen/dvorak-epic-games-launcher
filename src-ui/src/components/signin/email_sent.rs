use super::{Container, Logo, SignInProcess};
use crate::components::*;
use leptos::*;

#[component]
pub fn EmailSent() -> impl IntoView {
    let set_process = use_context::<WriteSignal<SignInProcess>>()
        .expect("cannot found Write Signal 'SignInProcess'");

    let handle_signin = move |_| {
        set_process(SignInProcess::ValidPassword);
    };

    view! {
        <Container>
            <Logo/>
            <div class="flex flex-col space-y-8">
                <div class="text-center font-bold mt-6">"Email Sent!"</div>
                <p class="text-sm text-neutral">
                    "An email has been sent to your email address with instructions on how to reset your passwor. "
                    "if your don't receive it within a few minutes, please check that you used the e-mail address for your Epic Games account and try again or contact us for help."
                </p>
                <Button class="btn-lg" on:click=handle_signin>
                    "SIGN IN"
                </Button>
            </div>
        </Container>
    }
}
