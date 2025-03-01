//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use core::ops::{Deref, DerefMut};

use crate::boxed::BoxedSlice;

#[crate::stabby]
pub struct Vec<T> {
    pub slice: BoxedSlice<T>,
    pub capacity: usize,
}
impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.slice.deref()
    }
}
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice.deref_mut()
    }
}
impl<T> From<alloc::vec::Vec<T>> for Vec<T> {
    fn from(value: alloc::vec::Vec<T>) -> Self {
        let capacity = value.capacity();
        let slice = value.into_boxed_slice().into();
        Self { slice, capacity }
    }
}
impl<T> From<Vec<T>> for alloc::vec::Vec<T> {
    fn from(value: Vec<T>) -> Self {
        let slice = BoxedSlice::leak(value.slice);
        unsafe { alloc::vec::Vec::from_raw_parts(slice.start, slice.len, value.capacity) }
    }
}
