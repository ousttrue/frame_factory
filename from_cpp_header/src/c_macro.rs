use regex::Regex;

pub fn to_const(tokens: &[String]) -> String {
    // pub const SDL_INIT_TIMER: i32 = 0x00000001;
    let name = &tokens[0];
    let mut value = String::new();
    for token in &tokens[1..] {
        value.push_str(token);
    }

    let re_hex = Regex::new(r"0x([0-9a-f]+)(u)?").unwrap();
    let mut is_unsinged = false;
    if let Some(caps) = re_hex.captures(&value) {
        if let Some(_) = caps.at(1) {
            is_unsinged = true;
        }
    } else {
        // TODO:
        is_unsinged = true;
    }

    if is_unsinged {
        format!(
            "pub const {}: u32 = {};\n",
            &name,
            value.trim_end_matches("u")
        )
    } else {
        format!("pub const {}: i32 = {};\n", &name, &value)
    }
}
