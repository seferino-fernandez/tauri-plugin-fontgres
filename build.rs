//! Generates the plugin's command permissions and its permission schema.

const COMMANDS: &[&str] = &["get_faces", "get_families", "query_face", "reload"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
