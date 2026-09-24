# @velora/tauri-template

The standardized, hardened **Tauri v2 (Rust)** application template cloned and configured by `@velora/worker` to generate native installers.

---

## Architecture & Compile-Time Embedding

Rather than reading configuration files and injection scripts from disk at runtime, the Rust binary embeds them into the compiled executable using the `include_str!` compile-time macro:

```rust
// src-tauri/src/lib.rs
const VELORA_CONFIG: &str = include_str!("../velora-config.json");
const INJECTION_CSS: &str = include_str!("../injection.css");
const INJECTION_JS: &str = include_str!("../injection.js");
```

During build preparation, `@velora/worker` writes the project-specific configuration to `velora-config.json`, `injection.css`, and `injection.js`. When `cargo tauri build` executes, these assets become an immutable part of the final application binary.

---

## Directory Layout

```text
packages/tauri-template/
├── package.json
├── scripts/
│   ├── generate-icons.js      # Multi-resolution icon converter (Sharp & ico-endec)
│   ├── render-ci-config.js    # Injects velora-config.json and permissions into Tauri manifest
│   └── validate-template.js   # CI template integrity and manifest validator
├── src/
│   ├── index.html             # Shell wrapper loading remote origin or offline screen
│   └── offline.html           # Embedded fallback screen with retry button
└── src-tauri/
    ├── Cargo.toml             # Rust dependencies (tauri v2, single-instance)
    ├── build.rs               # Cargo build hook
    ├── capabilities/
    │   └── default.json       # Security capabilities scope configuration
    ├── icons/                 # Platform icons (ico, icns, png)
    ├── injection.css          # Injected stylesheet buffer
    ├── injection.js           # Injected JavaScript buffer
    ├── tauri.conf.json        # Active rendered configuration
    ├── tauri.conf.json.template # Base template manifest
    ├── velora-config.json     # Active rendered app parameters
    └── src/
        ├── main.rs            # Native application entrypoint
        └── lib.rs             # Webview setup, system tray, domain allowlist checks
```

---

## Icon Generation Utility

To generate the required multi-format icons (`.ico` for Windows, `.icns` for macOS, `.png` for Linux/Android) from a single 512x512 source image:

```bash
node scripts/generate-icons.js /path/to/source-512x512.png
```
