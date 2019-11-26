#![cfg_attr(feature = "test_generator", feature(generators, generator_trait))]

fn main() {
    println!("Hello, world!");
}

#[cfg(feature = "test_generator")]
fn test_generator() {
    // Use some `yield`.
    use std::ops::{GeneratorState, Generator};
    use std::pin::Pin;

    let mut generator = || {
        yield 1;
        return "foo"
    };

    match Pin::new(&mut generator).resume() {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume() {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected return from resume"),
    }
}
