//! A themed message box overlay widget for [Iced](https://github.com/iced-rs/iced).
//!
//! Provides CTkMessagebox-style modal dialogs that render as in-app overlays.
//! Five icon types, four button layouts, and full theme integration.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use iced_message_box::{MessageBox, MessageBoxIcon, MessageBoxButtons, MessageBoxResult};
//!
//! // In your view, layer the overlay on top of your content:
//! if let Some(ref mb) = self.active_message_box {
//!     let overlay = mb.overlay(|result| MyMessage::DialogResult(result));
//!     iced::widget::stack![base_content, overlay].into()
//! } else {
//!     base_content
//! }
//! ```
//!
//! # Convenience constructors
//!
//! ```rust,no_run
//! use iced_message_box::MessageBox;
//!
//! let mb = MessageBox::info("Done", "Export completed successfully.");
//! let mb = MessageBox::warning("Warning", "File already exists.");
//! let mb = MessageBox::error("Error", "Failed to save.");
//! let mb = MessageBox::ask_yes_no("Confirm", "Delete this item?");
//! let mb = MessageBox::ask_yes_no_cancel("Save", "Save before closing?");
//! let mb = MessageBox::ask_ok_cancel("Proceed", "This cannot be undone.");
//! ```
//!
//! # Custom glyphs
//!
//! Each icon type has a default Unicode glyph, but you can override them:
//!
//! ```rust,no_run
//! use iced_message_box::MessageBox;
//!
//! let mb = MessageBox::info("Custom", "With a star icon!")
//!     .with_glyph("\u{2605}");  // ★
//! ```

use iced::widget::{button, column, container, row, stack, text, Space};
use iced::{Background, Border, Color, Element, Length, Theme};

// ── Icon types ──────────────────────────────────────────────────────────────

/// Visual icon displayed in the message box header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxIcon {
    /// Blue circle with "i" — informational messages.
    Info,
    /// Green circle with checkmark — success/completion messages.
    Success,
    /// Amber circle with "!" — warning messages.
    Warning,
    /// Red circle with "X" — error messages.
    Error,
    /// Purple circle with "?" — questions requiring user input.
    Question,
}

impl MessageBoxIcon {
    /// Numeric index for each icon type (useful for custom glyph arrays).
    ///
    /// Returns: Info=0, Success=1, Warning=2, Error=3, Question=4.
    pub fn index(self) -> usize {
        match self {
            Self::Info => 0,
            Self::Success => 1,
            Self::Warning => 2,
            Self::Error => 3,
            Self::Question => 4,
        }
    }

    /// Unicode glyph for the icon.
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Info => "i",
            Self::Success => "\u{2713}",  // ✓
            Self::Warning => "!",
            Self::Error => "\u{2717}",    // ✗
            Self::Question => "?",
        }
    }

    /// Default semantic color for the icon (used when no custom colors are set).
    pub fn default_color(self) -> Color {
        match self {
            Self::Info => Color::from_rgb(0.23, 0.56, 0.82),     // blue
            Self::Success => Color::from_rgb(0.18, 0.70, 0.40),  // green
            Self::Warning => Color::from_rgb(0.90, 0.72, 0.15),  // amber
            Self::Error => Color::from_rgb(0.85, 0.22, 0.22),    // red
            Self::Question => Color::from_rgb(0.55, 0.36, 0.80), // purple
        }
    }
}

// ── Button layouts ──────────────────────────────────────────────────────────

/// Which buttons to display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxButtons {
    /// Single OK button.
    Ok,
    /// Yes and No buttons.
    YesNo,
    /// Yes, No, and Cancel buttons.
    YesNoCancel,
    /// OK and Cancel buttons.
    OkCancel,
}

// ── Result ──────────────────────────────────────────────────────────────────

/// The value returned when the user clicks a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxResult {
    Ok,
    Yes,
    No,
    Cancel,
}

// ── Theme colors ────────────────────────────────────────────────────────────

