#![no_std]
#![feature(allow_internal_unstable)]

#[macro_export]
#[allow_internal_unstable(coroutines, coroutine_trait)]
macro_rules! loop_while_yield {
    ($pinned_coroutine:expr, $yield_match:pat => $yield_expr:expr) => {
        loop {
            match ::core::ops::Coroutine::resume($pinned_coroutine, ()) {
                ::core::ops::CoroutineState::Yielded($yield_match) => $yield_expr,
                ::core::ops::CoroutineState::Complete(r) => break r,
            }
        }
    };
    ($pinned_coroutine:expr, $init:expr, $yield_match:pat => $yield_expr:expr) => {
        match $init {
            mut arg => {
                loop {
                    arg = match ::core::ops::Coroutine::resume($pinned_coroutine, arg) {
                        ::core::ops::CoroutineState::Yielded($yield_match) => $yield_expr,
                        ::core::ops::CoroutineState::Complete(r) => break r,
                    }
                }
            }
        }
    };
}

#[macro_export]
#[allow_internal_unstable(coroutines)]
macro_rules! yield_gen {
    ($pinned_coroutine:expr) => {
        $crate::loop_while_yield!($pinned_coroutine, y => yield y)
    };
    ($pinned_coroutine:expr, $init:expr) => {
        $crate::loop_while_yield!($pinned_coroutine, $init, y => yield y)
    };
}

#[macro_export]
#[allow_internal_unstable(coroutines)]
macro_rules! yield_pin {
    ($coroutine:expr) => {
        match ::core::pin::pin!($coroutine) {
            mut gen => $crate::yield_gen!(gen.as_mut())
        }
    };
    ($coroutine:expr, $init:expr) => {
        match ::core::pin::pin!($coroutine) {
            mut gen => $crate::yield_gen!(gen.as_mut(), $init)
        }
    };
}