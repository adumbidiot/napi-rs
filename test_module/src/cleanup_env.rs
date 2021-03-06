use napi::{
  CallContext, CleanupEnvHook, ContextlessResult, Env, JsExternal, JsUndefined, Module, Result,
};

#[contextless_function]
fn add_cleanup_hook(mut env: Env) -> ContextlessResult<JsExternal> {
  let hook = env.add_env_cleanup_hook((), |_| {
    println!("cleanup hook executed");
  })?;
  env.create_external(hook).map(Some)
}

#[js_function(1)]
fn remove_cleanup_hook(ctx: CallContext) -> Result<JsUndefined> {
  let hook_external = ctx.get::<JsExternal>(0)?;
  let hook = *ctx
    .env
    .get_value_external::<CleanupEnvHook<()>>(&hook_external)?;
  ctx.env.remove_env_cleanup_hook(hook)?;
  ctx.env.get_undefined()
}

pub fn register_js(module: &mut Module) -> Result<()> {
  module.create_named_method("addCleanupHook", add_cleanup_hook)?;
  module.create_named_method("removeCleanupHook", remove_cleanup_hook)?;
  Ok(())
}
