use crate::env::*;
use crate::SkippyResult;
use rhai::plugin::*;
use std::sync::{Arc, Mutex};

pub fn bootstrap() -> Engine {
    let mut engine = Engine::new();
    let env = Arc::new(Mutex::new(Env::new()));

    engine
        .register_type::<Device>()
        .register_result_fn("connect", {
            let env = env.clone();
            move |addr: ImmutableString| env.lock().unwrap().connect(addr.as_str())
        })
        .register_result_fn("disconnect", {
            let env = env.clone();
            move |dev: Device| env.lock().unwrap().disconnect(dev)
        })
        .register_result_fn("scpi", {
            let env = env;
            move |dev: Device, raw: ImmutableString| env.lock().unwrap().scpi(dev, raw.as_str())
        })
        .register_result_fn("clear_status", clear_status)
        .register_result_fn("event_status_enable", event_status_enable)
        .register_result_fn("event_status_enable_query", event_status_enable_query)
        .register_result_fn("event_status_enable_register", event_status_enable_register)
        .register_result_fn("identify", identify)
        .register_result_fn("operation_complete_command", operation_complete_command)
        .register_result_fn("operation_complete_query", operation_complete_query)
        .register_result_fn("identify_options_query", identify_options_query)
        .register_result_fn("reset", reset)
        .register_result_fn("service_frequest_enable", service_frequest_enable)
        .register_result_fn(
            "service_frequest_enable_query",
            service_frequest_enable_query,
        )
        .register_result_fn("status_byte_query", status_byte_query)
        .register_result_fn("self_test", self_test)
        .register_result_fn("wait", wait);

    engine
}

#[export_fn(return_raw)]
pub fn clear_status(ctx: NativeCallContext, dev: Device) -> SkippyResult<()> {
    ctx.call_fn("scpi", (dev, "*CLS")).and_then(discard)
}

#[export_fn(return_raw)]
pub fn event_status_enable(ctx: NativeCallContext, dev: Device) -> SkippyResult<()> {
    ctx.call_fn("scpi", (dev, "*ESE")).and_then(discard)
}

#[export_fn(return_raw)]
pub fn event_status_enable_query(
    ctx: NativeCallContext,
    dev: Device,
) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*ESE?"))
}

#[export_fn(return_raw)]
pub fn event_status_enable_register(ctx: NativeCallContext, dev: Device) -> SkippyResult<()> {
    ctx.call_fn("scpi", (dev, "*ESR")).and_then(discard)
}

#[export_fn(return_raw)]
pub fn identify(ctx: NativeCallContext, dev: Device) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*IDN?"))
}

#[export_fn(return_raw)]
pub fn operation_complete_command(
    ctx: NativeCallContext,
    dev: Device,
) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*OPC"))
}

#[export_fn(return_raw)]
pub fn operation_complete_query(
    ctx: NativeCallContext,
    dev: Device,
) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*OPC?"))
}

#[export_fn(return_raw)]
pub fn identify_options_query(
    ctx: NativeCallContext,
    dev: Device,
) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*OPT?"))
}

#[export_fn(return_raw)]
pub fn reset(ctx: NativeCallContext, dev: Device) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*RST"))
}

#[export_fn(return_raw)]
pub fn service_frequest_enable(
    ctx: NativeCallContext,
    dev: Device,
) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*SRE"))
}

#[export_fn(return_raw)]
pub fn service_frequest_enable_query(
    ctx: NativeCallContext,
    dev: Device,
) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*SRE?"))
}

#[export_fn(return_raw)]
pub fn status_byte_query(ctx: NativeCallContext, dev: Device) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*STB?"))
}

#[export_fn(return_raw)]
pub fn self_test(ctx: NativeCallContext, dev: Device) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*TST?"))
}

#[export_fn(return_raw)]
pub fn wait(ctx: NativeCallContext, dev: Device) -> SkippyResult<ImmutableString> {
    ctx.call_fn("scpi", (dev, "*WAI"))
}

fn discard(_: ImmutableString) -> SkippyResult<()> {
    Ok(())
}
