/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::PerformanceTimingBinding;
use dom::bindings::codegen::Bindings::PerformanceTimingBinding::PerformanceTimingMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::window::Window;

magic_dom_struct! {
    pub struct PerformanceTiming {
        navigationStart: u64,
        navigationStartPrecise: f64,
    }
}

impl PerformanceTiming {
    fn new_inherited(&mut self, navStart: u64, navStartPrecise: f64)
                         {
        self.navigationStart.init(navStart);
        self.navigationStartPrecise.init(navStartPrecise);
    }

    #[allow(unrooted_must_root)]
    pub fn new(window: &Window,
               navigation_start: u64,
               navigation_start_precise: f64)
               -> Root<PerformanceTiming> {
        let mut obj = alloc_dom_object::<PerformanceTiming>(GlobalRef::Window(window));
        obj.new_inherited(navigation_start,navigation_start_precise);
        obj.into_root()
    }
}

impl PerformanceTimingMethods for PerformanceTiming {
    // https://dvcs.w3.org/hg/webperf/raw-file/tip/specs/
    // NavigationTiming/Overview.html#dom-performancetiming-navigationstart
    fn NavigationStart(&self) -> u64 {
        self.navigationStart.get()
    }
}


impl PerformanceTiming {
    pub fn NavigationStartPrecise(&self) -> f64 {
        self.navigationStartPrecise.get()
    }
}
