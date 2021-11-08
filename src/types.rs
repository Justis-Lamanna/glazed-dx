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

type TypePair = (Type, Type);

impl Type {
    pub fn attacking(&self, defending_type: &Type) -> Effectiveness {
        match self {
            Type::Normal => match defending_type {
                Type::Rock => Effectiveness::HALF,
                Type::Ghost => Effectiveness::Immune,
                Type::Steel => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Fighting => match defending_type {
                Type::Normal => Effectiveness::DOUBLE,
                Type::Flying => Effectiveness::HALF,
                Type::Poison => Effectiveness::HALF,
                Type::Rock => Effectiveness::DOUBLE,
                Type::Bug => Effectiveness::HALF,
                Type::Ghost => Effectiveness::Immune,
                Type::Steel => Effectiveness::DOUBLE,
                Type::Psychic => Effectiveness::HALF,
                Type::Ice => Effectiveness::DOUBLE,
                Type::Dark => Effectiveness::DOUBLE,
                Type::Fairy => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            }
            _ => Effectiveness::NORMAL
        }
    }
}

#[derive(Debug)]
pub enum Effectiveness {
    Immune,
    Effect(i8)
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

#[derive(Debug)]
pub enum PokemonType {
    Single(Type),
    Double(Type, Type)
}

impl PokemonType {
    pub fn defending_against(&self, attack_type: &Type) -> Effectiveness {
        match self {
            PokemonType::Single(t) => attack_type.attacking(t),
            PokemonType::Double(t1, t2) => attack_type.attacking(t1) * attack_type.attacking(t2)
        }
    }
}