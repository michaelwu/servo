/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use canvas_traits::{CanvasMsg, CanvasWebGLMsg, WebGLError, WebGLResult};
use dom::bindings::codegen::Bindings::WebGLProgramBinding;
use dom::bindings::codegen::Bindings::WebGLRenderingContextBinding::WebGLRenderingContextConstants as constants;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::magic::alloc_dom_object;
use dom::webglobject::WebGLObject;
use dom::webglrenderingcontext::MAX_UNIFORM_AND_ATTRIBUTE_LEN;
use dom::webglshader::WebGLShader;
use ipc_channel::ipc::{self, IpcSender};
use std::cell::Cell;

magic_dom_struct! {
    pub struct WebGLProgram {
        webgl_object: Base<WebGLObject>,
        id: u32,
        is_deleted: Mut<bool>,
        fragment_shader: Mut<Option<JS<WebGLShader>>>,
        vertex_shader: Mut<Option<JS<WebGLShader>>>,
    }
}

impl WebGLProgram {
    fn new_inherited(&mut self, id: u32) {
        self.webgl_object.new_inherited();
        self.id.init(id);
        self.is_deleted.init(false);
        self.fragment_shader.init(Default::default());
        self.vertex_shader.init(Default::default());
    }

    pub fn maybe_new(global: GlobalRef, renderer: &IpcSender<CanvasMsg>)
                     -> Option<Root<WebGLProgram>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateProgram(sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|program_id| WebGLProgram::new(global, *program_id))
    }

    pub fn new(global: GlobalRef, id: u32) -> Root<WebGLProgram> {
        let mut obj = alloc_dom_object::<WebGLProgram>(global);
        obj.new_inherited(id);
        obj.into_root()
    }
}

impl WebGLProgram {
    /// glDeleteProgram
    pub fn delete(&self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteProgram(self.id.get()))).unwrap();
        }
    }

    /// glLinkProgram
    pub fn link(&self, renderer: &IpcSender<CanvasMsg>) {
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::LinkProgram(self.id.get()))).unwrap();
    }

    /// glUseProgram
    pub fn use_program(&self, renderer: &IpcSender<CanvasMsg>) {
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::UseProgram(self.id.get()))).unwrap();
    }

    /// glAttachShader
    pub fn attach_shader(&self, renderer: &IpcSender<CanvasMsg>, shader: &WebGLShader) -> WebGLResult<()> {
        let shader_slot = match shader.gl_type() {
            constants::FRAGMENT_SHADER => &self.fragment_shader,
            constants::VERTEX_SHADER => &self.vertex_shader,
            _ => return Err(WebGLError::InvalidOperation),
        };

        // TODO(ecoal95): Differentiate between same shader already assigned and other previous
        // shader.
        if shader_slot.get().is_some() {
            return Err(WebGLError::InvalidOperation);
        }

        shader_slot.set(Some(shader));

        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::AttachShader(self.id.get(), shader.id()))).unwrap();

        Ok(())
    }

    /// glGetAttribLocation
    pub fn get_attrib_location(&self, renderer: &IpcSender<CanvasMsg>, name: String) -> WebGLResult<Option<i32>> {
        if name.len() > MAX_UNIFORM_AND_ATTRIBUTE_LEN {
            return Err(WebGLError::InvalidValue);
        }

        // Check if the name is reserved
        if name.starts_with("webgl") || name.starts_with("_webgl_") {
            return Ok(None);
        }

        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::GetAttribLocation(self.id.get(), name, sender))).unwrap();
        Ok(receiver.recv().unwrap())
    }

    /// glGetUniformLocation
    pub fn get_uniform_location(&self, renderer: &IpcSender<CanvasMsg>, name: String) -> WebGLResult<Option<i32>> {
        if name.len() > MAX_UNIFORM_AND_ATTRIBUTE_LEN {
            return Err(WebGLError::InvalidValue);
        }

        // Check if the name is reserved
        if name.starts_with("webgl") || name.starts_with("_webgl_") {
            return Ok(None);
        }

        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::GetUniformLocation(self.id.get(), name, sender))).unwrap();
        Ok(receiver.recv().unwrap())
    }
}
