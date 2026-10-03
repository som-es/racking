use crate::{TrackedRef, trail::Trail};

#[derive(Debug)]
pub struct TrackedOwned<T> {
    pub(super) trail: Trail,
    pub(super) data: T,
}

impl<T> TrackedOwned<T> {
    pub fn trail(&self) -> &Trail {
        &self.trail
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T> TrackedOwned<Option<T>> {
    #[inline]
    pub fn transpose(self) -> Option<TrackedOwned<T>> {
        let TrackedOwned { trail, data } = self;
        data.map(|data| TrackedOwned { trail, data })
    }
}

impl<T: Clone> TrackedOwned<T> {
    #[inline]
    pub fn as_tracked(&self) -> TrackedRef<'_, T> {
        TrackedRef {
            trail: self.trail.clone(),
            data: &self.data,
        }
    }
}
