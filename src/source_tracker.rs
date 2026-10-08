use crate::{BUILD, BuildInfo, TrackedOwned, steps::Step, trail::Trail};

#[derive(Debug)]
pub struct SourceTracker {
    build: BuildInfo,
    trail: Trail,
}

impl Default for SourceTracker {
    #[track_caller]
    fn default() -> Self {
        Self {
            build: BUILD,
            trail: Default::default(),
        }
    }
}

impl SourceTracker {
    pub fn build(&self) -> BuildInfo {
        self.build
    }

    pub fn trail(&self) -> &Trail {
        &self.trail
    }

    #[track_caller]
    pub fn add_source(&mut self, step: Step) {
        self.trail = self.trail.pushed(step)
    }

    #[track_caller]
    pub fn var<T>(&mut self, data: T) -> TrackedOwned<T> {
        TrackedOwned {
            trail: self.trail.clone(),
            data,
        }
    }
}
