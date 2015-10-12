/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use canvas_traits::{CanvasMsg, CanvasWebGLMsg, WebGLFramebufferBindingRequest};
use dom::bindings::codegen::Bindings::WebGLFramebufferBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::webglobject::WebGLObject;
use ipc_channel::ipc::{self, IpcSender};
use std::cell::Cell;

magic_dom_struct! {
    pub struct WebGLFramebuffer {
        webgl_object: Base<WebGLObject>,
        id: u32,
        is_deleted: Mut<bool>,
    }
}

impl WebGLFramebuffer {
    fn new_inherited(&mut self, id: u32) {
        self.webgl_object.new_inherited();
        self.id.init(id);
        self.is_deleted.init(false);
    }

    pub fn maybe_new(global: GlobalRef, renderer: &IpcSender<CanvasMsg>)
                     -> Option<Root<WebGLFramebuffer>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateFramebuffer(sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|fb_id| WebGLFramebuffer::new(global, *fb_id))
    }

    pub fn new(global: GlobalRef, id: u32) -> Root<WebGLFramebuffer> {
        let mut obj = alloc_dom_object::<WebGLFramebuffer>(global);
        obj.new_inherited(id);
        obj.into_root()
    }
}

impl WebGLFramebuffer {
    pub fn id(&self) -> u32 {
        self.id.get()
    }

    pub fn bind(&self, renderer: &IpcSender<CanvasMsg>, target: u32) {
        let cmd = CanvasWebGLMsg::BindFramebuffer(target, WebGLFramebufferBindingRequest::Explicit(self.id.get()));
        renderer.send(CanvasMsg::WebGL(cmd)).unwrap();
    }

    pub fn delete(&self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteFramebuffer(self.id.get()))).unwrap();
        }
    }
}
