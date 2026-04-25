# Core Algorithms

## Screenshot Capture Pipeline (core/runner.rs)

### Chromium Launch

```
chromiumoxide launches Chromium with flags:
  --no-sandbox
  --disabled-setupid-sandbox
  --disable-dev-shm-usage
  --disable-gpu
```

Sandbox disabled for CI environment compatibility.

### Capture Loop

For each `(story, breakpoint)` pair:

1. Create new browser page
2. Set viewport:
   - width = breakpoint value
   - height = 1080
   - mobile = true if width < 768
3. Navigate to `http://localhost:1337/iframe.html?id={story.id}&viewMode=story`
4. Wait `waitBeforeScreenshot` milliseconds
5. Capture PNG screenshot
6. Save to `.lumendiff/snapshots/{story-id}__[w{breakpoint}px].png`
7. Close page

### Concurrency Model

Uses `futures::stream::buffer_unordered(concurrency)` — at most `concurrency` screenshot tasks run simultaneously. I/O bound, so async concurrency (not threads) is appropriate.

---

## Story Fetching (providers/storybook.rs)

1. Read `{storybookUrl}/index.json`
2. Parse into `StorybookIndex`:
   ```json
   {
     "entries": {
       "button--primary": { "type": "story", "title": "Button", "name": "Primary" },
       "button--docs":    { "type": "docs",  ... }
     }
   }
   ```
3. Filter: skip any entry with `type == "docs"`
4. Build story ID from title/name:
   - Convert title and name to kebab-case via `heck`
   - Result: `{title-kebab}--{name-kebab}`
5. Build URL: `iframe.html?id={original-id}&viewMode=story`
6. Return `Vec<Story>`

---

## Image Diff Algorithm (core/diff.rs)

### Step 1 — Early Exit

```
if baseline.raw_bytes == snapshot.raw_bytes:
    score = 1.0  (identical)
    return PASS
```

Avoids all decoding and processing when images are bitwise identical.

### Step 2 — Dimension Check

```
if baseline.dimensions != snapshot.dimensions:
    return ERROR "dimension mismatch"
```

### Step 3 — Pixel Comparison

```
baseline_bytes = baseline.as_raw()   // RGBA u8 array
snapshot_bytes = snapshot.as_raw()   // RGBA u8 array

diff_pixels = 0
for chunk in zip(baseline_bytes.chunks(4), snapshot_bytes.chunks(4)):
    if chunk_baseline != chunk_snapshot:
        diff_pixels += 1
```

Each chunk = one pixel = [R, G, B, A] bytes.

### Step 4 — Similarity Score

```
total_pixels   = width × height
similarity     = 1.0 - (diff_pixels / total_pixels)
min_acceptable = 1.0 - threshold

PASS if similarity >= min_acceptable
FAIL otherwise
```

### Step 5 — Diff Image (on failure only)

For each pixel:
- **Unchanged**: copy RGBA from baseline, set A = 75 (semi-transparent)
- **Changed**: write (255, 0, 0, 255) — solid red

Saves to `.lumendiff/diffs/{snapshot-name}.png`.

### Parallelism

Diff processing across all stories runs via `rayon::par_iter()` — one thread per CPU core.

---

## Static File Server (server.rs)

`axum` + `tower-http::ServeDir` serves the Storybook static build on `localhost:1337`. Started before any browser automation. Chromium navigates to this local server, avoiding CORS and cross-origin issues with Storybook's iframe.