/// Custom color overrides for the message box.
///
/// Any field set to `None` will use the default derived from the icon type.
/// This allows partial customization — override just the accent color and
/// let everything else derive automatically.
#[derive(Debug, Clone, Copy)]
pub struct MessageBoxColors {
    /// Background color of the card. Defaults to a slight shift from theme background.
    pub card_background: Option<Color>,
    /// Border color of the card.
    pub card_border: Option<Color>,
    /// Title text color.
    pub title_color: Option<Color>,
    /// Body text color (slightly muted).
    pub body_color: Option<Color>,
    /// Accent color for the icon circle and primary button.
    pub accent: Option<Color>,
    /// Corner radius of the card and buttons.
    pub corner_radius: Option<f32>,
    /// Border width of the card.
    pub border_width: Option<f32>,
}

impl Default for MessageBoxColors {
    fn default() -> Self {
        Self {
            card_background: None,
            card_border: None,
            title_color: None,
            body_color: None,
            accent: None,
            corner_radius: None,
            border_width: None,
        }
    }
}

/// All resolved colors the renderer needs (internal).
#[derive(Clone, Copy)]
struct ResolvedColors {
    card_bg: Color,
    card_border: Color,
    title_color: Color,
    body_color: Color,
    icon_accent: Color,
    icon_text: Color,
    icon_shadow: Color,
    accent_btn_text: Color,
    subtle_btn_border: Color,
    subtle_btn_text: Color,
    corner_radius: f32,
    border_width: f32,
}

impl ResolvedColors {
    fn resolve(icon: MessageBoxIcon, overrides: &MessageBoxColors, is_dark: bool) -> Self {
        // Base palette — dark or light defaults
        let (default_bg, default_fg) = if is_dark {
            (
                Color::from_rgb(0.12, 0.12, 0.14),
                Color::from_rgb(0.92, 0.92, 0.92),
            )
        } else {
            (
                Color::from_rgb(0.96, 0.96, 0.97),
                Color::from_rgb(0.10, 0.10, 0.10),
            )
        };

        let accent = overrides.accent.unwrap_or_else(|| icon.default_color());

        let card_bg = overrides.card_background.unwrap_or_else(|| Color {
            r: default_bg.r + (default_fg.r - default_bg.r) * 0.08,
            g: default_bg.g + (default_fg.g - default_bg.g) * 0.08,
            b: default_bg.b + (default_fg.b - default_bg.b) * 0.08,
            a: 1.0,
        });

        let card_border = overrides.card_border.unwrap_or_else(|| Color {
            r: default_bg.r + (default_fg.r - default_bg.r) * 0.18,
            g: default_bg.g + (default_fg.g - default_bg.g) * 0.18,
            b: default_bg.b + (default_fg.b - default_bg.b) * 0.18,
            a: 1.0,
        });

        let title_color = overrides.title_color.unwrap_or(default_fg);

        let body_color = overrides.body_color.unwrap_or_else(|| Color {
            r: default_fg.r * 0.75 + default_bg.r * 0.25,
            g: default_fg.g * 0.75 + default_bg.g * 0.25,
            b: default_fg.b * 0.75 + default_bg.b * 0.25,
            a: 1.0,
        });

        // Icon text: high-contrast against the accent circle
        let accent_lum = 0.299 * accent.r + 0.587 * accent.g + 0.114 * accent.b;
        let icon_text = if accent_lum > 0.5 {
            // Dark glyph on light accent
            Color {
                r: accent.r * 0.15,
                g: accent.g * 0.15,
                b: accent.b * 0.15,
                a: 1.0,
            }
        } else {
            // Light glyph on dark accent
            Color {
                r: accent.r * 0.3 + 0.7,
                g: accent.g * 0.3 + 0.7,
                b: accent.b * 0.3 + 0.7,
                a: 1.0,
            }
        };

        // Shadow for stroke/outline effect on the glyph
        let icon_shadow = if accent_lum > 0.5 {
            Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 }
        } else {
            Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 }
        };

        let corner_radius = overrides.corner_radius.unwrap_or(12.0);
        let border_width = overrides.border_width.unwrap_or(1.0);

        Self {
            card_bg,
            card_border,
            title_color,
            body_color,
            icon_accent: accent,
            icon_text,
            icon_shadow,
            accent_btn_text: icon_text,
            subtle_btn_border: card_border,
            subtle_btn_text: body_color,
            corner_radius,
            border_width,
        }
    }
}

// ── MessageBox ──────────────────────────────────────────────────────────────

