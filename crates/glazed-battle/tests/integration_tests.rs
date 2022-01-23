use glazed_battle::*;
use glazed_data::attack::{BattleStat, Move, Weather};
use glazed_data::attack::StatChangeTarget::Target;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;

const FORWARD: SlotId = SlotId {
    side: BattleSideId::Forward,
    individual: DoubleBattleSideId::Left
};
const BACK: SlotId = SlotId {
    side: BattleSideId::Back,
    individual: DoubleBattleSideId::Left
};

fn create_battlefield() -> Battlefield {
    Battlefield::one_v_one(
        PokemonTemplate::pokemon(Species::Quilava, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)
    )
}

fn create_specific_battlefield(sp1: Species, sp2: Species) -> Battlefield {
    Battlefield::one_v_one(
        PokemonTemplate::pokemon(sp1, 20),
        PokemonTemplate::pokemon(sp2, 20)
    )
}

#[test]
fn test_protect() {
    let mut b = create_battlefield();
    // Sets up protection
    let fx = b.do_attack(FORWARD, Move::Protect, SelectedTarget::Implied);

    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::StartProtection(_, _)) => true,
            _ => false
        }
    });

    // Attempted attack fails
    let fx = b.do_attack(BACK, Move::Tackle, SelectedTarget::Implied);

    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::IsProtected(_, _)) => true,
            _ => false
        }
    });
}

#[test]
fn test_belly_drum() {
    let mut b = create_battlefield();
    // Cuts HP by 50% Max, increases Attack to +6
    let fx = b.do_attack(FORWARD, Move::BellyDrum, SelectedTarget::Implied);

    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::BasicDamage {..}) => true, _ => false
        }
    });

    assert!({
        match fx.get(1) {
            Some(ActionSideEffects::StatMaxed(_, BattleStat::Attack)) => true, _ => false
        }
    });
}

#[test]
fn test_belly_drum_failure() {
    let mut b = create_battlefield();
    b.get_by_id(&FORWARD).borrow_mut().current_hp = 1;
    // Fails if <50% health
    let fx = b.do_attack(FORWARD, Move::BellyDrum, SelectedTarget::Implied);

    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::Failed(_)) => true, _ => false
        }
    });
}

#[test]
fn test_spikes() {
    let mut b = create_battlefield();
    // First layer of spikes
    let fx = b.do_attack(FORWARD, Move::Spikes, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::EntryHazard(_, _, 1)) => true, _ => false
        }
    });

    // Second layers of spikes
    let fx = b.do_attack(FORWARD, Move::Spikes, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::EntryHazard(_, _, 2)) => true, _ => false
        }
    });

    // Third layer of spikes
    let fx = b.do_attack(FORWARD, Move::Spikes, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::EntryHazard(_, _, 3)) => true, _ => false
        }
    });

    // Attempted fourth layer fails
    let fx = b.do_attack(FORWARD, Move::Spikes, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::Failed(_)) => true, _ => false
        }
    });
}

#[test]
fn test_spikes_on_entry() {
    let mut b = create_battlefield();
    b.do_attack(FORWARD, Move::Spikes, SelectedTarget::Implied);
    let fx = b.enter_battle(BACK);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::BasicDamage {..}) => true, _ => false
        }
    });
}

#[test]
fn test_foresight() {
    let mut b = create_specific_battlefield(Species::Quilava, Species::Gastly);
    // Gastly is immune to Tackle
    let fx = b.do_attack(FORWARD, Move::Tackle, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::NoEffect(_)) => true, _ => false
        }
    });

    // Foresight on Gastly
    let fx = b.do_attack(FORWARD, Move::Foresight, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::Foresighted{..}) => true, _ => false
        }
    });

    // Gastly is no longer immune to Tackle
    let fx = b.do_attack(FORWARD, Move::Tackle, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::NoEffect(_)) => false, _ => true
        }
    });
}

#[test]
fn test_destiny_bond() {
    // Insanely OP Pokemon to force a KO
    let mut b = Battlefield::one_v_one(PokemonTemplate::pokemon(Species::Quilava, 20), PokemonTemplate::pokemon(Species::Lugia, 100));
    b.do_attack(FORWARD, Move::DestinyBond, SelectedTarget::Implied);
    let fx = b.do_attack(BACK, Move::WaterGun, SelectedTarget::Implied);
    assert!({
        match fx.get(1) {
            Some(ActionSideEffects::BasicDamage {cause: Cause::MoveSideEffect(Move::DestinyBond), ..}) => true, _ => false
        }
    });
}

#[test]
fn test_perish_song() {
    let mut b = Battlefield::one_v_one(PokemonTemplate::pokemon(Species::Quilava, 20), PokemonTemplate::pokemon(Species::Lugia, 100));
    let fx = b.do_attack(FORWARD, Move::PerishSong, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::StartPerishSong) => true, _ => false
        }
    });

    let fx = b.end_of_round();
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::PerishCount(_, 2)) => true, _ => false
        }
    });

    let fx = b.end_of_round();
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::PerishCount(_, 1)) => true, _ => false
        }
    });

    let fx = b.end_of_round();
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::PerishCount(_, 0)) => true, _ => false
        }
    });
    assert!({
        match fx.get(1) {
            Some(ActionSideEffects::BasicDamage {end_hp: 0, ..}) => true, _ => false
        }
    });
}

#[test]
fn test_sandstorm() {
    let mut b = create_battlefield();
    let fx = b.do_attack(FORWARD, Move::Sandstorm, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::StartWeather(Weather::Sandstorm)) => true, _ => false
        }
    });

    let fx = b.do_weather();
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::ContinueWeather(Weather::Sandstorm)) => true, _ => false
        }
    });
    assert!({
        match fx.get(1) {
            Some(ActionSideEffects::BasicDamage {..}) => true, _ => false
        }
    });
}

#[test]
fn test_sandstorm_immune() {
    let mut b = create_specific_battlefield(Species::Aron, Species::Onix);
    let fx = b.do_attack(FORWARD, Move::Sandstorm, SelectedTarget::Implied);
    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::StartWeather(Weather::Sandstorm)) => true, _ => false
        }
    });

    let fx = b.do_weather();
    assert_eq!(fx.len(), 1);
}