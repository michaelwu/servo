/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::PerformanceBinding;
use dom::bindings::codegen::Bindings::PerformanceBinding::PerformanceMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::num::Finite;
use dom::bindings::magic::alloc_dom_object;
use dom::performancetiming::PerformanceTiming;
use dom::window::Window;
use time;

pub type DOMHighResTimeStamp = Finite<f64>;

magic_dom_struct! {
    pub struct Performance {
        timing: JS<PerformanceTiming>,
    }
}

impl Performance {
    fn new_inherited(&mut self, window: &Window,
                     navigation_start: u64,
                     navigation_start_precise: f64) {
        self.timing.init(JS::from_rooted(&PerformanceTiming::new(window, navigation_start, navigation_start_precise)));
    }

    pub fn new(window: &Window,
               navigation_start: u64,
               navigation_start_precise: f64) -> Root<Performance> {
        let mut obj = alloc_dom_object::<Performance>(GlobalRef::Window(window));
        obj.new_inherited(window,
                                                          navigation_start,
                                                          navigation_start_precise);
        obj.into_root()
    }
}

impl PerformanceMethods for Performance {
    // https://dvcs.w3.org/hg/webperf/raw-file/tip/specs/NavigationTiming/Overview.html#performance-timing-attribute
    fn Timing(&self) -> Root<PerformanceTiming> {
        self.timing.root()
    }

    // https://dvcs.w3.org/hg/webperf/raw-file/tip/specs/HighResolutionTime/Overview.html#dom-performance-now
    fn Now(&self) -> DOMHighResTimeStamp {
        let navStart = self.timing.root().r().NavigationStartPrecise();
        let now = (time::precise_time_ns() as f64 - navStart) / 1000000 as f64;
        Finite::wrap(now)
    }
}
