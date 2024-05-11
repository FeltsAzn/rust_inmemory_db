use threadpool::ThreadPool;

use database::{
    DatabaseCommand,
    DatabaseController,
    DatabaseResult,
    DatabaseValue,
    DataModel,
};
use handler::{
    Manager,
    ProcessingResult,
    RequestParsedValue,
    RequestParser,
};
use listener::GateListener;

mod listener;
mod handler;
mod database;

fn main() {
    let port = 9999;
    let gate = GateListener::new(port);
    let manager = Manager::new(
        DatabaseController::new(),
        RequestParser::new(),
    );
    let scheduler = ThreadPool::new(3);

    gate.listen(scheduler, manager)
}
