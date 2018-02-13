pub mod pid;

use std::rc::Rc;

use self::pid::PID;

pub trait Process: Sized {
    fn managed(manage: bool);
    fn get_pid(&self) -> PID;
}

pub fn spawn<T: Process>(process: Rc<T>, manage: bool) -> PID {
    let mut spawned = false;

    // TODO: Check if ProcessManager is finalizing
    // TODO: Check if process is already initialized
    // TODO: Check if process is already spawned
    // TODO: Add process to process list
    spawned = true;

    // TODO: do this? process->pid.reference = process->reference;

    //if (!spawned) {
    //    if (manage) {
    //        drop(process);
    //    }
        // TODO: return UPID.new();
    //}

    //if (manage) {
    //    process.is_managed(true);
    //}

    let pid = process.get_pid();

    enqueue(process);

    println!("Spawned process {}", pid);

    return pid;
}

fn enqueue<T: Process>(process: Rc<T>) {
    // TODO: Check if libprocess is shutting shutting down
    // TODO: Enqueue in Run Queue

}

pub fn wait<T: Process>(process: Rc<T>) {}

pub fn finalize(finalize_wsa: bool) {}
