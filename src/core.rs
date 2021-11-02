use std::process::exit;
use actix::prelude::*;

pub struct Core;

impl Actor for Core {
    type Context = Context<Self>;
}

// Core Events

//Shutdown Event
#[derive(Message)]
#[rtype(result = "()")]
pub struct ShutdownEvent();

impl Handler<ShutdownEvent> for Core {
    type Result = ();

    fn handle(&mut self, msg: ShutdownEvent, ctx: &mut Self::Context) -> Self::Result {
        System::current().stop();
        exit(0);
    }
}