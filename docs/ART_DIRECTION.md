# ART DIRECTION AND UI SPECIFICATION

## 12.1 Concept: "Placard"
A museum wall. The wallpaper is the artwork; the interface is the mat board, the wall, and the label. Every item shows a placard: title in italic serif, artist in sans, then a mono line with year/source/license. This is not decoration; it is how the app credits the work.

Influences, each used in one specific way:
- **Muji**: restraint. If an element does not do a job, remove it.
- **Aesop**: warm, literary, material. Paper tones, editorial photography choices, quiet copy.
- **Pentagram**: typography carries the layout. Big, tight, grotesque headings on a strict grid.
- **Linear**: precision. Hairlines, tight spacing scale, keyboard-first, fast.
- **Notion**: structure. Block-based Home, database-style Library views, a sidebar that nests.
- **Airbnb**: one big, friendly search pill that expands into focused segments.
- **Duolingo**: onboarding mechanics only (one question per screen, progress, instant feedback, small wins). Not its colors, mascot, or tone.
- **Apple**: motion quality (springs, continuity, no gratuitous movement).
- **Framer**: tactile micro-interactions where they add meaning.
- **Stripe**: landing page craft (layered, precise product visuals, strong scroll storytelling).

## 12.2 Anti-slop rules (hard blacklist)
The result must not look like a generic AI-generated app. Do NOT use:
- Purple/blue/pink gradients, gradient text, mesh-gradient or aurora backgrounds, glow effects.
- Glassmorphism cards, heavy blur, frosted panels. There are no exceptions in v1.
- Emoji or sparkle icons anywhere in UI or copy. No mascot. No robot imagery.
- The default "centered hero + 3 feature cards with icons + testimonial row + pricing" template.
- Uniformly huge border radii (no `rounded-2xl` look) and heavy drop shadows.
- Stock photography of people, fake dashboards with random numbers, fake logos, fake reviews, fake stats.
- Generic phrases: "seamless", "elevate", "unlock", "supercharge", "powerful", "revolutionize", "next-generation", "AI-powered", "effortless", "game-changing".
- Mixed icon styles, mixed corner radii, or more than 2 font families plus 1 mono.
- Lorem ipsum, "Feature One", placeholder avatars.

Instead: real content (actual public-domain artworks and real wallpapers from the sources), real product UI built in HTML/CSS on the landing page, specific verbs, plain words.

## 12.3 Color tokens (CSS custom properties in `styles/tokens.css`)

Light ("Paper"):
```css
--bg:#F3F0E8; --surface:#EBE7DD; --raised:#FAF8F3;
--ink:#181715; --ink-2:#55514A; --ink-3:#8A857B;
--line:#D8D3C7; --line-strong:#BDB7A9;
--danger:#A8392B; --ok:#4F6B4A; --warn:#9A6A1F;
```
Dark ("Gallery"):
```css
--bg:#0E0E0D; --surface:#151513; --raised:#1C1C19;
--ink:#EDE9E0; --ink-2:#A29D91; --ink-3:#6E6A61;
--line:#262521; --line-strong:#38362F;
--danger:#D9695B; --ok:#8FAE88; --warn:#D1A04F;
```
Accent (`--accent`) is **sampled from the currently applied wallpaper**:
- Compute the dominant color in Oklab, convert to OKLCH, clamp lightness to 0.55 to 0.72 (light theme) or 0.70 to 0.82 (dark theme), clamp chroma to at most 0.12, keep hue.
- Fallback when nothing is applied: `#B0532E` (clay).
- Allowed uses for `--accent`: focus ring, selected-state indicator (2px line), progress fill, active-segment underline, link hover. Nothing else. Primary buttons are `--ink` on `--bg` (inverted), not accent.
- Contrast: all text/background pairs must meet WCAG 2.2 AA (4.5:1 body, 3:1 large and UI). Write an `xtask` that computes and fails the build when a token pair or the clamped accent breaks this. Do not claim compliance without that check passing.

