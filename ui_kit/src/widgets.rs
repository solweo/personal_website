mod buttons;

pub use buttons::*;

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