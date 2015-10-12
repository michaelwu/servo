/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use canvas_traits::{CanvasMsg, CanvasWebGLMsg};
use dom::bindings::codegen::Bindings::WebGLRenderbufferBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::webglobject::WebGLObject;
use ipc_channel::ipc::{self, IpcSender};
use std::cell::Cell;

magic_dom_struct! {
    pub struct WebGLRenderbuffer {
        webgl_object: Base<WebGLObject>,
        id: u32,
        is_deleted: Mut<bool>,
    }
}

impl WebGLRenderbuffer {
    fn new_inherited(&mut self, id: u32) {
        self.webgl_object.new_inherited();
        self.id.init(id);
        self.is_deleted.init(false);
    }

    pub fn maybe_new(global: GlobalRef, renderer: &IpcSender<CanvasMsg>)
                     -> Option<Root<WebGLRenderbuffer>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateRenderbuffer(sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|renderbuffer_id| WebGLRenderbuffer::new(global, *renderbuffer_id))
    }

    pub fn new(global: GlobalRef, id: u32) -> Root<WebGLRenderbuffer> {
        let mut obj = alloc_dom_object::<WebGLRenderbuffer>(global);
        obj.new_inherited(id);
        obj.into_root()
    }
}

impl WebGLRenderbuffer {
    pub fn id(&self) -> u32 {
        self.id.get()
    }

    pub fn bind(&self, renderer: &IpcSender<CanvasMsg>, target: u32) {
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::BindRenderbuffer(target, self.id.get()))).unwrap();
    }

    pub fn delete(&self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteRenderbuffer(self.id.get()))).unwrap();
        }
    }
}
