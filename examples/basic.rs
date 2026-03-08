//! Full-featured example: message box gallery with geometry controls, glyph picker,
//! dark/light toggle, overlay triggers, and inline card previews.

use iced::widget::{
    button, column, combo_box, container, row, slider, stack, text, toggler, Space,
};
use iced::{Color, Element, Length, Task, Theme};
use iced_message_box::{MessageBox, MessageBoxResult};

/// Preset glyphs available in the picker.
const GLYPH_OPTIONS: &[(&str, &str)] = &[
    ("Default", ""),
    ("\u{2713} Check", "\u{2713}"),
    ("\u{2717} Cross", "\u{2717}"),
    ("\u{2605} Star", "\u{2605}"),
    ("\u{2665} Heart", "\u{2665}"),
    ("\u{266B} Music", "\u{266B}"),
    ("\u{26A1} Bolt", "\u{26A1}"),
    ("\u{2602} Umbrella", "\u{2602}"),
    ("\u{263A} Smile", "\u{263A}"),
    ("\u{2622} Radiation", "\u{2622}"),
    ("\u{2699} Gear", "\u{2699}"),
    ("\u{270E} Pencil", "\u{270E}"),
    ("\u{2764} Heart2", "\u{2764}"),
    ("\u{2660} Spade", "\u{2660}"),
    ("\u{2663} Club", "\u{2663}"),
    ("\u{2666} Diamond", "\u{2666}"),
    ("\u{00A9} Copyright", "\u{00A9}"),
    ("\u{2620} Skull", "\u{2620}"),
];

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title("Message Box Example")
        .theme(App::theme)
        .window_size((960.0, 820.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    // Trigger each dialog type
    ShowInfo,
    ShowSuccess,
    ShowWarning,
    ShowError,
    ShowYesNo,
    ShowYesNoCancel,
    ShowOkCancel,

    // Dialog response
    DialogResult(MessageBoxResult),
    Dismiss,

    // Settings
    ToggleDarkMode(bool),
    CornerRadiusChanged(f32),
    BorderWidthChanged(f32),

    // Glyph picker
    GlyphInputChanged(String),
    GlyphComboSelected(String),
}

struct App {
    active_dialog: Option<MessageBox>,
    last_result: String,
    is_dark: bool,
    corner_radius: f32,
    border_width: f32,
    selected_glyph: Option<String>,
    glyph_combo_state: combo_box::State<String>,
    glyph_combo_input: String,
}

impl App {
    fn boot() -> (Self, Task<Message>) {
        let glyph_labels: Vec<String> = GLYPH_OPTIONS
            .iter()
            .map(|(label, _)| label.to_string())
            .collect();

        (
            Self {
                active_dialog: None,
                last_result: "No dialog shown yet".to_string(),
                is_dark: true,
                corner_radius: 12.0,
                border_width: 1.0,
                selected_glyph: None,
                glyph_combo_state: combo_box::State::new(glyph_labels),
                glyph_combo_input: String::new(),
            },
            Task::none(),
        )
    }

    /// Apply current settings to a MessageBox.
    fn apply_settings(&self, mut mb: MessageBox) -> MessageBox {
        mb = if self.is_dark { mb.dark() } else { mb.light() };
        mb = mb.with_corner_radius(self.corner_radius);
        mb = mb.with_border_width(self.border_width);
        if let Some(ref glyph) = self.selected_glyph {
            if !glyph.is_empty() {
                mb = mb.with_glyph(glyph.clone());
            }
        }
        mb
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ShowInfo => {
                let mb = MessageBox::info("Information", "The export completed successfully.");
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::ShowSuccess => {
                let mb = MessageBox::success("Success", "Theme saved to disk.");
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::ShowWarning => {
                let mb = MessageBox::warning("Warning", "File already exists and will be overwritten.");
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::ShowError => {
                let mb = MessageBox::error("Error", "Failed to save the theme file.");
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::ShowYesNo => {
                let mb = MessageBox::ask_yes_no("Confirm", "Apply the harmony palette to your theme?");
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::ShowYesNoCancel => {
                let mb = MessageBox::ask_yes_no_cancel("Save Changes", "Save changes before closing?");
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::ShowOkCancel => {
                let mb = MessageBox::ask_ok_cancel("Proceed", "This action cannot be undone.")
                    .with_accent(Color::from_rgb(0.90, 0.40, 0.10));
                self.active_dialog = Some(self.apply_settings(mb));
            }
            Message::DialogResult(result) => {
                self.last_result = format!("Result: {:?}", result);
                self.active_dialog = None;
            }
            Message::Dismiss => {
                self.active_dialog = None;
            }
            Message::ToggleDarkMode(is_dark) => {
                self.is_dark = is_dark;
            }
            Message::CornerRadiusChanged(v) => {
                self.corner_radius = v;
            }
            Message::BorderWidthChanged(v) => {
                self.border_width = v;
            }
            Message::GlyphInputChanged(input) => {
                self.glyph_combo_input = input;
            }
            Message::GlyphComboSelected(label) => {
                if let Some((_, glyph)) = GLYPH_OPTIONS.iter().find(|(l, _)| *l == label.as_str()) {
                    self.selected_glyph = if glyph.is_empty() {
                        None
                    } else {
                        Some(glyph.to_string())
                    };
                    self.glyph_combo_input = label;
                }
            }
        }
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Message Box Gallery").size(24);
        let result_text = text(self.last_result.as_str()).size(14);

        // ── Settings row ────────────────────────────────────────────
        let dark_toggle = row![
            text("Dark Mode").size(13),
            toggler(self.is_dark).on_toggle(Message::ToggleDarkMode),
        ]
        .spacing(8);

        let radius_control = row![
            text(format!("Corner Radius: {:.0}", self.corner_radius)).size(13),
            slider(0.0..=30.0, self.corner_radius, Message::CornerRadiusChanged)
                .step(1.0)
                .width(120),
        ]
        .spacing(8);

        let border_control = row![
            text(format!("Border: {:.1}px", self.border_width)).size(13),
            slider(0.0..=5.0, self.border_width, Message::BorderWidthChanged)
                .step(0.5)
                .width(120),
        ]
        .spacing(8);

        let glyph_label = match &self.selected_glyph {
            Some(g) => format!("Glyph: {}", g),
            None => "Glyph: default".to_string(),
        };

        let glyph_control = row![
            text(glyph_label).size(13),
            combo_box(
                &self.glyph_combo_state,
                "Search glyphs...",
                None::<&String>,
                Message::GlyphComboSelected,
            )
            .on_input(Message::GlyphInputChanged)
            .width(160),
        ]
        .spacing(8);

        let settings = row![
            dark_toggle,
            Space::new().width(16),
            radius_control,
            Space::new().width(16),
            border_control,
            Space::new().width(16),
            glyph_control,
        ];

        // ── Trigger buttons ─────────────────────────────────────────
        let triggers = column![
            text("Click a button to show the overlay:").size(14),
            Space::new().height(8),
            row![
                styled_trigger("Info", Message::ShowInfo),
                styled_trigger("Success", Message::ShowSuccess),
                styled_trigger("Warning", Message::ShowWarning),
                styled_trigger("Error", Message::ShowError),
            ]
            .spacing(8),
            Space::new().height(4),
            row![
                styled_trigger("Yes / No", Message::ShowYesNo),
                styled_trigger("Yes / No / Cancel", Message::ShowYesNoCancel),
                styled_trigger("OK / Cancel (orange)", Message::ShowOkCancel),
            ]
            .spacing(8),
        ];

        // ── Inline card previews ────────────────────────────────────
        let preview_title = text("Inline card previews (live geometry + glyph):").size(14);

        let noop = |_: MessageBoxResult| Message::Dismiss;

        let mb_info = self.apply_settings(MessageBox::info("Information", "Export completed."));
        let mb_success = self.apply_settings(MessageBox::success("Success", "Theme saved."));
        let mb_warning = self.apply_settings(MessageBox::warning("Warning", "File exists."));
        let mb_error = self.apply_settings(MessageBox::error("Error", "Save failed."));
        let mb_yesno = self.apply_settings(MessageBox::ask_yes_no("Confirm", "Apply palette?"));
        let mb_custom = self.apply_settings(
            MessageBox::ask_ok_cancel("Custom", "Orange accent!")
                .with_accent(Color::from_rgb(0.90, 0.40, 0.10)),
        );

        let preview_row_1 = row![
            mb_info.card(noop),
            mb_success.card(noop),
            mb_warning.card(noop),
        ]
        .spacing(12);

        let preview_row_2 = row![
            mb_error.card(noop),
            mb_yesno.card(noop),
            mb_custom.card(noop),
        ]
        .spacing(12);

        let base: Element<'_, Message> = container(
            column![
                row![title, Space::new().width(Length::Fill)],
                Space::new().height(8),
                settings,
                Space::new().height(12),
                result_text,
                Space::new().height(12),
                triggers,
                Space::new().height(20),
                preview_title,
                Space::new().height(8),
                preview_row_1,
                Space::new().height(12),
                preview_row_2,
            ]
            .padding(24)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        // Layer overlay on top if a dialog is active
        if let Some(ref dialog) = self.active_dialog {
            let overlay = dialog.overlay(Message::DialogResult);
            stack![base, overlay].into()
        } else {
            base
        }
    }
}

/// A styled trigger button.
fn styled_trigger(label: &str, on_press: Message) -> Element<'_, Message> {
    button(text(label.to_string()).size(13).center())
        .on_press(on_press)
        .padding([8, 16])
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            let base = palette.primary.base.color;
            let bg = match status {
                iced::widget::button::Status::Hovered => Color {
                    r: (base.r + 0.1).min(1.0),
                    g: (base.g + 0.1).min(1.0),
                    b: (base.b + 0.1).min(1.0),
                    a: base.a,
                },
                _ => base,
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: palette.primary.base.text,
                border: iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
                snap: false,
            }
        })
        .into()
}
