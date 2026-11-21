use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SegmentOption {
    pub id: String,
    pub label: String,
}

#[component]
pub fn SegmentControl(
    options: Vec<SegmentOption>,
    selected: ReadSignal<String>,
    on_select: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="segment-control" role="tablist">
            {options.into_iter().map(|opt| {
                let opt_id_class = opt.id.clone();
                let opt_id_aria = opt.id.clone();
                let opt_id_click = opt.id.clone();
                let sel_for_class = selected;
                let sel_for_aria = selected;

                view! {
                    <button
                        type="button"
                        class=move || {
                            if sel_for_class.get() == opt_id_class {
                                "segment-item segment-item--active"
                            } else {
                                "segment-item"
                            }
                        }
                        role="tab"
                        aria-selected=move || (sel_for_aria.get() == opt_id_aria).to_string()
                        on:click=move |_| {
                            on_select.run(opt_id_click.clone());
                        }
                    >
                        {opt.label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
