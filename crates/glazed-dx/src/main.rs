use glazed_battle::{Battlefield, SlotId, BattleSideId, Party};
use glazed_battle::SelectedTarget;
use glazed_data::attack::Move;
use glazed_data::constants::Species;
use glazed_data::item::Berry;
use glazed_data::pokemon::PokemonTemplate;

fn main() {
    // for _ in 1..=40 {
    let me = Party::create(vec![
        PokemonTemplate::pokemon(Species::Quilava, 20).shiny(),
        PokemonTemplate::pokemon(Species::Eevee, 20),
        PokemonTemplate::pokemon(Species::Buizel, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)]
    );
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20).holding(Berry::SitrusBerry));

    let mut battlefield = Battlefield::single_battle(me, them);

    let fx = battlefield.do_attack(SlotId::single(BattleSideId::Back), Move::Protect, SelectedTarget::Implied);
    println!("{:#?}", fx);
    let fx = battlefield.do_attack(SlotId::single(BattleSideId::Back), Move::Protect, SelectedTarget::Implied);
    println!("{:#?}", fx);
    let fx = battlefield.do_attack(SlotId::single(BattleSideId::Back), Move::Protect, SelectedTarget::Implied);
    println!("{:#?}", fx);
    let fx = battlefield.do_attack(SlotId::single(BattleSideId::Back), Move::Protect, SelectedTarget::Implied);
    println!("{:#?}", fx);
    let fx = battlefield.do_attack(SlotId::single(BattleSideId::Forward), Move::Tackle, SelectedTarget::Implied);
    println!("{:#?}", fx);
    // let fx = battlefield.end_of_round();
    // println!("{:#?}", fx);
    // }
}
