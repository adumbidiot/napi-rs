use napi::{CallContext, JsFunction, JsNull, JsObject, Module, Result};

#[js_function(1)]
pub fn call_function(ctx: CallContext) -> Result<JsNull> {
  let js_func = ctx.get::<JsFunction>(0)?;
  let js_string_hello = ctx.env.create_string("hello".as_ref())?.into_unknown();
  let js_string_world = ctx.env.create_string("world".as_ref())?.into_unknown();

  js_func.call(None, &[js_string_hello, js_string_world])?;

  ctx.env.get_null()
}

#[js_function(1)]
pub fn call_function_with_this(ctx: CallContext) -> Result<JsNull> {
  let js_this: JsObject = unsafe { ctx.this_unchecked() };
  let js_func = ctx.get::<JsFunction>(0)?;

  js_func.call(Some(&js_this), &[])?;

  ctx.env.get_null()
}

pub fn register_js(module: &mut Module) -> Result<()> {
  module.create_named_method("testCallFunction", call_function)?;
  module.create_named_method("testCallFunctionWithThis", call_function_with_this)?;
  Ok(())
}
