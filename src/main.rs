use std::time::{Duration, Instant};

fn main() {
    let mut lifecycle = GameContext::create();
    while !lifecycle.shutdown {
        lifecycle.update_ticks();
    }
    println!("Game was running for {:?}", lifecycle.start_time.elapsed());
}

#[derive(Debug)]
struct GameContext {
    shutdown: bool,
    start_time: Instant,
    running_time: Duration,
    ticks: u128
}

impl GameContext {
    fn create() -> GameContext {
        let now = Instant::now();
        GameContext {
            shutdown: false,
            start_time: now,
            running_time: Duration::ZERO,
            ticks: 0
        }
    }

    fn update_ticks(&mut self) {
        self.running_time = self.start_time.elapsed();
        let prev_ticks = self.ticks;
        self.ticks = (self.running_time.as_millis() * 6) / 100;
        if prev_ticks != self.ticks {
            println!("{} -> {}", prev_ticks, self.ticks);
        }
    }
}