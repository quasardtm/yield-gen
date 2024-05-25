#![feature(coroutines, coroutine_trait)]
use core::{ops::Coroutine};

use yield_gen::{loop_while_yield, yield_pin};

fn main() {
    let mut gen = linear_updown(3., -10., 10.);
    let mut gen = unsafe { core::pin::Pin::new_unchecked(&mut gen) };
    let over_val = loop_while_yield!(
        gen.as_mut(),
        y => {
            println!("yield : {}", y);
        }
    );
    println!("over_val : {}", over_val);
}

fn linear_step_one(a: f64, mut b: f64, threshold: f64) -> impl Coroutine<(), Yield = f64, Return = f64> + Unpin {
    #[coroutine] move || {
        if a > 0. {
            b += a;
            while b <= threshold {
                yield b;
                b += a;
            }
        } else if a < 0. {
            b += a;
            while b >= threshold {
                yield b;
                b += a;
            }

        } else {
            panic!("slope is zero.")
        }
        b - threshold
    }
}

fn linear_updown(a: f64, b: f64, threshold: f64) -> impl Coroutine<(), Yield = f64, Return = f64> {
    #[coroutine] static move || {
        let over_val = yield_pin!(linear_step_one(a, b, threshold));
        yield_pin!(linear_step_one(-a, threshold + over_val, b))
    }
}