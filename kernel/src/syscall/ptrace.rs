use super::SyscallReturn;
use int_to_c_enum::TryFromInt;
use crate:: {
    context::Context,
    prelude::*,
    process::{
        pid_talbe::pid_table_mut,
        posix_thread::AsPosixThread,
        Pid,
    },
    thread::Thread,
};

enum PtraceRequest {
    PTRACE_TRACEME = 0,
    PTRACE_PEEKTEXT = 1,
    PTRACE_PEEKDATA = 2,
    PTRACE_PEEKUSER = 3,
    PTRACE_POKETEXT = 4,
    PTRACE_POKEDATA = 5,
    PTRACE_POKEUSER = 6,
    PTRACE_GETREGS = 7,
    PTRACE_GETFPREGS = 8,
    PTRACE_GETREGSET = 9,
    PTRACE_SETREGS = 10,
    PTRACE_SETFPREGS = 11,
    PTRACE_SETREGSET = 12,
    PTRACE_GETSIGINFO = 13,
    PTRACE_SETSIGINFO = 14,
    PTRACE_PEEKSIGINFO = 15,
    PTRACE_GETSIGMASK = 16,
    PTRACE_SETSIGMASK = 17,
    PTRACE_SETOPTIONS = 18,
    PTRACE_GETEVENTMSG = 19,
    PTRACE_CONT = 20,
    PTRACE_SYSCALL = 21,
    PTRACE_SINGLESTEP = 22,
    PTRACE_SETSYSCALL = 23,
    PTRACE_SYSMU = 24,
    PTRACE_SYSMU_SINGLESTEP = 25,
    PTRACE_LISTEN = 26,
    PTRACE_KILL = 27,
    PTRACE_INTERRUPT = 28,
    PTRACE_ATTACH = 29,
    PTRACE_SEIZE = 30,
    PTRACE_DETACH = 31,
    PTRACE_GET_SYSCALL_INFO = 32,
}

pub fn sys_ptrace(
    request: u32,
    pid: u32,
    addr: usize,
    data: usize,
    ctx: &Context,
) -> Result<SyscallReturn> {
    debug!("ptrace call: request = {}, pid = {}, addr = {#x}", request, pid ,addr);
    let request = PtraceRequest::try_from(request).map_err(|_| {
        Error::with_message(Errno::EINVAL, "invalid ptrace request")
    })?;
    match request {
        PtraceRequest::PTRACE_ATTACH => ptrace_attach(pid, ctx);
        PtraceRequest::PTRACE_DETACH => ptrace_detach(pid, data, ctx);
        _ => {
            return_errno_with_message!(Errno::ENOSYS, "unsupported ptrace request");
        }
    }
}

/// Implementations of requests.
pub fn ptrace_attach(target_id: Pid, ctx: &Context) -> Result<SyscallReturn>{
    let tracer = ctx.posix_thread;

    if tracer.tid() == target_id {
        return_errno_with_message!(Errno::EPERM, "cannot attach to self");
    }
    
    let target_thread = pid_table_mut().get_thread(target_id)
            .ok_or_else(|| Error::with_message(Errno::ESRCH, "target thread does not exist"))?;
    let target = target_thread.as_posix_thread().
            .ok_or_else(|| Error::with_message(Errno::ESRCH, "target is not a POSIX thread"))?;
    
    /// TODO: Check access authority before attaching
    
    if target.process().status().is_zombie() {
        return_errno_with_message!(Errno::ESRCH, "target process is dead");
    }

    {
        let mut ptrace_info = target.ptrace_info.lock();

        if ptrace_info.tracer_tid().is_some() {
            return_errno_with_message!(Errno::EPERM, "target already traced");
        }

        ptrace_info.set_tracer(tracer.tid());
        ptrace_info.set_stop_reason(PtraceStopReason::Attach);
    }
    
    // TODO
    // 1. 是只停这个线程，还是还要影响 process stop status？
    // 2. 怎么唤醒 tracer 的 wait 路径？
    // 3. target 当前如果正在跑，怎么让它尽快观察到 stop？
    Ok(SyscallReturn::Return(0))
}

pub fn ptrace_detach(target_id: Pid, data: u32, ctx: &Context) -> Result<SyscallReturn> {
    let caller = ctx.posix_thread;

    let target_thread = pid_table_mut().get_thread(target_id)
            .ok_or_else(|| Error::with_message(Errno::ESRCH, "target thread does not exist"))?;
    let target = target_thread.as_posix_thread().
            .ok_or_else(|| Error::with_message(Errno::ESRCH, "target is not a POSIX thread"))?;

    if target.process().status().is_zombie() {
        return_errno_with_message!(Errno::ESRCH, "target process is dead");
    }

    {
        let mut ptrace_info = target.ptrace_info.lock();

        if let Some(tracer_tid) = ptrace_info.tracer_tid() {
            if caller.tid() != tracer_tid {
                return_errno_with_message!(Errno::ESRCH, "target thread is not traced by this thread");
            }
        } else {
            return_errno_with_message!(Errno::ESRCH, "target thread is not traced");
        }

        ptrace_info.clear_tracer();
        ptrace_info.clear_stop_reason();
    }

    // TODO:
    // 1. 把信号号为data的信号注入tracee
    // 2. 唤醒tracee
    // 3. 确保tracer退出后tracee不会永久等待restart

    Ok(SyscallReturn::Return(0))
}