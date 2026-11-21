use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub shortcut: Option<String>,
}

#[component]
pub fn CommandPalette(
    is_open: ReadSignal<bool>,
    on_close: Callback<()>,
    on_select: Callback<String>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());

    let all_commands = vec![
        CommandItem {
            id: "nav_discover".into(),
            title: "Go to Discover".into(),
            category: "Navigation".into(),
            shortcut: Some("G D".into()),
        },
        CommandItem {
            id: "nav_library".into(),
            title: "Go to Library".into(),
            category: "Navigation".into(),
            shortcut: Some("G L".into()),
        },
        CommandItem {
            id: "nav_settings".into(),
            title: "Go to Settings".into(),
            category: "Navigation".into(),
            shortcut: Some("G S".into()),
        },
        CommandItem {
            id: "nav_styleguide".into(),
            title: "Open Styleguide".into(),
            category: "Design System".into(),
            shortcut: None,
        },
        CommandItem {
            id: "act_rotate".into(),
            title: "Rotate Wallpaper Now".into(),
            category: "Actions".into(),
            shortcut: Some("Space".into()),
        },
        CommandItem {
            id: "act_toggle_theme".into(),
            title: "Toggle Light / Dark Theme".into(),
            category: "Appearance".into(),
            shortcut: Some("T".into()),
        },
    ];

    let filtered_commands = move || {
        let q = query.get().to_lowercase();
        if q.is_empty() {
            all_commands.clone()
        } else {
            all_commands
                .iter()
                .filter(|c| {
                    c.title.to_lowercase().contains(&q) || c.category.to_lowercase().contains(&q)
                })
                .cloned()
                .collect()
        }
    };

    view! {
        <div
            class=move || {
                if is_open.get() {
                    "command-palette-backdrop command-palette-backdrop--open"
                } else {
                    "command-palette-backdrop"
                }
            }
            on:click=move |_| on_close.run(())
        >
            <div
                class="command-palette"
                on:click=move |ev| ev.stop_propagation()
            >
                <div class="command-palette__search">
                    <input
                        type="text"
                        class="command-palette__input"
                        placeholder="Type a command or search..."
                        prop:value=move || query.get()
                        on:input=move |ev| set_query.set(event_target_value(&ev))
                        autofocus=true
                    />
                </div>

                <div class="command-palette__results">
                    {move || {
                        let list = filtered_commands();
                        if list.is_empty() {
                            view! {
                                <div style="padding: 16px; text-align: center; color: var(--ink-3); font-size: 13px;">
                                    "No matching commands"
                                </div>
                            }.into_any()
                        } else {
                            list.into_iter().map(|item| {
                                let id = item.id.clone();
                                view! {
                                    <div
                                        class="command-palette__item"
                                        on:click=move |_| {
                                            on_select.run(id.clone());
                                            on_close.run(());
                                        }
                                    >
                                        <div class="command-palette__item-label">
                                            <span>{item.title}</span>
                                        </div>
                                        {item.shortcut.map(|sc| view! {
                                            <kbd class="command-palette__item-shortcut">{sc}</kbd>
                                        })}
                                    </div>
                                }
                            }).collect_view().into_any()
                        }
                    }}
                </div>

                <div class="command-palette__footer">
                    <span>"Navigation: ↑ ↓"</span>
                    <span>"Select: ↵"</span>
                    <span>"Close: Esc"</span>
                </div>
            </div>
        </div>
    }
}
