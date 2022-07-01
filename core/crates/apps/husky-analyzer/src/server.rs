pub(crate) mod client_comm;
pub(crate) mod event_loop_comm;

use crossbeam_channel::Sender;

use __husky_root::__root_defn;
use client_comm::ClientCommunicator;
use event_loop_comm::EventLoopCommunicator;
use husky_compile_time::HuskyCompileTime;
use threadpool::ThreadPool;

pub(crate) struct Server {
    pub(crate) client_comm: ClientCommunicator,
    pub(crate) event_loop_comm: EventLoopCommunicator,
    pub(crate) db: HuskyCompileTime,
    pub(crate) threadpool: ThreadPool,
}

impl Server {
    pub fn new(sender: Sender<lsp_server::Message>) -> Server {
        Server {
            client_comm: ClientCommunicator::new(sender),
            threadpool: ThreadPool::default(),
            event_loop_comm: EventLoopCommunicator::default(),
            db: HuskyCompileTime::new(__root_defn),
        }
    }
}

pub enum TaskSet {
    Nothing,
    Shutdown,
    SendUpdates,
    Respond(lsp_server::Response),
}

impl TaskSet {
    pub fn then(&mut self, next: TaskSet) {
        match self {
            TaskSet::Nothing => *self = next,
            TaskSet::Shutdown => (),
            TaskSet::SendUpdates => todo!(),
            TaskSet::Respond(_) => todo!(),
        }
    }
}
