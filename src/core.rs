use std::collections::HashMap;
use std::process::exit;
use std::time::{Duration, Instant};

use actix::prelude::*;

#[derive(Debug)]
pub struct Core {
    start_time: Instant,
    tick: u128,
    tick_hooks: HashMap<String, Recipient<TickEvent>>
}

impl Default for Core {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            tick: 0,
            tick_hooks: HashMap::new()
        }
    }
}

impl Actor for Core {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Starting Core");
        ctx.run_interval(Duration::from_millis(8), move |act, _| {
            let runtime = act.start_time.elapsed();
            let ticks = (runtime.as_millis() * 6) / 100;
            if ticks != act.tick {
                act.tick = ticks;
                act.tick_hooks.retain(|id, hook| {
                    match hook.do_send(TickEvent(ticks)) {
                        Err(SendError::Closed(_)) => {
                            println!("{} was closed. Queueing for removal...", id);
                            false
                        }
                        _ => true
                    }
                });
            }
        });
    }
}

// Core Events
#[derive(Message)]
#[rtype(result = "()")]
pub struct ShutdownEvent();

#[derive(Message)]
#[rtype(result = "()")]
pub struct TickEvent(pub u128);

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterTickHook(pub String, pub Recipient<TickEvent>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct DeregisterTickHook(pub String);

impl Handler<ShutdownEvent> for Core {
    type Result = ();

    fn handle(&mut self, _: ShutdownEvent, _: &mut Self::Context) -> Self::Result {
        System::current().stop();
        exit(0);
    }
}

impl Handler<RegisterTickHook> for Core {
    type Result = ();

    fn handle(&mut self, msg: RegisterTickHook, _: &mut Self::Context) -> Self::Result {
        println!("Registered tick hook {}", msg.0);
        self.tick_hooks.insert(msg.0, msg.1);
    }
}

impl Handler<DeregisterTickHook> for Core {
    type Result = ();

    fn handle(&mut self, msg: DeregisterTickHook, _: &mut Self::Context) -> Self::Result {
        println!("Deregistered tick hook {}", msg.0);
        self.tick_hooks.remove(msg.0.as_str());
    }
}