//! Loading the system fonts and answering the commands from them.

use std::sync::{Arc, Mutex, PoisonError};

use fontgres_core::models::{self, FontQuery};
use fontgres_core::{Database, Family, GenericFamily};

use crate::{Error, Face};

/// The loaded fonts, kept in the app's managed state and shared by the
/// commands. They load on first use.
#[derive(Default)]
pub(crate) struct Fonts(Mutex<Option<Arc<Database>>>);

impl Fonts {
    /// Returns the loaded fonts, loading them first if no command has yet.
    pub(crate) fn get(&self) -> Arc<Database> {
        let mut loaded = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        Arc::clone(loaded.get_or_insert_with(|| Arc::new(load_system_fonts())))
    }

    /// Reads the system fonts again, so fonts installed or removed since the
    /// last load show up. Commands running meanwhile keep the old fonts.
    pub(crate) fn reload(&self) {
        let database = Arc::new(load_system_fonts());
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = Some(database);
    }
}

/// Loads the system fonts, without the hidden families. A font file that
/// cannot be read or parsed is skipped.
fn load_system_fonts() -> Database {
    let mut database = Database::new();
    let _report = database.load_system_fonts();
    without_hidden(&database)
}

/// Whether a family is one CSS cannot name, such as Apple's `.SF NS`.
fn is_hidden(family: &str) -> bool {
    family.starts_with('.')
}

/// Returns a copy of `database` without the faces whose primary family is
/// hidden, and with the same generic families. Queries then never return a
/// hidden face, and a family later in a query wins over a hidden one.
fn without_hidden(database: &Database) -> Database {
    let mut visible = Database::new();
    for generic in GenericFamily::ALL {
        let generic_family = Family::Generic(generic);
        visible.set_generic_family(generic, database.family_name(&generic_family));
    }
    for info in database.faces() {
        let hidden = info
            .families
            .first()
            .is_some_and(|(name, _)| is_hidden(name));
        if !hidden {
            visible.push_face_info(info.clone());
        }
    }
    visible
}

/// The faces in `database`, sorted by family, then weight, style, stretch and
/// `PostScript` name; with `family`, only that family's (ignoring ASCII case).
pub(crate) fn faces(database: &Database, family: Option<&str>) -> Vec<Face> {
    models::faces(database, family)
        .into_iter()
        .map(|info| Face::from(models::Face::from(info)))
        .collect()
}

/// The distinct primary family names in `database`, sorted.
pub(crate) fn families(database: &Database) -> Vec<String> {
    models::family_names(database)
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// The face in `database` that best matches `query`, by the CSS font
/// matching rules.
pub(crate) fn query(database: &Database, query: &FontQuery) -> Result<Face, Error> {
    Ok(Face::from(query.run(database)?))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fontgres_core::{FaceInfo, ID, Language, Source, Stretch, Style, Weight};
    use serde_json::json;

    use super::*;

    const TUFFY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fonts/Tuffy.ttf");

    /// A database holding Tuffy.ttf, plus a hidden face and a second visible
    /// family that share Tuffy's file.
    fn database() -> Database {
        let mut database = Database::new();
        let report = database.load_font_file(TUFFY).unwrap();
        assert!(report.failures().is_empty());
        database.push_face_info(pushed(".Hidden UI", "HiddenUI"));
        database.push_face_info(pushed("Fallback Sans", "FallbackSans"));
        database.set_generic_family(GenericFamily::SansSerif, "Tuffy");
        database
    }

    fn pushed(family: &str, post_script_name: &str) -> FaceInfo {
        FaceInfo {
            id: ID::dummy(),
            source: Source::File(Arc::from(Path::new(TUFFY))),
            index: 0,
            families: vec![(family.to_owned(), Language::English_UnitedStates)],
            subfamily: "Regular".to_owned(),
            post_script_name: post_script_name.to_owned(),
            style: Style::Normal,
            weight: Weight::NORMAL,
            stretch: Stretch::Normal,
            monospaced: false,
        }
    }

    #[test]
    fn hidden_families_are_left_out_of_every_listing() {
        let visible = without_hidden(&database());
        assert_eq!(families(&visible), ["Fallback Sans", "Tuffy"]);
        let names: Vec<String> = faces(&visible, None)
            .into_iter()
            .map(|face| face.family)
            .collect();
        assert_eq!(names, ["Fallback Sans", "Tuffy"]);
        assert_eq!(faces(&visible, Some(".hidden ui")), Vec::<Face>::new());
    }

    #[test]
    fn a_query_skips_a_hidden_family_for_a_later_visible_one() {
        let visible = without_hidden(&database());
        let wanted = FontQuery::new(vec![".Hidden UI".into(), "Fallback Sans".into()]);
        assert_eq!(query(&visible, &wanted).unwrap().family, "Fallback Sans");
        let only_hidden = FontQuery::new(vec![".Hidden UI".into()]);
        assert_eq!(
            query(&visible, &only_hidden),
            Err(Error::NoMatch(vec![".Hidden UI".into()]))
        );
    }

    #[test]
    fn generic_families_survive_the_filter() {
        let visible = without_hidden(&database());
        let wanted = FontQuery::new(vec!["sans-serif".into()]);
        assert_eq!(query(&visible, &wanted).unwrap().family, "Tuffy");
    }

    #[test]
    fn query_errors_keep_their_kind() {
        let visible = without_hidden(&database());
        assert_eq!(
            query(&visible, &FontQuery::new(Vec::new())),
            Err(Error::NoFamilies)
        );
        let mut heavy = FontQuery::new(vec!["Tuffy".into()]);
        heavy.weight = 1001;
        assert_eq!(query(&visible, &heavy), Err(Error::WeightOutOfRange(1001)));
    }

    /// The JSON the TypeScript types in guest-js/index.ts describe.
    #[test]
    fn a_face_serializes_in_camel_case_with_css_keywords() {
        let visible = without_hidden(&database());
        let tuffy = faces(&visible, Some("Tuffy")).remove(0);
        assert_eq!(
            serde_json::to_value(tuffy).unwrap(),
            json!({
                "family": "Tuffy",
                "families": ["Tuffy"],
                "subfamily": "Regular",
                "postScriptName": "Tuffy",
                "style": "normal",
                "weight": 500,
                "stretch": "normal",
                "monospaced": false,
                "path": TUFFY,
                "index": 0,
            })
        );
    }

    #[test]
    fn a_query_reads_the_javascript_arguments() {
        let wanted: FontQuery = serde_json::from_value(json!({
            "families": ["Tuffy"],
            "weight": 700,
            "style": "italic",
            "stretch": "semi-condensed",
        }))
        .unwrap();
        assert_eq!(wanted.style, Style::Italic);
        assert_eq!(wanted.stretch, Stretch::SemiCondensed);
        let defaults: FontQuery = serde_json::from_value(json!({ "families": ["Tuffy"] })).unwrap();
        assert_eq!(defaults, FontQuery::new(vec!["Tuffy".into()]));
    }
}
