use crate::{bars::mapping, Bar, MappingBar, Progress};
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign},
};

/// A progress-bar counting successes (e.g. `42 out of 60`) and respective attempts (e.g. `130`).
///
/// # Mini-Example
///
/// ```
/// use progressing::Bar;
///
/// /// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// /// [============>-----] (42 / 60 # 130)
/// fn main() {
///     println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
///     let mut progress_bar = progressing::BernoulliBar::from_goal(60);
///     progress_bar.set_len(20);
///     progress_bar.set((42, 130));
///     println!("{}", progress_bar);
/// }
/// ```
#[derive(Debug)]
pub struct BernoulliBar {
    bar: MappingBar<usize>,
    attempts: usize,
}

impl BernoulliBar {
    pub fn start(&self) -> usize {
        self.bar.start()
    }

    pub fn end(&self) -> usize {
        self.bar.end()
    }
}

impl BernoulliBar {
    pub fn from_goal(n: usize) -> BernoulliBar {
        BernoulliBar {
            bar: MappingBar::new(0..=n),
            attempts: 0,
        }
    }
}

impl Bar for BernoulliBar {
    type Progress = BernoulliProgress;

    fn len(&self) -> usize {
        self.bar.len()
    }

    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len)
    }

    fn progress(&self) -> BernoulliProgress {
        BernoulliProgress {
            successes: self.bar.progress(),
            attempts: self.attempts,
        }
    }

    fn set<P>(&mut self, outcome: P)
    where
        P: Into<BernoulliProgress>,
    {
        let outcome = outcome.into();
        self.bar.set(outcome.successes);
        self.attempts = outcome.attempts;
    }
}

impl Display for BernoulliBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} / {} # {})",
            mapping::inner_bar(&self.bar),
            mapping::inner_k(&self.bar),
            self.bar.end(),
            self.attempts
        )
    }
}

/// Just a simple struct capsuling access to successes and attempts.
#[derive(Copy, Clone)]
pub struct BernoulliProgress {
    pub successes: usize,
    pub attempts: usize,
}

impl From<(usize, usize)> for BernoulliProgress {
    fn from((successes, attempts): (usize, usize)) -> Self {
        BernoulliProgress {
            successes,
            attempts,
        }
    }
}

impl From<usize> for BernoulliProgress {
    fn from(successes: usize) -> Self {
        BernoulliProgress {
            successes,
            attempts: successes,
        }
    }
}

impl From<bool> for BernoulliProgress {
    fn from(is_successful: bool) -> Self {
        BernoulliProgress {
            successes: if is_successful { 1 } else { 0 },
            attempts: 1,
        }
    }
}

impl Add for BernoulliProgress {
    type Output = BernoulliProgress;

    fn add(self, other: BernoulliProgress) -> BernoulliProgress {
        BernoulliProgress {
            successes: self.successes + other.successes,
            attempts: self.attempts + other.attempts,
        }
    }
}

impl AddAssign for BernoulliProgress {
    fn add_assign(&mut self, other: BernoulliProgress) {
        *self = *self + other;
    }
}

impl Progress for BernoulliProgress {
    fn add(self, summand: BernoulliProgress) -> BernoulliProgress {
        self + summand
    }

    fn sub(self, subtrahend: BernoulliProgress) -> BernoulliProgress {
        BernoulliProgress {
            successes: self.successes - subtrahend.successes,
            attempts: self.attempts - subtrahend.attempts,
        }
    }

    fn div(self, divisor: BernoulliProgress) -> f64 {
        (self.successes as f64) / (divisor.successes as f64)
    }
}
