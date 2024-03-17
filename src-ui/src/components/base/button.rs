use crate::components::*;
use leptos::*;

#[component]
pub fn Button(
    children: ChildrenFn,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] loading: Option<MaybeSignal<bool>>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
        <button class=format!("btn {}", class) disabled=disabled>

            {move || {
                match loading {
                    Some(v) if v() => {
                        view! {
                            <span class="animate-spin w-5 fill-primary">
                                <ArrowRepeat/>
                            </span>
                        }
                            .into_view()
                    }
                    _ => children().into_view(),
                }
            }}

        </button>
    }
}
