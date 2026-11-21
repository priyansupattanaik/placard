use leptos::prelude::*;

mod components;
mod views;

use components::*;
use views::*;

#[component]
fn App() -> impl IntoView {
    let (active_view, set_active_view) = signal(NavView::Styleguide);
    let (is_dark_theme, set_dark_theme) = signal(false);

    let active_wallpaper = ActiveWallpaperInfo {
        title: "Wheat Field with Cypresses".into(),
        artist: "Vincent van Gogh".into(),
        thumbnail_url: "https://images.metmuseum.org/CRDImages/ep/original/DP130999.jpg".into(),
    };

    view! {
        <div
            class=move || if is_dark_theme.get() { "theme-dark" } else { "theme-light" }
            style="display: flex; width: 100vw; height: 100vh; overflow: hidden; background-color: var(--bg); color: var(--ink);"
        >
            <Sidebar
                active_view=active_view
                on_navigate=Callback::new(move |v| set_active_view.set(v))
                active_wallpaper=active_wallpaper
            />

            <main style="flex: 1; height: 100vh; overflow: hidden; position: relative;">
                <div style="position: absolute; top: 16px; right: 24px; z-index: 10; display: flex; gap: 12px; align-items: center;">
                    <Button
                        variant=ButtonVariant::Secondary
                        size=ButtonSize::Sm
                        on_click=Callback::new(move |_| set_dark_theme.update(|d| *d = !*d))
                    >
                        {move || if is_dark_theme.get() { "☼ Light Mode" } else { "☽ Dark Mode" }}
                    </Button>
                </div>

                {move || match active_view.get() {
                    NavView::Styleguide => view! { <StyleguideView /> }.into_any(),
                    NavView::Discover => view! {
                        <div style="padding: 32px;">
                            <h2 class="text-h2" style="font-family: var(--font-serif); font-style: italic;">"Discover Archival Wallpapers"</h2>
                            <p class="text-body" style="color: var(--ink-2); margin-top: 8px;">
                                "Connect to verified public domain and Creative Commons sources."
                            </p>
                        </div>
                    }.into_any(),
                    NavView::Home => view! {
                        <div style="padding: 32px;">
                            <h2 class="text-h2" style="font-family: var(--font-serif); font-style: italic;">"Current Exhibition"</h2>
                            <p class="text-body" style="color: var(--ink-2); margin-top: 8px;">
                                "Wallpapers scheduled across active displays."
                            </p>
                        </div>
                    }.into_any(),
                    NavView::Library => view! {
                        <div style="padding: 32px;">
                            <h2 class="text-h2" style="font-family: var(--font-serif); font-style: italic;">"Local Collection"</h2>
                            <p class="text-body" style="color: var(--ink-2); margin-top: 8px;">
                                "Stored and categorized wallpapers with museum metadata."
                            </p>
                        </div>
                    }.into_any(),
                    NavView::Sources => view! {
                        <div style="padding: 32px;">
                            <h2 class="text-h2" style="font-family: var(--font-serif); font-style: italic;">"Configured Sources"</h2>
                            <p class="text-body" style="color: var(--ink-2); margin-top: 8px;">
                                "The Met, Art Institute of Chicago, Wallhaven (SFW), NASA APOD, Pexels, Pixabay."
                            </p>
                        </div>
                    }.into_any(),
                    NavView::Settings => view! {
                        <div style="padding: 32px;">
                            <h2 class="text-h2" style="font-family: var(--font-serif); font-style: italic;">"Settings & Preferences"</h2>
                            <p class="text-body" style="color: var(--ink-2); margin-top: 8px;">
                                "Rotation schedules, storage quotas, display assignments."
                            </p>
                        </div>
                    }.into_any(),
                }}
            </main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
