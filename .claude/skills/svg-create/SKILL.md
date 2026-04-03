# SVG Creation Skill

**Trigger:** "create SVG", "design SVG", "make a logo", "generate icon", "SVG logo", "vector graphic", "draw SVG"

You are an expert SVG designer. When asked to create an SVG graphic, follow these principles.

## Design Process

1. **Identify the key landmark points** of the shape before writing any paths. Sketch on paper or in comments.
2. **Use bezier curves** (`C`/`Q` commands) for organic, natural shapes. Use straight lines (`L`, `H`, `V`) only for truly rigid geometric edges.
3. **Close all shape paths** with `Z`.
4. **Test legibility at target size** — if creating an icon, verify it reads at the minimum display size.

## SVG Template

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 W H" role="img" aria-label="Description">
  <defs>
    <!-- gradients, masks, clipPaths go here -->
  </defs>

  <!-- Group related elements -->
  <g id="body">
    <!-- main shape -->
  </g>

  <!-- Highlights / overlays last so they render on top -->
  <g id="highlights" opacity="0.6">
    <!-- thin rects or paths for edge highlights -->
  </g>
</svg>
```

## Path Command Reference

| Command | Meaning | Example |
|---------|---------|---------|
| `M x,y` | Move to (start new subpath) | `M 10,50` |
| `L x,y` | Line to | `L 80,50` |
| `H x` | Horizontal line to | `H 100` |
| `V y` | Vertical line to | `V 80` |
| `C cx1,cy1 cx2,cy2 x,y` | Cubic bezier curve | `C 20,30 60,30 80,50` |
| `Q cx,cy x,y` | Quadratic bezier curve | `Q 50,20 80,50` |
| `A rx,ry rot laf sf x,y` | Arc | `A 10,10 0 0 1 80,50` |
| `Z` | Close path | `Z` |

## Silhouette Tips

For complex silhouettes (animals, tools, objects):
- Split into **structural layers**: base/body → mid-detail → highlights → overlay cutouts
- Use `fill="currentColor"` when the graphic must adapt to dark/light themes
- Use `fill="white"` for cutouts (holes) when on a solid background, or `fill-rule="evenodd"` with a compound path for theme-agnostic cutouts

## Color Palettes

**Dark metal (machinery, tools):**
- Body: `#2D3748`
- Face/surface: `#4A5568`
- Edge highlights: `#718096`

**Monochrome / adaptive:**
- Use `fill="currentColor"` — inherits CSS color
- Cutouts: compound path with `fill-rule="evenodd"` so holes remain transparent

## Verification Checklist

Before finalizing any SVG:
- [ ] `viewBox` dimensions match the design intent
- [ ] All fill paths are closed with `Z`
- [ ] No external `href` references (self-contained SVG)
- [ ] No `<image>` tags unless using data URIs
- [ ] Icon variant: legible at 32×32 (squint test — can you tell what it is?)
- [ ] Logo variant: wordmark and icon optically balanced
- [ ] Renders correctly in both light and dark contexts (or colors are intentionally fixed)
