const COMMANDS: &[&str] = &["get_faces", "get_families", "query_face", "reload"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
