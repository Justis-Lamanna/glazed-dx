use std::cmp::Ordering;
use rand::Rng;
use crate::Action;

pub fn determine_stat_multiplier(stage: i8) -> f64 {
    match stage {
        -5 => 2f64/ 7f64,
        -4 => 2f64/ 6f64,
        -3 => 2f64/ 5f64,
        -2 => 2f64/ 4f64,
        -1 => 2f64/ 3f64,
        0 => 1f64,
        1 => 3f64 / 2f64,
        2 => 2f64,
        3 => 5f64 / 2f64,
        4 => 3f64,
        5 => 7f64 / 2f64,
        a => if a <= -6 { 2f64 / 8f64 } else { 4f64 }
    }
}

pub fn determine_accuracy_stat_multiplier(stage: i8) -> f64 {
    match stage {
        -5 => 3f64/ 8f64,
        -4 => 3f64/ 7f64,
        -3 => 3f64/ 6f64,
        -2 => 3f64/ 5f64,
        -1 => 3f64/ 4f64,
        0 => 1f64,
        1 => 4f64 / 3f64,
        2 => 5f64 / 3f64,
        3 => 2f64,
        4 => 7f64 / 3f64,
        5 => 8f64 / 3f64,
        a => if a <= -6 { 3f64 / 9f64 } else { 3f64 }
    }
}

pub fn get_action_order<T>(mut actions: Vec<(T, u16)>) -> Vec<(T, u16)> where
    T: Action {
    actions.sort_unstable_by(|left, right| {
        let (left_action, left_speed) = left;
        let (right_action, right_speed) = right;

        match left_action.get_priority().cmp(&right_action.get_priority()) {
            Ordering::Equal => {
                match left_speed.cmp(&right_speed) {
                    Ordering::Equal => {
                        println!("Random first start");
                        if rand::thread_rng().gen_bool(0.5) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    },
                    o => o.reverse()
                }
            }
            o => o.reverse()
        }
    });
    actions
}