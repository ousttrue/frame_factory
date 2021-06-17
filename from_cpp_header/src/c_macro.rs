use regex::Regex;

struct Matcher {
    hex: Regex,
}

impl Matcher {
    fn new() -> Matcher {
        Matcher {
            hex: Regex::new(r"(\(\w+\))?([~])?(0x[0-9a-fA-F]+)(ll|u|ull)?").unwrap(),
        }
    }

    fn name_value(&self, name: &str, value: &str) -> String {
        if let Some(caps) = self.hex.captures(&value) {
            let cast = caps.at(1);
            let tilda = caps.at(2);
            let mut hex = caps.at(3).unwrap().to_owned();
            let suffix = caps.at(4);

            if tilda.is_some() {
                hex = format!("!{}", hex);
            }

            if let Some(t) = cast {
                // with cast
                let rust_type = match t {
                    "(Sint8)" => "i8",
                    "(Sint16)" => "i16",
                    "(Sint32)" => "i32",
                    "(Sint64)" => "i64",
                    "(Uint8)" => "u8",
                    "(Uint16)" => "u16",
                    "(Uint32)" => "u32",
                    "(Uint64)" => "u64",
                    _ => panic!(),
                };
                format!("pub const {}: {} = {};\n", name, rust_type, hex)
            } else if let Some(suffix) = suffix {
                let rust_type = match suffix {
                    "u" => "u32",
                    "ull" => "u64",
                    "ll" => "i64",
                    _ => panic!(),
                };
                // unsigned
                format!("pub const {}: {} = {};\n", name, rust_type, hex)
            } else {
                format!("//{} {}\n", name, value)
            }
        } else {
            format!("//{} {}\n", name, value)
        }
    }
}

static mut G_MATCHER: Option<Matcher> = None;

pub fn to_const(tokens: &[String]) -> Option<String> {
    if tokens.len() < 2 {
        return None;
    }

    let name = &tokens[0];
    let mut value = String::new();
    for token in &tokens[1..] {
        value.push_str(token);
    }

    if unsafe { G_MATCHER.is_none() } {
        unsafe { G_MATCHER = Some(Matcher::new()) };
    }

    Some(
        unsafe { G_MATCHER.as_ref() }
            .unwrap()
            .name_value(name, &value),
    )
}

pub fn to_func(tokens: &[String]) -> Option<String> {
    let name = &tokens[0];

    let mut value = String::new();
    for token in &tokens[1..] {
        value.push_str(token);
    }

    let result = if value.contains("\\") {
        format!("/* {}{} */\n", name, value)
    } else {
        format!("// {}{}\n", name, value)
    };
    Some(result)
}
