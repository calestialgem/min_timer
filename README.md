# min_timer

---

This is a small library that provides `f64` based duration and timer. Standard
library's implementation uses integers. Thus, for a clock that gives time as
`f64`, this library should have higher performance.

A small statistics and profiling functionality is also provided. This are all
intended to be used in a real-time application.

Additionally, there are lesschecks. Although, there is strong type safety for SI
units (seconds), which is hopefully optimized away by the compiler.

---

## Examples

```rust
fn count(upto: u32) {
    use min_timer::{now::Std, Sec, Timer};

    let dur = Sec::MINUTE; // strong type safety.
    let now = Std::new();  // there is a std::time implementation.
    let mut timer = Timer::new(&now);
    let mut count = 0;

    while count < upto {
        if timer >= dur { // straight-forward checking,
            timer += dur; // flexible manuplation.
            count += 1;
            println!("Counting {}...", count);
        }
    }
}

fn subroutine() {}

fn main_routine() {
    use min_timer::{now::Std, Profile, Stat};

    let mut stat = Stat::new();
    let now = Std::new();

    for _ in 0..10 {
        let _ = Profile::new(&now, &mut stat); // create and forget.
        subroutine();
    }

    // End of cycle.
    // This can be anything.
    // For example: every second in a game engine.
    // This way the rate will be the FPS counter.
    stat.refresh();

    for _ in 0..15 {
        let _ = Profile::new(&now, &mut stat);
        subroutine();
    }

    println!(
        "Subroutine called {} times, with {} average runtime and {} times per cycle.",
        stat.get_count(), // will be 25
        stat.find_average(),
        stat.get_rate()   // will be 15
    );
}
```

---

## Motivation

Why write this when there is the standard library?

1. Education: I got to practice **Rust**, espacially _newtype_ pattern with
   `Sec`.
2. I didn't now much about `std::time` before writing this.
3. I will use this with **GLFW** timer, which returns the time as a `double` in
   seconds. This way I will implement `Now` with **GLFW** and there will be no
   conversions compared to:

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

4. This crate provided a space where I could put more stuff about time, like
   profiling.

---

Copyright (C) 2022 Cem Geçgel <gecgelcem@outlook.com>
