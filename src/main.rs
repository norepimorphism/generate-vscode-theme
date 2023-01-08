// SPDX-License-Identifier: MPL-2.0

use std::fs;

use sailfish::TemplateOnce;

fn main() {
    let file = fs::File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open("package/themes/norepi-color-theme.json")
        .expect("failed to open output file");
    let theme = Theme::default()
        .render_once()
        .expect("failed to expand template");
    minifier::json::minify(theme.as_str())
        .write(file)
        .expect("failed to write output to file");
}

type Color = &'static str;

macro_rules! hex_color {
    ($inner:literal) => { concat!("\"#", $inner, "\"") };
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ThemeColors {
                ui: UiColors::dark(),
                tokens: TokenColors {
                    comments: hex_color!("b0b0b0"),
                    constants: ConstantColors {
                        lang: hex_color!("ffff00"),
                    },
                    keywords: hex_color!("0000ff"),
                    lit: LiteralColors {
                        chars: hex_color!("00ffff"),
                        numeric: hex_color!("00ffff"),
                        strings: hex_color!("00ffff"),
                    },
                    operators: hex_color!("ff0000"),
                    types: hex_color!("00ff00"),
                },
            },
        }
    }
}

#[derive(TemplateOnce)]
#[template(path = "theme.stpl")]
struct Theme {
    colors: ThemeColors,
}

struct ThemeColors {
    ui: UiColors,
    tokens: TokenColors,
}

impl UiColors {
    fn dark() -> Self {
        Self {
            fg: UiForegroundColors {
                default: hex_color!("d0d0d0"),
            },
            bg: UiBackgroundColors {
                default: hex_color!("101010"),
                highlighted_lines: hex_color!("202020"),
            },
        }
    }
}

struct UiColors {
    fg: UiForegroundColors,
    bg: UiBackgroundColors,
}

struct UiForegroundColors {
    default: Color,
}

struct UiBackgroundColors {
    default: Color,
    highlighted_lines: Color,
}

struct TokenColors {
    comments: Color,
    constants: ConstantColors,
    keywords: Color,
    lit: LiteralColors,
    operators: Color,
    types: Color,
}

struct ConstantColors {
    lang: Color,
}

struct LiteralColors {
    chars: Color,
    numeric: Color,
    strings: Color,
}
