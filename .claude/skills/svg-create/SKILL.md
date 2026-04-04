# SVG Creation Skill

Generate clean, professional SVG graphics — silhouettes, icons, and logo lockups — using pure SVG path geometry with no external dependencies.

## When to use

Trigger this skill when asked to:
- Create or redesign an SVG icon, logo, or mascot
- Produce a silhouette from a reference shape description
- Evaluate why an existing SVG looks poor and fix it

## Core Design Process

### 1. Identify the silhouette skeleton

Before drawing, identify the 5–8 structural points that define the outline of the shape. For complex objects (anvil, animal, tool), sketch the profile in terms of:
- **Widest span** (x-axis extremes)
- **Tallest span** (y-axis extremes)
- **Key inflection points** — where the outline changes direction significantly

Rule: a recognizable silhouette requires every major anatomical feature to be proportionally represented. If a feature defines the object (e.g., an anvil horn, a hammer head), it must occupy at least 25–35% of the relevant axis.

### 2. Choose a viewBox

| Use case | Recommended viewBox |
|----------|---------------------|
| Icon (square) | `0 0 64 64` or `0 0 32 32` |
| Logo lockup (wide) | `0 0 240 100` |
| Emblem (medium) | `0 0 120 80` |

Leave a ~8% margin around the shape so it doesn't clip.

### 3. Trace the outline as a single path

Use a **clockwise** compound `<path d="...">` for the main silhouette. This keeps the fill rule consistent and allows easy editing.

**Path command reference:**

| Command | Meaning |
|---------|---------|
| `M x,y` | Move to (start point) |
| `L x,y` | Line to |
| `Q cx,cy x,y` | Quadratic bezier (one control point) |
| `C c1x,c1y c2x,c2y x,y` | Cubic bezier (two control points) |
| `Z` | Close path |

**Horn / taper tips:**
- A tapered projection (e.g., anvil horn, beak, spike) should use `Q` bezier curves for both the top and bottom edges, with control points that bow slightly away from the center line.
- The tip ends at the intersection of the two bezier curves — both curves end at the same `M` anchor point.
- Taper length should be ≥ 25% of total shape width to be visually prominent.

**Example: left-pointing horn (face at x=22–60, y=10–18; tip at x=4, y=14):**
```svg
M 4,14
Q 13,9 22,10     ← top edge: curves upward toward face
L 60,10          ← face top
...
L 22,18          ← face bottom-left
Q 13,19 4,14     ← bottom edge: curves downward back to tip
Z
```

### 4. Add surface depth with fills (no gradients)

Use **2–3 flat tones** from the same palette:

| Layer | Purpose | Typical approach |
|-------|---------|-----------------|
| Body fill | Main silhouette | `fill="#2D3748"` |
| Surface highlight | Top/lit surface | Lighter `<rect>` or `<path>` over face area |
| Cutout / hole | Functional detail | Darker `<rect>` inside body |
| Edge accent | Right or top rim | Short `<line>` or `<polyline>` in accent color |

Avoid: `filter`, `gradient`, `feDropShadow`, external `<image>`, embedded raster data.

### 5. Verify the output

Before declaring done, confirm all of the following:

- [ ] Valid XML — no unclosed tags, no duplicate `xmlns`
- [ ] Correct `viewBox` matches the intended dimensions
- [ ] Shape is recognizable at 32×32 (scale it down mentally)
- [ ] Every major anatomical feature is proportionally represented
- [ ] Self-contained — no `href`, no `xlink:href` to external files
- [ ] No system font dependency in icon-only variants (text okay in logo lockups)
- [ ] Palette follows spec: `#2D3748` body, `#4A5568` face, `#718096` highlights

## Anvil-specific design reference

The Anvil brand icon is a **blacksmith's anvil in left-profile silhouette**.

Key proportions (verified design, 64×64 viewBox):
- **Horn**: tip at x=4, y=14; attaches to face at x=22, y=10–18 (18px long, ~32% of width)
- **Face**: x=22–60 (38px wide), y=10–18 (8px deep); top highlight rect in `#4A5568`
- **Hardy hole**: `<rect x="48" y="14" width="5" height="4">` in `#1A202C`
- **Waist**: body narrows from 34px (face) to 24px (waist) then flares to 48px (base)
- **Base**: y=52–58, x=8–56 (48px wide, widest element)

Logo lockup (240×100): use `transform="scale(1.5) translate(-1,-1)"` on the icon group, then add wordmark text at x=112, y=72, font-size=44, font-weight=600, letter-spacing=-1, fill=#2D3748, font-family='Helvetica Neue', Helvetica, Arial, sans-serif.

## Common mistakes to avoid

| Mistake | Fix |
|---------|-----|
| Horn is a tiny stub triangle | Horn must be ≥ 25% of total width; use bezier curves for both edges |
| Body proportions look wrong | Waist should be noticeably narrower than both face and base |
| SVG renders blank | Check that `fill` is set; `fill="none"` on outer element overrides everything |
| Text looks different across platforms | Embed font as path, or use only system-safe fallback stack |
| Shape clips at edge | Add 6–10% margin inside viewBox |
