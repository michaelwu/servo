/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use canvas_traits::{CanvasMsg, CanvasWebGLMsg};
use dom::bindings::codegen::Bindings::WebGLRenderbufferBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::reflect_dom_object;
use dom::webglobject::WebGLObject;
use ipc_channel::ipc::{self, IpcSender};
use std::cell::Cell;

#[dom_struct]
pub struct WebGLRenderbuffer {
    webgl_object: WebGLObject,
    id: u32,
    is_deleted: Cell<bool>,
}

impl WebGLRenderbuffer {
    fn new_inherited(id: u32) -> WebGLRenderbuffer {
        WebGLRenderbuffer {
            webgl_object: WebGLObject::new_inherited(),
            id: id,
            is_deleted: Cell::new(false),
        }
    }

    pub fn maybe_new(global: GlobalRef, renderer: &IpcSender<CanvasMsg>)
                     -> Option<Root<WebGLRenderbuffer>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateRenderbuffer(sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|renderbuffer_id| WebGLRenderbuffer::new(global, *renderbuffer_id))
    }

    pub fn new(global: GlobalRef, id: u32) -> Root<WebGLRenderbuffer> {
        reflect_dom_object(box WebGLRenderbuffer::new_inherited(id), global, WebGLRenderbufferBinding::Wrap)
    }
}

impl WebGLRenderbuffer {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self, renderer: &IpcSender<CanvasMsg>, target: u32) {
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::BindRenderbuffer(target, self.id))).unwrap();
    }

    pub fn delete(&self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteRenderbuffer(self.id))).unwrap();
        }
    }
}
