/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use canvas_traits::{FillOrStrokeStyle, RepetitionStyle, SurfaceStyle};
use dom::bindings::codegen::Bindings::CanvasPatternBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::canvasgradient::ToFillOrStrokeStyle;
use euclid::size::Size2D;

// https://html.spec.whatwg.org/multipage/#canvaspattern
magic_dom_struct! {
    pub struct CanvasPattern {
        extra: Box<CanvasPatternExtra>,
    }
}

#[derive(JSTraceable, HeapSizeOf)]
pub struct CanvasPatternExtra {
    surface_data: Vec<u8>,
    surface_size: Size2D<i32>,
    repeat_x: bool,
    repeat_y: bool,
}

impl CanvasPattern {
    fn new_inherited(&mut self, surface_data: Vec<u8>, surface_size: Size2D<i32>, repeat: RepetitionStyle) {
        let (x, y) = match repeat {
            RepetitionStyle::Repeat => (true, true),
            RepetitionStyle::RepeatX => (true, false),
            RepetitionStyle::RepeatY => (false, true),
            RepetitionStyle::NoRepeat => (false, false),
        };

        self.extra.init(box CanvasPatternExtra {
            surface_data: surface_data,
            surface_size: surface_size,
            repeat_x: x,
            repeat_y: y,
        });
    }
    pub fn new(global: GlobalRef,
               surface_data: Vec<u8>,
               surface_size: Size2D<i32>,
               repeat: RepetitionStyle)
               -> Root<CanvasPattern> {
        let mut obj = alloc_dom_object::<CanvasPattern>(global);
        obj.new_inherited(surface_data, surface_size, repeat);
        obj.into_root()
    }
}

impl<'a> ToFillOrStrokeStyle for &'a CanvasPattern {
    fn to_fill_or_stroke_style(self) -> FillOrStrokeStyle {
        FillOrStrokeStyle::Surface(
            SurfaceStyle::new(self.extra.surface_data.clone(), self.extra.surface_size, self.extra.repeat_x, self.extra.repeat_y))
    }
}
