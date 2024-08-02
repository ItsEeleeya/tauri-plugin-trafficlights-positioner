# Tauri Plugin Traffic Lights Positioner
This plugin helps you set a custom inset for the window controls on macOS. Currently only for Tauri v1.*

_This is done without any visible artifacts on resize._

## Get the plugin
Using the command line from crates.io:<br>
`cargo add tauri-plugin-trafficlights-positioner`

Or add it manually to `Cargo.toml`:
```toml
[target.'cfg(target_os = "macos")'.dependencies]
tauri-plugin-trafficlights-positioner = "1.0.0"
```

With git:
```toml
[target.'cfg(target_os = "macos")'.dependencies]
tauri-plugin-trafficlights-positioner = { git = "https://github.com/ItsEeleeya/tauri-plugin-trafficlights-positioner/" }
```

## Usage
```rs
fn main() {
  tauri::Builder::default()
    .setup(move |app| {
      if let Some(window) = app.get_window("main") {
        #[cfg(target_os = "macos")]
        // NOTE: Make sure you only call this ONCE per window.
        let _ = window.setup_traffic_lights_inset(LogicalPosition::new(20.0, 24.0));
      };

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```


If you need extra customization (especially for Windows) use [clearlysid/tauri-plugin-decorum](https://github.com/clearlysid/tauri-plugin-decorum/)

## Credits

Almost all credits for `positioner.rs` goes to [@haasal](https://github.com/haasal), [@charrondev](https://gist.github.com/charrondev) and [Hoppscotch](https://github.com/hoppscotch/hoppscotch)
<br>This is also similar to how it is implemented in the Zed editor.<br>
Original Tauri Issue: https://github.com/tauri-apps/tauri/issues/4789 <br>

(Gist) https://gist.github.com/charrondev/43150e940bd2771b1ea88256d491c7a9

(Hoppscotch) https://github.com/hoppscotch/hoppscotch/blob/286fcd2bb08a84f027b10308d1e18da368f95ebf/packages/hoppscotch-selfhost-desktop/src-tauri/src/mac/window.rs
 
