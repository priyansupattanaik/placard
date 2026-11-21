use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PlacardData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub year: Option<String>,
    pub source: String,
    pub license: String,
    pub dimensions: String,
    pub palette: Vec<String>,
    pub is_favorite: bool,
}

#[component]
pub fn Placard(
    data: PlacardData,
    #[prop(optional)] on_apply: Option<Callback<String>>,
    #[prop(optional)] on_toggle_favorite: Option<Callback<String>>,
    #[prop(optional)] on_view_details: Option<Callback<String>>,
) -> impl IntoView {
    let id_for_apply = data.id.clone();
    let id_for_fav = data.id.clone();
    let id_for_details = data.id.clone();
    let is_fav = data.is_favorite;

    view! {
        <article class="placard">
            <div class="placard__header">
                <h3 class="placard__title">{data.title}</h3>
                <p class="placard__artist">{data.artist}</p>
            </div>

            <div class="placard__meta">
                {data.year.map(|yr| view! {
                    <span>{yr}</span>
                    <span class="placard__separator">"·"</span>
                })}
                <span>{data.source}</span>
                <span class="placard__separator">"·"</span>
                <span class="placard__license-pill">{data.license}</span>
                <span class="placard__separator">"·"</span>
                <span>{data.dimensions}</span>
            </div>

            {if !data.palette.is_empty() {
                let swatches = data.palette.clone();
                view! {
                    <div class="placard__palette">
                        {swatches.into_iter().map(|hex| view! {
                            <span
                                class="placard__palette-swatch"
                                style=format!("background-color: {}", hex)
                                title=hex
                            />
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            <div class="placard__actions">
                <div class="placard__actions-group">
                    <button
                        type="button"
                        class="btn btn--primary btn--sm"
                        on:click=move |_| {
                            if let Some(cb) = &on_apply {
                                cb.run(id_for_apply.clone());
                            }
                        }
                    >
                        "Set Wallpaper"
                    </button>
                    <button
                        type="button"
                        class=if is_fav { "btn btn--secondary btn--sm btn--icon" } else { "btn btn--ghost btn--sm btn--icon" }
                        aria-label="Favorite"
                        on:click=move |_| {
                            if let Some(cb) = &on_toggle_favorite {
                                cb.run(id_for_fav.clone());
                            }
                        }
                    >
                        {if is_fav { "♥" } else { "♡" }}
                    </button>
                </div>

                <button
                    type="button"
                    class="btn btn--ghost btn--sm"
                    on:click=move |_| {
                        if let Some(cb) = &on_view_details {
                            cb.run(id_for_details.clone());
                        }
                    }
                >
                    "Details →"
                </button>
            </div>
        </article>
    }
}
