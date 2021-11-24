use std::ops::Index;
use glazed_data::attack::{Accuracy, Move};
use glazed_data::item::Item;
use glazed_data::pokemon::Pokemon;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Party {
    number_of_members: usize,
    members: [Option<Pokemon>; 6]
}
impl Party {
    fn create_one<T>(one: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 1,
            members: [Some(one.into()), None, None, None, None, None]
        }
    }

    fn create_two<T>(one: T, two: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 2,
            members: [Some(one.into()), Some(two.into()), None, None, None, None]
        }
    }

    fn create_three<T>(one: T, two: T, three: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 3,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), None, None, None]
        }
    }

    fn create_four<T>(one: T, two: T, three: T, four: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 4,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), None, None]
        }
    }

    fn create_five<T>(one: T, two: T, three: T, four: T, five: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 5,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), Some(five.into()), None]
        }
    }

    fn create_six<T>(one: T, two: T, three: T, four: T, five: T, six: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 6,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), Some(five.into()), Some(six.into())]
        }
    }

    fn len(&self) -> usize {
        self.number_of_members
    }
}
impl Index<u8> for Party {
    type Output = Option<Pokemon>;

    fn index(&self, index: u8) -> &Self::Output {
        if index > 6 {
            &None
        } else {
            &self.members[usize::from(index)]
        }
    }
}

struct SinglesBattlefield {
    user_party: Party,
    opponent_party: Party,
    user: BattlePokemon,
    opponent: BattlePokemon,
    field: Field,
    record: Record
}
impl SinglesBattlefield {
}

struct DoublesBattlefield {
    user_left: Option<BattlePokemon>,
    user_right: Option<BattlePokemon>,
    opponent_left: Option<BattlePokemon>,
    opponent_right: Option<BattlePokemon>,
    field: Field,
    record: Record
}
impl DoublesBattlefield {
}

struct BattlePokemon {
    pokemon: Pokemon,
    attack_stage: i8,
    defense_stage: i8,
    special_attack_stage: i8,
    special_defense_stage: i8,
    speed_stage: i8,
    accuracy_stage: i8,
    evasion_stage: i8
}
impl From<Pokemon> for BattlePokemon {
    fn from(pokemon: Pokemon) -> BattlePokemon {
        BattlePokemon {
            pokemon,
            attack_stage: 0,
            defense_stage: 0,
            special_attack_stage: 0,
            special_defense_stage: 0,
            speed_stage: 0,
            accuracy_stage: 0,
            evasion_stage: 0
        }
    }
}

#[derive(Default)]
struct Field {

}

#[derive(Default)]
struct Record {

}

enum TrainerAction {
    Attack{user: u8, targets: Option<u8>, attack: Move},
    Item{target: u8, item: Item},
    Swap{pokemon: u8, }
}

struct ActionResult {
    action: TrainerAction,
    effects: Vec<ActionSideEffect>
}

enum ActionSideEffect {

}

// struct BattleCalculations;
// impl BattleCalculations {
//     fn calculate_raw_accuracy(user: &BattlePokemon, target: &BattlePokemon, attack: &Move) -> f32 {
//         let move_data = attack.data();
//         let move_accuracy = match move_data.accuracy {
//             Accuracy::AlwaysHits => return 100f32,
//             Accuracy::Percentage(a) => f32::from(a) / 100f32
//         };
//         let effective_pokemon_accuracy = match target.evasion_stage - user.accuracy_stage {
//             a if a <= -6 => 1f32 / 3f32,
//             -5 => 3f32 / 8f32,
//             -4 => 3f32 / 7f32,
//             -3 => 1f32 / 2f32,
//             -2 => 3f32 / 5f32,
//             -1 => 3f32 / 4f32,
//             0 => 1f32,
//             1 => 4f32 / 3f32,
//             2 => 5f32 / 3f32,
//             3 => 2f32,
//             4 => 7f32 / 3f32,
//             5 => 8f32 / 3f32,
//             a if a >= 6 => 3f32,
//             _ => panic!("Should not happen: clamped between -6 and 6")
//         };
//
//         move_accuracy * effective_pokemon_accuracy
//     }
//
//     fn calculate_damage(user: &BattlePokemon, target: &BattlePokemon, attack: &Move, field: &Field) -> u16 {
//         let move_data = attack.data();
//         let term = ((2f64 * f64::from(user.pokemon.level)) / 5f64) + 2f64;
//         move_data.effects
//     }
// }