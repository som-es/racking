use core::fmt;
use std::{panic::Location, sync::Arc};

use crate::steps::Step;

#[derive(Debug, Clone)]
pub struct Trail {
    parent: Option<Arc<Trail>>,
    step: Step,
    at: &'static Location<'static>,
}

impl Default for Trail {
    #[track_caller]
    fn default() -> Self {
        Self {
            parent: Default::default(),
            step: Default::default(),
            at: Location::caller(),
        }
    }
}

impl Trail {
    #[track_caller]
    pub fn pushed(&self, step: Step) -> Self {
        self.pushed_at(step, Location::caller())
    }

    pub fn pushed_at(&self, step: Step, at: &'static Location<'static>) -> Self {
        Trail {
            parent: Some(Arc::new(self.clone())),
            step,
            at,
        }
    }

    pub fn steps(&self) -> impl Iterator<Item = &Trail> {
        let mut cursor = Some(self);
        std::iter::from_fn(move || {
            let trail = cursor?;
            cursor = trail.parent.as_deref();
            Some(trail)
        })
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn at(&self) -> &'static Location<'static> {
        self.at
    }
}

impl fmt::Display for Trail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let steps: Vec<&Trail> = self.steps().collect();
        for (depth, trail) in steps.iter().rev().enumerate() {
            if depth > 0 {
                writeln!(f)?;
            }
            write!(f, "{depth:>3} | {} @ {}", trail.step, trail.at)?;
        }
        Ok(())
    }
}
