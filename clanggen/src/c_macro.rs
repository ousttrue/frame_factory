use regex::Regex;

struct Matcher {
    hex: Regex,
}

impl Matcher {
    fn new() -> Matcher {
        Matcher {
            // 0x00000001
            hex: Regex::new(r"^(\(\w+\))?([~])?((?:0x)?[0-9a-fA-F]+)(ll|u|U|ull)?$").unwrap(),
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
                format!("pub const {}: {} = {};", name, rust_type, hex)
            } else if let Some(suffix) = suffix {
                let rust_type = match suffix {
                    "u" | "U" => "u32",
                    "ull" => "u64",
                    "ll" => "i64",
                    _ => panic!(),
                };
                // unsigned
                format!("pub const {}: {} = {};", name, rust_type, hex)
            } else {
                let mut value_type = "i32";
                if value.starts_with("0x") {
                    if let Ok(u) = u32::from_str_radix("value", 16) {
                        if u > (i32::MAX as u32) {
                            value_type = "u32";
                        }
                    }
                } else {
                    if let Ok(u) = u32::from_str_radix("value", 10) {
                        if u > i32::MAX as u32 {
                            value_type = "u32";
                        }
                    }
                }
                format!("pub const {}: {} = {};", name, value_type, value)
            }
        } else {
            format!("//{} {}", name, value)
        }
    }
}

static mut G_MATCHER: Option<Matcher> = None;

pub fn to_const(tokens: &[String]) -> Option<String> {
    if tokens.len() < 2 {
        return None;
    }

    let name = &tokens[0];

    if ["SDL_Colour", "SDL_BlitSurface", "SDL_BlitScaled"].contains(&name.as_str()) {
        return None;
    }

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

fn join(tokens: &[String]) -> String {
    let mut value = String::new();
    for token in tokens {
        value.push_str(token);
    }
    value
}

pub fn to_func(tokens: &[String]) -> Option<String> {
    let name = &tokens[0];
    let value = join(&tokens[1..]);

    if value.contains("sizeof") || value.contains("?") || value.contains("_") {
        return Some(format!("/* {}{} */", name, value));
    }

    match name.as_str() {
        "SDL_TABLESIZE"
        | "SDL_iconv_utf8_ucs2"
        | "SDL_iconv_utf8_ucs4"
        | "SDL_STRINGIFY_ARG"
        | "SDL_MUSTLOCK"
        | "SDL_LoadBMP"
        | "SDL_stack_free" => {
            return Some(format!("/* {}{} */", name, value));
        }
        _ => (),
    };

    let result = if value.contains("\\") {
        // multi line
        format!("/* {}{} */", name, value)
    } else {
        // SDL_WINDOWPOS_CENTERED_DISPLAY(X)(SDL_WINDOWPOS_CENTERED_MASK|(X))
        if tokens.len() > 4 && tokens[1] == "(" && tokens[3] == ")" {
            // 1 args
            format!(
                "pub const fn {}({}: i32) -> i32{{{}}}",
                tokens[0],
                tokens[2],
                &join(&tokens[4..])
            )
        } else {
            format!("// {}{}", name, value)
        }
    };
    Some(result)
}
