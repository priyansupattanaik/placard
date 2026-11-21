use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Destructive,
}

impl ButtonVariant {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Primary => "btn--primary",
            Self::Secondary => "btn--secondary",
            Self::Ghost => "btn--ghost",
            Self::Destructive => "btn--destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ButtonSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Sm => "btn--sm",
            Self::Md => "",
            Self::Lg => "btn--lg",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] icon_only: bool,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "btn {} {} {} {}",
        variant.as_class(),
        size.as_class(),
        if icon_only { "btn--icon" } else { "" },
        class
    );

    view! {
        <button
            class=classes
            disabled=disabled
            on:click=move |ev| {
                if let Some(cb) = &on_click {
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
