use glazed_battle::*;
use glazed_data::attack::Move;
use glazed_data::attack::StatChangeTarget::Target;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;

const FORWARD: Battler = Battler {
    side: BattleSide::Forward,
    individual: DoubleBattleSide::Left
};
const BACK: Battler = Battler {
    side: BattleSide::Back,
    individual: DoubleBattleSide::Left
};

fn create_battlefield() -> Battlefield {
    Battlefield::one_v_one(
        PokemonTemplate::pokemon(Species::Quilava, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)
    )
}

#[test]
fn test_protect() {
    let mut b = create_battlefield();
    let fx = b.do_attack(FORWARD, Move::Protect, SelectedTarget::Implied);

    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::StartProtection(_, _)) => true,
            _ => false
        }
    });

    let fx = b.do_attack(BACK, Move::Tackle, SelectedTarget::Implied);

    assert!({
        match fx.get(0) {
            Some(ActionSideEffects::IsProtected(_, _)) => true,
            _ => false
        }
    });
}