## 12.4 Typography
Fonts (all must be open-licensed; VERIFY each license and include the license files; self-host as WOFF2; no runtime CDN):
- **Geist** (sans): UI and headings.
- **Instrument Serif** (italic and regular): placard titles and rare editorial accents only.
- **Geist Mono**: labels, metadata, shortcuts, numbers.
If any of these fails the license check, stop and record it in `docs/OPEN_QUESTIONS.md` with two alternatives.

Scale (px / line-height / tracking / weight):
```
micro   11 / 16 / +0.06em / 500   (mono, uppercase)
caption 12 / 16 / +0.01em / 400
body    14 / 20 / 0       / 400
lead    16 / 24 / -0.005em/ 400
h3      20 / 26 / -0.015em/ 500
h2      28 / 32 / -0.025em/ 500
h1      44 / 46 / -0.035em/ 500
display 72 / 72 / -0.045em/ 500   (app: onboarding and empty states; site: up to 120)
```
Rules: sentence case everywhere (no Title Case, no ALL CAPS except mono micro labels). Never bold body text for emphasis; use ink vs ink-2. Numbers use tabular figures (`font-variant-numeric: tabular-nums`). Max line length 64ch for prose.

Placard anatomy (component `Placard`):
1. Title: Instrument Serif italic 20px (h3 size), `--ink`.
2. Artist/author: Geist 13px, `--ink-2`. Omit the line if unknown (never print "Unknown").
3. Meta: Geist Mono micro, `--ink-3`: `YEAR / SOURCE / LICENSE / WxH`, separated by ` / `.

## 12.5 Layout, spacing, shape
- 4px base unit. Spacing scale: 4, 8, 12, 16, 24, 32, 48, 64, 96, 128.
- Radius: images 2px; controls 6px; panels 10px; search pill 999px (the only pill).
- 1px hairlines (`--line`) separate regions. No shadows except popovers/menus: `0 8px 32px rgba(0,0,0,0.18)` in light, `0 8px 32px rgba(0,0,0,0.5)` in dark.
- App shell: left sidebar 232px (collapsible to 56px), content area with max-width 1440px, 12 columns, 24px gutters.
- Image cells are a uniform 16:10 "mat" with `object-fit: cover` and a 12px inner mat of `--raised`; the placard sits beneath. The true aspect ratio is printed in the meta line.
- Density: Linear-like. Row height 32px in lists, 28px in menus.

## 12.6 Onboarding (Duolingo mechanics, Placard look)
Full-window, one decision per screen, no sidebar. A 2px progress line at the very top (ink-3 track, `--accent` fill, spring-animated). Back arrow top-left, "Skip" top-right on every step except 1 and 7. Primary button bottom-right: "Continue", disabled until the step is valid. Selection cards respond instantly: scale 0.98 on press, spring to 1, a 2px accent underline draws in. A step transition is a horizontal shared-axis move of 24px with fade, 480ms.

Steps (exact):
1. **Welcome.** Display text and one line. Button "Begin". On entry, three lines (headline, support line, button) fade up with a 120ms stagger. No typewriter effect.
2. **Taste.** "Choose three or more." Grid of 12 bundled tiles: landscape, architecture, abstract, space, ocean, forest, minimal, city, painting, texture, night, macro. Tiles are real CC0 images that you download once during Phase 4 from The Met / AIC APIs (or NASA for space), compress to 800px wide JPEG/WebP under 120 KB each, commit to `assets/onboarding/`, and list in `docs/LICENSES.md` with source URLs and credits. The tile label uses the placard format. Needs 3 to 6 selected.
3. **Format.** Pictures / Motion / Both. If the platform has no video capability, the Motion and Both cards are shown disabled with the calm explanation (13.4).
4. **Displays.** Draw the detected displays to scale (simple rectangles, hairline borders). Click to toggle which receive wallpapers. Show "Same on all" vs "Different on each".
5. **Rhythm.** Options: every 30 minutes, every hour, every 6 hours, daily, on login, never. Toggle: pause on battery (only if battery detected). Toggle: start at login (default off).
6. **Sources.** Keyless sources on by default. Keyed sources show "Add a key" with the inline 3-step guide (5.4). Skipping is fine; one source must be enabled to continue.
7. **First hang.** The app searches with the taste profile, downloads one item, shows it large with its placard, and a primary button "Hang it". On press, apply to the chosen displays, then show a quiet confirmation line "Hung on <display names>." The only celebration is the wallpaper changing. Then "Open PLACARD" button.

