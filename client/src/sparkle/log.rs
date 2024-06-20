use cliclack::log;
use std::sync::Arc;
use workflow_log::prelude::*;
// use cliclack::{intro, log, outro_cancel};

struct Sink;

impl workflow_log::Sink for Sink {
    fn write(&self, _target: Option<&str>, level: Level, args: &std::fmt::Arguments<'_>) -> bool {
        match level {
            Level::Error => {
                let _ = log::error(format!("{args}"));
            }
            Level::Warn => {
                let _ = log::warning(format!("{args}"));
            }
            Level::Info => {
                let _ = log::info(format!("{args}"));
            }
            Level::Debug => {
                let _ = log::remark(format!("{args}"));
            }
            Level::Trace => {
                let _ = log::remark(format!("{args}"));
            }
        }
        true
    }
}

pub fn init() {
    // let sink = Sink {};
    workflow_log::pipe(Some(Arc::new(Sink)));
    // workflow_log::set_log_level(workflow_log::LevelFilter::Trace);
}
