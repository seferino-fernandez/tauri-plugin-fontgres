//! The commands JavaScript invokes as `plugin:fontgres|<name>`.

#[cfg(desktop)]
#[expect(
    clippy::unreachable,
    reason = "#[tauri::command] checks an async command's return type with `let _check: T = unreachable!()`"
)]
mod desktop {
    use fontgres_core::models::FontQuery;
    use tauri::State;

    use crate::fonts::{self, Fonts};
    use crate::{Error, Face};

    /// Lists the faces, optionally only those of one family.
    #[tauri::command]
    pub(crate) async fn get_faces(
        fonts: State<'_, Fonts>,
        family: Option<String>,
    ) -> Result<Vec<Face>, Error> {
        Ok(fonts::faces(&fonts.get(), family.as_deref()))
    }

    /// Lists the family names.
    #[tauri::command]
    pub(crate) async fn get_families(fonts: State<'_, Fonts>) -> Result<Vec<String>, Error> {
        Ok(fonts::families(&fonts.get()))
    }

    /// Returns the face that best matches a CSS-style query.
    #[tauri::command]
    pub(crate) async fn query_face(
        fonts: State<'_, Fonts>,
        query: FontQuery,
    ) -> Result<Face, Error> {
        fonts::query(&fonts.get(), &query)
    }

    /// Reads the system fonts again.
    #[tauri::command]
    pub(crate) async fn reload(fonts: State<'_, Fonts>) -> Result<(), Error> {
        fonts.reload();
        Ok(())
    }
}

#[cfg(desktop)]
pub(crate) use desktop::*;

#[cfg(mobile)]
#[expect(
    clippy::unreachable,
    reason = "#[tauri::command] checks an async command's return type with `let _check: T = unreachable!()`"
)]
mod mobile {
    use serde::de::IgnoredAny;

    use crate::{Error, Face};

    #[tauri::command]
    pub(crate) async fn get_faces(family: Option<String>) -> Result<Vec<Face>, Error> {
        drop(family);
        Err(Error::UnsupportedPlatform)
    }

    #[tauri::command]
    pub(crate) async fn get_families() -> Result<Vec<String>, Error> {
        Err(Error::UnsupportedPlatform)
    }

    #[tauri::command]
    pub(crate) async fn query_face(query: IgnoredAny) -> Result<Face, Error> {
        let IgnoredAny = query;
        Err(Error::UnsupportedPlatform)
    }

    #[tauri::command]
    pub(crate) async fn reload() -> Result<(), Error> {
        Err(Error::UnsupportedPlatform)
    }
}

#[cfg(mobile)]
pub(crate) use mobile::*;