/// A themed message box configuration.
///
/// Create one with the convenience constructors ([`info`](Self::info),
/// [`warning`](Self::warning), etc.) and optionally customize with
/// [`with_colors`](Self::with_colors) or [`dark`](Self::dark)/[`light`](Self::light).
/// Then call [`overlay`](Self::overlay) or [`card`](Self::card) to render.
#[derive(Debug, Clone)]
pub struct MessageBox {
    /// Title text displayed in bold.
    pub title: String,
    /// Body / description text.
    pub message: String,
    /// Icon type (determines default accent color).
    pub icon: MessageBoxIcon,
    /// Button layout.
    pub buttons: MessageBoxButtons,
    /// Whether to use dark-mode defaults (true) or light-mode (false).
    pub is_dark: bool,
    /// Optional color overrides.
    pub colors: MessageBoxColors,
    /// Optional custom glyph for the icon (overrides the default).
    pub custom_glyph: Option<String>,
}

impl MessageBox {
    /// Create a new message box with full control.
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        icon: MessageBoxIcon,
        buttons: MessageBoxButtons,
    ) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            icon,
            buttons,
            is_dark: true,
            colors: MessageBoxColors::default(),
            custom_glyph: None,
        }
    }

    /// Informational message (OK button, blue icon).
    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Info, MessageBoxButtons::Ok)
    }

    /// Success message (OK button, green icon).
    pub fn success(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Success, MessageBoxButtons::Ok)
    }

    /// Warning message (OK button, amber icon).
    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Warning, MessageBoxButtons::Ok)
    }

    /// Error message (OK button, red icon).
    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Error, MessageBoxButtons::Ok)
    }

    /// Yes/No question (purple icon).
    pub fn ask_yes_no(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Question, MessageBoxButtons::YesNo)
    }

    /// Yes/No/Cancel question (purple icon).
    pub fn ask_yes_no_cancel(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Question, MessageBoxButtons::YesNoCancel)
    }

    /// OK/Cancel question (purple icon).
    pub fn ask_ok_cancel(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message, MessageBoxIcon::Question, MessageBoxButtons::OkCancel)
    }

    /// Use dark-mode defaults (this is the default).
    pub fn dark(mut self) -> Self {
        self.is_dark = true;
        self
    }

    /// Use light-mode defaults.
    pub fn light(mut self) -> Self {
        self.is_dark = false;
        self
    }

    /// Apply custom color overrides.
    pub fn with_colors(mut self, colors: MessageBoxColors) -> Self {
        self.colors = colors;
        self
    }

    /// Set a custom accent color (icon circle + primary button).
    pub fn with_accent(mut self, accent: Color) -> Self {
        self.colors.accent = Some(accent);
        self
    }

    /// Set the card corner radius.
    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.colors.corner_radius = Some(radius);
        self
    }

    /// Set the card and button border width.
    pub fn with_border_width(mut self, width: f32) -> Self {
        self.colors.border_width = Some(width);
        self
    }

    /// Set a custom glyph for the icon circle (overrides the default symbol).
    ///
    /// Useful for using custom Unicode symbols or single characters:
    /// ```rust,no_run
    /// # use iced_message_box::MessageBox;
    /// let mb = MessageBox::info("Star", "You earned a star!")
    ///     .with_glyph("\u{2605}");  // ★
    /// ```
    pub fn with_glyph(mut self, glyph: impl Into<String>) -> Self {
        self.custom_glyph = Some(glyph.into());
        self
    }

    /// Render as a full-window overlay (semi-transparent backdrop + centered card).
    ///
    /// `on_result` maps a [`MessageBoxResult`] to your app's message type.
    pub fn overlay<'a, M: Clone + 'a>(
        &self,
        on_result: impl Fn(MessageBoxResult) -> M + 'a,
    ) -> Element<'a, M> {
        let colors = ResolvedColors::resolve(self.icon, &self.colors, self.is_dark);
        let glyph = self.custom_glyph.as_deref().unwrap_or(self.icon.glyph());

        let card = build_card(
            glyph,
            self.title.clone(),
            self.message.clone(),
            self.buttons,
            colors,
            &on_result,
        );

        let backdrop_color = if self.is_dark {
            Color::from_rgba(0.0, 0.0, 0.0, 0.55)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.35)
        };

        container(
            container(card)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &Theme| container::Style {
            background: Some(Background::Color(backdrop_color)),
            ..Default::default()
        })
        .into()
    }

    /// Render just the card (no backdrop) for embedding in layouts or previews.
    ///
    /// `on_result` maps a [`MessageBoxResult`] to your app's message type.
    pub fn card<'a, M: Clone + 'a>(
        &self,
        on_result: impl Fn(MessageBoxResult) -> M + 'a,
    ) -> Element<'a, M> {
        let colors = ResolvedColors::resolve(self.icon, &self.colors, self.is_dark);
        let glyph = self.custom_glyph.as_deref().unwrap_or(self.icon.glyph());
        build_card(
            glyph,
            self.title.clone(),
            self.message.clone(),
            self.buttons,
            colors,
            &on_result,
        )
    }
}

