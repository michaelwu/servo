/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::codegen::Bindings::WebGLTextureBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::reflect_dom_object;
use dom::webglobject::WebGLObject;

use canvas_traits::{CanvasMsg, CanvasWebGLMsg};
use ipc_channel::ipc::{self, IpcSender};
use std::cell::Cell;

#[dom_struct]
pub struct WebGLTexture {
    webgl_object: WebGLObject,
    id: u32,
    is_deleted: Cell<bool>,
}

impl WebGLTexture {
    fn new_inherited(id: u32) -> WebGLTexture {
        WebGLTexture {
            webgl_object: WebGLObject::new_inherited(),
            id: id,
            is_deleted: Cell::new(false),
        }
    }

    pub fn maybe_new(global: GlobalRef, renderer: &IpcSender<CanvasMsg>)
                     -> Option<Root<WebGLTexture>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateTexture(sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|texture_id| WebGLTexture::new(global, *texture_id))
    }

    pub fn new(global: GlobalRef, id: u32) -> Root<WebGLTexture> {
        reflect_dom_object(box WebGLTexture::new_inherited(id), global, WebGLTextureBinding::Wrap)
    }
}

pub trait WebGLTextureHelpers {
    fn id(self) -> u32;
    fn bind(self, renderer: &IpcSender<CanvasMsg>, target: u32);
    fn delete(self, renderer: &IpcSender<CanvasMsg>);
}

impl<'a> WebGLTextureHelpers for &'a WebGLTexture {
    fn id(self) -> u32 {
        self.id
    }

    fn bind(self, renderer: &IpcSender<CanvasMsg>, target: u32) {
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::BindTexture(self.id, target))).unwrap();
    }

    fn delete(self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteTexture(self.id))).unwrap();
        }
    }
}