Persist onboarding answers as settings; allow re-running from Settings.

## 12.7 Screens

**Global chrome:** sidebar items: Home, Discover, Library, Collections (nested, drag to reorder), Schedule, Sources, Settings. Bottom of sidebar: "Now hanging" mini placard with prev/next/pause. Top bar: breadcrumb (left), search pill (center, only on Discover and Home), Cmd/Ctrl+K hint (right).

**Home (Notion-like blocks).** A vertical page of reorderable blocks; "Edit page" toggles drag handles. Default blocks:
1. Now hanging: large image with placard, per-display switcher, prev/next/pause/favorite.
2. Today's pick: one item chosen from the taste profile, cached per day.
3. Recently added: horizontal strip.
4. Scheduled: next fire time per display.
5. Sources: status chips with quota remaining.
Persist block order and visibility in `settings`.

**Discover (Airbnb-like search).**
- Search pill with four segments separated by hairlines: **Subject | Palette | Format | Source**. Collapsed summary example: `Ocean / Blue / 4K and up / All sources`.
- Clicking a segment opens a floating panel anchored under the pill (shared-element expand, 280ms spring):
  - Subject: text input plus suggested chips from the taste profile.
  - Palette: 12 swatches, up to 3 selectable, plus "Match current wallpaper".
  - Format: media kind (Pictures, Motion, Both), minimum resolution (1080p, 1440p, 4K, 5K and up), orientation (Landscape, Portrait, Any).
  - Source: checkbox list of enabled sources with status.
- Results: uniform grid of mats with placards, streamed in per source, with a thin per-source progress indicator under the pill. Hover: video items scrub with pointer x; images lift 1.5% scale. Click opens a detail sheet (right-side panel 480px): large preview, placard, tags, "Download", "Hang now", "Add to collection", "Open at source".
- Debounce 250ms. Cancel in-flight searches when the query changes.

**Library (Notion database views).** View toggle: Gallery / Table. Properties as columns or chips: Source, Resolution, Media, Added, Colors, License, Favorite. Filter chips, sort menu, group by (Source, Added month, Dominant hue). Multi-select with Shift/Cmd/Ctrl, bulk actions bar (Add to collection, Favorite, Delete). Selection uses accent underline, not filled backgrounds.

**Collections.** Page per collection: title (editable inline, h1), description line, same views as Library. Drag items to reorder (FLIP animation).

**Schedule.** One card per display showing its schedule: pool, trigger, interval, media kinds, next fire. Timeline strip (24h) with past applications as ticks.

**Sources.** One row per source: name, media kinds, enabled switch, key state, quota/limits text from the descriptor, "Test", "Open docs". Credits link.

**Settings.** Sections: General, Appearance (theme, reduce motion, accent source), Storage (location, quota, tidy), Rotation, Startup, Privacy (static text), Updates, About, Credits and licenses.

**Credits and licenses.** Two tabs: Artworks (every item in the library with its full attribution, searchable) and Software (generated license report from `cargo` metadata via `xtask`).

**Command palette (Cmd/Ctrl+K).** Centered 560px panel. Sections: Actions (Next wallpaper, Pause rotation, Favorite current), Go to (screens, collections), Search (typed text becomes a Discover query). Fuzzy match. Fully keyboard-operable.

**Tray / menu bar popover (320px).** Current placard (small), prev / pause / next / favorite buttons, display selector if more than one, "Open PLACARD", "Quit".

**Shortcuts (Cmd on macOS, Ctrl elsewhere):** K palette, F focus search, J/K or arrows move selection, Space quick preview, Enter hang, D download, S favorite, [ and ] previous/next wallpaper, comma open Settings.

