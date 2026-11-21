# Wallpaper Sources Reference & Verification Facts

Verified against public documentation and official APIs on 2026-11-21.

---

## 1. Allowed Sources (v1)

### 1. The Met Collection API (Metropolitan Museum of Art)
- **Media**: Pictures (public domain open access)
- **Authentication**: None
- **Rate Limit**: Recommended max 80 req/sec
- **Rules**: Objects filtered where `isPublicDomain == true` and primary image is populated. Image license: CC0.
- **Attribution Format**: Title, Artist, Date, Credit Line, Object URL at metmuseum.org.

### 2. Art Institute of Chicago API
- **Media**: Pictures (public domain)
- **Authentication**: None
- **Rate Limit**: ~60 req/min per IP
- **Rules**: Filter `is_public_domain = true`. IIIF base endpoint combined with `image_id`. Core metadata CC0.
- **Attribution Format**: Title, Artist display, Date, Credit line, artic.edu link.

### 3. Wallhaven API
- **Media**: Pictures
- **Authentication**: Optional user key (lifts search rate limits)
- **Rate Limit**: 45 req/min
- **Rules**: Strictly SFW purity `purity=100` hardcoded.
- **License Notice**: "Personal use, license not specified by source", links to wallhaven.cc.

### 4. NASA APOD (Astronomy Picture of the Day)
- **Media**: Pictures
- **Authentication**: Free personal key or `DEMO_KEY` (tight limit)
- **Rules**: Filter `media_type == "image"`. Discard entries with copyright field. Credit "NASA APOD".

### 5. Pexels API
- **Media**: Pictures + Videos
- **Authentication**: Required user key (free)
- **Rate Limit**: 200 req/hr, 20,000/month; respect `X-Ratelimit-*` headers.
- **Attribution**: Photographer name and Pexels attribution link.

### 6. Pixabay API
- **Media**: Pictures + Videos
- **Authentication**: Required user key (free)
- **Rate Limit**: 100 req/60s. Responses cached for 24h. Standard key caps images at 1280px (honestly labeled in UI).
- **Attribution**: Pixabay and creator credit.

### 7. Local Import
- **Media**: Pictures (jpg, png, webp, avif) & Videos (mp4, webm, mov, gif).
- **Authentication**: None.
- **License Notice**: "User-provided".

---

## 2. Excluded Sources & Rationale
- **Unsplash**: API policy forbids client-side user API key entry and requires central server proxies and hotlink tracking.
- **Bing Daily Image**: Unofficial, undocumented reverse-engineered API.
- **Reddit / Pinterest / DeviantArt**: No official public royalty-free wallpaper API without restrictive terms or scraping.
