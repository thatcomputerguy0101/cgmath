// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use self::channel::Channel;
pub use self::hsv::{HSV, ToHSV, HSVA, ToHSVA};
pub use self::rgb::{RGB, ToRGB, RGBA, ToRGBA};
pub use self::srgb::{SRGB, SRGBA};

pub mod channel;
pub mod hsv;
pub mod rgb;
pub mod srgb;

pub trait Color<T> {
    pub fn inverse(&self) -> Self;
    pub fn invert_self(&mut self);
}