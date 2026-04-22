use crate::{
    process::{pid_table, Pid},
    thread::{Thread, Tid},
}

pub enum PtraceStopReason {
    // TODO: complete this enum.
    SignalDelivery,
    SyscallEnter,
    SyscallExit,
    Attach,
    Event,
}

pub struct PtraceInfo {
    tracer_tid: Option<Tid>,
    stop_reason: Option<PtraceStopReason>,
    step_mode: bool,
}

impl Default for PtraceInfo {
    fn default() -> Self {
        Self {
            tracer_tid: None,
            stop_reason: None,
            step_mode: false,
        }
    }
}

impl PtraceInfo {
    pub fn tracer_tid(&self) -> Option<Tid> {
        self.tracer_tid
    }
    pub fn set_tracer(&mut self, tracer_tid: Tid) {
        self.tracer_tid = Some(tracer_tid);
    }
    pub fn clear_tracer(&mut self) {
        self.tracer_tid = None;
    }
    pub fn set_stop_reason(&mut self, stop_reason: PtraceStopReason) {
        self.stop_reason = Some(stop_reason);
    }
    pub fn clear_stop_reason(&mut self) {
        self.stop_reason = None;
    }
}