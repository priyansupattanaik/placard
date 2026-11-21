use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MatPadding {
    Compact,
    #[default]
    Normal,
    Wide,
}

impl MatPadding {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Compact => "mat--compact",
            Self::Normal => "",
            Self::Wide => "mat--wide",
        }
    }
}

#[component]
pub fn Mat(
    image_url: String,
    title: String,
    #[prop(optional)] badge: Option<&'static str>,
    #[prop(optional)] padding: MatPadding,
) -> impl IntoView {
    let mat_class = format!("mat {}", padding.as_class());

    view! {
        <div class=mat_class>
            <div class="mat__frame">
                <img
                    class="mat__media"
                    src=image_url
                    alt=title
                    loading="lazy"
                />
                {badge.map(|b| view! {
                    <span class="mat__badge">{b}</span>
                })}
            </div>
        </div>
    }
}
