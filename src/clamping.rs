use crate::{timing, Baring};
use log::warn;
use std::{
    cmp::min,
    fmt::{self, Display},
};

#[derive(Debug)]
struct PrintController {
    last_printed_progress: Option<f64>,
    interesting_progress_step: f64,
}

impl PrintController {
    fn from(interesting_progress_step: f64) -> PrintController {
        PrintController {
            last_printed_progress: None,
            interesting_progress_step,
        }
    }

    fn map(&self, progress: f64) -> usize {
        let scale = 1_000.0;
        (progress * scale) as usize / ((scale * self.interesting_progress_step) as usize)
    }

    fn has_progressed_significantly(&self, progress: f64) -> bool {
        self.map(progress) > self.map(self.last_printed_progress.unwrap_or(0.0))
    }

    fn update(&mut self, progress: f64) {
        self.last_printed_progress = Some(progress);
    }
}

pub struct Config {
    pub bar_len: usize,
    pub style: String,
    pub interesting_progress_step: f64,
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            bar_len: 42,
            style: String::from("[=>.]"),
            interesting_progress_step: 0.1,
        }
    }
}

/// A progress-bar clamping values to `[0, 1]`.
///
///
/// # Mini-Example
///
/// ```
/// use progressing::{clamping::Bar as ClampingBar, Baring};
///
/// /// Printing value 0.3 clamped to [0, 1]
/// /// [=====>------------]
/// fn main() {
///     println!("Printing value 0.3 clamped to [0, 1]");
///     let mut progress_bar = ClampingBar::new();
///     progress_bar.set_len(20);
///     progress_bar.set(0.3);
///     println!("{}", progress_bar);
/// }
/// ```
///
/// It is only optimized for single-length-strings, but strings are more handy than chars and hence used as implementation.
#[derive(Debug)]
pub struct Bar {
    bar_len: usize,
    style: String,
    progress: f64,
    print_controller: PrintController,
}

impl Bar {
    pub fn new() -> Bar {
        Bar::default()
    }

    pub fn with(cfg: Config) -> Bar {
        Bar {
            bar_len: cfg.bar_len,
            style: cfg.style,
            progress: 0.0,
            print_controller: PrintController::from(cfg.interesting_progress_step),
        }
    }

    pub fn timed(self) -> timing::Bar<Bar> {
        timing::Bar::with(self)
    }

    pub fn set_style<S>(&mut self, style: S)
    where
        S: Into<String>,
    {
        let style = style.into();

        if style.len() != 5 {
            warn!("The bar-style has to consist of 5 characters, e.g. [=>-]");
        };
        self.style = style;
    }

    fn inner_bar_len(&self) -> usize {
        self.len() - self.brackets_len()
    }

    fn brackets_len(&self) -> usize {
        2
    }

    fn left_bracket(&self) -> char {
        self.style.chars().nth(0).unwrap()
    }

    fn line(&self) -> char {
        self.style.chars().nth(1).unwrap()
    }

    fn hat(&self) -> char {
        self.style.chars().nth(2).unwrap()
    }

    fn empty_line(&self) -> char {
        self.style.chars().nth(3).unwrap()
    }

    fn right_bracket(&self) -> char {
        self.style.chars().nth(4).unwrap()
    }
}

impl Default for Bar {
    fn default() -> Bar {
        Bar::with(Config::default())
    }
}

impl Baring for Bar {
    type Progress = f64;

    fn len(&self) -> usize {
        self.bar_len
    }

    /// panics if length is `< 3`
    fn set_len(&mut self, new_bar_len: usize) {
        self.bar_len = new_bar_len;
    }

    fn progress(&self) -> f64 {
        self.progress
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<f64>,
    {
        let mut new_progress = new_progress.into();

        if new_progress < 0.0 {
            new_progress = 0.0;
        }
        if 1.0 < new_progress {
            new_progress = 1.0;
        }
        self.progress = new_progress;
    }

    fn start(&self) -> f64 {
        0.0
    }

    fn end(&self) -> f64 {
        1.0
    }

    fn has_progressed_significantly(&self) -> bool {
        self.print_controller
            .has_progressed_significantly(self.progress())
    }

    fn remember_significant_progress(&mut self) {
        self.print_controller.update(self.progress());
    }
}

impl Display for Bar {
    /// Progress is clamped to `[0, 1]`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // calc progress
        // -> bar needs to be calculated
        // -> no brackets involved
        let reached: usize = (self.progress * self.inner_bar_len() as f64) as usize;

        let line = String::from(self.line()).repeat(reached);

        // crop hat if end of bar is reached
        let hat = &String::from(self.hat())[0..min(1, self.inner_bar_len() - reached)];

        // fill up rest with empty line
        let empty_line =
            String::from(self.empty_line()).repeat(self.inner_bar_len() - reached - hat.len());
        write!(
            f,
            "{}{}{}{}{}",
            self.left_bracket(),
            line,
            hat,
            empty_line,
            self.right_bracket()
        )
    }
}
