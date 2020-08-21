#![cfg_attr(feature="array", feature(min_const_generics))]
#![cfg_attr(feature="timeout", feature(thread_spawn_unchecked,duration_zero))]

pub use core::{ops, result, convert, fmt}; // derive_more
/*pub*/ use cfg_if::cfg_if;
pub mod option;
//#[macro_export] macro_rules! dbg { ( $first:expr $(,$A:expr)* ) => ( eprint!("{} = {:?}", stringify!($first), $first); $( eprint!(", {} = {:?}", stringify!($A), $A); )* eprintln!(""); ) }
pub mod error; //pub use error::{Error, Result/*bail, ensure, Ok*/}; #[cfg(feature="fehler")] pub use error::throws;
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
