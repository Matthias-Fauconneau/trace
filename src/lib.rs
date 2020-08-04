#![cfg_attr(feature="array", allow(incomplete_features),feature(const_generics,maybe_uninit_extra,maybe_uninit_uninit_array))]
#![cfg_attr(feature="timeout", feature(thread_spawn_unchecked,duration_zero))]
#![cfg_attr(feature="vector", feature(iterator_fold_self))]

pub use core::{ops, result, convert, fmt}; // derive_more
/*pub*/ use cfg_if::cfg_if;
pub mod none;
pub mod option;
//#[macro_export] macro_rules! dbg { ( $first:expr $(,$A:expr)* ) => ( eprint!("{} = {:?}", stringify!($first), $first); $( eprint!(", {} = {:?}", stringify!($A), $A); )* eprintln!(""); ) }
pub mod error; //pub use error::{Error, Result/*bail, ensure, Ok*/}; #[cfg(feature="fehler")] pub use error::throws;
pub mod iter;
pub mod slice;
#[cfg(feature="array")] pub mod array; //pub use array::{Iterator, map};
pub mod num; //pub use num::{Zero, Ratio, abs};
cfg_if! { if #[cfg(feature="vector")] { #[macro_use] pub mod vector; pub use vector::{Bounds, MinMax}; }} //xy, int2, uint2, size, vec2
/*pub fn time<T>(task: impl FnOnce() -> T) -> T {
	let start = std::time::Instant::now();
	let result = task();
	eprintln!("{}", start.elapsed().as_millis());
	result
}
pub fn call<T>(task: impl FnOnce() -> T) -> T { task() }*/
pub mod vec;
cfg_if! { if #[cfg(feature="trace")] { mod trace; pub use trace::rstack_self; }}
cfg_if! { if #[cfg(feature="timeout")] { mod timeout; pub use timeout::timeout; }}
#[cfg(feature="signal-hook")] pub use trace::sigint_trace;
#[cfg(feature="unicode-segmentation")] pub mod unicode_segmentation;
