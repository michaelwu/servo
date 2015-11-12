/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use canvas_traits::{CanvasMsg, CanvasWebGLMsg, WebGLError, WebGLResult};
use dom::bindings::codegen::Bindings::WebGLBufferBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::webglobject::WebGLObject;
use ipc_channel::ipc::{self, IpcSender};
use std::cell::Cell;

magic_dom_struct! {
    pub struct WebGLBuffer {
        webgl_object: Base<WebGLObject>,
        id: u32,
        /// The target to which this buffer was bound the first time
        target: Mut<Option<u32>>,
        is_deleted: Mut<bool>,
    }
}

impl WebGLBuffer {
    fn new_inherited(&mut self, id: u32) {
        self.webgl_object.new_inherited();
        self.id.init(id);
        self.target.init(None);
        self.is_deleted.init(false);
    }

    pub fn maybe_new(global: GlobalRef, renderer: &IpcSender<CanvasMsg>)
                     -> Option<Root<WebGLBuffer>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateBuffer(sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|buffer_id| WebGLBuffer::new(global, *buffer_id))
    }

    pub fn new(global: GlobalRef, id: u32) -> Root<WebGLBuffer> {
        let mut obj = alloc_dom_object::<WebGLBuffer>(global);
        obj.new_inherited(id);
        obj.into_root()
    }
}

impl WebGLBuffer {
    pub fn id(&self) -> u32 {
        self.id
    }

    // NB: Only valid buffer targets come here
    pub fn bind(&self, renderer: &IpcSender<CanvasMsg>, target: u32) -> WebGLResult<()> {
        if let Some(previous_target) = self.target.get() {
            if target != previous_target {
                return Err(WebGLError::InvalidOperation);
            }
        } else {
            self.target.set(Some(target));
        }
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::BindBuffer(target, self.id))).unwrap();

        Ok(())
    }

    pub fn delete(&self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteBuffer(self.id))).unwrap();
        }
    }
}
