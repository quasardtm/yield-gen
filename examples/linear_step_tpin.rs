#![feature(coroutines, coroutine_trait)]
// #![feature(pin_macro)]
use core::{ops::Coroutine};

use yield_gen::{loop_while_yield, yield_pin};

fn main() {
    let mut gen = linear_updown(3., -10., 10.);
    let mut gen = unsafe { core::pin::Pin::new_unchecked(&mut gen) };
    let over_val = loop_while_yield!(
        gen.as_mut(),
        1./6.,
        y => {
            println!("yield : {}", y);
            1./6.
        }
    );
    println!("over_val : {}", over_val);
}

fn linear_step_t(a: f64, mut b: f64, threshold: f64) -> impl Coroutine<f64, Yield = f64, Return = f64> + Unpin {
    #[coroutine] move |mut dt| {
        if a > 0. {
            b += a * dt;
            while b <= threshold {
                dt = yield b;
                b += a * dt;
            }
        } else if a < 0. {
            b += a * dt;
            while b >= threshold {
                dt = yield b;
                b += a * dt;
            }

        } else {
            panic!("slope is zero.")
        }
        (b - threshold) / a
    }
}

fn linear_updown(a: f64, b: f64, threshold: f64) -> impl Coroutine<f64, Yield = f64, Return = f64> {
    #[coroutine] static move |dt| {
        let dt = yield_pin!(linear_step_t(a, b, threshold), dt);
        yield_pin!(linear_step_t(-a, threshold, b), dt)
    }
}