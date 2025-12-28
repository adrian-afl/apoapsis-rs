pub fn serde_err_map(error: serde_json::Error) -> String {
    format!("Cannot parse input: {error}").to_string()
}
