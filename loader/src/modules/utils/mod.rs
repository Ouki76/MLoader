pub mod fs {
    #[tauri::command]
    pub fn get_file_content(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }
}
