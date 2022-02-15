mod env;
mod skippy;

pub type SkippyError = Box<rhai::EvalAltResult>;
pub type SkippyResult<T> = Result<T, SkippyError>;

pub fn main() -> SkippyResult<()> {
    let engine = skippy::bootstrap();
    engine.run_file("scripts/demo.skippy".into())
}