## 12.8 States (every list, grid, and panel must implement all)
- **Loading:** blurhash placeholder for images; a single hairline progress bar for sources; never spinners on image grids. Text skeletons are a 1px line in `--line`, not shimmering blocks.
- **Empty:** a short sentence and one action (copy in 13.2).
- **Error:** one sentence what happened, one sentence what to do, a retry button. No stack traces, no error codes in the primary text (put the code in a collapsed "Details" row).
- **Offline:** banner line under the top bar; downloaded library remains fully usable.
- **Rate limited:** show the source chip as "Resting until HH:MM" using the reset time; never hammer.
- **Partial:** one source failing never blocks others; show a small per-source note.

## 12.9 Motion and interaction
Principles: motion explains continuity (where something came from, where it went). Nothing moves for decoration. Only animate `transform` and `opacity`, plus FLIP for layout changes. Target 60 fps; if a frame budget is missed in a test, simplify the animation.

Tokens:
```
dur-micro 120ms   dur-standard 280ms   dur-page 480ms   dur-hero 720ms
spring.snappy   stiffness 320, damping 30, mass 1     (buttons, toggles, chips)
spring.gentle   stiffness 170, damping 26, mass 1     (panels, sheets, cards)
spring.slow     stiffness 120, damping 22, mass 1     (page transitions, hero)
```
Implementation (Rust only):
- An `xtask` generates CSS `linear()` easing strings for each spring and writes `styles/motion.generated.css`. Provide a `@supports not (transition-timing-function: linear(0, 1))` fallback of `cubic-bezier(0.22, 1, 0.36, 1)`. VERIFY the webviews on all three OSes support `linear()`; record results.
- Shared-element transitions (card to detail sheet, segment to panel) use a small Rust `motion` module with the Web Animations API via `web-sys` and FLIP. If the View Transitions API is available and reliable in the webviews you can use it, but VERIFY on each OS first.
- Required interactions:
  1. Primary button: magnetic pull of at most 6px toward the pointer on fine pointers only; spring back on leave.
  2. Card hover: image scale 1.015 over `dur-page`; placard unaffected.
  3. Video card hover: horizontal pointer position scrubs the preview; leave resets.
  4. Press feedback: scale 0.98, spring back.
  5. Drag reorder (collections, Home blocks): FLIP for siblings, lifted item gets a hairline outline, no shadow.
  6. Apply: the detail sheet image cross-dissolves to the "Now hanging" block over 480ms while the OS wallpaper changes.
  7. Page transitions: 24px shared-axis with fade, `spring.slow`.
- `prefers-reduced-motion` and the in-app "Reduce motion" setting: replace all movement with 120ms opacity fades; disable magnetic and scrub-follow effects.

## 12.10 Accessibility
Keyboard reachable everywhere with a visible 2px accent focus ring (offset 2px). Correct roles and names for the pill segments, the palette, menus, dialogs (focus trap, Escape closes). Hit targets at least 32x32 px. Never convey state by color alone. Minimum text 11px, only for mono micro labels. Screen reader labels for every icon-only button. Test with the OS screen reader at least once per platform if available; otherwise report "not verified".

## 12.11 Icons and imagery
- Icons: one set, 20px grid, 1.5px stroke, round caps. Use a permissively licensed set (Lucide, ISC; VERIFY license text and include it) and inline only the SVGs actually used. Do not mix sets. Do not draw custom icons unless one is missing; custom ones must match the stroke spec.
- App icon and wordmark: a plain mark that fits the concept (a square frame with a small offset label line). Provide SVG sources, plus the required PNG/ICO/ICNS sizes generated by the Tauri icon tooling (VERIFY the command). Wordmark set in Geist 500, tracking -0.03em, lowercase "placard" unless `brand.toml` says otherwise.
- Imagery in the app is always the actual wallpapers. Imagery on the landing page is real public-domain artworks and real wallpapers from the sources, credited in captions.
