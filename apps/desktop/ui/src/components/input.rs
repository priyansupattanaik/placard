use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional)] value: Option<ReadSignal<String>>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] has_icon: bool,
) -> impl IntoView {
    let wrapper_class = if has_icon {
        "input-wrapper input-wrapper--with-icon"
    } else {
        "input-wrapper"
    };

    view! {
        <div class="input-group">
            {label.map(|lbl| view! { <span class="input-label">{lbl}</span> })}
            <div class=wrapper_class>
                <input
                    type="text"
                    class="input"
                    placeholder=placeholder
                    prop:value=move || value.map(|v| v.get()).unwrap_or_default()
                    disabled=disabled
                    on:input=move |ev| {
                        if let Some(cb) = &on_input {
                            cb.run(event_target_value(&ev));
                        }
                    }
                />
            </div>
        </div>
    }
}
