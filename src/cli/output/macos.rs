use objc2_app_kit::{NSPasteboard, NSPasteboardTypeHTML, NSPasteboardTypeString};
use objc2_foundation::NSString;

pub fn copy_to_clipboard(html: &str, text: &str) {
    let pasteboard = { NSPasteboard::generalPasteboard() };

    pasteboard.clearContents();

    let html = NSString::from_str(html);
    let text = NSString::from_str(text);

    let apple_html_type = NSString::from_str("Apple HTML pasteboard type");

    unsafe {
        pasteboard.setString_forType(&html, NSPasteboardTypeHTML);

        pasteboard.setString_forType(&html, &apple_html_type);

        pasteboard.setString_forType(&text, NSPasteboardTypeString);
    }
}
