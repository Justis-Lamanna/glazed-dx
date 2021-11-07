use std::collections::HashMap;
use std::ops::Mul;
use crate::types::Effectiveness::Effect;

#[derive(Debug)]
pub enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy
}

#[derive(Debug)]
pub enum Effectiveness {
    Immune,
    Effect(u8)
}

impl Effectiveness {
    pub const HALF: Effectiveness = Effectiveness::Effect(-1);
    pub const NORMAL: Effectiveness = Effectiveness::Effect(0);
    pub const DOUBLE: Effectiveness = Effectiveness::Effect(1);
}

impl Mul for Effectiveness {
    type Output = Effectiveness;

    fn mul(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (_, Effectiveness::Immune) => Effectiveness::Immune,
            (Effectiveness::Immune, _) => Effectiveness::Immune,
            (Effect(left_pow), Effect(right_pow)) => Effectiveness::Effect(left_pow + right_pow)
        }
    }
}

struct TypePair<'a>(&'a Type, &'a Type);

const MATCHUP_TABLE: HashMap<TypePair, Effectiveness> = HashMap::from([
    (TypePair(&Type::Normal, &Type::Rock), Effectiveness::HALF),
    (TypePair(&Type::Normal, &Type::Ghost), Effectiveness::Immune),
    (TypePair(&Type::Normal, &Type::Steel), Effectiveness::HALF),

    (TypePair(&Type::Fighting, &Type::Normal), Effectiveness::DOUBLE),
    (TypePair(&Type::Fighting, &Type::Flying), Effectiveness::HALF),
    (TypePair(&Type::Fighting, &Type::Poison), Effectiveness::HALF),
    (TypePair(&Type::Fighting, &Type::Rock), Effectiveness::DOUBLE),
    (TypePair(&Type::Fighting, &Type::Bug), Effectiveness::HALF),
    (TypePair(&Type::Fighting, &Type::Ghost), Effectiveness::Immune),
    (TypePair(&Type::Fighting, &Type::Steel), Effectiveness::DOUBLE),
    (TypePair(&Type::Fighting, &Type::Psychic), Effectiveness::HALF),
    (TypePair(&Type::Fighting, &Type::Ice), Effectiveness::DOUBLE),
    (TypePair(&Type::Fighting, &Type::Dark), Effectiveness::DOUBLE),
    (TypePair(&Type::Fighting, &Type::Fairy), Effectiveness::HALF)
    //More...
]);

#[derive(Debug)]
pub enum PokemonType {
    Single(Type),
    Double(Type, Type)
}

impl PokemonType {
    pub fn get_effectiveness(&self, attack_type: &Type) -> Effectiveness {
        match self {
            PokemonType::Single(t) => {
                let m = TypePair(attack_type, t);
                match MATCHUP_TABLE.get(&m) {
                    None => Effectiveness::NORMAL,
                    Some(e) => e
                }
            },
            PokemonType::Double(t1, t2) => {
                let m1 = match MATCHUP_TABLE.get(&TypePair(attack_type, t1)) {
                    None => Effectiveness::NORMAL,
                    Some(e) => e
                };
                let m2 = match MATCHUP_TABLE.get(&TypePair(attack_type, t2)) {
                    None => Effectiveness::NORMAL,
                    Some(e) => e
                };
                m1 * m2
            }
        }
    }
}