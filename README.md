```bash
$ cargo run --features "test_generator"
error[E0658]: yield syntax is experimental
  --> src\main.rs:15:9
   |
15 |         yield 1;
   |         ^^^^^^^
   |
   = note: for more information, see https://github.com/rust-lang/rust/issues/43122
   = help: add `#![feature(generators)]` to the crate attributes to enable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\cfg-yield-test.exe`
Hello, world!
$ cargo run
   Compiling cfg-yield-test v0.1.0 (C:\Users\hato2\Desktop\cfg-yield-test)
error[E0658]: yield syntax is experimental
  --> src\main.rs:15:9
   |
15 |         yield 1;
   |         ^^^^^^^
   |
   = note: for more information, see https://github.com/rust-lang/rust/issues/43122
   = help: add `#![feature(generators)]` to the crate attributes to enable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `cfg-yield-test`.

To learn more, run the command again with --verbose.
```