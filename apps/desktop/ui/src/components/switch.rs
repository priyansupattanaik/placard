use leptos::prelude::*;

#[component]
pub fn Switch(
    checked: ReadSignal<bool>,
    on_toggle: Callback<bool>,
    #[prop(optional)] label: Option<&'static str>,
) -> impl IntoView {
    view! {
        <label class="switch">
            <input
                type="checkbox"
                class="switch__input"
                prop:checked=move || checked.get()
                on:change=move |ev| {
                    let checked = event_target_checked(&ev);
                    on_toggle.run(checked);
                }
            />
            <span class="switch__track">
                <span class="switch__thumb" />
            </span>
            {label.map(|lbl| view! { <span class="switch__label">{lbl}</span> })}
        </label>
    }
}
