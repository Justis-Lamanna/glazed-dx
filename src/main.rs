use std::thread;
use std::time::Duration;
use actix::Actor;
use crate::core::{Core, ShutdownEvent};

mod core;

#[actix_rt::main]
async fn main() {
    let core = Core.start();
    thread::sleep(Duration::from_secs(5));

    let res = core.send(ShutdownEvent()).await;

    loop{}
}