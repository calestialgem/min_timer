use crate::{now::Now, Prf, Sec, Stat, Timer};
use std::ops::{Add, Mul};

/// Rendering limitations.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lim {
    /// 0 FPS.
    Never,
    /// 1 FPS.
    Once,
    /// Unlimited FPS.
    Always,
}

impl Lim {
    fn draw(&self, rate: u64) -> bool {
        match self {
            Self::Never => false,
            Self::Once => rate == 0,
            Self::Always => true,
        }
    }
}

impl Default for Lim {
    fn default() -> Self {
        Self::Always
    }
}

/// State of an application the heart runs.
pub trait Stt<T: Now>: Default + Copy + Add<Self, Output = Self> + Mul<f64, Output = Self> {
    /// Initializes the state at the start.
    /// Passed timer can be used to measure initialization time.
    fn init(&mut self, hrt: &mut Hrt<T>, timer: Timer<T>);

    /// Updates the state each tick.
    fn update(&mut self, hrt: &mut Hrt<T>);

    /// Profiles the state each second.
    fn sec(&mut self, hrt: &mut Hrt<T>);
}

/// Renderer of an application the heart runs.
pub trait Render<T: Now, U: Stt<T>>: Default {
    /// Renders the state each frame.
    fn render(&mut self, hrt: &Hrt<T>, stt: &U);
}

/// Heart of a real-time application.
/// Updates and renders in a loop.
///
/// Updates and renders are decoupled:
/// the tick rate can be much lower than the frame rate.
/// Smooth graphics are achived by interpolating the current and previous ticks when rendering.
/// The distance from the previous tick is captured by the member [rem], which is the short for remaining ticks.
///
/// # Example
///
/// ```
/// use min_timer::{Hrt, Now, Render, Std, Stt, Timer};
/// use std::ops::{Add, Mul};
///
/// struct Bar {
///     len: u32,
///     pre: Option<u32>,
/// }
///
/// impl Default for Bar {
///     // Creating the render
///     fn default() -> Self {
///         Self { len: 50, pre: None }
///     }
/// }
///
/// impl Bar {
///     fn print(&mut self, per: f64, len: u32) {
///         self.pre = Some(len);
///         print!("[");
///         for _ in 0..len {
///             print!("=");
///         }
///         if self.len != len {
///             print!(">");
///             for _ in 0..(self.len - len - 1) {
///                 print!(" ");
///             }
///         }
///         println!("] {}%", per);
///     }
/// }
///
/// impl<T: Now> Render<T, Ex> for Bar {
///     // Rendering
///     fn render(&mut self, _: &Hrt<T>, stt: &Ex) {
///         let len = self.len as f64 * stt.0;
///         let len = len.floor() as u32;
///         let len = len.min(self.len);
///         let per = (stt.0 * 100.0).floor();
///         if let Some(pre) = self.pre {
///             if len != pre {
///                 self.print(per, len);
///             }
///         } else {
///             self.print(per, len);
///         }
///     }
/// }
///
/// #[derive(Default, Clone, Copy)]
/// struct Ex(f64);
///
/// impl Mul<f64> for Ex {
///     type Output = Ex;
///
///     // Scaling
///     fn mul(self, rhs: f64) -> Self::Output {
///         Self(self.0 * rhs)
///     }
/// }
///
/// impl Add for Ex {
///     type Output = Ex;
///
///     // Superposing
///     fn add(self, rhs: Ex) -> Self::Output {
///         Self(self.0 + rhs.0)
///     }
/// }
///
/// impl<T: Now> Stt<T> for Ex {
///     // Initialization; timer provided for profiling
///     fn init(&mut self, _: &mut Hrt<T>, timer: Timer<T>) {
///         println!("Initialization done in {}!", timer);
///     }
///
///     // Updating; heart provided for manuplation
///     fn update(&mut self, hrt: &mut Hrt<T>) {
///         self.0 += 1e-1;
///         if self.0 >= 1.0 {
///             hrt.stop();
///         }
///     }
///
///     // Profiling every second; heart provided for manuplation
///     fn sec(&mut self, hrt: &mut Hrt<T>) {
///         println!(
///             "Tick Rate: {} Frame Rate: {}",
///             hrt.ticks().avg_rate(),
///             hrt.frames().avg_rate()
///         );
///     }
/// }
///
/// let now = Std::new(); // using the standard library's clock
/// let mut hrt = Hrt::new(1e2, &now); // target tick rate 100.0
/// hrt.start::<Ex, Bar>(); // creates from defaults
/// ```
pub struct Hrt<'a, T: Now> {
    beat: bool,
    lim: Lim,
    tar: Sec,
    now: &'a T,
    ticks: Stat,
    frames: Stat,
}

impl<'a, T: Now> Hrt<'a, T> {
    /// Creates with the given target tick rate, and closures for updating, drawing, and profiling at every second.
    pub fn new(tar: f64, now: &'a T) -> Self {
        Self {
            beat: false,
            lim: Lim::default(),
            tar: Sec::new(1.0 / tar),
            now,
            ticks: Stat::new(),
            frames: Stat::new(),
        }
    }

    /// Returns update statistics.
    pub fn ticks(&self) -> &Stat {
        &self.ticks
    }

    /// Returns draw statistics.
    pub fn frames(&self) -> &Stat {
        &self.frames
    }

    /// Starts the heart.
    ///
    /// # Panic
    ///
    /// If it is already running.
    pub fn start<U: Stt<T>, V: Render<T, U>>(&mut self) {
        if self.beat {
            panic!("Already running!")
        }
        self.beat = true;

        let init = Timer::new(self.now);
        let ren = V::default();
        let mut cur = U::default();
        cur.init(self, init);

        self.beat(cur, ren);
    }

    fn beat<U: Stt<T>, V: Render<T, U>>(&mut self, mut cur: U, mut ren: V) {
        let mut pre = U::default();
        let mut sec = Timer::new(self.now);
        let mut iter = Timer::new(self.now);

        while self.beat {
            while iter >= self.tar {
                let _ = Prf::new(self.now, &mut self.ticks);
                pre = cur;
                cur.update(self);
                iter -= self.tar;
            }

            if self.lim.draw(self.frames.rate()) {
                let _ = Prf::new(self.now, &mut self.frames);
                let rem = (iter / self.tar.as_f64()).as_f64();
                let drawn = pre * (1.0 - rem) + cur * rem;
                ren.render(self, &drawn);
            }

            if sec >= Sec::ONE {
                sec -= Sec::ONE;
                cur.sec(self);
                self.ticks.refresh();
                self.frames.refresh();
            }
        }
    }

    /// Flags the heart to stop it.
    ///
    /// The heart might update many times and render once and profile once before stopping after this call in update.
    pub fn stop(&mut self) {
        self.beat = false;
    }

    /// Sets the rendering limit.
    ///
    /// Consider disabling or limiting rendering when doing an intense task.
    ///
    /// Consider splitting the task to smaller chunks that will be done on consequent updates.
    /// Otherwise updates will pile up, which will come crashing down after the intense task is done!
    /// This depends on wheter the intense task must be done parallel to the real-time tasks.
    ///
    /// For example, in a game, the loding of a map should be split to multiple updates; but collision calculations should be done on a single update. If they take too long, profile and optimize or reduce the update rate.
    pub fn set_lim(&mut self, lim: Lim) {
        self.lim = lim;
    }
}
