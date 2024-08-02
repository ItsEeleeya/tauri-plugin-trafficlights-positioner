use tauri::{LogicalPosition, Runtime, Window};

#[cfg(target_os = "macos")]
mod positioner;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate cocoa;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
pub trait WindowExt {
    fn setup_traffic_lights_inset(&self, offset: LogicalPosition<f64>) -> tauri::Result<()>;
}

#[cfg(target_os = "macos")]
impl<R: Runtime> WindowExt for Window<R> {
    fn setup_traffic_lights_inset(&self, inset: LogicalPosition<f64>) -> tauri::Result<()> {
        let win = self.clone();
        self.on_window_event(move |event| match event {
            tauri::WindowEvent::ThemeChanged(_) => {
                let win_clone = win.clone();

                let _ = win.clone().run_on_main_thread(move || {
                    positioner::update(&win_clone, &inset);
                });
            }
            _ => (),
        });

        let win = self.clone();
        self.run_on_main_thread(move || {
            positioner::setup_nswindow_delegates(win, inset);
        })
    }
}
