#[doc(hidden)]
pub use dev_log_macros::*;

#[cfg(feature = "stack-trace")]
#[doc(hidden)]
pub mod stack_trace;

#[cfg(all(feature = "stack-trace", feature = "allow-print"))]
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::stack_trace::StackTrace::print_current();
        ::std::println!($($args)*);
    };
}

#[cfg(not(feature = "allow-print"))]
#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {};
}

#[cfg(all(not(feature = "stack-trace"), feature = "allow-print"))]
#[macro_export]
macro_rules!  {
    ($($args:tt)*) => {
        ::std::println!($($args)*);
    };
}