// ── Card builder (free function — owns all data) ────────────────────────────

fn build_card<'a, M: Clone + 'a>(
    glyph: &str,
    title_text: String,
    body_text: String,
    buttons: MessageBoxButtons,
    c: ResolvedColors,
    on_result: &(impl Fn(MessageBoxResult) -> M + 'a),
) -> Element<'a, M> {
    // ── Icon badge with shadow/stroke effect ────────────────────────
    let icon_accent = c.icon_accent;
    let icon_text_color = c.icon_text;
    let icon_shadow_color = c.icon_shadow;
    let glyph_owned = glyph.to_string();
    let glyph_owned2 = glyph_owned.clone();

    // Stack: shadow glyph (slightly larger) behind the main glyph for outline effect
    let glyph_layer = stack![
        container(
            text(glyph_owned)
                .size(30)
                .color(icon_shadow_color)
                .center(),
        )
        .width(52)
        .height(52)
        .center_x(52)
        .center_y(52),
        container(
            text(glyph_owned2)
                .size(28)
                .color(icon_text_color)
                .center(),
        )
        .width(52)
        .height(52)
        .center_x(52)
        .center_y(52),
    ];

    let icon_badge: Element<'a, M> = container(glyph_layer)
        .width(52)
        .height(52)
        .center_x(52)
        .center_y(52)
        .style(move |_theme: &Theme| {
            // Brighten the accent for the border to ensure visibility
            let border_color = Color {
                r: (icon_accent.r * 0.7 + 0.3).min(1.0),
                g: (icon_accent.g * 0.7 + 0.3).min(1.0),
                b: (icon_accent.b * 0.7 + 0.3).min(1.0),
                a: 0.6,
            };
            container::Style {
                background: Some(Background::Color(icon_accent)),
                border: Border {
                    radius: 26.0.into(),
                    width: 2.0,
                    color: border_color,
                },
                shadow: iced::Shadow {
                    color: Color { a: 0.5, ..icon_accent },
                    offset: iced::Vector::new(0.0, 0.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            }
        })
        .into();

    // ── Title ───────────────────────────────────────────────────────
    let title: Element<'a, M> = text(title_text)
        .size(17)
        .color(c.title_color)
        .center()
        .into();

    // ── Body ────────────────────────────────────────────────────────
    let body: Element<'a, M> = text(body_text)
        .size(13)
        .color(c.body_color)
        .center()
        .into();

    // ── Buttons ─────────────────────────────────────────────────────
    let button_row: Element<'a, M> = match buttons {
        MessageBoxButtons::Ok => {
            let msg = on_result(MessageBoxResult::Ok);
            row![accent_button("OK", c, msg)]
                .spacing(8)
                .into()
        }
        MessageBoxButtons::YesNo => {
            let yes = on_result(MessageBoxResult::Yes);
            let no = on_result(MessageBoxResult::No);
            row![
                subtle_button("No", c, no),
                accent_button("Yes", c, yes),
            ]
            .spacing(8)
            .into()
        }
        MessageBoxButtons::YesNoCancel => {
            let yes = on_result(MessageBoxResult::Yes);
            let no = on_result(MessageBoxResult::No);
            let cancel = on_result(MessageBoxResult::Cancel);
            row![
                subtle_button("Cancel", c, cancel),
                subtle_button("No", c, no),
                accent_button("Yes", c, yes),
            ]
            .spacing(8)
            .into()
        }
        MessageBoxButtons::OkCancel => {
            let ok = on_result(MessageBoxResult::Ok);
            let cancel = on_result(MessageBoxResult::Cancel);
            row![
                subtle_button("Cancel", c, cancel),
                accent_button("OK", c, ok),
            ]
            .spacing(8)
            .into()
        }
    };

    let buttons_centered: Element<'a, M> = container(button_row)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into();

    // ── Card container ──────────────────────────────────────────────
    let card_bg = c.card_bg;
    let card_border = c.card_border;
    let radius = c.corner_radius;
    let border_width = c.border_width;

    container(
        column![
            container(icon_badge)
                .width(Length::Fill)
                .center_x(Length::Fill),
            Space::new().height(10),
            container(title)
                .width(Length::Fill)
                .center_x(Length::Fill),
            Space::new().height(6),
            container(body)
                .width(Length::Fill)
                .center_x(Length::Fill),
            Space::new().height(18),
            buttons_centered,
        ]
        .padding([24, 28])
        .width(360),
    )
    .style(move |_theme: &Theme| container::Style {
        background: Some(Background::Color(card_bg)),
        border: Border {
            radius: radius.into(),
            width: border_width,
            color: card_border,
        },
        ..Default::default()
    })
    .into()
}

// ── Button helpers ──────────────────────────────────────────────────────────

/// Primary accent button.
fn accent_button<'a, M: Clone + 'a>(
    label: &str,
    colors: ResolvedColors,
    on_press: M,
) -> Element<'a, M> {
    let bg_color = colors.icon_accent;
    let txt_color = colors.accent_btn_text;
    let radius = colors.corner_radius;
    let bw = colors.border_width;
    let border_color = colors.card_border;

    button(
        container(text(label.to_string()).size(13).color(txt_color).center())
            .width(Length::Fill)
            .center_x(Length::Fill),
    )
    .on_press(on_press)
    .width(100)
    .padding([9, 18])
    .style(move |_theme: &Theme, status| {
        let bg = match status {
            iced::widget::button::Status::Hovered => Color {
                r: (bg_color.r + 0.12).min(1.0),
                g: (bg_color.g + 0.12).min(1.0),
                b: (bg_color.b + 0.12).min(1.0),
                a: bg_color.a,
            },
            iced::widget::button::Status::Pressed => Color {
                r: (bg_color.r - 0.08).max(0.0),
                g: (bg_color.g - 0.08).max(0.0),
                b: (bg_color.b - 0.08).max(0.0),
                a: bg_color.a,
            },
            _ => bg_color,
        };
        iced::widget::button::Style {
            background: Some(Background::Color(bg)),
            text_color: txt_color,
            border: Border {
                radius: radius.into(),
                width: bw,
                color: border_color,
            },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    })
    .into()
}

/// Secondary outline button.
fn subtle_button<'a, M: Clone + 'a>(
    label: &str,
    colors: ResolvedColors,
    on_press: M,
) -> Element<'a, M> {
    let border_color = colors.subtle_btn_border;
    let txt_color = colors.subtle_btn_text;
    let card_bg = colors.card_bg;
    let radius = colors.corner_radius;
    let bw = colors.border_width.max(1.0);

    button(
        container(text(label.to_string()).size(13).color(txt_color).center())
            .width(Length::Fill)
            .center_x(Length::Fill),
    )
    .on_press(on_press)
    .width(100)
    .padding([9, 18])
    .style(move |_theme: &Theme, status| {
        let bg = match status {
            iced::widget::button::Status::Hovered => Color {
                r: (card_bg.r + 0.06).min(1.0),
                g: (card_bg.g + 0.06).min(1.0),
                b: (card_bg.b + 0.06).min(1.0),
                a: 1.0,
            },
            iced::widget::button::Status::Pressed => Color {
                r: (card_bg.r - 0.03).max(0.0),
                g: (card_bg.g - 0.03).max(0.0),
                b: (card_bg.b - 0.03).max(0.0),
                a: 1.0,
            },
            _ => Color::TRANSPARENT,
        };
        iced::widget::button::Style {
            background: Some(Background::Color(bg)),
            text_color: txt_color,
            border: Border {
                radius: radius.into(),
                width: bw,
                color: border_color,
            },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    })
    .into()
}
