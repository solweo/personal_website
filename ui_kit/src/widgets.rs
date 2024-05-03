mod buttons;
mod header;

pub use buttons::*;
pub use header::*;

#[allow(dead_code)]
pub enum WidgetState {
    Initial,
    Tipstering,
    Entered,
    Loading,
    Success,
    Error,
    Disabled,
    Focus,
    Hover,
} 