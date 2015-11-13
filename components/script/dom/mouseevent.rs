/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MouseEventBinding;
use dom::bindings::codegen::Bindings::MouseEventBinding::MouseEventMethods;
use dom::bindings::codegen::Bindings::UIEventBinding::UIEventMethods;
use dom::bindings::conversions::Castable;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root, RootedReference};
use dom::bindings::magic::alloc_dom_object;
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::eventtarget::EventTarget;
use dom::uievent::UIEvent;
use dom::window::Window;
use std::cell::Cell;
use std::default::Default;
use util::prefs;
use util::str::DOMString;

magic_dom_struct! {
    pub struct MouseEvent {
        uievent: Base<UIEvent>,
        screen_x: Mut<i32>,
        screen_y: Mut<i32>,
        client_x: Mut<i32>,
        client_y: Mut<i32>,
        ctrl_key: Mut<bool>,
        shift_key: Mut<bool>,
        alt_key: Mut<bool>,
        meta_key: Mut<bool>,
        button: Mut<i16>,
        related_target: Mut<Option<JS<EventTarget>>>,
    }
}

impl MouseEvent {
    fn new_inherited(&mut self) {
        self.uievent.new_inherited();
        self.screen_x.init(0);
        self.screen_y.init(0);
        self.client_x.init(0);
        self.client_y.init(0);
        self.ctrl_key.init(false);
        self.shift_key.init(false);
        self.alt_key.init(false);
        self.meta_key.init(false);
        self.button.init(0);
        self.related_target.init(Default::default());
    }

    pub fn new_uninitialized(window: &Window) -> Root<MouseEvent> {
        let mut obj = alloc_dom_object::<MouseEvent>(GlobalRef::Window(window));
        obj.new_inherited();
        obj.into_root()
    }

    pub fn new(window: &Window,
               type_: DOMString,
               canBubble: EventBubbles,
               cancelable: EventCancelable,
               view: Option<&Window>,
               detail: i32,
               screenX: i32,
               screenY: i32,
               clientX: i32,
               clientY: i32,
               ctrlKey: bool,
               altKey: bool,
               shiftKey: bool,
               metaKey: bool,
               button: i16,
               relatedTarget: Option<&EventTarget>) -> Root<MouseEvent> {
        let ev = MouseEvent::new_uninitialized(window);
        ev.r().InitMouseEvent(type_, canBubble == EventBubbles::Bubbles, cancelable == EventCancelable::Cancelable,
                              view, detail,
                              screenX, screenY, clientX, clientY,
                              ctrlKey, altKey, shiftKey, metaKey,
                              button, relatedTarget);
        ev
    }

    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &MouseEventBinding::MouseEventInit) -> Fallible<Root<MouseEvent>> {
        let bubbles = if init.parent.parent.parent.bubbles {
            EventBubbles::Bubbles
        } else {
            EventBubbles::DoesNotBubble
        };
        let cancelable = if init.parent.parent.parent.cancelable {
            EventCancelable::Cancelable
        } else {
        EventCancelable::NotCancelable
        };
        let event = MouseEvent::new(global.as_window(), type_,
                                    bubbles,
                                    cancelable,
                                    init.parent.parent.view.r(),
                                    init.parent.parent.detail,
                                    init.screenX, init.screenY,
                                    init.clientX, init.clientY, init.parent.ctrlKey,
                                    init.parent.altKey, init.parent.shiftKey, init.parent.metaKey,
                                    init.button, init.relatedTarget.r());
        Ok(event)
    }
}

impl MouseEventMethods for MouseEvent {
    // https://w3c.github.io/uievents/#widl-MouseEvent-screenX
    fn ScreenX(&self) -> i32 {
        self.screen_x.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-screenY
    fn ScreenY(&self) -> i32 {
        self.screen_y.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-clientX
    fn ClientX(&self) -> i32 {
        self.client_x.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-clientY
    fn ClientY(&self) -> i32 {
        self.client_y.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-ctrlKey
    fn CtrlKey(&self) -> bool {
        self.ctrl_key.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-shiftKey
    fn ShiftKey(&self) -> bool {
        self.shift_key.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-altKey
    fn AltKey(&self) -> bool {
        self.alt_key.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-metaKey
    fn MetaKey(&self) -> bool {
        self.meta_key.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-button
    fn Button(&self) -> i16 {
        self.button.get()
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-relatedTarget
    fn GetRelatedTarget(&self) -> Option<Root<EventTarget>> {
        self.related_target.get().map(Root::from_rooted)
    }

    // See discussion at:
    //  - https://github.com/servo/servo/issues/6643
    //  - https://bugzilla.mozilla.org/show_bug.cgi?id=1186125
    // This returns the same result as current gecko.
    // https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/which
    fn Which(&self) -> i32 {
        if prefs::get_pref("dom.mouseevent.which.enabled").as_boolean().unwrap_or(false) {
            (self.button.get() + 1) as i32
        } else {
            0
        }
    }

    // https://w3c.github.io/uievents/#widl-MouseEvent-initMouseEvent
    fn InitMouseEvent(&self,
                      typeArg: DOMString,
                      canBubbleArg: bool,
                      cancelableArg: bool,
                      viewArg: Option<&Window>,
                      detailArg: i32,
                      screenXArg: i32,
                      screenYArg: i32,
                      clientXArg: i32,
                      clientYArg: i32,
                      ctrlKeyArg: bool,
                      altKeyArg: bool,
                      shiftKeyArg: bool,
                      metaKeyArg: bool,
                      buttonArg: i16,
                      relatedTargetArg: Option<&EventTarget>) {
        if self.upcast::<Event>().dispatching() {
            return;
        }

        self.upcast::<UIEvent>()
            .InitUIEvent(typeArg, canBubbleArg, cancelableArg, viewArg, detailArg);
        self.screen_x.set(screenXArg);
        self.screen_y.set(screenYArg);
        self.client_x.set(clientXArg);
        self.client_y.set(clientYArg);
        self.ctrl_key.set(ctrlKeyArg);
        self.alt_key.set(altKeyArg);
        self.shift_key.set(shiftKeyArg);
        self.meta_key.set(metaKeyArg);
        self.button.set(buttonArg);
        self.related_target.set(relatedTargetArg.map(JS::from_ref));
    }
}
