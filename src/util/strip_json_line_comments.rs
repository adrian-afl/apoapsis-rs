use regex::Regex;

pub fn strip_json_line_comments(input: &str) -> String {
    let re = Regex::new(r"//.*");
    let output = re.unwrap().replace_all(input, "");
    output.to_string()
}
