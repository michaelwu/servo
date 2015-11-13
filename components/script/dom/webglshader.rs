/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use angle::hl::{BuiltInResources, Output, ShaderValidator};
use canvas_traits::{CanvasMsg, CanvasWebGLMsg, WebGLError, WebGLResult, WebGLShaderParameter};
use dom::bindings::codegen::Bindings::WebGLRenderingContextBinding::WebGLRenderingContextConstants as constants;
use dom::bindings::codegen::Bindings::WebGLShaderBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::magic::alloc_dom_object;
use dom::webglobject::WebGLObject;
use ipc_channel::ipc::{self, IpcSender};
use std::cell::{Cell, RefCell};
use std::sync::{ONCE_INIT, Once};

#[derive(Clone, Copy, PartialEq, Debug, NumFromPrimitive, JSTraceable, HeapSizeOf)]
pub enum ShaderCompilationStatus {
    NotCompiled,
    Succeeded,
    Failed,
}

magic_dom_struct! {
    pub struct WebGLShader {
        webgl_object: Base<WebGLObject>,
        id: u32,
        gl_type: u32,
        source: Layout<Option<String>>,
        info_log: Layout<Option<String>>,
        is_deleted: Mut<bool>,
        compilation_status: Mut<ShaderCompilationStatus>,
    }
}

#[cfg(not(target_os = "android"))]
const SHADER_OUTPUT_FORMAT: Output = Output::Glsl;

#[cfg(target_os = "android")]
const SHADER_OUTPUT_FORMAT: Output = Output::Essl;

static GLSLANG_INITIALIZATION: Once = ONCE_INIT;

impl WebGLShader {
    fn new_inherited(&mut self, id: u32, shader_type: u32) {
        GLSLANG_INITIALIZATION.call_once(|| ::angle::hl::initialize().unwrap());
        self.webgl_object.new_inherited();
        self.id.init(id);
        self.gl_type.init(shader_type);
        self.source.init(None);
        self.info_log.init(None);
        self.is_deleted.init(false);
        self.compilation_status.init(ShaderCompilationStatus::NotCompiled);
    }

    pub fn maybe_new(global: GlobalRef,
                     renderer: &IpcSender<CanvasMsg>,
                     shader_type: u32) -> Option<Root<WebGLShader>> {
        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::CreateShader(shader_type, sender))).unwrap();

        let result = receiver.recv().unwrap();
        result.map(|shader_id| WebGLShader::new(global, *shader_id, shader_type))
    }

    pub fn new(global: GlobalRef,
               id: u32,
               shader_type: u32) -> Root<WebGLShader> {
        let mut obj = alloc_dom_object::<WebGLShader>(global);
        obj.new_inherited(id, shader_type);
        obj.into_root()
    }
}

impl WebGLShader {
    pub fn id(&self) -> u32 {
        self.id.get()
    }

    pub fn gl_type(&self) -> u32 {
        self.gl_type.get()
    }

    /// glCompileShader
    pub fn compile(&self, renderer: &IpcSender<CanvasMsg>) {
        if self.compilation_status.get() != ShaderCompilationStatus::NotCompiled {
            debug!("Compiling already compiled shader {}", self.id.get());
        }

        if let Some(ref source) = self.source.get() {
            let validator = ShaderValidator::for_webgl(self.gl_type.get(),
                                                       SHADER_OUTPUT_FORMAT,
                                                       &BuiltInResources::default()).unwrap();
            match validator.compile_and_translate(&[source.as_bytes()]) {
                Ok(translated_source) => {
                    // NOTE: At this point we should be pretty sure that the compilation in the paint task
                    // will succeed.
                    // It could be interesting to retrieve the info log from the paint task though
                    let msg = CanvasWebGLMsg::CompileShader(self.id.get(), translated_source);
                    renderer.send(CanvasMsg::WebGL(msg)).unwrap();
                    self.compilation_status.set(ShaderCompilationStatus::Succeeded);
                },
                Err(error) => {
                    self.compilation_status.set(ShaderCompilationStatus::Failed);
                    debug!("Shader {} compilation failed: {}", self.id.get(), error);
                },
            }

            self.info_log.set(Some(validator.info_log()));
        }
    }

    /// Mark this shader as deleted (if it wasn't previously)
    /// and delete it as if calling glDeleteShader.
    pub fn delete(&self, renderer: &IpcSender<CanvasMsg>) {
        if !self.is_deleted.get() {
            self.is_deleted.set(true);
            renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::DeleteShader(self.id.get()))).unwrap()
        }
    }

    /// glGetShaderInfoLog
    pub fn info_log(&self) -> Option<String> {
        self.info_log.get()
    }

    /// glGetShaderParameter
    pub fn parameter(&self, renderer: &IpcSender<CanvasMsg>, param_id: u32) -> WebGLResult<WebGLShaderParameter> {
        match param_id {
            constants::SHADER_TYPE | constants::DELETE_STATUS | constants::COMPILE_STATUS => {},
            _ => return Err(WebGLError::InvalidEnum),
        }

        let (sender, receiver) = ipc::channel().unwrap();
        renderer.send(CanvasMsg::WebGL(CanvasWebGLMsg::GetShaderParameter(self.id.get(), param_id, sender))).unwrap();
        Ok(receiver.recv().unwrap())
    }

    /// Get the shader source
    pub fn source(&self) -> Option<String> {
        self.source.get()
    }

    /// glShaderSource
    pub fn set_source(&self, source: String) {
        self.source.set(Some(source));
    }
}
