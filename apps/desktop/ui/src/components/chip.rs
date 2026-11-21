use leptos::prelude::*;

#[component]
pub fn Chip(
    label: &'static str,
    #[prop(optional)] active: bool,
    #[prop(optional)] color_hex: Option<&'static str>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let mut class_str = "chip".to_string();
    if active {
        class_str.push_str(" chip--active");
    }
    if color_hex.is_some() {
        class_str.push_str(" chip--color");
    }

    view! {
        <button
            class=class_str
            type="button"
            on:click=move |_| {
                if let Some(cb) = &on_click {
                    cb.run(());
                }
            }
        >
            {color_hex.map(|hex| view! {
                <span
                    class="chip__color-dot"
                    style=format!("background-color: {}", hex)
                />
            })}
            <span>{label}</span>
        </button>
    }
}
