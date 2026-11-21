use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavView {
    Discover,
    Home,
    Library,
    Sources,
    Settings,
    Styleguide,
}

impl NavView {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Discover => "Discover",
            Self::Home => "Home",
            Self::Library => "Library",
            Self::Sources => "Sources",
            Self::Settings => "Settings",
            Self::Styleguide => "Styleguide",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveWallpaperInfo {
    pub title: String,
    pub artist: String,
    pub thumbnail_url: String,
}

#[component]
pub fn Sidebar(
    active_view: ReadSignal<NavView>,
    on_navigate: Callback<NavView>,
    #[prop(optional)] active_wallpaper: Option<ActiveWallpaperInfo>,
) -> impl IntoView {
    let nav_items = vec![
        NavView::Discover,
        NavView::Home,
        NavView::Library,
        NavView::Sources,
        NavView::Settings,
        NavView::Styleguide,
    ];

    view! {
        <aside class="sidebar">
            <div class="sidebar__brand">
                <h1 class="sidebar__logo-title">"PLACARD"</h1>
                <span class="sidebar__tagline">"The wall is the work"</span>
            </div>

            <nav class="sidebar__nav">
                {nav_items.into_iter().map(|item| {
                    let is_active = move || active_view.get() == item;
                    view! {
                        <button
                            type="button"
                            class=move || {
                                if is_active() {
                                    "sidebar__nav-item sidebar__nav-item--active"
                                } else {
                                    "sidebar__nav-item"
                                }
                            }
                            on:click=move |_| on_navigate.run(item)
                        >
                            <span>{item.label()}</span>
                        </button>
                    }
                }).collect_view()}
            </nav>

            {active_wallpaper.map(|wall| view! {
                <div class="sidebar__footer">
                    <span class="sidebar__current-label">"On the Wall"</span>
                    <div class="sidebar__current-wall">
                        <img
                            class="sidebar__current-thumb"
                            src=wall.thumbnail_url
                            alt=wall.title.clone()
                        />
                        <div class="sidebar__current-info">
                            <span class="sidebar__current-title">{wall.title}</span>
                            <span class="sidebar__current-artist">{wall.artist}</span>
                        </div>
                    </div>
                </div>
            })}
        </aside>
    }
}
