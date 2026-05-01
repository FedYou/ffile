pub fn string_to_elipse(max_len: usize, text: String) -> String {
    if text.chars().count() > max_len {
        let truncated: String = text.chars().take(max_len.saturating_sub(3)).collect();

        return format!("{}...", truncated.trim());
    }
    text
}

pub fn string_to_elipse_inverse(max_len: usize, text: String) -> String {
    let len = text.chars().count();
    if len > max_len {
        let truncated: String = text.chars().skip(len - max_len.saturating_sub(3)).collect();

        return format!("...{}", truncated.trim());
    }
    text
}
