use leptos::*;

#[component]
pub fn CheckBox(
    #[prop(optional)] label: &'static str,
    #[prop(optional)] default: bool,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] checked: Option<ReadSignal<bool>>,
    #[prop(optional, into)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    let (inner_checked, set_inner_checked) = create_signal(default);

    let handle_click_checkbox = move |_| {
        set_inner_checked.update(|v| {
            *v = !*v;
            if let Some(cb) = on_change {
                cb(*v);
            }
        });
    };

    create_effect(move |_| {
        if let Some(checked) = checked {
            set_inner_checked(checked());
        }
    });

    view! {
        <div class="checkbox-group" on:click=handle_click_checkbox>
            <input class=format!("checkbox {}", class) type="checkbox" prop:checked=inner_checked/>
            <label>{label}</label>
        </div>
    }
}
