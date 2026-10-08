use std::{any::type_name, borrow::Cow, ops::Index, panic::Location};

use serde_json::Value;

use crate::{
    TrackedOwned,
    steps::{Step, Transform},
    trail::Trail,
};

#[derive(Debug)]
pub struct TrackedRef<'d, T: Clone> {
    pub(super) trail: Trail,
    pub(super) data: &'d T,
}

impl<'d, T: Clone> TrackedRef<'d, T> {
    fn with_trail(&self, trail: Trail, data: &'d T) -> Self {
        TrackedRef { trail, data }
    }

    pub fn inner(&self) -> &'d T {
        self.data
    }

    pub fn trail(&self) -> &Trail {
        &self.trail
    }

    #[track_caller]
    #[inline]
    pub fn get(&self, idx: &str) -> TrackedRef<'d, T>
    where
        for<'a> T: Index<&'a str, Output = T>,
    {
        let data = &self.data[idx];
        self.with_trail(self.trail.pushed(Step::Key(idx.to_string())), data)
    }

    #[track_caller]
    #[inline]
    pub fn get_index(&self, idx: usize) -> TrackedRef<'d, T>
    where
        T: Index<usize, Output = T>,
    {
        let data = &self.data[idx];
        self.with_trail(self.trail.pushed(Step::Index(idx)), data)
    }

    #[inline]
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> TrackedOwned<U> {
        let trail = self.trail.pushed(Step::Map((
            type_name::<T>().to_string(),
            type_name::<U>().to_string(),
        )));
        TrackedOwned {
            trail,
            data: f(self.data),
        }
    }

    #[track_caller]
    #[inline]
    pub fn transform<R>(
        &self,
        name: impl Into<Cow<'static, str>>,
        f: impl FnOnce(&T) -> R,
    ) -> TrackedOwned<R> {
        self.transform_at(name, Location::caller(), f)
    }

    #[inline]
    fn transform_at<R>(
        &self,
        name: impl Into<Cow<'static, str>>,
        at: &'static Location<'static>,
        f: impl FnOnce(&T) -> R,
    ) -> TrackedOwned<R> {
        let step = Step::Transform(Transform::new::<T, R>(name));
        TrackedOwned {
            trail: self.trail.pushed_at(step, at),
            data: f(self.data),
        }
    }

    #[track_caller]
    #[inline]
    pub fn transform_opt<R>(
        &self,
        name: impl Into<Cow<'static, str>>,
        f: impl FnOnce(&T) -> Option<R>,
    ) -> Option<TrackedOwned<R>> {
        let step = Step::Transform(Transform::new::<T, R>(name));
        let trail = self.trail.pushed_at(step, Location::caller());
        let data = f(self.data)?;
        Some(TrackedOwned { trail, data })
    }
}

impl<'d> TrackedRef<'d, Value> {
    #[track_caller]
    #[inline]
    pub fn as_str(&self) -> TrackedOwned<Option<&'d str>> {
        let trail = self.trail.pushed(Step::Str);
        TrackedOwned {
            trail,
            data: self.data.as_str(),
        }
    }

    // as_u64

    #[track_caller]
    pub fn into_string(&self) -> TrackedOwned<Option<String>> {
        let tracked = self.as_str();
        TrackedOwned {
            trail: tracked.trail,
            data: tracked.data.map(|data| data.to_string()),
        }
    }

    pub fn as_array(&self) -> Option<TrackedOwned<&'d [Value]>> {
        todo!()
        // self.data.as_array().map(Vec::as_slice)
    }

    #[track_caller]
    pub fn items(&self) -> Option<Vec<TrackedRef<'d, Value>>> {
        todo!()
        // let at = Location::caller();
        // let arr: &'d [Value] = self.as_array()?;
        // Some(
        //     arr.iter()
        //         .enumerate()
        //         .map(|(idx, v)| self.with_trail(self.trail.pushed_at(Step::Index(idx), at), v))
        //         .collect(),
        // )
    }

    #[track_caller]
    pub fn object_values(&self) -> Option<Vec<TrackedRef<'d, Value>>> {
        let at = Location::caller();
        let obj = self.data.as_object()?;
        Some(
            obj.iter()
                .map(|(k, v)| self.with_trail(self.trail.pushed_at(Step::Key(k.clone()), at), v))
                .collect(),
        )
    }
}
