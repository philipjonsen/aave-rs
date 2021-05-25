#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate bigdecimal;

use bigdecimal::BigDecimal;
use core::str;
use napi::{CallContext, Env, JsNumber, JsObject, JsString, Result, Task};
use std::str::FromStr;
use std::{convert::TryInto, ops::Div};

struct AsyncTask(u32);

impl Task for AsyncTask {
    type Output = u32;
    type JsValue = JsNumber;

    fn compute(&mut self) -> Result<Self::Output> {
        use std::thread::sleep;
        use std::time::Duration;
        sleep(Duration::from_millis(self.0 as u64));
        Ok(self.0 * 2)
    }

    fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_uint32(output)
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("sync", sync_fn)?;

    exports.create_named_method("sleep", sleep)?;

    exports.create_named_method("normalize", normalize)?;
    Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

    ctx.env.create_uint32(argument + 100)
}

#[js_function(1)]
fn sleep(ctx: CallContext) -> Result<JsObject> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let task = AsyncTask(argument);
    let async_task = ctx.env.spawn(task)?;
    Ok(async_task.promise_object())
}

fn native_normalize(base: BigDecimal, decimals: u32) -> BigDecimal {
    return base.div(10_i64.pow(decimals));
}

#[js_function(2)]
fn normalize(ctx: CallContext) -> Result<JsString> {
    let base = ctx.get::<JsString>(0)?.into_utf8()?.into_owned().unwrap();
    let base_big_d = BigDecimal::from_str(&base).unwrap();
    let decimals: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
    let result = native_normalize(base_big_d, decimals);
    ctx.env.create_string(&result.to_string())
}
