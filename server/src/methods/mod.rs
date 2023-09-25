//! Network protocol used for data exfiltration
#[doc(inline)]
pub use dns::*;
#[doc(inline)]
pub use https::*;
#[doc(inline)]
pub use icmp::*;

pub mod dns;
pub mod https;
// TODO ICMP
pub mod icmp;