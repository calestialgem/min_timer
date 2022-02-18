# min_timer

---

As things grow, this library grew a lot over the last two days. Now it consists
of three parts:

1. Timer
2. Profiler
3. Main Loop

Each section depends on the previous one.

---

## Timer

This is a small library that provides `f64` based duration and timer. Standard
library's implementation uses integers. Thus, for a clock that gives time as
`f64`, this library should have higher performance.

Additionally, there are less checks. Although, there is strong type safety for
SI units (seconds), which is hopefully optimized away by the compiler.

```rust
fn count(upto: u32) {
    use min_timer::{Std, Sec, Timer};

    let dur = Sec::MINUTE; // strong type safety.
    let now = Std::new();  // there is a std::time implementation.
    let mut timer = Timer::new(&now);
    let mut count = 0;

    while count < upto {
        if timer >= dur { // straight-forward checking,
            timer -= dur; // flexible manupilation.
            count += 1;
            println!("Counting {}...", count);
        }
    }
}
```

---

## Profiler

A small statistics and profiling functionality is also provided. These are all
intended to be used in a real-time application.

```rust
fn subroutine() {}

fn main_routine() {
    use min_timer::{Std, Prf, Stat};

    let mut stat = Stat::new();
    let now = Std::new();

    for _ in 0..10 {
        let _ = Prf::new(&now, &mut stat); // create and forget.
        subroutine();
    }

    // End of cycle.
    // This can be anything.
    // For example: every second in a game engine.
    // This way the rate will be the FPS counter.
    stat.refresh();

    for _ in 0..15 {
        let _ = Prf::new(&now, &mut stat);
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

## Main Loop

This is the _heart_ of a real-time application. The design is such that, you
provide a state class and a render that can draw it. The tick rate and the frame
rate are different; such that, smooth visuals can be achived without updating at
the same frequency.

This is done by interpolating the previous and current states of the program
before drawing using the remaning ticks to be done. Thus, states must implement
scaling and superposing; linearly combining.

```rust
use min_timer::{Hrt, Now, Render, Std, Stt, Timer};
use std::ops::{Add, Mul};

struct Bar {
    len: u32,
    pre: Option<u32>,
}

impl Default for Bar {
    // Creating the render
    fn default() -> Self {
        Self { len: 50, pre: None }
    }
}

impl Bar {
    fn print(&mut self, per: f64, len: u32) {
        self.pre = Some(len);
        print!("[");
        for _ in 0..len {
            print!("=");
        }
        if self.len != len {
            print!(">");
            for _ in 0..(self.len - len - 1) {
                print!(" ");
            }
        }
        println!("] {}%", per);
    }
}

impl<T: Now> Render<T, Ex> for Bar {
    // Rendering
    fn render(&mut self, _: &Hrt<T>, stt: &Ex) {
        let len = self.len as f64 * stt.0;
        let len = len.floor() as u32;
        let len = len.min(self.len);
        let per = (stt.0 * 100.0).floor();
        if let Some(pre) = self.pre {
            if len != pre {
                self.print(per, len);
            }
        } else {
            self.print(per, len);
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Ex(f64);

impl Mul<f64> for Ex {
    type Output = Ex;

    // Scaling
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Add for Ex {
    type Output = Ex;

    // Superposing
    fn add(self, rhs: Ex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Now> Stt<T> for Ex {
    // Initialization; timer provided for profiling
    fn init(&mut self, _: &mut Hrt<T>, timer: Timer<T>) {
        println!("Initialization done in {}!", timer);
    }

    // Updating; heart provided for manupilation
    fn update(&mut self, hrt: &mut Hrt<T>) {
        self.0 += 1e-1;
        if self.0 >= 1.0 {
            hrt.stop();
        }
    }

    // Profiling every second; heart provided for manupilation
    fn sec(&mut self, hrt: &mut Hrt<T>) {
        println!(
            "Tick Rate: {} Frame Rate: {}",
            hrt.ticks().avg_rate(),
            hrt.frames().avg_rate()
        );
    }
}

fn main() {
    let now = Std::new(); // using the standard library's clock
    let mut hrt = Hrt::new(1e2, &now); // target tick rate 100.0
    hrt.start::<Ex, Bar>(); // creates from defaults
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
5. Working with `f64`s is a lot more comfortable; I saw this as I worked on the
   main loop.

---

Copyright (C) 2022 Cem Geçgel <gecgelcem@outlook.com>
