
use termimad::crossterm::style::Color::{Rgb, Yellow};
use termimad::{rgb, MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(rgb(28, 28, 28));
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.set_headers_fg(Yellow);
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');
    skin.quote_mark.set_fg(rgb(215, 255, 135));
    skin.inline_code.set_fg(Rgb {
        r: 255,
        g: 0,
        b: 200,
    });
    skin.italic.set_fg(Rgb {
        r: 215,
        g: 255,
        b: 0,
    });

    skin
}
