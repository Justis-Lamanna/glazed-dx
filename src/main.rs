use actix::{Actor, Arbiter, Context, Handler};

use crate::core::{Core, RegisterTickHook, TickEvent};

mod core;

#[actix_rt::main]
async fn main() {
    let core_arbiter = Arbiter::new();
    let core = Actor::start_in_arbiter(&core_arbiter, |_| Core::default());
    let test = Actor::start_in_arbiter(&core_arbiter, |_| TestPingActor::default());

    core.do_send(RegisterTickHook(String::from("tickprint"), test.recipient()));

    loop {}
}

#[derive(Default)]
struct TestPingActor;

impl Actor for TestPingActor {
    type Context = Context<Self>;
}
impl Handler<TickEvent> for TestPingActor {
    type Result = ();

    fn handle(&mut self, msg: TickEvent, _: &mut Self::Context) -> Self::Result {
        println!("{}", msg.0);
    }
}