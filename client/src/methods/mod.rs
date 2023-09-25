//! Network protocol used for data exfiltration
#[doc(inline)]
pub use dns::*;
#[doc(inline)]
pub use https::*;

pub mod dns;
pub mod https;