# brosdk-sdk

Rust bindings for the native Brosdk Browser SDK.

Dynamically loads the platform DLL/dylib at runtime and exposes a safe, idiomatic Rust API with optional Tauri integration.

## Project layout

```
brosdk-sdk-rust/
├── Cargo.toml              # Crate config (lib + brosdk-demo binary)
├── build.rs                # Tauri build script (tauri-app feature)
├── tauri.conf.json         # Tauri window / bundle config
├── libs/
│   ├── windows-x64/        # brosdk.dll
│   └── macos-arm64/        # brosdk.dylib
├── src/                    # Library crate
│   ├── lib.rs              # Public API re-exports
│   └── brosdk/
│       ├── mod.rs
│       ├── ffi.rs          # Raw C FFI bindings (libloading)
│       └── manager.rs      # High-level safe wrappers + Tauri event bridge
├── src-tauri/              # Tauri demo binary
│   ├── main.rs             # App entry point
│   └── commands.rs         # Tauri invoke commands
└── dist/
    └── index.html          # Demo UI (static, no build step)
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `tauri-app` | yes | Enables Tauri integration (`AppHandle`, event emission) |

Without the feature, `load(lib_path)` takes no `AppHandle` and SDK callbacks only log to stdout.

## Requirements

- Rust 2021 edition
- Tauri v2 prerequisites — see [tauri.app/start/prerequisites](https://v2.tauri.app/start/prerequisites/)
- Native library downloaded from [github.com/browsersdk/brosdk-sdk/releases](https://github.com/browsersdk/brosdk-sdk/releases) and placed under `libs/`:

```
libs/
├── windows-x64/brosdk.dll
└── macos-arm64/brosdk.dylib
```

## Running the demo

```bash
cargo run --bin brosdk-demo
```

The Tauri window loads `dist/index.html`. No frontend build step required.

The demo flow:
1. Enter API Key → click **SDK 初始化**: exchanges the key for a `userSig` via REST, then initializes the native SDK.
2. Click **创建环境**: calls the REST API to create a new browser environment and auto-fills the env ID.
3. Enter the env ID → click **启动环境** / **关闭环境**.

## Library usage

Add to your `Cargo.toml`:

```toml
[dependencies]
brosdk-sdk = { path = "../brosdk-sdk-rust" }
```

### With Tauri

```rust
use brosdk_sdk::{load, init, browser_open, browser_close, shutdown};

// Load the native library and register callbacks
load(app_handle, "libs/windows-x64/brosdk.dll")?;

// Initialize with userSig (obtained by exchanging your API key)
init("your_user_sig", "/path/to/work_dir", 8080)?;

// Open a browser environment — result arrives via "brosdk-event"
browser_open("env-001")?;

// Close a browser environment
browser_close("env-001")?;

shutdown()?;
```

### Without Tauri (feature = no `tauri-app`)

```rust
brosdk_sdk::load("libs/windows-x64/brosdk.dll")?;
brosdk_sdk::init("your_user_sig", "/path/to/work_dir", 8080)?;
```

### Listening to SDK events (Tauri)

The library emits a `brosdk-event` Tauri event for every async SDK callback.

```rust
use brosdk_sdk::SdkEvent;

app.listen("brosdk-event", |event| {
    let e: SdkEvent = serde_json::from_str(event.payload()).unwrap();
    println!("code={} data={}", e.code, e.data);
});
```

Frontend:

```js
const { listen } = window.__TAURI__.event;
await listen("brosdk-event", ({ payload }) => console.log(payload));
```

## API reference

### Core functions

| Function | Description |
|----------|-------------|
| `load(app, path)` | Load native library, register result + cookies-storage callbacks |
| `init(user_sig, work_dir, port)` | Initialize SDK with credentials, returns JSON result string |
| `browser_open(env_id)` | Start a browser environment (async — result via `brosdk-event`) |
| `browser_close(env_id)` | Close a browser environment |
| `token_update(token_json)` | Refresh access token |
| `shutdown()` | Graceful shutdown |

### `SdkEvent`

```rust
pub struct SdkEvent {
    pub code: i32,    // SDK status code
    pub data: String, // JSON payload from the native callback
}
```

## Building

```bash
# Debug
cargo build

# Release
cargo build --release
```

## License

MIT
