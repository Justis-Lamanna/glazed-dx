use rand::Rng;
use glazed_data::attack::{Accuracy, Move};
use crate::{ActionSideEffects, ActivePokemon, Cause};
use crate::core::{ActionCheck, MoveContext};

pub fn get_accuracy_factor(attacker: &ActivePokemon, defender: &ActivePokemon) -> f64 {
    let net_stages = attacker.data.borrow().accuracy_stage - defender.data.borrow().evasion_stage;
    match net_stages {
        i8::MIN..=-6 => 3f64/9f64,
        -5 => 0.375,
        -4 => 3f64/7f64,
        -3 => 0.5,
        -2 => 0.6,
        -1 => 0.75,
        0 => 1f64,
        1 => 4f64/3f64,
        2 => 5f64/3f64,
        3 => 2f64,
        4 => 7f64/3f64,
        5 => 8f64/3f64,
        6..=i8::MAX => 3f64,
    }
}

pub fn do_accuracy_check<F>(attacker: &ActivePokemon, attack: F, defender: &ActivePokemon) -> ActionCheck<bool>
    where F: Into<MoveContext>
{
    let MoveContext { attack, data: move_data, .. } = attack.into();
    match move_data.accuracy {
        Accuracy::AlwaysHits => ActionCheck::Ok(true),
        Accuracy::Percentage(p) => {
            let evasion_accuracy = get_accuracy_factor(attacker, defender);
            let move_accuracy = f64::from(p) / 100f64;
            let multiply = evasion_accuracy * move_accuracy;
            if multiply > 1f64 {
                ActionCheck::Ok(true)
            } else {
                ActionCheck::Ok(rand::thread_rng().gen_bool(evasion_accuracy * move_accuracy))
            }
        },
        Accuracy::Variable => match attack {
            Move::Guillotine | Move::Fissure | Move::HornDrill | Move::SheerCold => {
                let level_user = attacker.borrow().level;
                let level_target = defender.borrow().level;
                if level_user < level_target {
                    ActionCheck::Err(ActionSideEffects::Failed(Cause::Natural))
                } else {
                    let acc = (level_user - level_target + 30) as f64 / 100f64;
                    ActionCheck::Ok(if acc > 1f64 { true } else {rand::thread_rng().gen_bool(acc)})
                }
            },
            _ => unimplemented!("Unknown accuracy for {:?}", attack)
        }
    }
}