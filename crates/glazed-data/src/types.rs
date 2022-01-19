use std::ops::Mul;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Represents the Type of a Pokemon
#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
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

impl Type {
    /// Determine the effectiveness of this type trying to hit another type
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
            },
            Type::Flying => match defending_type {
                Type::Fighting => Effectiveness::DOUBLE,
                Type::Rock => Effectiveness::HALF,
                Type::Bug => Effectiveness::DOUBLE,
                Type::Steel => Effectiveness::HALF,
                Type::Grass => Effectiveness::DOUBLE,
                Type::Electric => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Poison => match defending_type {
                Type::Poison => Effectiveness::HALF,
                Type::Ground => Effectiveness::HALF,
                Type::Rock => Effectiveness::HALF,
                Type::Ghost => Effectiveness::HALF,
                Type::Steel => Effectiveness::Immune,
                Type::Grass => Effectiveness::DOUBLE,
                Type::Fairy => Effectiveness::DOUBLE,
                _ => Effectiveness::NORMAL
            },
            Type::Ground => match defending_type {
                Type::Flying => Effectiveness::Immune,
                Type::Poison => Effectiveness::DOUBLE,
                Type::Rock => Effectiveness::DOUBLE,
                Type::Bug => Effectiveness::HALF,
                Type::Steel => Effectiveness::DOUBLE,
                Type::Fire => Effectiveness::DOUBLE,
                Type::Grass => Effectiveness::HALF,
                Type::Electric => Effectiveness::DOUBLE,
                _ => Effectiveness::NORMAL
            },
            Type::Rock => match defending_type {
                Type::Fighting => Effectiveness::HALF,
                Type::Flying => Effectiveness::DOUBLE,
                Type::Ground => Effectiveness::HALF,
                Type::Bug => Effectiveness::DOUBLE,
                Type::Steel => Effectiveness::HALF,
                Type::Fire => Effectiveness::DOUBLE,
                Type::Ice => Effectiveness::DOUBLE,
                _ => Effectiveness::NORMAL
            },
            Type::Bug => match defending_type {
                Type::Fighting => Effectiveness::HALF,
                Type::Flying => Effectiveness::HALF,
                Type::Poison => Effectiveness::HALF,
                Type::Ghost => Effectiveness::HALF,
                Type::Steel => Effectiveness::HALF,
                Type::Fire => Effectiveness::HALF,
                Type::Grass => Effectiveness::DOUBLE,
                Type::Psychic => Effectiveness::DOUBLE,
                Type::Dark => Effectiveness::DOUBLE,
                Type::Fairy => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Ghost => match defending_type {
                Type::Normal => Effectiveness::Immune,
                Type::Ghost => Effectiveness::DOUBLE,
                Type::Psychic => Effectiveness::DOUBLE,
                Type::Dark => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Steel => match defending_type {
                Type::Rock => Effectiveness::DOUBLE,
                Type::Steel => Effectiveness::HALF,
                Type::Fire => Effectiveness::HALF,
                Type::Water => Effectiveness::HALF,
                Type::Electric => Effectiveness::HALF,
                Type::Ice => Effectiveness::DOUBLE,
                Type::Fairy => Effectiveness::DOUBLE,
                _ => Effectiveness::NORMAL
            },
            Type::Fire => match defending_type {
                Type::Rock => Effectiveness::HALF,
                Type::Bug => Effectiveness::DOUBLE,
                Type::Steel => Effectiveness::DOUBLE,
                Type::Fire => Effectiveness::HALF,
                Type::Water => Effectiveness::HALF,
                Type::Grass => Effectiveness::DOUBLE,
                Type::Ice => Effectiveness::DOUBLE,
                Type::Dragon => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Water => match defending_type {
                Type::Ground => Effectiveness::DOUBLE,
                Type::Rock => Effectiveness::DOUBLE,
                Type::Fire => Effectiveness::DOUBLE,
                Type::Water => Effectiveness::HALF,
                Type::Grass => Effectiveness::HALF,
                Type::Dragon => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Grass => match defending_type {
                Type::Flying => Effectiveness::HALF,
                Type::Poison => Effectiveness::HALF,
                Type::Ground => Effectiveness::DOUBLE,
                Type::Rock => Effectiveness::DOUBLE,
                Type::Bug => Effectiveness::HALF,
                Type::Steel => Effectiveness::HALF,
                Type::Fire => Effectiveness::HALF,
                Type::Water => Effectiveness::DOUBLE,
                Type::Grass => Effectiveness::HALF,
                Type::Dragon => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Electric => match defending_type {
                Type::Flying => Effectiveness::DOUBLE,
                Type::Ground => Effectiveness::Immune,
                Type::Water => Effectiveness::DOUBLE,
                Type::Grass => Effectiveness::HALF,
                Type::Electric => Effectiveness::HALF,
                Type::Dragon => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Psychic => match defending_type {
                Type::Fighting => Effectiveness::DOUBLE,
                Type::Poison => Effectiveness::DOUBLE,
                Type::Steel => Effectiveness::HALF,
                Type::Psychic => Effectiveness::HALF,
                Type::Dark => Effectiveness::Immune,
                _ => Effectiveness::NORMAL
            },
            Type::Ice => match defending_type {
                Type::Flying => Effectiveness::DOUBLE,
                Type::Ground => Effectiveness::DOUBLE,
                Type::Steel => Effectiveness::HALF,
                Type::Fire => Effectiveness::HALF,
                Type::Water => Effectiveness::HALF,
                Type::Grass => Effectiveness::DOUBLE,
                Type::Ice => Effectiveness::HALF,
                Type::Dragon => Effectiveness::DOUBLE,
                _ => Effectiveness::NORMAL
            },
            Type::Dragon => match defending_type {
                Type::Steel => Effectiveness::HALF,
                Type::Dragon => Effectiveness::DOUBLE,
                Type::Fairy => Effectiveness::Immune,
                _ => Effectiveness::NORMAL
            },
            Type::Dark => match defending_type {
                Type::Fighting => Effectiveness::HALF,
                Type::Ghost => Effectiveness::DOUBLE,
                Type::Psychic => Effectiveness::DOUBLE,
                Type::Dark => Effectiveness::HALF,
                Type::Fairy => Effectiveness::HALF,
                _ => Effectiveness::NORMAL
            },
            Type::Fairy => match defending_type {
                Type::Fighting => Effectiveness::DOUBLE,
                Type::Poison => Effectiveness::HALF,
                Type::Steel => Effectiveness::HALF,
                Type::Fire => Effectiveness::HALF,
                Type::Dragon => Effectiveness::DOUBLE,
                Type::Dark => Effectiveness::DOUBLE,
                _ => Effectiveness::NORMAL
            }
        }
    }
}

/// Represents the effectiveness of a Move matchup
#[derive(Debug, Copy, Clone)]
pub enum Effectiveness {
    Immune,
    Effect(i8)
}
impl Effectiveness {
    pub const HALF: Effectiveness = Effectiveness::Effect(-1);
    pub const NORMAL: Effectiveness = Effectiveness::Effect(0);
    pub const DOUBLE: Effectiveness = Effectiveness::Effect(1);

    pub fn dont_care(&self) -> Effectiveness {
        match self {
            Effectiveness::Immune => Effectiveness::Immune,
            Effectiveness::Effect(_) => Effectiveness::NORMAL
        }
    }

    pub fn is_immune(&self) -> bool {
        if let Effectiveness::Immune = self {
            true
        } else {
            false
        }
    }

    pub fn is_super_effective(&self) -> bool {
        if let Effectiveness::Effect(i) = self {
            *i > 0
        } else {
            false
        }
    }
}

impl Mul for Effectiveness {
    type Output = Effectiveness;

    fn mul(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (_, Effectiveness::Immune) => Effectiveness::Immune,
            (Effectiveness::Immune, _) => Effectiveness::Immune,
            (Effectiveness::Effect(left_pow), Effectiveness::Effect(right_pow)) => Effectiveness::Effect(left_pow + right_pow)
        }
    }
}

/// Represents the type(s) for a Pokemon
#[derive(Debug, Clone, Copy)]
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

    pub fn is_stab(&self, attack_type: &Type) -> bool {
        match self {
            PokemonType::Single(t) => t == attack_type,
            PokemonType::Double(t1, t2) => t1 == attack_type || t2 == attack_type
        }
    }

    pub fn has_type(&self, attack_type: &Type) -> bool {
        self.is_stab(attack_type)
    }
}