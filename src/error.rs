#[macro_export] macro_rules! assert { ($cond:expr $(, $val:expr)* ) => { std::assert!($cond, "{}. {:?}", stringify!($cond), ( $( format!("{} = {:?}", stringify!($val), $val), )* ) ); } }

#[cfg(feature="fehler")] pub use fehler::{throws, throw}; // fehler should $crate

cfg_if::cfg_if! { if #[cfg(feature="anyhow")] {
	pub use anyhow::{Error, bail, ensure, Context};
} else {
    #[derive(Debug)] pub struct Error(Box<dyn std::error::Error>);
    impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Display::fmt(&self.0, f) } }
    impl<E:std::error::Error+'static> From<E> for Error { fn from(error: E) -> Self { Error(Box::new(error)) } }
    struct MessageError<M>(M);
    impl<M:std::fmt::Debug> std::fmt::Debug for MessageError<M> { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) } }
    impl<M:std::fmt::Display> std::fmt::Display for MessageError<M> { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Display::fmt(&self.0, f) } }
    impl<M:std::fmt::Debug+std::fmt::Display> std::error::Error for MessageError<M> {}
    impl Error { pub fn msg(msg: impl std::fmt::Debug+std::fmt::Display+'static) -> Error { Error(Box::new(MessageError(msg))) } }
    #[macro_export] macro_rules! bail { ($val:expr) => { fehler::throw!($crate::error::Error::msg(format!("{:?}", $val))); } }
	#[macro_export] macro_rules! ensure { ($cond:expr) => { if !$cond { $crate::bail!(stringify!($cond)) } } }
}}

pub type Result<T=(), E=Error> = std::result::Result<T, E>;
