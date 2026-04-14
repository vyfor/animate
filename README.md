# animate

Animation library for Rust.

## Disclaimer

Using `animate` in multithreaded environments will likely cause undefined behavior. The library is designed around the assumption that render loops run on a single thread, the inverse is not supported (yet).

## Features

- **Lightweight**: Zero dependencies by default.
- **Ergonomic**: Macro-driven API with minimal boilerplate.
- **Extensible**: Many built-in types with support for custom interpolators.
- **Animation modes**: `#[once]`, `#[cycle]`, and `#[alternate]`.
- **Easing**: Built-in and custom easing functions.
- **Ratatui-friendly**: Interpolators for ratatui types, gated behind the `ratatui` feature flag.

## Installation

```sh
cargo add animate
```

## Getting started

Add `#[animate]` to a struct and mark the fields you want to animate:

```rust
#[animate]
pub struct MyWidget {
    #[once(duration = 300)]
    progress: f64,

    #[cycle(duration = 400, easing = ease_in_cubic)]
    color: Color,

    #[alternate(duration = 500, easing = ease_in_out_quad)]
    status: String,
}
```

Use `get()` to read and `set()` to write animated fields. Place `animate::tick()` at the start of each frame as this avoids unnecessary computations within same render frame.

```rust
loop {
    animate::tick();
    draw(|frame| {
        // render logic
    })?;
}
```

## Minimal example

```rust
use animate::animate;
use std::{io::{stdout, Write}, thread, time::Duration};

#[animate]
struct Counter {
    #[once(duration = 400)]
    value: u32,
}

fn main() -> std::io::Result<()> {
    // new() is auto-generated
    let mut c = Counter::new(0);

    loop {
        // must be called at the start of each frame
        animate::tick();

        let v = *c.value;
        if v == 0 {
            c.value.set(100);
        }

        print!("\rCounter value: {v}");
        stdout().flush()?;

        if v == 100 {
            break;
        }

        thread::sleep(Duration::from_millis(8));
    }

    Ok(())
}
```