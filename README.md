A crate for using Spatie Ray to dump debug a rust project.

## Get started

```bash
cargo add rs-ray
```

To use within a tokio runtime add the "with_tokio" feature flag.

```bash
cargo add rs-ray --features with_tokio
```

## Usage

To dump to ray simply use the `ray!` macro and pass it anything that implements the Debug trait

```rust
use rs_ray::*;

ray!(foo);

// just comma seperate for multiple values
ray!(foo, bar);

// add color
ray!(foo).color("green");

// log
ray!().log(vec!["log".to_string()]);

// text
ray!().text("text".to_string());

// confetti
ray!().confetti();

// clear ray screen
ray!().clear_all();
```
