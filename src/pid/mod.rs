//! Process-specific information from `/proc/[pid]/`.

mod cwd;
mod io;
mod limits;
mod mountinfo;
mod stat;
mod statm;
mod status;
mod cpu;

pub use pid::cwd::{cwd, cwd_self};
pub use pid::io::{Io, io, io_self, io_task};
pub use pid::limits::{Limit, Limits, limits, limits_self, limits_task};
pub use pid::mountinfo::{Mountinfo, mountinfo, mountinfo_self, mountinfo_task};
pub use pid::statm::{Statm, statm, statm_self, statm_task};
pub use pid::status::{SeccompMode, Status, status, status_self, status_task};
pub use pid::stat::{Stat, stat, stat_self, stat_task};
pub use pid::cpu::{Cpu};

/// The state of a process.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum State {
    /// Running.
    Running,
    /// Sleeping in an interruptible wait.
    Sleeping,
    /// Waiting in uninterruptible disk sleep.
    Waiting,
    /// Zombie.
    Zombie,
    /// Stopped (on a signal) or (before Linux 2.6.33) trace stopped.
    Stopped,
    /// trace stopped.
    ///
    /// Linux 2.6.33 onward.
    TraceStopped,
    /// Paging.
    ///
    /// Only before linux 2.6.0.
    Paging,
    /// Dead.
    ///
    /// Linux 2.6.33 to 3.13 only.
    Dead,
    /// Wakekill.
    ///
    /// Linux 2.6.33 to 3.13 only.
    Wakekill,
    /// Waking.
    ///
    /// Linux 2.6.33 to 3.13 only.
    Waking,
    /// Parked.
    ///
    /// Linux 3.9 to 3.13 only.
    Parked,
}

impl Default for State {
    fn default() -> State {
        State::Running
    }
}
