# iced_message_box

A themed message box overlay widget for [Iced](https://github.com/iced-rs/iced).

![Iced 0.14](https://img.shields.io/badge/iced-0.14-blue)
![License: MIT](https://img.shields.io/badge/license-MIT-green)

No full-featured themed message box exists in the Iced ecosystem — this fills that gap. Inspired by [CTkMessagebox](https://github.com/Akascape/CTkMessagebox) for Python/CustomTkinter.

![Gallery](https://raw.githubusercontent.com/Zhanjii/iced_message_box/master/assets/gallery.png)

| Glyph Picker | Overlay Dialog |
|---|---|
| ![Glyph Picker](https://raw.githubusercontent.com/Zhanjii/iced_message_box/master/assets/glyph_picker.png) | ![Overlay](https://raw.githubusercontent.com/Zhanjii/iced_message_box/master/assets/overlay.png) |

## Features

- Five icon types: Info, Success, Warning, Error, Question
- Four button layouts: OK, Yes/No, Yes/No/Cancel, OK/Cancel
- Dark and light mode support
- Icon badges with shadow/stroke outline effect for crisp readability
- Custom glyph support — use any Unicode symbol as the icon
- Customizable accent color, corner radius, border width, and card colors
- Renders as an in-app overlay (no native OS dialogs)
- Builder-pattern API with convenience constructors
- Generic over your app's message type
- Inline card mode for embedding in layouts/previews

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
iced_message_box = "0.1"
```

### Show a dialog overlay

```rust
use iced::widget::stack;
use iced_message_box::{MessageBox, MessageBoxResult};

// Store in your app state:
struct App {
    dialog: Option<MessageBox>,
}

// Trigger a dialog:
self.dialog = Some(MessageBox::ask_yes_no(
    "Confirm",
    "Delete this item?",
));

// In your view, layer the overlay:
if let Some(ref dialog) = self.dialog {
    let overlay = dialog.overlay(|result| MyMessage::DialogResult(result));
    stack![base_content, overlay].into()
} else {
    base_content
}

// Handle the result:
MyMessage::DialogResult(result) => {
    match result {
        MessageBoxResult::Yes => delete_item(),
        _ => {}
    }
    self.dialog = None;
}
```

### All convenience constructors

```rust
use iced_message_box::MessageBox;

// Single OK button
let mb = MessageBox::info("Title", "Informational message.");
let mb = MessageBox::success("Title", "Operation succeeded.");
let mb = MessageBox::warning("Title", "Something may be wrong.");
let mb = MessageBox::error("Title", "Something went wrong.");

// Question dialogs
let mb = MessageBox::ask_yes_no("Title", "Yes or No?");
let mb = MessageBox::ask_yes_no_cancel("Title", "Yes, No, or Cancel?");
let mb = MessageBox::ask_ok_cancel("Title", "OK or Cancel?");
```

### Customization

```rust
use iced::Color;
use iced_message_box::{MessageBox, MessageBoxColors};

// Dark/light mode
let mb = MessageBox::info("Title", "Message").light();
let mb = MessageBox::info("Title", "Message").dark();  // default

// Custom accent color
let mb = MessageBox::info("Title", "Message")
    .with_accent(Color::from_rgb(0.8, 0.2, 0.5));

// Custom corner radius
let mb = MessageBox::info("Title", "Message")
    .with_corner_radius(20.0);

// Custom border width
let mb = MessageBox::info("Title", "Message")
    .with_border_width(2.0);

// Custom glyph (any Unicode character)
let mb = MessageBox::info("Star!", "You earned a star!")
    .with_glyph("\u{2605}");  // ★

// Full color customization
let mb = MessageBox::info("Title", "Message")
    .with_colors(MessageBoxColors {
        card_background: Some(Color::from_rgb(0.1, 0.1, 0.15)),
        accent: Some(Color::from_rgb(0.0, 0.8, 0.6)),
        ..Default::default()
    });
```

### Inline card (no backdrop)

```rust
// Render just the card for embedding in layouts or previews:
let card = MessageBox::info("Title", "Message").card(|r| MyMessage::Result(r));
```

## Examples

```bash
cargo run --example basic
```

## License

MIT
