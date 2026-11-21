use crate::components::*;
use leptos::prelude::*;

#[component]
pub fn StyleguideView() -> impl IntoView {
    let (is_sheet_open, set_sheet_open) = signal(false);
    let (is_palette_open, set_palette_open) = signal(false);
    let (input_val, set_input_val) = signal(String::from("Search Impressionism..."));
    let (selected_segment, set_selected_segment) = signal(String::from("all"));
    let (switch_val, set_switch_val) = signal(true);
    let (chip_active, set_chip_active) = signal(true);

    let real_placards = vec![
        PlacardData {
            id: "met-436535".into(),
            title: "Wheat Field with Cypresses".into(),
            artist: "Vincent van Gogh".into(),
            year: Some("1889".into()),
            source: "The Met".into(),
            license: "CC0".into(),
            dimensions: "3840×2400".into(),
            palette: vec![
                "#2d4a3e".into(),
                "#c5a059".into(),
                "#4b7189".into(),
                "#dfd2a5".into(),
                "#18261e".into(),
            ],
            is_favorite: true,
        },
        PlacardData {
            id: "aic-27992".into(),
            title: "A Sunday on La Grande Jatte".into(),
            artist: "Georges Seurat".into(),
            year: Some("1884–1886".into()),
            source: "Art Institute of Chicago".into(),
            license: "Public Domain".into(),
            dimensions: "4096×2732".into(),
            palette: vec![
                "#3b5437".into(),
                "#98a66f".into(),
                "#4f4538".into(),
                "#d2c7a3".into(),
                "#282c22".into(),
            ],
            is_favorite: false,
        },
        PlacardData {
            id: "met-45434".into(),
            title: "Under the Wave off Kanagawa (The Great Wave)".into(),
            artist: "Katsushika Hokusai".into(),
            year: Some("ca. 1830–1832".into()),
            source: "The Met".into(),
            license: "CC0".into(),
            dimensions: "3840×2560".into(),
            palette: vec![
                "#1b2d42".into(),
                "#e4d8c5".into(),
                "#5c798a".into(),
                "#95a8b0".into(),
                "#111a24".into(),
            ],
            is_favorite: true,
        },
    ];

    let segment_opts = vec![
        SegmentOption {
            id: "all".into(),
            label: "All Formats".into(),
        },
        SegmentOption {
            id: "photo".into(),
            label: "Still Photo".into(),
        },
        SegmentOption {
            id: "painting".into(),
            label: "Painting".into(),
        },
        SegmentOption {
            id: "video".into(),
            label: "Motion".into(),
        },
    ];

    view! {
        <div style="padding: 32px; overflow-y: auto; height: 100vh; display: flex; flex-direction: column; gap: 40px; box-sizing: border-box;">
            <div>
                <h1 class="text-h1" style="font-family: var(--font-serif); font-style: italic; margin-bottom: 8px;">
                    "PLACARD Design System Styleguide"
                </h1>
                <p class="text-lead" style="color: var(--ink-2); max-width: 680px;">
                    "Interactive component reference for the museum gallery aesthetic. Built with mathematical token curves, WCAG 2.2 AA contrast adherence, and zero synthetic ornamentation."
                </p>
            </div>

            <!-- BUTTONS -->
            <section style="display: flex; flex-direction: column; gap: 16px;">
                <h2 class="text-h3" style="border-bottom: 1px solid var(--line); padding-bottom: 8px;">
                    "1. Buttons & Actions"
                </h2>
                <div style="display: flex; flex-wrap: wrap; gap: 12px; align-items: center;">
                    <Button variant=ButtonVariant::Primary>"Primary Action"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary Action"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost Button"</Button>
                    <Button variant=ButtonVariant::Destructive>"Destructive Action"</Button>
                    <Button variant=ButtonVariant::Primary disabled=true>"Disabled Primary"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Small Button"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>"Large Button"</Button>
                    <Button variant=ButtonVariant::Secondary icon_only=true>"♥"</Button>
                </div>
            </section>

            <!-- INPUTS & CONTROLS -->
            <section style="display: flex; flex-direction: column; gap: 16px;">
                <h2 class="text-h3" style="border-bottom: 1px solid var(--line); padding-bottom: 8px;">
                    "2. Inputs, Segments & Switches"
                </h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px; align-items: start;">
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <Input
                            label="SEARCH THE ARCHIVE"
                            placeholder="Type artist, movement, or subject..."
                            value=input_val
                            on_input=Callback::new(move |val| set_input_val.set(val))
                        />
                        <Input
                            label="DISABLED INPUT"
                            placeholder="Read-only field"
                            disabled=true
                        />
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 16px;">
                        <div>
                            <span class="text-micro" style="display: block; margin-bottom: 8px; color: var(--ink-2);">
                                "FORMAT FILTER"
                            </span>
                            <SegmentControl
                                options=segment_opts
                                selected=selected_segment
                                on_select=Callback::new(move |s| set_selected_segment.set(s))
                            />
                        </div>
                        <div style="display: flex; align-items: center; gap: 16px; margin-top: 8px;">
                            <Switch
                                checked=switch_val
                                on_toggle=Callback::new(move |b| set_switch_val.set(b))
                                label="Automatic Rotation Active"
                            />
                        </div>
                    </div>
                </div>
            </section>

            <!-- CHIPS & PALETTES -->
            <section style="display: flex; flex-direction: column; gap: 16px;">
                <h2 class="text-h3" style="border-bottom: 1px solid var(--line); padding-bottom: 8px;">
                    "3. Chips & Color Swatches"
                </h2>
                <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                    <Chip
                        label="Active Filter"
                        active=chip_active.get()
                        on_click=Callback::new(move |_| set_chip_active.update(|v| *v = !*v))
                    />
                    <Chip label="Impressionism" />
                    <Chip label="Post-Impressionism" />
                    <Chip label="Japanese Woodblock" />
                    <Chip
                        label="Ochre #c5a059"
                        color_hex="#c5a059"
                    />
                    <Chip
                        label="Prussian Blue #1b2d42"
                        color_hex="#1b2d42"
                    />
                    <Chip
                        label="Verdigris #2d4a3e"
                        color_hex="#2d4a3e"
                    />
                </div>
            </section>

            <!-- PLACARDS -->
            <section style="display: flex; flex-direction: column; gap: 16px;">
                <h2 class="text-h3" style="border-bottom: 1px solid var(--line); padding-bottom: 8px;">
                    "4. Museum Placards (Core Signature)"
                </h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 24px;">
                    {real_placards.into_iter().map(|item| {
                        view! {
                            <Placard
                                data=item
                                on_apply=Callback::new(|id| leptos::logging::log!("Set wallpaper: {}", id))
                                on_toggle_favorite=Callback::new(|id| leptos::logging::log!("Toggle fav: {}", id))
                                on_view_details=Callback::new(move |_| set_sheet_open.set(true))
                            />
                        }
                    }).collect_view()}
                </div>
            </section>

            <!-- MAT FRAMING -->
            <section style="display: flex; flex-direction: column; gap: 16px;">
                <h2 class="text-h3" style="border-bottom: 1px solid var(--line); padding-bottom: 8px;">
                    "5. Passe-partout Mat Framing"
                </h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(340px, 1fr)); gap: 24px;">
                    <Mat
                        image_url="https://images.metmuseum.org/CRDImages/ep/original/DP130999.jpg".into()
                        title="Wheat Field with Cypresses".into()
                        badge="4K UHD"
                        padding=MatPadding::Normal
                    />
                    <Mat
                        image_url="https://images.metmuseum.org/CRDImages/as/original/DP130155.jpg".into()
                        title="The Great Wave".into()
                        badge="CC0"
                        padding=MatPadding::Compact
                    />
                    <Mat
                        image_url="https://images.metmuseum.org/CRDImages/ep/original/DP130999.jpg".into()
                        title="Wheat Field with Cypresses (Framed)".into()
                        badge="Wide Mat"
                        padding=MatPadding::Wide
                    />
                </div>
            </section>

            <!-- OVERLAYS & DRAWERS -->
            <section style="display: flex; flex-direction: column; gap: 16px; margin-bottom: 40px;">
                <h2 class="text-h3" style="border-bottom: 1px solid var(--line); padding-bottom: 8px;">
                    "6. Drawers & Command Palette"
                </h2>
                <div style="display: flex; gap: 12px;">
                    <Button
                        variant=ButtonVariant::Primary
                        on_click=Callback::new(move |_| set_sheet_open.set(true))
                    >
                        "Open Artwork Sheet"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=Callback::new(move |_| set_palette_open.set(true))
                    >
                        "Trigger Command Palette (Ctrl+K)"
                    </Button>
                </div>
            </section>

            <!-- Detail Sheet Drawer -->
            <Sheet
                is_open=is_sheet_open
                on_close=Callback::new(move |_| set_sheet_open.set(false))
                title="Wheat Field with Cypresses"
            >
                <div style="display: flex; flex-direction: column; gap: 16px;">
                    <div class="placard-artist" style="font-size: 15px;">"Vincent van Gogh (Dutch, 1853–1890)"</div>
                    <div class="text-body" style="color: var(--ink-2); line-height: 1.6;">
                        "Cypresses gained prominence in Van Gogh's work in late June 1889 when he resolved to dedicate one of his first series in Saint-Rémy to the towering trees. Painted on location in the Alpilles countryside, this canvas was conceived as one of his 'most characteristic' landscapes."
                    </div>
                    <div style="border-top: 1px solid var(--line); padding-top: 12px; display: flex; flex-direction: column; gap: 8px;">
                        <div class="text-micro" style="color: var(--ink-3);">"ACQUISITION & CITATION"</div>
                        <div class="text-body" style="font-size: 13px;">"Purchase, The Annenberg Foundation Gift, 1993 (1993.132)"</div>
                        <div class="text-micro" style="color: var(--ink-3); margin-top: 8px;">"LICENSING & RIGHTS"</div>
                        <div class="text-body" style="font-size: 13px;">"Open Access / Creative Commons Zero (CC0 1.0 Universal)"</div>
                    </div>
                </div>
            </Sheet>

            <!-- Command Palette -->
            <CommandPalette
                is_open=is_palette_open
                on_close=Callback::new(move |_| set_palette_open.set(false))
                on_select=Callback::new(|cmd| leptos::logging::log!("Selected command: {}", cmd))
            />
        </div>
    }
}
