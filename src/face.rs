use serde::Serialize;

/// One font face, as the commands return it to JavaScript.
///
/// The fields are those of `fontgres_core::models::Face`, serialized in
/// camelCase. Style and stretch are CSS keywords, such as `italic` and
/// `semi-condensed`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    /// The face's primary family name (English US when the font has one).
    pub family: String,
    /// Every family name the face carries, in every language, primary first.
    pub families: Vec<String>,
    /// The name that tells the face from the others of its family, such as
    /// `Bold Italic`; empty when the font has none.
    pub subfamily: String,
    /// The face's `PostScript` name.
    pub post_script_name: String,
    /// `normal`, `italic` or `oblique`.
    pub style: &'static str,
    /// CSS weight, from 1 to 1000; 400 is normal and 700 bold.
    pub weight: u16,
    /// CSS stretch, from `ultra-condensed` to `ultra-expanded`.
    pub stretch: &'static str,
    /// Whether every glyph has the same advance width.
    pub monospaced: bool,
    /// The file the face was loaded from, if any. A path that is not valid
    /// UTF-8 has its invalid parts replaced with U+FFFD.
    pub path: Option<String>,
    /// The face's index within its file (non-zero only in font collections).
    pub index: u32,
}

#[cfg(desktop)]
impl From<fontgres_core::models::Face<'_>> for Face {
    fn from(face: fontgres_core::models::Face<'_>) -> Self {
        Self {
            family: face.family.to_owned(),
            families: face.families.iter().map(|(name, _)| name.clone()).collect(),
            subfamily: face.subfamily.to_owned(),
            post_script_name: face.post_script_name.to_owned(),
            style: face.style.css_name(),
            weight: face.weight,
            stretch: face.stretch.css_name(),
            monospaced: face.monospaced,
            path: face.path.map(|path| path.to_string_lossy().into_owned()),
            index: face.index,
        }
    }
}
