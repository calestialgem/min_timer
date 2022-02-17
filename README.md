# min_timer

---

This is a small library that provides `f64` based duration and timer. Standard
library's implementation uses integers. Thus, for a clock that gives time as
`f64`, this library should have higher performance. Additionally, there are less
checks. Although, there is strong type safety for SI units (seconds), which is
hopefully optimized away by the compiler.

---

## Example

```rust
use min_timer::{now::Std, Sec, Timer};

fn wait(dur: Sec) {
    let now = Std::new(); // Uses std::time implementation of min_timer::now::Now.
    let timer = Timer::new(&now);
    while timer < dur {} // A resource consuming software wait.
}

fn count(upto: u32) {
    let dur = Sec::MINUTE;
    let now = Std::new();
    let mut timer = Timer::new(&now);
    let mut count = 0;

    while count < upto {
        if timer >= dur {
            timer += dur; // Move the timer forwards.
            count += 1;
            println!("Counting {}...", count);
        }
    }
}
```

---

## Usage

```rust
// Type safety.
// Seconds can be only added and subtracted with other seconds.
// Can be scaled with scalars tho.
// 4s * 5s = 20 s^2 -> Result has a different dimension: T^2.
// 4s + 5 = ? -> Meaningless, adding numbers of different dimensions: T + 1.
let duration = Sec::new(0.01);
// Or...
let duration = 10.0 * Sec::MILLI;

// To and from standard library's duration.
let std_dur = std::time::Duration::from(duration);

// Provided implementation (using standard library).
// As written previously this has conversions to and from, doubles and integers.
// You should prefer directly using the standard library `timer` (std::time::Instant).
let now = min_timer::now::Std::new();

// Get the time.
// You won't be using this manually.
let start = now.now();
let elapsed = now.now() - start;

// Creating timer, holds a ref to `now`.
let timer = Timer::new(&now);
// Holds the start and gives you elapsed time.
let elapsed = timer.elapsed();

// Usefullness of timer:

// Moving forwards...
let timer = timer + Sec::new(8.0);
// Or...
let mut timer = Timer::new(&now);
timer += Sec::new(1.0);

// Checking elapsed time...
// Simply, `timer` is taken as `elapsed` for these calcualtions.
let passed = timer > duration;
let remaining = duration - timer;
let same = timer == duration;

// Do not thrust `same` that is above!
// These are floating point numbers!
// Use the one on the next line.
let tolerance = Sec::new(1e-3); // 1 us
```

---

## Motivation

Why write this when there is the standard library? First, education: I got to
practice **Rust**, espacially _newtype_ pattern with `Sec`. Second, I didn't now
much about `std::time` before writing this. Third, I will use this with **GLFW**
timer, which returns the time as a `double` in seconds. This way I will
implement `Now` with **GLFW** and there will be no conversions compared to:

```rust
fn time(glfw: &Glfw) -> Duration {
    Duration::from_sec_f64(glfw.get_time()) // conversion!
}

let start = time(&glfw);
let elapsed = time(&glfw) - start;
let seconds = elapsed.as_sec_f64(); // conversion!
```

Check out my other crate, **min_gl**, for seeing the `Now` implementation for
the **GLFW** timer.

---

Copyright (C) 2022 Cem Geçgel <gecgelcem@outlook.com>
