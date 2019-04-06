#![feature(trace_macros)]
#![feature(proc_macro_hygiene)]
#![feature(slice_patterns)]
#![allow(
    clippy::type_complexity,
    clippy::infallible_destructuring_match,
    clippy::many_single_char_names
)]

mod normalize;
pub use crate::normalize::*;
pub mod binary;
mod dhall_type;
pub mod imports;
pub mod typecheck;
pub use crate::dhall_type::*;
pub use dhall_generator::expr;
pub use dhall_generator::subexpr;
pub use dhall_generator::StaticType;
pub use crate::imports::*;

// pub struct DhallExpr(dhall_core::DhallExpr);

// impl DhallExpr {
//     pub fn normalize(self) -> Self {
//         DhallExpr(crate::normalize::normalize(self.0))
//     }
// }
