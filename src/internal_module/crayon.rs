use crate::{quickjs_sys::AsObject, Context, JsValue};

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "crayon_host")]
extern "C" {
    fn __crayon_get_event(ptr: i32, len: i32) -> i32;
    fn __crayon_send_response(ptr: i32, len: i32) -> i32;
}

#[cfg(not(target_arch = "wasm32"))]
mod host_shim {
    pub unsafe fn __crayon_get_event(_ptr: i32, _len: i32) -> i32 {
        -1
    }
    pub unsafe fn __crayon_send_response(_ptr: i32, _len: i32) -> i32 {
        -1
    }
}

#[cfg(not(target_arch = "wasm32"))]
use host_shim::{__crayon_get_event, __crayon_send_response};

pub fn init_crayon_host(ctx: &mut Context) {
    fn read_arg(argv: &[JsValue], idx: usize) -> i32 {
        match argv.get(idx) {
            Some(JsValue::Int(v)) => *v,
            Some(JsValue::Float(v)) => *v as i32,
            Some(JsValue::Bool(v)) => {
                if *v {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn js_crayon_get_event(_ctx: &mut Context, _this: JsValue, argv: &[JsValue]) -> JsValue {
        let ptr = read_arg(argv, 0);
        let len = read_arg(argv, 1);
        let rc = unsafe { __crayon_get_event(ptr, len) };
        JsValue::Int(rc)
    }

    fn js_crayon_send_response(_ctx: &mut Context, _this: JsValue, argv: &[JsValue]) -> JsValue {
        let ptr = read_arg(argv, 0);
        let len = read_arg(argv, 1);
        let rc = unsafe { __crayon_send_response(ptr, len) };
        JsValue::Int(rc)
    }

    let mut global = ctx.get_global();
    global.set(
        "__crayon_get_event",
        ctx.wrap_function("__crayon_get_event", js_crayon_get_event)
            .into(),
    );
    global.set(
        "__crayon_send_response",
        ctx.wrap_function("__crayon_send_response", js_crayon_send_response)
            .into(),
    );
}
