//! A Tauri v2 plugin that lists the fonts installed on the system and answers
//! CSS-style font queries about them, built on
//! [fontgres](https://github.com/seferino-fernandez/fontgres).
//!
//! Register it with [`init`] and grant `fontgres:default` in a capability.
//! The JavaScript bindings are the `tauri-plugin-fontgres-api` npm package.
//!
//! The fonts load once, on the first command that needs them, and stay
//! loaded until the `reload` command reads them again. Families whose
//! name starts with `.`, such as Apple's private `.SF NS` UI faces, are left
//! out: CSS cannot use them by name.
//!
//! Desktop only. On Android and iOS every command fails with
//! [`Error::UnsupportedPlatform`].

use tauri::{
    Runtime,
    plugin::{Builder, TauriPlugin},
};

mod commands;
mod error;
mod face;
#[cfg(desktop)]
mod fonts;

pub use error::Error;
pub use face::Face;

/// Creates the plugin, to pass to `tauri::Builder::plugin`.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("fontgres")
        .invoke_handler(tauri::generate_handler![
            commands::get_faces,
            commands::get_families,
            commands::query_face,
            commands::reload,
        ])
        .setup(|app, _api| {
            #[cfg(desktop)]
            {
                use tauri::Manager as _;
                app.manage(fonts::Fonts::default());
            }
            #[cfg(mobile)]
            let _ = app;
            Ok(())
        })
        .build()
}
