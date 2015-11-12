/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use canvas_traits::{CanvasGradientStop, FillOrStrokeStyle, LinearGradientStyle, RadialGradientStyle};
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::CanvasGradientBinding;
use dom::bindings::codegen::Bindings::CanvasGradientBinding::CanvasGradientMethods;
use dom::bindings::error::{Error, ErrorResult};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::num::Finite;
use dom::bindings::magic::alloc_dom_object;
use dom::canvasrenderingcontext2d::parse_color;

// https://html.spec.whatwg.org/multipage/#canvasgradient
magic_dom_struct! {
    pub struct CanvasGradient {
        extra: Box<CanvasGradientExtra>,
    }
}

#[derive(JSTraceable, HeapSizeOf)]
pub struct CanvasGradientExtra {
    style: CanvasGradientStyle,
    stops: DOMRefCell<Vec<CanvasGradientStop>>,
}

#[derive(JSTraceable, Clone, HeapSizeOf)]
pub enum CanvasGradientStyle {
    Linear(LinearGradientStyle),
    Radial(RadialGradientStyle),
}

impl CanvasGradient {
    fn new_inherited(&mut self, style: CanvasGradientStyle) {
        self.extra.init(box CanvasGradientExtra {
            style: style,
            stops: DOMRefCell::new(Vec::new()),
        });
    }

    pub fn new(global: GlobalRef, style: CanvasGradientStyle) -> Root<CanvasGradient> {
        let mut obj = alloc_dom_object::<CanvasGradient>(global);
        obj.new_inherited(style);
        obj.into_root()
    }
}

impl CanvasGradientMethods for CanvasGradient {
    // https://html.spec.whatwg.org/multipage/#dom-canvasgradient-addcolorstop
    fn AddColorStop(&self, offset: Finite<f64>, color: String) -> ErrorResult {
        if *offset < 0f64 || *offset > 1f64 {
            return Err(Error::IndexSize);
        }

        let color = match parse_color(&color) {
            Ok(color) => color,
            _ => return Err(Error::Syntax)
        };

        self.extra.stops.borrow_mut().push(CanvasGradientStop {
            offset: (*offset) as f64,
            color: color,
        });
        Ok(())
    }
}

pub trait ToFillOrStrokeStyle {
    fn to_fill_or_stroke_style(self) -> FillOrStrokeStyle;
}

impl<'a> ToFillOrStrokeStyle for &'a CanvasGradient {
    fn to_fill_or_stroke_style(self) -> FillOrStrokeStyle {
        let gradient_stops = self.extra.stops.borrow().clone();
        match self.extra.style {
            CanvasGradientStyle::Linear(ref gradient) =>  {
                FillOrStrokeStyle::LinearGradient(
                    LinearGradientStyle::new(gradient.x0, gradient.y0,
                                             gradient.x1, gradient.y1,
                                             gradient_stops))
            },
            CanvasGradientStyle::Radial(ref gradient) => {
                FillOrStrokeStyle::RadialGradient(
                    RadialGradientStyle::new(gradient.x0, gradient.y0, gradient.r0,
                                             gradient.x1, gradient.y1, gradient.r1,
                                             gradient_stops))
            }
        }
    }
}
