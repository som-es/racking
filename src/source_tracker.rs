use crate::{BUILD, BuildInfo, TrackedRef, steps::Step, trail::Trail};

#[derive(Debug, Clone)]
pub struct SourceTracker<T: Clone> {
    build: BuildInfo,
    trail: Trail,
    data: Option<T>,
}

impl<T: Clone> Default for SourceTracker<T> {
    #[track_caller]
    fn default() -> Self {
        Self {
            build: BUILD,
            trail: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T: Clone> SourceTracker<T> {
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
    pub fn var<'d>(&'d mut self, data: T) -> TrackedRef<'d, T> {
        self.data = Some(data);
        TrackedRef {
            trail: self.trail.clone(),
            data: self.data.as_ref().unwrap(),
        }
    }
}
