pub mod tagline;

pub use tagline::*;

use stylance::import_style as get_css;
get_css!(pub style_baseline, "./index.css");