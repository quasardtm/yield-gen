#![feature(coroutines, coroutine_trait)]
use core::{ops::Coroutine, pin::pin};

use yield_gen::{loop_while_yield, yield_gen};

fn main() {
    let mut gen = pin!(linear_updown(3., -10., 10.));
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
        let mut up = pin!(linear_step_one(a, b, threshold));
        let over_val = yield_gen!(up.as_mut());
        let mut down = pin!(linear_step_one(-a, threshold + over_val, b));
        yield_gen!(down.as_mut())
    }
}