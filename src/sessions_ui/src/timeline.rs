pub use splat_session_records::timeline::*;

#[path = "../../timeline/page.rs"]
pub mod page;

#[cfg(test)]
#[path = "../../timeline/test_support.rs"]
mod test_support;
#[cfg(test)]
pub use test_support::{TestUndo, press_footer_undo};
