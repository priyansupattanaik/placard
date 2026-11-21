use leptos::prelude::*;

#[component]
pub fn Sheet(
    is_open: ReadSignal<bool>,
    on_close: Callback<()>,
    title: &'static str,
    children: Children,
    #[prop(optional)] footer: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                if is_open.get() {
                    "sheet-backdrop sheet-backdrop--open"
                } else {
                    "sheet-backdrop"
                }
            }
            on:click=move |_| on_close.run(())
        />
        <aside
            class=move || {
                if is_open.get() {
                    "sheet sheet--open"
                } else {
                    "sheet"
                }
            }
            aria-modal="true"
            role="dialog"
        >
            <div class="sheet__header">
                <h2 class="sheet__title">{title}</h2>
                <button
                    type="button"
                    class="btn btn--ghost btn--icon"
                    aria-label="Close sheet"
                    on:click=move |_| on_close.run(())
                >
                    "✕"
                </button>
            </div>
            <div class="sheet__body">
                {children()}
            </div>
            {footer.map(|f| view! {
                <div class="sheet__footer">
                    {f()}
                </div>
            })}
        </aside>
    }
}
