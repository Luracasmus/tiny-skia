// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

// Right now, there are no visible benefits of using SIMD for f32x2. So we don't.
/// A pair of f32 numbers.
///
/// Mainly for internal use. Do not rely on it!
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct f32x2(pub [f32; 2]);

impl f32x2 {
    /// Creates a new pair.
    pub const fn new(a: f32, b: f32) -> Self {
        Self([a, b])
    }

    /// Creates a new pair from a single value.
    pub const fn splat(x: f32) -> Self {
        Self([x, x])
    }

    /// Returns an absolute value.
    pub fn abs(self) -> Self {
        Self([self.x().abs(), self.y().abs()])
    }

    /// Returns a minimum value.
    pub fn min(self, other: Self) -> Self {
        Self([pmin(self.x(), other.x()), pmin(self.y(), other.y())])
    }

    /// Returns a maximum value.
    pub fn max(self, other: Self) -> Self {
        Self([pmax(self.x(), other.x()), pmax(self.y(), other.y())])
    }

    /// Returns a maximum of both values.
    pub fn max_component(self) -> f32 {
        pmax(self.x(), self.y())
    }

    /// Returns the first value.
    pub const fn x(&self) -> f32 {
        self.0[0]
    }

    /// Returns the second value.
    pub const fn y(&self) -> f32 {
        self.0[1]
    }
}

impl core::ops::Add<Self> for f32x2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self([self.x() + other.x(), self.y() + other.y()])
    }
}

impl core::ops::Sub<Self> for f32x2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self([self.x() - other.x(), self.y() - other.y()])
    }
}

impl core::ops::Mul<Self> for f32x2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self([self.x() * other.x(), self.y() * other.y()])
    }
}

impl core::ops::Div<Self> for f32x2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self([self.x() / other.x(), self.y() / other.y()])
    }
}

// A faster and more forgiving f32 min/max implementation.
//
// Unlike std one, we do not care about NaN.

fn pmax(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

fn pmin(a: f32, b: f32) -> f32 {
    if b < a {
        b
    } else {
        a
    }
}
