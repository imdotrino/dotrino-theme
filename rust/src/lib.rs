//! «Cool & Cozy», el tema de Dotrino, para ventanas de escritorio hechas con iced.
//!
//! Es la misma fuente única que `theme.css`: los colores son los tokens `--dt-*` en claro y en
//! oscuro, y las fuentes son Quicksand (texto) y Comfortaa (marca). Las fuentes van como TTF
//! estáticos: el motor de texto de iced no elige el peso de una fuente variable y caía a la del
//! sistema.
//!
//! ```ignore
//! iced::application(App::boot, App::update, App::view)
//!     .theme(|app: &App| dotrino_theme::theme(app.mode))
//!     .font(dotrino_theme::fonts::QUICKSAND)
//!     .font(dotrino_theme::fonts::QUICKSAND_BOLD)
//!     .font(dotrino_theme::fonts::COMFORTAA)
//!     .default_font(dotrino_theme::BODY)
//! ```

use iced::widget::{button, container};
use iced::{Border, Color, Font, Theme, font, theme};

pub mod fonts {
    pub const QUICKSAND: &[u8] = include_bytes!("../fonts/Quicksand-Regular.ttf");
    pub const QUICKSAND_BOLD: &[u8] = include_bytes!("../fonts/Quicksand-Bold.ttf");
    pub const COMFORTAA: &[u8] = include_bytes!("../fonts/Comfortaa-Bold.ttf");
}

pub const BODY: Font = Font::with_name("Quicksand");
pub const BOLD: Font = Font { family: font::Family::Name("Quicksand"), weight: font::Weight::Bold, ..Font::DEFAULT };
pub const BRAND: Font = Font { family: font::Family::Name("Comfortaa"), weight: font::Weight::Bold, ..Font::DEFAULT };

/// Los tokens del tema. Mismos nombres que `--dt-*` en `theme.css`.
#[derive(Clone, Copy, Debug)]
pub struct Pal {
    pub bg: Color,
    pub surface: Color,
    pub surface_2: Color,
    pub surface_3: Color,
    pub line: Color,
    pub line_soft: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_faint: Color,
    pub accent: Color,
    pub accent_press: Color,
    pub accent_ink: Color,
    pub accent_soft: Color,
    pub accent_soft_ink: Color,
    pub ok: Color,
    pub busy: Color,
    pub danger: Color,
    pub warn_soft: Color,
}

pub const fn hex(v: u32) -> Color {
    Color::from_rgb8(((v >> 16) & 0xff) as u8, ((v >> 8) & 0xff) as u8, (v & 0xff) as u8)
}

impl Pal {
    pub fn of(mode: theme::Mode) -> Pal {
        if mode == theme::Mode::Dark {
            Pal {
                bg: hex(0x0f1416),
                surface: hex(0x1c2022),
                surface_2: hex(0x262a2d),
                surface_3: hex(0x313538),
                line: hex(0x3f484e),
                line_soft: hex(0x262a2d),
                text: hex(0xdfe3e6),
                text_dim: hex(0xbfc8cf),
                text_faint: hex(0x899296),
                accent: hex(0x81cfff),
                accent_press: hex(0x5fb8ee),
                accent_ink: hex(0x003549),
                accent_soft: hex(0x004c6b),
                accent_soft_ink: hex(0xc6e7ff),
                ok: hex(0x7ad7c2),
                busy: hex(0xf5b041),
                danger: hex(0xffb4ab),
                warn_soft: hex(0x3d2e10),
            }
        } else {
            Pal {
                bg: hex(0xf7fafc),
                surface: hex(0xffffff),
                surface_2: hex(0xf1f4f6),
                surface_3: hex(0xe9eef1),
                line: hex(0xbfc8cf),
                line_soft: hex(0xe3e9ed),
                text: hex(0x181c1e),
                text_dim: hex(0x3f484e),
                text_faint: hex(0x70787f),
                accent: hex(0x00658c),
                accent_press: hex(0x00506f),
                accent_ink: hex(0xffffff),
                accent_soft: hex(0xd4ecfa),
                accent_soft_ink: hex(0x004e6e),
                ok: hex(0x006b5c),
                busy: hex(0xa35a00),
                danger: hex(0xba1a1a),
                warn_soft: hex(0xfff0d6),
            }
        }
    }
}

