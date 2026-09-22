pub fn normalize_name(name: &str) -> String {
    let regex = regex::Regex::new(r"[-_. ]+").expect("a valid regex expression");
    regex.replace_all(&name.to_lowercase(), "-").to_string()
}
