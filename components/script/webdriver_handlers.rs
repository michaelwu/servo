/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use webdriver_traits::{EvaluateJSReply};
use dom::bindings::conversions::FromJSValConvertible;
use dom::bindings::conversions::StringificationBehavior;
use dom::bindings::js::{OptionalRootable, Rootable};
use dom::window::ScriptHelpers;
use dom::document::DocumentHelpers;
use page::Page;
use msg::constellation_msg::PipelineId;
use script_task::get_page;
use js::jsapi::RootedValue;
use js::jsval::UndefinedValue;

use std::rc::Rc;
use std::sync::mpsc::Sender;

pub fn handle_evaluate_js(page: &Rc<Page>, pipeline: PipelineId, eval: String, reply: Sender<Result<EvaluateJSReply, ()>>){
    let page = get_page(&*page, pipeline);
    let window = page.window().root();
    let cx = window.r().get_cx();
    let mut rval = RootedValue::new(cx, UndefinedValue());
    window.r().evaluate_js_on_global_with_result(&eval, rval.handle_mut());

    reply.send(if rval.ptr.is_undefined() {
        Ok(EvaluateJSReply::VoidValue)
    } else if rval.ptr.is_boolean() {
        Ok(EvaluateJSReply::BooleanValue(rval.ptr.to_boolean()))
    } else if rval.ptr.is_double() {
        Ok(EvaluateJSReply::NumberValue(FromJSValConvertible::from_jsval(cx, rval.handle(), ()).unwrap()))
    } else if rval.ptr.is_string() {
        //FIXME: use jsstring_to_str when jsval grows to_jsstring
        Ok(EvaluateJSReply::StringValue(FromJSValConvertible::from_jsval(cx, rval.handle(), StringificationBehavior::Default).unwrap()))
    } else if rval.ptr.is_null() {
        Ok(EvaluateJSReply::NullValue)
    } else {
        Err(())
    }).unwrap();
}