/// El `Theme` de iced con la paleta de Dotrino.
pub fn theme(mode: theme::Mode) -> Theme {
    let p = Pal::of(mode);
    Theme::custom(
        "Cool & Cozy",
        theme::Palette { background: p.bg, text: p.text, primary: p.accent, success: p.ok, warning: p.busy, danger: p.danger },
    )
}

/// Una etiqueta redonda (estado, versión).
pub fn chip(bg: Color, fg: Color) -> container::Style {
    container::Style {
        background: Some(bg.into()),
        text_color: Some(fg),
        border: Border { radius: 999.0.into(), ..Default::default() },
        ..Default::default()
    }
}

/// Una tarjeta o una fila de lista.
pub fn card(pal: Pal, selected: bool) -> container::Style {
    container::Style {
        background: Some(if selected { pal.accent_soft } else { pal.surface }.into()),
        text_color: Some(if selected { pal.accent_soft_ink } else { pal.text }),
        border: Border { color: if selected { pal.accent } else { pal.line_soft }, width: 1.0, radius: 12.0.into() },
        ..Default::default()
    }
}

pub fn btn(bg: Color, fg: Color, border: Color) -> button::Style {
    button::Style {
        background: Some(bg.into()),
        text_color: fg,
        border: Border { color: border, width: 1.0, radius: 999.0.into() },
        ..Default::default()
    }
}

fn pressed(st: button::Status) -> bool {
    matches!(st, button::Status::Hovered | button::Status::Pressed)
}

/// Un botón deshabilitado se ve apagado, no desaparece.
fn disabled(pal: Pal) -> button::Style {
    btn(pal.surface_2, pal.text_faint, pal.line_soft)
}

pub fn primary(pal: Pal, st: button::Status) -> button::Style {
    if st == button::Status::Disabled {
        return disabled(pal);
    }
    let bg = if pressed(st) { pal.accent_press } else { pal.accent };
    btn(bg, pal.accent_ink, bg)
}

pub fn ghost(pal: Pal, st: button::Status) -> button::Style {
    if st == button::Status::Disabled {
        return disabled(pal);
    }
    let bg = if pressed(st) { pal.surface_3 } else { pal.surface_2 };
    btn(bg, pal.text, pal.line_soft)
}

pub fn danger_outline(pal: Pal, st: button::Status) -> button::Style {
    if st == button::Status::Disabled {
        return disabled(pal);
    }
    let bg = if pressed(st) { pal.surface_3 } else { pal.surface_2 };
    btn(bg, pal.danger, pal.danger)
}

pub fn tab(pal: Pal, active: bool, st: button::Status) -> button::Style {
    if active {
        btn(pal.accent_soft, pal.accent_soft_ink, pal.accent_soft)
    } else if pressed(st) {
        btn(pal.surface_3, pal.text, pal.surface_3)
    } else {
        btn(Color::TRANSPARENT, pal.text_dim, Color::TRANSPARENT)
    }
}

/// ¿El sistema está en español? Para los textos propios de una ventana (es/en).
pub fn system_is_spanish() -> bool {
    let lang = std::env::var("LC_ALL")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("LC_MESSAGES").ok().filter(|s| !s.is_empty()))
        .or_else(|| std::env::var("LANG").ok())
        .unwrap_or_default();
    lang.to_lowercase().starts_with("es")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_azul_es_el_del_tema() {
        let p = Pal::of(theme::Mode::Light);
        assert_eq!(p.accent, hex(0x00658c));
    }

    #[test]
    fn las_fuentes_van_embebidas() {
        assert!(fonts::QUICKSAND.len() > 10_000);
        assert!(fonts::COMFORTAA.len() > 10_000);
    }
}
