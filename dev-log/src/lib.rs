#[doc(hidden)]
pub use dev_log_macros::*;

#[cfg(feature = "stack-trace")]
#[doc(hidden)]
pub mod stack_trace;

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        #[cfg(feature = "stack-trace")]
        {
            $crate::stack_trace::StackTrace::print_current();
        }
        #[cfg(feature = "allow-print")]
        {
            ::std::println!($($args)*);
        }
    };
}