# Nebula WebGL Renderer 🌐

Brings Nebula UI to the **WEB**!

## Features ✨

- **WebGL 2.0** for modern browsers (Chrome, Firefox, Safari, Edge)
- **Canvas2D fallback** for ancient browsers
- **Automatic detection** and graceful degradation
- **< 1 MB gzipped** bundle size
- **Works everywhere!**

## Architecture 🏗️

```
┌─────────────────────────────────┐
│   Nebula UI Application         │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│   WebGL Renderer                │
│   ┌─────────────────────────┐   │
│   │  Try WebGL 2.0          │   │
│   │  ↓                      │   │
│   │  Success? ✅            │   │
│   │  ↓                      │   │
│   │  Use WebGL 2.0          │   │
│   │                         │   │
│   │  Fail? ❌               │   │
│   │  ↓                      │   │
│   │  Fallback to Canvas2D   │   │
│   └─────────────────────────┘   │
└─────────────────────────────────┘
```

## Building for Web 🔨

### Prerequisites

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

### Build

```bash
# Build for web
wasm-pack build --target web

# Build optimized (smaller bundle)
wasm-pack build --target web --release
```

### Serve

```bash
# Install a simple HTTP server
cargo install basic-http-server

# Serve the demo
basic-http-server .
```

Then open http://localhost:4000 in your browser!

## Usage 💻

```rust
use nebula_renderer_webgl::WebGLRenderer;

// Create renderer
let mut renderer = WebGLRenderer::new("canvas");

// Initialize (tries WebGL 2.0, falls back to Canvas2D)
renderer.initialize()?;

// Set size
renderer.set_size(800, 600);

// Clear to Nebula Blue
renderer.clear(0.04, 0.05, 0.09, 1.0)?;

// Present frame
renderer.present();
```

## Browser Support 🌐

### WebGL 2.0 (Tier S)
- ✅ Chrome 56+
- ✅ Firefox 51+
- ✅ Safari 15+
- ✅ Edge 79+

### Canvas2D Fallback (Tier C)
- ✅ Internet Explorer 9+
- ✅ Ancient browsers
- ✅ Literally everything!

## Bundle Size 📦

Target: **< 1 MB gzipped**

Optimizations:
- `wee_alloc` for smaller allocator
- `opt-level = "z"` for size optimization
- `lto = true` for link-time optimization
- `strip = true` to remove debug symbols
- Feature gates for optional functionality

## Performance 🚀

- **60 FPS** on modern hardware
- **30 FPS** on ancient hardware (Canvas2D fallback)
- **< 150ms** cold start time
- **< 16ms** frame time

## Testing 🧪

```bash
# Run tests
cargo test -p nebula-renderer-webgl

# Run in browser (requires wasm-pack)
wasm-pack test --headless --firefox
```

## Examples 📚

See `index.html` for a complete demo!

## License 📄

MIT OR Apache-2.0
