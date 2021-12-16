// use crate::{BattleData, Battlefield, BattlePokemon, Battler, BattleTypeTrait, EntryHazard, Field, Party, Side};
// use crate::double::DoubleTurnAction;
//
// /// One side of battle in a tag battle (two trainers, one pokemon each)
// #[derive(Debug)]
// pub struct TagBattleSide {
//     party: (Party, Party),
//     current_out: (u8, u8),
//     current_inflictions: (BattleData, BattleData),
//     side: Side
// }
// impl TagBattleSide {
//     pub fn start(party_left: Party, party_right: Party) -> TagBattleSide {
//         TagBattleSide {
//             party: (party_left, party_right),
//             current_out: (0, 0),
//             current_inflictions: (BattleData::default(), BattleData::default()),
//             side: Side::default()
//         }
//     }
// }
// impl From<(Party, Party)> for TagBattleSide {
//     fn from(pair: (Party, Party)) -> Self {
//         TagBattleSide::start(pair.0, pair.1)
//     }
// }
// impl BattleTypeTrait for TagBattleSide {
//     fn get_by_id(&self, id: &Battler) -> Option<BattlePokemon> {
//         let Battler(_, slot) = id;
//         match slot {
//             0 => {
//                 let (party, _) = &self.party;
//                 let (idx, _) = self.current_out;
//                 let (affl, _) = &self.current_inflictions;
//                 let pkmn = party.members[usize::from(idx)].as_ref();
//                 match pkmn {
//                     Some(p) => Some(BattlePokemon {
//                         id: id.clone(),
//                         pokemon: p,
//                         battle_data: affl
//                     }),
//                     None => None
//                 }
//             },
//             _ => {
//                 let (_, party) = &self.party;
//                 let (_, idx) = self.current_out;
//                 let (_, affl) = &self.current_inflictions;
//                 let pkmn = party.members[usize::from(idx)].as_ref();
//                 match pkmn {
//                     Some(p) => Some(BattlePokemon {
//                         id: id.clone(),
//                         pokemon: p,
//                         battle_data: affl
//                     }),
//                     None => None
//                 }
//             }
//         }
//     }
//
//     fn get_side(&self) -> &Side {
//         &self.side
//     }
// }
//
// impl Battlefield<TagBattleSide> {
//     pub fn tag_battle<F>(user: F, opponent: F) -> Battlefield<TagBattleSide> where
//         F: Into<TagBattleSide>
//     {
//         Battlefield {
//             user: user.into(),
//             opponent: opponent.into(),
//             field: Field::default(),
//             turn_record: Vec::new()
//         }
//     }
//
//     pub fn do_turn(&self, user_left: DoubleTurnAction, user_right: DoubleTurnAction, opponent_left: DoubleTurnAction, opponent_right: DoubleTurnAction) -> () {
//
//     }
// }