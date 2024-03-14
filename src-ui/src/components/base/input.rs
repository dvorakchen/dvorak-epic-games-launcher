use leptos::*;

#[component]
pub fn Input(
    #[prop(optional)] node_ref: NodeRef<html::Input>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] name: &'static str,
    #[prop(optional, default = "text")] input_type: &'static str,
    #[prop(optional)] default: String,
    #[prop(optional)] label: &'static str,
    #[prop(optional)] invalid_message: &'static str,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    let id = format!("input-{}", rand::random::<usize>());

    let handle_keyup = move |ev| {
        let email = event_target_value(&ev);
        if let Some(cb) = on_change {
            cb(email);
        }
    };

    view! {
        <div class="input-group">
            <input
                class=format!("input {}", class)
                id=id.clone()
                name=name
                node_ref=node_ref
                on:keyup=handle_keyup
                type=input_type
                required=required
                placeholder=""
                value=default
                disabled=disabled
            />
            <label for=id>{label}</label>
            <span class="error text-error">{invalid_message}</span>
        </div>
    }
}
