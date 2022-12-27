#![no_std]
#![feature(allow_internal_unstable)]

#[macro_export]
#[allow_internal_unstable(generators, generator_trait)]
macro_rules! loop_while_yield {
    ($pinned_generator:expr, $yield_match:pat => $yield_expr:expr) => {
        loop {
            match ::core::ops::Generator::resume($pinned_generator, ()) {
                ::core::ops::GeneratorState::Yielded($yield_match) => $yield_expr,
                ::core::ops::GeneratorState::Complete(r) => break r,
            }
        }
    };
    ($pinned_generator:expr, $init:expr, $yield_match:pat => $yield_expr:expr) => {
        match $init {
            mut arg => {
                loop {
                    arg = match ::core::ops::Generator::resume($pinned_generator, arg) {
                        ::core::ops::GeneratorState::Yielded($yield_match) => $yield_expr,
                        ::core::ops::GeneratorState::Complete(r) => break r,
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! yield_gen {
    ($pinned_generator:expr) => {
        $crate::loop_while_yield!($pinned_generator, y => yield y)
    };
    ($pinned_generator:expr, $init:expr) => {
        $crate::loop_while_yield!($pinned_generator, $init, y => yield y)
    };
}

#[macro_export]
#[allow_internal_unstable(pin_macro)]
macro_rules! yield_pin {
    ($generator:expr) => {
        match ::core::pin::pin!($generator) {
            mut gen => $crate::yield_gen!(gen.as_mut())
        }
    };
    ($generator:expr, $init:expr) => {
        match ::core::pin::pin!($generator) {
            mut gen => $crate::yield_gen!(gen.as_mut(), $init)
        }
    };
}