# SVG Element Creation from WASM

This guide shows how to create SVG elements from your Rust WASM code using rsdomlib.

## Creating SVG Elements

### 1. Create an SVG Container

First, create an SVG element in your HTML:

```html
<div id="svg-container"></div>
```

Then in JavaScript, call:

```javascript
import init, { create_svg_element } from './pkg/rsdomlib.js';

await init();

// Create 600x600 SVG in #svg-container
create_svg_element("svg-container", 600, 600);
// The SVG will have id="rsdomlib-svg" automatically
```

### 2. Add Shapes to the SVG

#### Circle

```javascript
import { add_svg_circle } from './pkg/rsdomlib.js';

// Add red circle at (100, 100) with radius 50
add_svg_circle(
    "rsdomlib-svg",  // parent SVG element id
    100,             // cx (center x)
    100,             // cy (center y)
    50,              // r (radius)
    "red",           // fill color
    "black",         // stroke color
    2                // stroke width
);
```

#### Rectangle

```javascript
import { add_svg_rect } from './pkg/rsdomlib.js';

// Add blue rectangle
add_svg_rect(
    "rsdomlib-svg",  // parent SVG element id
    50,              // x coordinate
    50,              // y coordinate
    200,             // width
    100,             // height
    "blue",          // fill color
    "black",         // stroke color
    2                // stroke width
);
```

#### Line

```javascript
import { add_svg_line } from './pkg/rsdomlib.js';

// Add green line
add_svg_line(
    "rsdomlib-svg",  // parent SVG element id
    0,               // x1
    0,               // y1
    300,             // x2
    300,             // y2
    "green",         // stroke color
    3                // stroke width
);
```

#### Path

```javascript
import { add_svg_path } from './pkg/rsdomlib.js';

// Add path (SVG path data string)
add_svg_path(
    "rsdomlib-svg",
    "M 10 10 L 90 90 Q 50 100, 10 90 Z",  // SVG path data
    "yellow",        // fill color
    "orange",        // stroke color
    2                // stroke width
);
```

#### Text

```javascript
import { add_svg_text } from './pkg/rsdomlib.js';

// Add text
add_svg_text(
    "rsdomlib-svg",  // parent SVG element id
    100,             // x coordinate
    150,             // y coordinate
    "Hello SVG!",    // text content
    "purple",        // fill color
    24               // font size
);
```

### 3. Set SVG Attributes

```javascript
import { set_svg_attr } from './pkg/rsdomlib.js';

// Set custom attributes on any SVG element
set_svg_attr(
    "rsdomlib-svg",     // element id
    "viewBox",          // attribute name
    "0 0 600 600"       // attribute value
);
```

## Available Functions

| Function | Parameters | Description |
|----------|-----------|-------------|
| `create_svg_element(parent_id, width, height)` | parent HTML element id, width, height | Creates SVG in parent, returns id "rsdomlib-svg" |
| `add_svg_circle(svg_id, cx, cy, r, fill, stroke, stroke_width)` | SVG id, x, y, radius, fill, stroke, stroke width | Adds a circle to SVG |
| `add_svg_rect(svg_id, x, y, width, height, fill, stroke, stroke_width)` | SVG id, position, size, colors, stroke | Adds a rectangle to SVG |
| `add_svg_line(svg_id, x1, y1, x2, y2, stroke, stroke_width)` | SVG id, start & end coords, stroke, width | Adds a line to SVG |
| `add_svg_path(svg_id, d, fill, stroke, stroke_width)` | SVG id, path data, fill, stroke, width | Adds a path to SVG |
| `add_svg_text(svg_id, x, y, text, fill, font_size)` | SVG id, coords, text, color, size | Adds text to SVG |
| `set_svg_attr(element_id, attr, value)` | Element id, attribute name, value | Sets any SVG attribute |

## Complete Example

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        svg { border: 1px solid black; display: block; margin: 20px; }
    </style>
</head>
<body>
    <div id="svg-container"></div>
    
    <script type="module">
        import init, { 
            create_svg_element, 
            add_svg_circle, 
            add_svg_rect, 
            add_svg_text 
        } from './pkg/rsdomlib.js';

        async function main() {
            await init();
            
            // Create SVG
            create_svg_element("svg-container", 400, 400);
            
            // Add shapes
            add_svg_rect("rsdomlib-svg", 50, 50, 300, 300, "lightblue", "navy", 2);
            add_svg_circle("rsdomlib-svg", 200, 200, 80, "red", "darkred", 3);
            add_svg_text("rsdomlib-svg", 200, 200, "SVG from Rust!", "white", 20);
        }
        
        main();
    </script>
</body>
</html>
```

## Rust Implementation Details

All SVG functions are in `src/dom.rs` and exposed as WASM via `#[wasm_bindgen]` in `src/lib.rs`. 

SVG elements are created using `Document::create_element_ns()` with the SVG namespace:
```
http://www.w3.org/2000/svg
```

All coordinates and sizes are floating-point values that are converted to strings for SVG attributes.
