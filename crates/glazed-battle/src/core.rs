use std::cmp::Ordering;
use rand::Rng;
use crate::TurnAction;

pub fn get_action_order(mut actions: Vec<(TurnAction, f64)>) -> Vec<(TurnAction, f64)> {
    actions.sort_unstable_by(|left, right| {
        let (left_action, left_speed) = left;
        let (right_action, right_speed) = right;

        match left_action.get_priority().cmp(&right_action.get_priority()) {
            Ordering::Equal => {
                if left_speed < right_speed {
                    Ordering::Greater
                } else if right_speed > left_speed {
                    Ordering::Less
                } else {
                    println!("Random first start");
                    if rand::thread_rng().gen_bool(0.5) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
            o => o.reverse()
        }
    });
    actions
}