A crate for using Spatie Ray to dump debug a rust project.

## Usage

To dump to ray simply use the `ray!` macro and pass it anything that implements the Debug trait

```rust
use rs_ray::*;

ray!(foo);

// just comma seperate for multiple values
ray!(foo, bar);

// add color
ray!(foo).color("green");
```
