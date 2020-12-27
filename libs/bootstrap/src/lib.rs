// #[macro_use]
// #[macro_export]
// macro_rules! div {
//     ($classes: expr, $($x: expr),*) => {
//         html! {
//             <div class=$classes>
//               $($x)*
//             </div>
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! main_container {
//     ($($h: expr),*) => {
//         $crate::div!("main container", $($h),*)
//     }
// }
//
// #[macro_export]
// macro_rules! col {
//     ($($x: expr),*) => {
//         $crate::div!("col", $($x),*)
//     };
// }
//
// #[macro_export]
// macro_rules! row {
//     ($($x: expr),*) => {
//         $crate::div!("row", $($x),*)
//     };
// }
//
// #[macro_use]
mod bootstrap;
pub use crate::bootstrap::*;
