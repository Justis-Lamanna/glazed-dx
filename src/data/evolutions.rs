use crate::data::attack::Move;
use crate::data::constants::Species;
use crate::data::item::{EvolutionHeldItem, EvolutionStone, Incense};
use crate::data::types::Type;

/// Represents evolution + breeding data for a species
/// Breeding rules:
/// 1. if `baby`, and parent is holding `baby.incense`, offspring is `baby.species`
/// 2. else, offspring is `base`
pub struct Evolution {
    pub base: Species,
    pub baby: Option<IncenseBaby>,
    pub paths: &'static[EvolutionPath]
}

/// Represents a baby Pokemon that can be obtained by breeding while holding an incense
pub struct IncenseBaby {
    pub species: Species,
    pub incense: Incense
}

/// Represents one possible evolution for this species
pub struct EvolutionPath {
    pub to: Species,
    pub trigger: EvolutionTrigger
}

/// Represents all evolution triggers accepted by the game
pub enum EvolutionTrigger {
    AtLevel(u8),
    AtLevelWithGender(u8, EvolutionTriggerGender),
    AtLevelSpawn(u8, Species),//
    AtLevelRandomly(u8, EvolutionTriggerRandom),//
    AtLevelWithHighStat(u8, EvolutionTriggerStats),
    HighFriendship,
    HighFriendshipAtTime(EvolutionTriggerTime),
    HighFriendshipWithMoveType(Type),
    KnowsMove(Move),
    AtLocation(EvolutionTriggerLocation),
    HoldingItemAtTime(EvolutionHeldItem, EvolutionTriggerTime),
    WithPartyPokemon(Species),
    HighBeauty,
    EvolutionStone(EvolutionStone),
    EvolutionStoneWithGender(EvolutionStone, EvolutionTriggerGender),
    Trading,
    TradingWithItem(EvolutionHeldItem),
    TradingForPokemon(Species)
}

/// Represents a type of location that can trigger Evolution
pub enum EvolutionTriggerLocation {
    MossRock,
    IceRock,
    MagneticField
}

/// Represents a time of day that can trigger Evolution
pub enum EvolutionTriggerTime {
    Day,
    Night
}

/// Represents a gender that can trigger Evolution
pub enum EvolutionTriggerGender {
    Male,
    Female
}

/// Represents a trigger that chooses a random Evolution based on Personality
pub enum EvolutionTriggerRandom {
    Low,
    High
}

/// Represents a trigger depending on a high stat
pub enum EvolutionTriggerStats {
    Attack,
    Defense,
    Equal
}

#[allow(non_upper_case_globals)]
impl Species {
    pub fn get_evolutions(&self) -> Evolution {
        match self {
            Bulbasaur => Evolution {
                base: Species::Bulbasaur,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ivysaur, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Ivysaur => Evolution {
                base: Species::Bulbasaur,
                baby: None,
                paths: &[EvolutionPath { to: Species::Venusaur, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Venusaur => Evolution {
                base: Species::Bulbasaur,
                baby: None,
                paths: &[]
            },
            Charmander => Evolution {
                base: Species::Charmander,
                baby: None,
                paths: &[EvolutionPath { to: Species::Charmeleon, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Charmeleon => Evolution {
                base: Species::Charmander,
                baby: None,
                paths: &[EvolutionPath { to: Species::Charizard, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Charizard => Evolution {
                base: Species::Charmander,
                baby: None,
                paths: &[]
            },
            Squirtle => Evolution {
                base: Species::Squirtle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Wartortle, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Wartortle => Evolution {
                base: Species::Squirtle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Blastoise, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Blastoise => Evolution {
                base: Species::Squirtle,
                baby: None,
                paths: &[]
            },
            Caterpie => Evolution {
                base: Species::Caterpie,
                baby: None,
                paths: &[EvolutionPath { to: Species::Metapod, trigger: EvolutionTrigger::AtLevel(7) }]
            },
            Metapod => Evolution {
                base: Species::Caterpie,
                baby: None,
                paths: &[EvolutionPath { to: Species::Butterfree, trigger: EvolutionTrigger::AtLevel(10) }]
            },
            Butterfree => Evolution {
                base: Species::Caterpie,
                baby: None,
                paths: &[]
            },
            Weedle => Evolution {
                base: Species::Weedle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kakuna, trigger: EvolutionTrigger::AtLevel(7) }]
            },
            Kakuna => Evolution {
                base: Species::Weedle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Beedrill, trigger: EvolutionTrigger::AtLevel(10) }]
            },
            Beedrill => Evolution {
                base: Species::Weedle,
                baby: None,
                paths: &[]
            },
            Pidgey => Evolution {
                base: Species::Pidgey,
                baby: None,
                paths: &[EvolutionPath { to: Species::Pidgeotto, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Pidgeotto => Evolution {
                base: Species::Pidgey,
                baby: None,
                paths: &[EvolutionPath { to: Species::Pidgeot, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Pidgeot => Evolution {
                base: Species::Pidgey,
                baby: None,
                paths: &[]
            },
            Rattata => Evolution {
                base: Species::Rattata,
                baby: None,
                paths: &[EvolutionPath { to: Species::Raticate, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Raticate => Evolution {
                base: Species::Rattata,
                baby: None,
                paths: &[]
            },
            Spearow => Evolution {
                base: Species::Spearow,
                baby: None,
                paths: &[EvolutionPath { to: Species::Fearow, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Fearow => Evolution {
                base: Species::Spearow,
                baby: None,
                paths: &[]
            },
            Ekans => Evolution {
                base: Species::Ekans,
                baby: None,
                paths: &[EvolutionPath { to: Species::Arbok, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Arbok => Evolution {
                base: Species::Ekans,
                baby: None,
                paths: &[]
            },
            Pichu => Evolution {
                base: Species::Pichu,
                baby: None,
                paths: &[EvolutionPath { to: Species::Pikachu, trigger: EvolutionTrigger::HighFriendship }]
            },
            Pikachu => Evolution {
                base: Species::Pichu,
                baby: None,
                paths: &[EvolutionPath { to: Species::Raichu, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
            },
            Raichu => Evolution {
                base: Species::Pichu,
                baby: None,
                paths: &[]
            },
            Sandshrew => Evolution {
                base: Species::Sandshrew,
                baby: None,
                paths: &[EvolutionPath { to: Species::Sandslash, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Sandslash => Evolution {
                base: Species::Sandshrew,
                baby: None,
                paths: &[]
            },
            NidoranF => Evolution {
                base: Species::NidoranF,
                baby: None,
                paths: &[EvolutionPath { to: Species::Nidorina, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Nidorina => Evolution {
                base: Species::NidoranF,
                baby: None,
                paths: &[EvolutionPath { to: Species::Nidoqueen, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
            },
            Nidoqueen => Evolution {
                base: Species::NidoranF,
                baby: None,
                paths: &[]
            },
            NidoranM => Evolution {
                base: Species::NidoranM,
                baby: None,
                paths: &[EvolutionPath { to: Species::Nidorino, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Nidorino => Evolution {
                base: Species::NidoranM,
                baby: None,
                paths: &[EvolutionPath { to: Species::Nidoking, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
            },
            Nidoking => Evolution {
                base: Species::NidoranM,
                baby: None,
                paths: &[]
            },
            Cleffa => Evolution {
                base: Species::Cleffa,
                baby: None,
                paths: &[EvolutionPath { to: Species::Clefairy, trigger: EvolutionTrigger::HighFriendship }]
            },
            Clefairy => Evolution {
                base: Species::Cleffa,
                baby: None,
                paths: &[EvolutionPath { to: Species::Clefable, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
            },
            Clefable => Evolution {
                base: Species::Cleffa,
                baby: None,
                paths: &[]
            },
            Vulpix => Evolution {
                base: Species::Vulpix,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ninetales, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }]
            },
            Ninetales => Evolution {
                base: Species::Vulpix,
                baby: None,
                paths: &[]
            },
            Igglybuff => Evolution {
                base: Species::Igglybuff,
                baby: None,
                paths: &[EvolutionPath { to: Species::Jigglypuff, trigger: EvolutionTrigger::HighFriendship }]
            },
            Jigglypuff => Evolution {
                base: Species::Igglybuff,
                baby: None,
                paths: &[EvolutionPath { to: Species::Wigglytuff, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
            },
            Wigglytuff => Evolution {
                base: Species::Igglybuff,
                baby: None,
                paths: &[]
            },
            Zubat => Evolution {
                base: Species::Zubat,
                baby: None,
                paths: &[EvolutionPath { to: Species::Golbat, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Golbat => Evolution {
                base: Species::Zubat,
                baby: None,
                paths: &[EvolutionPath { to: Species::Crobat, trigger: EvolutionTrigger::HighFriendship }]
            },
            Crobat => Evolution {
                base: Species::Zubat,
                baby: None,
                paths: &[]
            },
            Oddish => Evolution {
                base: Species::Oddish,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gloom, trigger: EvolutionTrigger::AtLevel(21) }]
            },
            Gloom => Evolution {
                base: Species::Oddish,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vileplume, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }, EvolutionPath { to: Species::Bellossom, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
            },
            Vileplume => Evolution {
                base: Species::Oddish,
                baby: None,
                paths: &[]
            },
            Bellossom => Evolution {
                base: Species::Oddish,
                baby: None,
                paths: &[]
            },
            Paras => Evolution {
                base: Species::Paras,
                baby: None,
                paths: &[EvolutionPath { to: Species::Parasect, trigger: EvolutionTrigger::AtLevel(24) }]
            },
            Parasect => Evolution {
                base: Species::Paras,
                baby: None,
                paths: &[]
            },
            Venonat => Evolution {
                base: Species::Venonat,
                baby: None,
                paths: &[EvolutionPath { to: Species::Venomoth, trigger: EvolutionTrigger::AtLevel(31) }]
            },
            Venomoth => Evolution {
                base: Species::Venonat,
                baby: None,
                paths: &[]
            },
            Diglett => Evolution {
                base: Species::Diglett,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dugtrio, trigger: EvolutionTrigger::AtLevel(26) }]
            },
            Dugtrio => Evolution {
                base: Species::Diglett,
                baby: None,
                paths: &[]
            },
            Meowth => Evolution {
                base: Species::Meowth,
                baby: None,
                paths: &[EvolutionPath { to: Species::Persian, trigger: EvolutionTrigger::AtLevel(28) }, EvolutionPath { to: Species::Perrserker, trigger: EvolutionTrigger::AtLevel(28) }]
            },
            Persian => Evolution {
                base: Species::Meowth,
                baby: None,
                paths: &[]
            },
            Perrserker => Evolution {
                base: Species::Meowth,
                baby: None,
                paths: &[]
            },
            Psyduck => Evolution {
                base: Species::Psyduck,
                baby: None,
                paths: &[EvolutionPath { to: Species::Golduck, trigger: EvolutionTrigger::AtLevel(33) }]
            },
            Golduck => Evolution {
                base: Species::Psyduck,
                baby: None,
                paths: &[]
            },
            Mankey => Evolution {
                base: Species::Mankey,
                baby: None,
                paths: &[EvolutionPath { to: Species::Primeape, trigger: EvolutionTrigger::AtLevel(28) }]
            },
            Primeape => Evolution {
                base: Species::Mankey,
                baby: None,
                paths: &[]
            },
            Growlithe => Evolution {
                base: Species::Growlithe,
                baby: None,
                paths: &[EvolutionPath { to: Species::Arcanine, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }]
            },
            Arcanine => Evolution {
                base: Species::Growlithe,
                baby: None,
                paths: &[]
            },
            Poliwag => Evolution {
                base: Species::Poliwag,
                baby: None,
                paths: &[EvolutionPath { to: Species::Poliwhirl, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Poliwhirl => Evolution {
                base: Species::Poliwag,
                baby: None,
                paths: &[EvolutionPath { to: Species::Poliwrath, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }, EvolutionPath { to: Species::Politoed, trigger: EvolutionTrigger::TradingWithItem(Item::KingsRock) }]
            },
            Poliwrath => Evolution {
                base: Species::Poliwag,
                baby: None,
                paths: &[]
            },
            Politoed => Evolution {
                base: Species::Poliwag,
                baby: None,
                paths: &[]
            },
            Abra => Evolution {
                base: Species::Abra,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kadabra, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Kadabra => Evolution {
                base: Species::Abra,
                baby: None,
                paths: &[EvolutionPath { to: Species::Alakazam, trigger: EvolutionTrigger::Trading }]
            },
            Alakazam => Evolution {
                base: Species::Abra,
                baby: None,
                paths: &[]
            },
            Machop => Evolution {
                base: Species::Machop,
                baby: None,
                paths: &[EvolutionPath { to: Species::Machoke, trigger: EvolutionTrigger::AtLevel(28) }]
            },
            Machoke => Evolution {
                base: Species::Machop,
                baby: None,
                paths: &[EvolutionPath { to: Species::Machamp, trigger: EvolutionTrigger::Trading }]
            },
            Machamp => Evolution {
                base: Species::Machop,
                baby: None,
                paths: &[]
            },
            Bellsprout => Evolution {
                base: Species::Bellsprout,
                baby: None,
                paths: &[EvolutionPath { to: Species::Weepinbell, trigger: EvolutionTrigger::AtLevel(21) }]
            },
            Weepinbell => Evolution {
                base: Species::Bellsprout,
                baby: None,
                paths: &[EvolutionPath { to: Species::Victreebel, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
            },
            Victreebel => Evolution {
                base: Species::Bellsprout,
                baby: None,
                paths: &[]
            },
            Tentacool => Evolution {
                base: Species::Tentacool,
                baby: None,
                paths: &[EvolutionPath { to: Species::Tentacruel, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Tentacruel => Evolution {
                base: Species::Tentacool,
                baby: None,
                paths: &[]
            },
            Geodude => Evolution {
                base: Species::Geodude,
                baby: None,
                paths: &[EvolutionPath { to: Species::Graveler, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Graveler => Evolution {
                base: Species::Geodude,
                baby: None,
                paths: &[EvolutionPath { to: Species::Golem, trigger: EvolutionTrigger::Trading }]
            },
            Golem => Evolution {
                base: Species::Geodude,
                baby: None,
                paths: &[]
            },
            Ponyta => Evolution {
                base: Species::Ponyta,
                baby: None,
                paths: &[EvolutionPath { to: Species::Rapidash, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Rapidash => Evolution {
                base: Species::Ponyta,
                baby: None,
                paths: &[]
            },
            Slowpoke => Evolution {
                base: Species::Slowpoke,
                baby: None,
                paths: &[EvolutionPath { to: Species::Slowbro, trigger: EvolutionTrigger::AtLevel(37) }, EvolutionPath { to: Species::Slowking, trigger: EvolutionTrigger::TradingWithItem(Item::KingsRock) }]
            },
            Slowbro => Evolution {
                base: Species::Slowpoke,
                baby: None,
                paths: &[]
            },
            Slowking => Evolution {
                base: Species::Slowpoke,
                baby: None,
                paths: &[]
            },
            Magnemite => Evolution {
                base: Species::Magnemite,
                baby: None,
                paths: &[EvolutionPath { to: Species::Magneton, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Magneton => Evolution {
                base: Species::Magnemite,
                baby: None,
                paths: &[EvolutionPath { to: Species::Magnezone, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::MagneticField) }, EvolutionPath { to: Species::Magnezone, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
            },
            Magnezone => Evolution {
                base: Species::Magnemite,
                baby: None,
                paths: &[]
            },
            Farfetchd => Evolution {
                base: Species::Farfetchd,
                baby: None,
                paths: &[]
            },
            Sirfetchd => Evolution {
                base: Species::Farfetchd,
                baby: None,
                paths: &[]
            },
            Doduo => Evolution {
                base: Species::Doduo,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dodrio, trigger: EvolutionTrigger::AtLevel(31) }]
            },
            Dodrio => Evolution {
                base: Species::Doduo,
                baby: None,
                paths: &[]
            },
            Seel => Evolution {
                base: Species::Seel,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dewgong, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Dewgong => Evolution {
                base: Species::Seel,
                baby: None,
                paths: &[]
            },
            Grimer => Evolution {
                base: Species::Grimer,
                baby: None,
                paths: &[EvolutionPath { to: Species::Muk, trigger: EvolutionTrigger::AtLevel(38) }]
            },
            Muk => Evolution {
                base: Species::Grimer,
                baby: None,
                paths: &[]
            },
            Shellder => Evolution {
                base: Species::Shellder,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cloyster, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
            },
            Cloyster => Evolution {
                base: Species::Shellder,
                baby: None,
                paths: &[]
            },
            Gastly => Evolution {
                base: Species::Gastly,
                baby: None,
                paths: &[EvolutionPath { to: Species::Haunter, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Haunter => Evolution {
                base: Species::Gastly,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gengar, trigger: EvolutionTrigger::Trading }]
            },
            Gengar => Evolution {
                base: Species::Gastly,
                baby: None,
                paths: &[]
            },
            Onix => Evolution {
                base: Species::Onix,
                baby: None,
                paths: &[EvolutionPath { to: Species::Steelix, trigger: EvolutionTrigger::TradingWithItem(Item::MetalCoat) }]
            },
            Steelix => Evolution {
                base: Species::Onix,
                baby: None,
                paths: &[]
            },
            Drowzee => Evolution {
                base: Species::Drowzee,
                baby: None,
                paths: &[EvolutionPath { to: Species::Hypno, trigger: EvolutionTrigger::AtLevel(26) }]
            },
            Hypno => Evolution {
                base: Species::Drowzee,
                baby: None,
                paths: &[]
            },
            Krabby => Evolution {
                base: Species::Krabby,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kingler, trigger: EvolutionTrigger::AtLevel(28) }]
            },
            Kingler => Evolution {
                base: Species::Krabby,
                baby: None,
                paths: &[]
            },
            Voltorb => Evolution {
                base: Species::Voltorb,
                baby: None,
                paths: &[EvolutionPath { to: Species::Electrode, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Electrode => Evolution {
                base: Species::Voltorb,
                baby: None,
                paths: &[]
            },
            Exeggcute => Evolution {
                base: Species::Exeggcute,
                baby: None,
                paths: &[EvolutionPath { to: Species::Exeggutor, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
            },
            Exeggutor => Evolution {
                base: Species::Exeggcute,
                baby: None,
                paths: &[]
            },
            Cubone => Evolution {
                base: Species::Cubone,
                baby: None,
                paths: &[EvolutionPath { to: Species::Marowak, trigger: EvolutionTrigger::AtLevel(28) }]
            },
            Marowak => Evolution {
                base: Species::Cubone,
                baby: None,
                paths: &[]
            },
            Tyrogue => Evolution {
                base: Species::Tyrogue,
                baby: None,
                paths: &[EvolutionPath { to: Species::Hitmonlee, trigger: EvolutionTrigger::AtLevelWithHighStat(20, EvolutionTriggerStats::Attack) }, EvolutionPath { to: Species::Hitmonchan, trigger: EvolutionTrigger::AtLevelWithHighStat(20, EvolutionTriggerStats::Defense) }, EvolutionPath { to: Species::Hitmontop, trigger: EvolutionTrigger::AtLevelWithHighStat(20, EvolutionTriggerStats::Equal) }]
            },
            Hitmonlee => Evolution {
                base: Species::Tyrogue,
                baby: None,
                paths: &[]
            },
            Hitmonchan => Evolution {
                base: Species::Tyrogue,
                baby: None,
                paths: &[]
            },
            Hitmontop => Evolution {
                base: Species::Tyrogue,
                baby: None,
                paths: &[]
            },
            Lickitung => Evolution {
                base: Species::Lickitung,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lickilicky, trigger: EvolutionTrigger::KnowsMove(Move::Rollout) }]
            },
            Lickilicky => Evolution {
                base: Species::Lickitung,
                baby: None,
                paths: &[]
            },
            Koffing => Evolution {
                base: Species::Koffing,
                baby: None,
                paths: &[EvolutionPath { to: Species::Weezing, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Weezing => Evolution {
                base: Species::Koffing,
                baby: None,
                paths: &[]
            },
            Rhyhorn => Evolution {
                base: Species::Rhyhorn,
                baby: None,
                paths: &[EvolutionPath { to: Species::Rhydon, trigger: EvolutionTrigger::AtLevel(42) }]
            },
            Rhydon => Evolution {
                base: Species::Rhyhorn,
                baby: None,
                paths: &[EvolutionPath { to: Species::Rhyperior, trigger: EvolutionTrigger::TradingWithItem(Item::Protector) }]
            },
            Rhyperior => Evolution {
                base: Species::Rhyhorn,
                baby: None,
                paths: &[]
            },
            Happiny => Evolution {
                base: Species::Chansey,
                baby: Some({species: Species::Happiny, incense: Incense::LuckIncense},
                           paths: &[EvolutionPath { to: Species::Chansey, trigger: EvolutionTrigger::HoldingItemAtTime(EvolutionHeldItem::OvalStone, EvolutionTriggerTime::Day) }]
            },
            Chansey => Evolution {
                base: Species::Chansey,
                baby: Some({species: Species::Happiny, incense: Incense::LuckIncense},
                           paths: &[EvolutionPath { to: Species::Blissey, trigger: EvolutionTrigger::HighFriendship }]
            },
            Blissey => Evolution {
                base: Species::Chansey,
                baby: Some({species: Species::Happiny, incense: Incense::LuckIncense},
                           paths: &[]
            },
            Tangela => Evolution {
                base: Species::Tangela,
                baby: None,
                paths: &[EvolutionPath { to: Species::Tangrowth, trigger: EvolutionTrigger::KnowsMove(Move::AncientPower) }]
            },
            Tangrowth => Evolution {
                base: Species::Tangela,
                baby: None,
                paths: &[]
            },
            Kangaskhan => Evolution {
                base: Species::Kangaskhan,
                baby: None,
                paths: &[]
            },
            Horsea => Evolution {
                base: Species::Horsea,
                baby: None,
                paths: &[EvolutionPath { to: Species::Seadra, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Seadra => Evolution {
                base: Species::Horsea,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kingdra, trigger: EvolutionTrigger::TradingWithItem(Item::DragonScale) }]
            },
            Kingdra => Evolution {
                base: Species::Horsea,
                baby: None,
                paths: &[]
            },
            Goldeen => Evolution {
                base: Species::Goldeen,
                baby: None,
                paths: &[EvolutionPath { to: Species::Seaking, trigger: EvolutionTrigger::AtLevel(33) }]
            },
            Seaking => Evolution {
                base: Species::Goldeen,
                baby: None,
                paths: &[]
            },
            Staryu => Evolution {
                base: Species::Staryu,
                baby: None,
                paths: &[EvolutionPath { to: Species::Starmie, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
            },
            Starmie => Evolution {
                base: Species::Staryu,
                baby: None,
                paths: &[]
            },
            MimeJr => Evolution {
                base: Species::MrMime,
                baby: Some({species: Species::MimeJr, incense: Incense::OddIncense},
                           paths: &[EvolutionPath { to: Species::MrMime, trigger: EvolutionTrigger::KnowsMove(Move::Mimic) }]
            },
            MrMime => Evolution {
                base: Species::MrMime,
                baby: Some({species: Species::MimeJr, incense: Incense::OddIncense},
                           paths: &[EvolutionPath { to: Species::MrRime, trigger: EvolutionTrigger::AtLevel(42) }]
            },
            MrRime => Evolution {
                base: Species::MrMime,
                baby: Some({species: Species::MimeJr, incense: Incense::OddIncense},
                           paths: &[]
            },
            Scyther => Evolution {
                base: Species::Scyther,
                baby: None,
                paths: &[EvolutionPath { to: Species::Scizor, trigger: EvolutionTrigger::TradingWithItem(Item::MetalCoat) }]
            },
            Scizor => Evolution {
                base: Species::Scyther,
                baby: None,
                paths: &[]
            },
            Smoochum => Evolution {
                base: Species::Smoochum,
                baby: None,
                paths: &[EvolutionPath { to: Species::Jynx, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Jynx => Evolution {
                base: Species::Smoochum,
                baby: None,
                paths: &[]
            },
            Elekid => Evolution {
                base: Species::Elekid,
                baby: None,
                paths: &[EvolutionPath { to: Species::Electabuzz, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Electabuzz => Evolution {
                base: Species::Elekid,
                baby: None,
                paths: &[EvolutionPath { to: Species::Electivire, trigger: EvolutionTrigger::TradingWithItem(Item::Electirizer) }]
            },
            Electivire => Evolution {
                base: Species::Elekid,
                baby: None,
                paths: &[]
            },
            Magby => Evolution {
                base: Species::Magby,
                baby: None,
                paths: &[EvolutionPath { to: Species::Magmar, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Magmar => Evolution {
                base: Species::Magby,
                baby: None,
                paths: &[EvolutionPath { to: Species::Magmortar, trigger: EvolutionTrigger::TradingWithItem(Item::Magmarizer) }]
            },
            Magmortar => Evolution {
                base: Species::Magby,
                baby: None,
                paths: &[]
            },
            Pinsir => Evolution {
                base: Species::Pinsir,
                baby: None,
                paths: &[]
            },
            Tauros => Evolution {
                base: Species::Tauros,
                baby: None,
                paths: &[]
            },
            Magikarp => Evolution {
                base: Species::Magikarp,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gyarados, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Gyarados => Evolution {
                base: Species::Magikarp,
                baby: None,
                paths: &[]
            },
            Lapras => Evolution {
                base: Species::Lapras,
                baby: None,
                paths: &[]
            },
            Ditto => Evolution {
                base: Species::Ditto,
                baby: None,
                paths: &[]
            },
            Eevee => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vaporeon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }, EvolutionPath { to: Species::Jolteon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }, EvolutionPath { to: Species::Flareon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }, EvolutionPath { to: Species::Espeon, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Day) }, EvolutionPath { to: Species::Umbreon, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Night) }, EvolutionPath { to: Species::Leafeon, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::MossRock) }, EvolutionPath { to: Species::Leafeon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }, EvolutionPath { to: Species::Glaceon, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::IceRock) }, EvolutionPath { to: Species::Glaceon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::IceStone) }, EvolutionPath { to: Species::Sylveon, trigger: EvolutionTrigger::HighFriendshipWithMoveType(Type::Fairy) }]
            },
            Vaporeon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Jolteon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Flareon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Espeon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Umbreon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Leafeon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Glaceon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Sylveon => Evolution {
                base: Species::Eevee,
                baby: None,
                paths: &[]
            },
            Porygon => Evolution {
                base: Species::Porygon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Porygon2, trigger: EvolutionTrigger::TradingWithItem(Item::UpGrade) }]
            },
            Porygon2 => Evolution {
                base: Species::Porygon,
                baby: None,
                paths: &[EvolutionPath { to: Species::PorygonZ, trigger: EvolutionTrigger::TradingWithItem(Item::DubiousDisc) }]
            },
            PorygonZ => Evolution {
                base: Species::Porygon,
                baby: None,
                paths: &[]
            },
            Omanyte => Evolution {
                base: Species::Omanyte,
                baby: None,
                paths: &[EvolutionPath { to: Species::Omastar, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Omastar => Evolution {
                base: Species::Omanyte,
                baby: None,
                paths: &[]
            },
            Kabuto => Evolution {
                base: Species::Kabuto,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kabutops, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Kabutops => Evolution {
                base: Species::Kabuto,
                baby: None,
                paths: &[]
            },
            Aerodactyl => Evolution {
                base: Species::Aerodactyl,
                baby: None,
                paths: &[]
            },
            Munchlax => Evolution {
                base: Species::Snorlax,
                baby: Some({species: Species::Munchlax, incense: Incense::FullIncense},
                           paths: &[EvolutionPath { to: Species::Snorlax, trigger: EvolutionTrigger::HighFriendship }]
            },
            Snorlax => Evolution {
                base: Species::Snorlax,
                baby: Some({species: Species::Munchlax, incense: Incense::FullIncense},
                           paths: &[]
            },
            Articuno => Evolution {
                base: Species::Articuno,
                baby: None,
                paths: &[]
            },
            Zapdos => Evolution {
                base: Species::Zapdos,
                baby: None,
                paths: &[]
            },
            Moltres => Evolution {
                base: Species::Moltres,
                baby: None,
                paths: &[]
            },
            Dratini => Evolution {
                base: Species::Dratini,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dragonair, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Dragonair => Evolution {
                base: Species::Dratini,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dragonite, trigger: EvolutionTrigger::AtLevel(55) }]
            },
            Dragonite => Evolution {
                base: Species::Dratini,
                baby: None,
                paths: &[]
            },
            Mewtwo => Evolution {
                base: Species::Mewtwo,
                baby: None,
                paths: &[]
            },
            Mew => Evolution {
                base: Species::Mew,
                baby: None,
                paths: &[]
            },
            Chikorita => Evolution {
                base: Species::Chikorita,
                baby: None,
                paths: &[EvolutionPath { to: Species::Bayleef, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Bayleef => Evolution {
                base: Species::Chikorita,
                baby: None,
                paths: &[EvolutionPath { to: Species::Meganium, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Meganium => Evolution {
                base: Species::Chikorita,
                baby: None,
                paths: &[]
            },
            Cyndaquil => Evolution {
                base: Species::Cyndaquil,
                baby: None,
                paths: &[EvolutionPath { to: Species::Quilava, trigger: EvolutionTrigger::AtLevel(14) }]
            },
            Quilava => Evolution {
                base: Species::Cyndaquil,
                baby: None,
                paths: &[EvolutionPath { to: Species::Typhlosion, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Typhlosion => Evolution {
                base: Species::Cyndaquil,
                baby: None,
                paths: &[]
            },
            Totodile => Evolution {
                base: Species::Totodile,
                baby: None,
                paths: &[EvolutionPath { to: Species::Croconaw, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Croconaw => Evolution {
                base: Species::Totodile,
                baby: None,
                paths: &[EvolutionPath { to: Species::Feraligatr, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Feraligatr => Evolution {
                base: Species::Totodile,
                baby: None,
                paths: &[]
            },
            Sentret => Evolution {
                base: Species::Sentret,
                baby: None,
                paths: &[EvolutionPath { to: Species::Furret, trigger: EvolutionTrigger::AtLevel(15) }]
            },
            Furret => Evolution {
                base: Species::Sentret,
                baby: None,
                paths: &[]
            },
            Hoothoot => Evolution {
                base: Species::Hoothoot,
                baby: None,
                paths: &[EvolutionPath { to: Species::Noctowl, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Noctowl => Evolution {
                base: Species::Hoothoot,
                baby: None,
                paths: &[]
            },
            Ledyba => Evolution {
                base: Species::Ledyba,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ledian, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Ledian => Evolution {
                base: Species::Ledyba,
                baby: None,
                paths: &[]
            },
            Spinarak => Evolution {
                base: Species::Spinarak,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ariados, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Ariados => Evolution {
                base: Species::Spinarak,
                baby: None,
                paths: &[]
            },
            Chinchou => Evolution {
                base: Species::Chinchou,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lanturn, trigger: EvolutionTrigger::AtLevel(27) }]
            },
            Lanturn => Evolution {
                base: Species::Chinchou,
                baby: None,
                paths: &[]
            },
            Togepi => Evolution {
                base: Species::Togepi,
                baby: None,
                paths: &[EvolutionPath { to: Species::Togetic, trigger: EvolutionTrigger::HighFriendship }]
            },
            Togetic => Evolution {
                base: Species::Togepi,
                baby: None,
                paths: &[EvolutionPath { to: Species::Togekiss, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ShinyStone) }]
            },
            Togekiss => Evolution {
                base: Species::Togepi,
                baby: None,
                paths: &[]
            },
            Natu => Evolution {
                base: Species::Natu,
                baby: None,
                paths: &[EvolutionPath { to: Species::Xatu, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Xatu => Evolution {
                base: Species::Natu,
                baby: None,
                paths: &[]
            },
            Mareep => Evolution {
                base: Species::Mareep,
                baby: None,
                paths: &[EvolutionPath { to: Species::Flaaffy, trigger: EvolutionTrigger::AtLevel(15) }]
            },
            Flaaffy => Evolution {
                base: Species::Mareep,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ampharos, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Ampharos => Evolution {
                base: Species::Mareep,
                baby: None,
                paths: &[]
            },
            Azurill => Evolution {
                base: Species::Marill,
                baby: Some({species: Species::Azurill, incense: Incense::SeaIncense},
                           paths: &[EvolutionPath { to: Species::Marill, trigger: EvolutionTrigger::HighFriendship }]
            },
            Marill => Evolution {
                base: Species::Marill,
                baby: Some({species: Species::Azurill, incense: Incense::SeaIncense},
                           paths: &[EvolutionPath { to: Species::Azumarill, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Azumarill => Evolution {
                base: Species::Marill,
                baby: Some({species: Species::Azurill, incense: Incense::SeaIncense},
                           paths: &[]
            },
            Bonsly => Evolution {
                base: Species::Sudowoodo,
                baby: Some({species: Species::Bonsly, incense: Incense::RockIncense},
                           paths: &[EvolutionPath { to: Species::Sudowoodo, trigger: EvolutionTrigger::KnowsMove(Move::Mimic) }]
            },
            Sudowoodo => Evolution {
                base: Species::Sudowoodo,
                baby: Some({species: Species::Bonsly, incense: Incense::RockIncense},
                           paths: &[]
            },
            Hoppip => Evolution {
                base: Species::Hoppip,
                baby: None,
                paths: &[EvolutionPath { to: Species::Skiploom, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Skiploom => Evolution {
                base: Species::Hoppip,
                baby: None,
                paths: &[EvolutionPath { to: Species::Jumpluff, trigger: EvolutionTrigger::AtLevel(27) }]
            },
            Jumpluff => Evolution {
                base: Species::Hoppip,
                baby: None,
                paths: &[]
            },
            Aipom => Evolution {
                base: Species::Aipom,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ambipom, trigger: EvolutionTrigger::KnowsMove(Move::DoubleHit) }]
            },
            Ambipom => Evolution {
                base: Species::Aipom,
                baby: None,
                paths: &[]
            },
            Sunkern => Evolution {
                base: Species::Sunkern,
                baby: None,
                paths: &[EvolutionPath { to: Species::Sunflora, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
            },
            Sunflora => Evolution {
                base: Species::Sunkern,
                baby: None,
                paths: &[]
            },
            Yanma => Evolution {
                base: Species::Yanma,
                baby: None,
                paths: &[EvolutionPath { to: Species::Yanmega, trigger: EvolutionTrigger::KnowsMove(Move::AncientPower) }]
            },
            Yanmega => Evolution {
                base: Species::Yanma,
                baby: None,
                paths: &[]
            },
            Wooper => Evolution {
                base: Species::Wooper,
                baby: None,
                paths: &[EvolutionPath { to: Species::Quagsire, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Quagsire => Evolution {
                base: Species::Wooper,
                baby: None,
                paths: &[]
            },
            Murkrow => Evolution {
                base: Species::Murkrow,
                baby: None,
                paths: &[EvolutionPath { to: Species::Honchkrow, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::DuskStone) }]
            },
            Honchkrow => Evolution {
                base: Species::Murkrow,
                baby: None,
                paths: &[]
            },
            Misdreavus => Evolution {
                base: Species::Misdreavus,
                baby: None,
                paths: &[EvolutionPath { to: Species::Mismagius, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::DuskStone) }]
            },
            Mismagius => Evolution {
                base: Species::Misdreavus,
                baby: None,
                paths: &[]
            },
            Unown => Evolution {
                base: Species::Unown,
                baby: None,
                paths: &[]
            },
            Wynaut => Evolution {
                base: Species::Wobbuffet,
                baby: Some({species: Species::Wynaut, incense: Incense::LaxIncense},
                           paths: &[EvolutionPath { to: Species::Wobbuffet, trigger: EvolutionTrigger::AtLevel(15) }]
            },
            Wobbuffet => Evolution {
                base: Species::Wobbuffet,
                baby: Some({species: Species::Wynaut, incense: Incense::LaxIncense},
                           paths: &[]
            },
            Girafarig => Evolution {
                base: Species::Girafarig,
                baby: None,
                paths: &[]
            },
            Pineco => Evolution {
                base: Species::Pineco,
                baby: None,
                paths: &[EvolutionPath { to: Species::Forretress, trigger: EvolutionTrigger::AtLevel(31) }]
            },
            Forretress => Evolution {
                base: Species::Pineco,
                baby: None,
                paths: &[]
            },
            Dunsparce => Evolution {
                base: Species::Dunsparce,
                baby: None,
                paths: &[]
            },
            Gligar => Evolution {
                base: Species::Gligar,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gliscor, trigger: EvolutionTrigger::HoldingItemAtTime(EvolutionHeldItem::RazorFang, EvolutionTriggerTime::Night) }]
            },
            Gliscor => Evolution {
                base: Species::Gligar,
                baby: None,
                paths: &[]
            },
            Snubbull => Evolution {
                base: Species::Snubbull,
                baby: None,
                paths: &[EvolutionPath { to: Species::Granbull, trigger: EvolutionTrigger::AtLevel(23) }]
            },
            Granbull => Evolution {
                base: Species::Snubbull,
                baby: None,
                paths: &[]
            },
            Qwilfish => Evolution {
                base: Species::Qwilfish,
                baby: None,
                paths: &[]
            },
            Shuckle => Evolution {
                base: Species::Shuckle,
                baby: None,
                paths: &[]
            },
            Heracross => Evolution {
                base: Species::Heracross,
                baby: None,
                paths: &[]
            },
            Sneasel => Evolution {
                base: Species::Sneasel,
                baby: None,
                paths: &[EvolutionPath { to: Species::Weavile, trigger: EvolutionTrigger::HoldingItemAtTime(EvolutionHeldItem::RazorClaw, EvolutionTriggerTime::Night) }]
            },
            Weavile => Evolution {
                base: Species::Sneasel,
                baby: None,
                paths: &[]
            },
            Teddiursa => Evolution {
                base: Species::Teddiursa,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ursaring, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Ursaring => Evolution {
                base: Species::Teddiursa,
                baby: None,
                paths: &[]
            },
            Slugma => Evolution {
                base: Species::Slugma,
                baby: None,
                paths: &[EvolutionPath { to: Species::Magcargo, trigger: EvolutionTrigger::AtLevel(38) }]
            },
            Magcargo => Evolution {
                base: Species::Slugma,
                baby: None,
                paths: &[]
            },
            Swinub => Evolution {
                base: Species::Swinub,
                baby: None,
                paths: &[EvolutionPath { to: Species::Piloswine, trigger: EvolutionTrigger::AtLevel(33) }]
            },
            Piloswine => Evolution {
                base: Species::Swinub,
                baby: None,
                paths: &[EvolutionPath { to: Species::Mamoswine, trigger: EvolutionTrigger::KnowsMove(Move::AncientPower) }]
            },
            Mamoswine => Evolution {
                base: Species::Swinub,
                baby: None,
                paths: &[]
            },
            Corsola => Evolution {
                base: Species::Corsola,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cursola, trigger: EvolutionTrigger::AtLevel(38) }]
            },
            Cursola => Evolution {
                base: Species::Corsola,
                baby: None,
                paths: &[]
            },
            Remoraid => Evolution {
                base: Species::Remoraid,
                baby: None,
                paths: &[EvolutionPath { to: Species::Octillery, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Octillery => Evolution {
                base: Species::Remoraid,
                baby: None,
                paths: &[]
            },
            Delibird => Evolution {
                base: Species::Delibird,
                baby: None,
                paths: &[]
            },
            Mantyke => Evolution {
                base: Species::Mantine,
                baby: Some({species: Species::Mantyke, incense: Incense::WaveIncense},
                           paths: &[EvolutionPath { to: Species::Mantine, trigger: EvolutionTrigger::WithPartyPokemon(Species::Remoraid) }]
            },
            Mantine => Evolution {
                base: Species::Mantine,
                baby: Some({species: Species::Mantyke, incense: Incense::WaveIncense},
                           paths: &[]
            },
            Skarmory => Evolution {
                base: Species::Skarmory,
                baby: None,
                paths: &[]
            },
            Houndour => Evolution {
                base: Species::Houndour,
                baby: None,
                paths: &[EvolutionPath { to: Species::Houndoom, trigger: EvolutionTrigger::AtLevel(24) }]
            },
            Houndoom => Evolution {
                base: Species::Houndour,
                baby: None,
                paths: &[]
            },
            Phanpy => Evolution {
                base: Species::Phanpy,
                baby: None,
                paths: &[EvolutionPath { to: Species::Donphan, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Donphan => Evolution {
                base: Species::Phanpy,
                baby: None,
                paths: &[]
            },
            Stantler => Evolution {
                base: Species::Stantler,
                baby: None,
                paths: &[]
            },
            Smeargle => Evolution {
                base: Species::Smeargle,
                baby: None,
                paths: &[]
            },
            Miltank => Evolution {
                base: Species::Miltank,
                baby: None,
                paths: &[]
            },
            Raikou => Evolution {
                base: Species::Raikou,
                baby: None,
                paths: &[]
            },
            Entei => Evolution {
                base: Species::Entei,
                baby: None,
                paths: &[]
            },
            Suicune => Evolution {
                base: Species::Suicune,
                baby: None,
                paths: &[]
            },
            Larvitar => Evolution {
                base: Species::Larvitar,
                baby: None,
                paths: &[EvolutionPath { to: Species::Pupitar, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Pupitar => Evolution {
                base: Species::Larvitar,
                baby: None,
                paths: &[EvolutionPath { to: Species::Tyranitar, trigger: EvolutionTrigger::AtLevel(55) }]
            },
            Tyranitar => Evolution {
                base: Species::Larvitar,
                baby: None,
                paths: &[]
            },
            Lugia => Evolution {
                base: Species::Lugia,
                baby: None,
                paths: &[]
            },
            HoOh => Evolution {
                base: Species::HoOh,
                baby: None,
                paths: &[]
            },
            Celebi => Evolution {
                base: Species::Celebi,
                baby: None,
                paths: &[]
            },
            Treecko => Evolution {
                base: Species::Treecko,
                baby: None,
                paths: &[EvolutionPath { to: Species::Grovyle, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Grovyle => Evolution {
                base: Species::Treecko,
                baby: None,
                paths: &[EvolutionPath { to: Species::Sceptile, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Sceptile => Evolution {
                base: Species::Treecko,
                baby: None,
                paths: &[]
            },
            Torchic => Evolution {
                base: Species::Torchic,
                baby: None,
                paths: &[EvolutionPath { to: Species::Combusken, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Combusken => Evolution {
                base: Species::Torchic,
                baby: None,
                paths: &[EvolutionPath { to: Species::Blaziken, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Blaziken => Evolution {
                base: Species::Torchic,
                baby: None,
                paths: &[]
            },
            Mudkip => Evolution {
                base: Species::Mudkip,
                baby: None,
                paths: &[EvolutionPath { to: Species::Marshtomp, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Marshtomp => Evolution {
                base: Species::Mudkip,
                baby: None,
                paths: &[EvolutionPath { to: Species::Swampert, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Swampert => Evolution {
                base: Species::Mudkip,
                baby: None,
                paths: &[]
            },
            Poochyena => Evolution {
                base: Species::Poochyena,
                baby: None,
                paths: &[EvolutionPath { to: Species::Mightyena, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Mightyena => Evolution {
                base: Species::Poochyena,
                baby: None,
                paths: &[]
            },
            Zigzagoon => Evolution {
                base: Species::Zigzagoon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Linoone, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Linoone => Evolution {
                base: Species::Zigzagoon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Obstagoon, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Obstagoon => Evolution {
                base: Species::Zigzagoon,
                baby: None,
                paths: &[]
            },
            Wurmple => Evolution {
                base: Species::Wurmple,
                baby: None,
                paths: &[EvolutionPath { to: Species::Silcoon, trigger: EvolutionTrigger::AtLevelRandomly(7, EvolutionTriggerGender::Low) }, EvolutionPath { to: Species::Cascoon, trigger: EvolutionTrigger::AtLevelRandomly(7, EvolutionTriggerGender::High) }]
            },
            Silcoon => Evolution {
                base: Species::Wurmple,
                baby: None,
                paths: &[EvolutionPath { to: Species::Beautifly, trigger: EvolutionTrigger::AtLevel(10) }]
            },
            Beautifly => Evolution {
                base: Species::Wurmple,
                baby: None,
                paths: &[]
            },
            Cascoon => Evolution {
                base: Species::Wurmple,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dustox, trigger: EvolutionTrigger::AtLevel(10) }]
            },
            Dustox => Evolution {
                base: Species::Wurmple,
                baby: None,
                paths: &[]
            },
            Lotad => Evolution {
                base: Species::Lotad,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lombre, trigger: EvolutionTrigger::AtLevel(14) }]
            },
            Lombre => Evolution {
                base: Species::Lotad,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ludicolo, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
            },
            Ludicolo => Evolution {
                base: Species::Lotad,
                baby: None,
                paths: &[]
            },
            Seedot => Evolution {
                base: Species::Seedot,
                baby: None,
                paths: &[EvolutionPath { to: Species::Nuzleaf, trigger: EvolutionTrigger::AtLevel(14) }]
            },
            Nuzleaf => Evolution {
                base: Species::Seedot,
                baby: None,
                paths: &[EvolutionPath { to: Species::Shiftry, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
            },
            Shiftry => Evolution {
                base: Species::Seedot,
                baby: None,
                paths: &[]
            },
            Taillow => Evolution {
                base: Species::Taillow,
                baby: None,
                paths: &[EvolutionPath { to: Species::Swellow, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Swellow => Evolution {
                base: Species::Taillow,
                baby: None,
                paths: &[]
            },
            Wingull => Evolution {
                base: Species::Wingull,
                baby: None,
                paths: &[EvolutionPath { to: Species::Pelipper, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Pelipper => Evolution {
                base: Species::Wingull,
                baby: None,
                paths: &[]
            },
            Ralts => Evolution {
                base: Species::Ralts,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kirlia, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Kirlia => Evolution {
                base: Species::Ralts,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gardevoir, trigger: EvolutionTrigger::AtLevel(30) }, EvolutionPath { to: Species::Gallade, trigger: EvolutionTrigger::EvolutionStoneWithGender(EvolutionStone::DawnStone, EvolutionTriggerGender::Male) }]
            },
            Gardevoir => Evolution {
                base: Species::Ralts,
                baby: None,
                paths: &[]
            },
            Gallade => Evolution {
                base: Species::Ralts,
                baby: None,
                paths: &[]
            },
            Surskit => Evolution {
                base: Species::Surskit,
                baby: None,
                paths: &[EvolutionPath { to: Species::Masquerain, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Masquerain => Evolution {
                base: Species::Surskit,
                baby: None,
                paths: &[]
            },
            Shroomish => Evolution {
                base: Species::Shroomish,
                baby: None,
                paths: &[EvolutionPath { to: Species::Breloom, trigger: EvolutionTrigger::AtLevel(23) }]
            },
            Breloom => Evolution {
                base: Species::Shroomish,
                baby: None,
                paths: &[]
            },
            Slakoth => Evolution {
                base: Species::Slakoth,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vigoroth, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Vigoroth => Evolution {
                base: Species::Slakoth,
                baby: None,
                paths: &[EvolutionPath { to: Species::Slaking, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Slaking => Evolution {
                base: Species::Slakoth,
                baby: None,
                paths: &[]
            },
            Nincada => Evolution {
                base: Species::Nincada,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ninjask, trigger: EvolutionTrigger::AtLevelSpawn(20, Species::Shedinja) }]
            },
            Ninjask => Evolution {
                base: Species::Nincada,
                baby: None,
                paths: &[]
            },
            Shedinja => Evolution {
                base: Species::Nincada,
                baby: None,
                paths: &[]
            },
            Whismur => Evolution {
                base: Species::Whismur,
                baby: None,
                paths: &[EvolutionPath { to: Species::Loudred, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Loudred => Evolution {
                base: Species::Whismur,
                baby: None,
                paths: &[EvolutionPath { to: Species::Exploud, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Exploud => Evolution {
                base: Species::Whismur,
                baby: None,
                paths: &[]
            },
            Makuhita => Evolution {
                base: Species::Makuhita,
                baby: None,
                paths: &[EvolutionPath { to: Species::Hariyama, trigger: EvolutionTrigger::AtLevel(24) }]
            },
            Hariyama => Evolution {
                base: Species::Makuhita,
                baby: None,
                paths: &[]
            },
            Nosepass => Evolution {
                base: Species::Nosepass,
                baby: None,
                paths: &[EvolutionPath { to: Species::Probopass, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::MagneticField) }, EvolutionPath { to: Species::Probopass, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
            },
            Probopass => Evolution {
                base: Species::Nosepass,
                baby: None,
                paths: &[]
            },
            Skitty => Evolution {
                base: Species::Skitty,
                baby: None,
                paths: &[EvolutionPath { to: Species::Delcatty, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
            },
            Delcatty => Evolution {
                base: Species::Skitty,
                baby: None,
                paths: &[]
            },
            Sableye => Evolution {
                base: Species::Sableye,
                baby: None,
                paths: &[]
            },
            Mawile => Evolution {
                base: Species::Mawile,
                baby: None,
                paths: &[]
            },
            Aron => Evolution {
                base: Species::Aron,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lairon, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Lairon => Evolution {
                base: Species::Aron,
                baby: None,
                paths: &[EvolutionPath { to: Species::Aggron, trigger: EvolutionTrigger::AtLevel(42) }]
            },
            Aggron => Evolution {
                base: Species::Aron,
                baby: None,
                paths: &[]
            },
            Meditite => Evolution {
                base: Species::Meditite,
                baby: None,
                paths: &[EvolutionPath { to: Species::Medicham, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Medicham => Evolution {
                base: Species::Meditite,
                baby: None,
                paths: &[]
            },
            Electrike => Evolution {
                base: Species::Electrike,
                baby: None,
                paths: &[EvolutionPath { to: Species::Manectric, trigger: EvolutionTrigger::AtLevel(26) }]
            },
            Manectric => Evolution {
                base: Species::Electrike,
                baby: None,
                paths: &[]
            },
            Plusle => Evolution {
                base: Species::Plusle,
                baby: None,
                paths: &[]
            },
            Minun => Evolution {
                base: Species::Minun,
                baby: None,
                paths: &[]
            },
            Volbeat => Evolution {
                base: Species::Volbeat,
                baby: None,
                paths: &[]
            },
            Illumise => Evolution {
                base: Species::Illumise,
                baby: None,
                paths: &[]
            },
            Budew => Evolution {
                base: Species::Roselia,
                baby: Some({species: Species::Budew, incense: Incense::RoseIncense},
                           paths: &[EvolutionPath { to: Species::Roselia, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Day) }]
            },
            Roselia => Evolution {
                base: Species::Roselia,
                baby: Some({species: Species::Budew, incense: Incense::RoseIncense},
                           paths: &[EvolutionPath { to: Species::Roserade, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ShinyStone) }]
            },
            Roserade => Evolution {
                base: Species::Roselia,
                baby: Some({species: Species::Budew, incense: Incense::RoseIncense},
                           paths: &[]
            },
            Gulpin => Evolution {
                base: Species::Gulpin,
                baby: None,
                paths: &[EvolutionPath { to: Species::Swalot, trigger: EvolutionTrigger::AtLevel(26) }]
            },
            Swalot => Evolution {
                base: Species::Gulpin,
                baby: None,
                paths: &[]
            },
            Carvanha => Evolution {
                base: Species::Carvanha,
                baby: None,
                paths: &[EvolutionPath { to: Species::Sharpedo, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Sharpedo => Evolution {
                base: Species::Carvanha,
                baby: None,
                paths: &[]
            },
            Wailmer => Evolution {
                base: Species::Wailmer,
                baby: None,
                paths: &[EvolutionPath { to: Species::Wailord, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Wailord => Evolution {
                base: Species::Wailmer,
                baby: None,
                paths: &[]
            },
            Numel => Evolution {
                base: Species::Numel,
                baby: None,
                paths: &[EvolutionPath { to: Species::Camerupt, trigger: EvolutionTrigger::AtLevel(33) }]
            },
            Camerupt => Evolution {
                base: Species::Numel,
                baby: None,
                paths: &[]
            },
            Torkoal => Evolution {
                base: Species::Torkoal,
                baby: None,
                paths: &[]
            },
            Spoink => Evolution {
                base: Species::Spoink,
                baby: None,
                paths: &[EvolutionPath { to: Species::Grumpig, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Grumpig => Evolution {
                base: Species::Spoink,
                baby: None,
                paths: &[]
            },
            Spinda => Evolution {
                base: Species::Spinda,
                baby: None,
                paths: &[]
            },
            Trapinch => Evolution {
                base: Species::Trapinch,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vibrava, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Vibrava => Evolution {
                base: Species::Trapinch,
                baby: None,
                paths: &[EvolutionPath { to: Species::Flygon, trigger: EvolutionTrigger::AtLevel(45) }]
            },
            Flygon => Evolution {
                base: Species::Trapinch,
                baby: None,
                paths: &[]
            },
            Cacnea => Evolution {
                base: Species::Cacnea,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cacturne, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Cacturne => Evolution {
                base: Species::Cacnea,
                baby: None,
                paths: &[]
            },
            Swablu => Evolution {
                base: Species::Swablu,
                baby: None,
                paths: &[EvolutionPath { to: Species::Altaria, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Altaria => Evolution {
                base: Species::Swablu,
                baby: None,
                paths: &[]
            },
            Zangoose => Evolution {
                base: Species::Zangoose,
                baby: None,
                paths: &[]
            },
            Seviper => Evolution {
                base: Species::Seviper,
                baby: None,
                paths: &[]
            },
            Lunatone => Evolution {
                base: Species::Lunatone,
                baby: None,
                paths: &[]
            },
            Solrock => Evolution {
                base: Species::Solrock,
                baby: None,
                paths: &[]
            },
            Barboach => Evolution {
                base: Species::Barboach,
                baby: None,
                paths: &[EvolutionPath { to: Species::Whiscash, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Whiscash => Evolution {
                base: Species::Barboach,
                baby: None,
                paths: &[]
            },
            Corphish => Evolution {
                base: Species::Corphish,
                baby: None,
                paths: &[EvolutionPath { to: Species::Crawdaunt, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Crawdaunt => Evolution {
                base: Species::Corphish,
                baby: None,
                paths: &[]
            },
            Baltoy => Evolution {
                base: Species::Baltoy,
                baby: None,
                paths: &[EvolutionPath { to: Species::Claydol, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Claydol => Evolution {
                base: Species::Baltoy,
                baby: None,
                paths: &[]
            },
            Lileep => Evolution {
                base: Species::Lileep,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cradily, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Cradily => Evolution {
                base: Species::Lileep,
                baby: None,
                paths: &[]
            },
            Anorith => Evolution {
                base: Species::Anorith,
                baby: None,
                paths: &[EvolutionPath { to: Species::Armaldo, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Armaldo => Evolution {
                base: Species::Anorith,
                baby: None,
                paths: &[]
            },
            Feebas => Evolution {
                base: Species::Feebas,
                baby: None,
                paths: &[EvolutionPath { to: Species::Milotic, trigger: EvolutionTrigger::HighBeauty }, EvolutionPath { to: Species::Milotic, trigger: EvolutionTrigger::TradingWithItem(Item::PrismScale) }]
            },
            Milotic => Evolution {
                base: Species::Feebas,
                baby: None,
                paths: &[]
            },
            Castform => Evolution {
                base: Species::Castform,
                baby: None,
                paths: &[]
            },
            Kecleon => Evolution {
                base: Species::Kecleon,
                baby: None,
                paths: &[]
            },
            Shuppet => Evolution {
                base: Species::Shuppet,
                baby: None,
                paths: &[EvolutionPath { to: Species::Banette, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Banette => Evolution {
                base: Species::Shuppet,
                baby: None,
                paths: &[]
            },
            Duskull => Evolution {
                base: Species::Duskull,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dusclops, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Dusclops => Evolution {
                base: Species::Duskull,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dusknoir, trigger: EvolutionTrigger::TradingWithItem(Item::ReaperCloth) }]
            },
            Dusknoir => Evolution {
                base: Species::Duskull,
                baby: None,
                paths: &[]
            },
            Tropius => Evolution {
                base: Species::Tropius,
                baby: None,
                paths: &[]
            },
            Chingling => Evolution {
                base: Species::Chimecho,
                baby: Some({species: Species::Chingling, incense: Incense::PureIncense},
                           paths: &[EvolutionPath { to: Species::Chimecho, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Night) }]
            },
            Chimecho => Evolution {
                base: Species::Chimecho,
                baby: Some({species: Species::Chingling, incense: Incense::PureIncense},
                           paths: &[]
            },
            Absol => Evolution {
                base: Species::Absol,
                baby: None,
                paths: &[]
            },
            Snorunt => Evolution {
                base: Species::Snorunt,
                baby: None,
                paths: &[EvolutionPath { to: Species::Glalie, trigger: EvolutionTrigger::AtLevel(42) }, EvolutionPath { to: Species::Froslass, trigger: EvolutionTrigger::EvolutionStoneWithGender(EvolutionStone::DawnStone, EvolutionTriggerGender::Female) }]
            },
            Glalie => Evolution {
                base: Species::Snorunt,
                baby: None,
                paths: &[]
            },
            Froslass => Evolution {
                base: Species::Snorunt,
                baby: None,
                paths: &[]
            },
            Spheal => Evolution {
                base: Species::Spheal,
                baby: None,
                paths: &[EvolutionPath { to: Species::Sealeo, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Sealeo => Evolution {
                base: Species::Spheal,
                baby: None,
                paths: &[EvolutionPath { to: Species::Walrein, trigger: EvolutionTrigger::AtLevel(44) }]
            },
            Walrein => Evolution {
                base: Species::Spheal,
                baby: None,
                paths: &[]
            },
            Clamperl => Evolution {
                base: Species::Clamperl,
                baby: None,
                paths: &[EvolutionPath { to: Species::Huntail, trigger: EvolutionTrigger::TradingWithItem(Item::DeepSeaTooth) }, EvolutionPath { to: Species::Gorebyss, trigger: EvolutionTrigger::TradingWithItem(Item::DeepSeaScale) }]
            },
            Huntail => Evolution {
                base: Species::Clamperl,
                baby: None,
                paths: &[]
            },
            Gorebyss => Evolution {
                base: Species::Clamperl,
                baby: None,
                paths: &[]
            },
            Relicanth => Evolution {
                base: Species::Relicanth,
                baby: None,
                paths: &[]
            },
            Luvdisc => Evolution {
                base: Species::Luvdisc,
                baby: None,
                paths: &[]
            },
            Bagon => Evolution {
                base: Species::Bagon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Shelgon, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Shelgon => Evolution {
                base: Species::Bagon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Salamence, trigger: EvolutionTrigger::AtLevel(50) }]
            },
            Salamence => Evolution {
                base: Species::Bagon,
                baby: None,
                paths: &[]
            },
            Beldum => Evolution {
                base: Species::Beldum,
                baby: None,
                paths: &[EvolutionPath { to: Species::Metang, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Metang => Evolution {
                base: Species::Beldum,
                baby: None,
                paths: &[EvolutionPath { to: Species::Metagross, trigger: EvolutionTrigger::AtLevel(45) }]
            },
            Metagross => Evolution {
                base: Species::Beldum,
                baby: None,
                paths: &[]
            },
            Regirock => Evolution {
                base: Species::Regirock,
                baby: None,
                paths: &[]
            },
            Regice => Evolution {
                base: Species::Regice,
                baby: None,
                paths: &[]
            },
            Registeel => Evolution {
                base: Species::Registeel,
                baby: None,
                paths: &[]
            },
            Latias => Evolution {
                base: Species::Latias,
                baby: None,
                paths: &[]
            },
            Latios => Evolution {
                base: Species::Latios,
                baby: None,
                paths: &[]
            },
            Kyogre => Evolution {
                base: Species::Kyogre,
                baby: None,
                paths: &[]
            },
            Groudon => Evolution {
                base: Species::Groudon,
                baby: None,
                paths: &[]
            },
            Rayquaza => Evolution {
                base: Species::Rayquaza,
                baby: None,
                paths: &[]
            },
            Jirachi => Evolution {
                base: Species::Jirachi,
                baby: None,
                paths: &[]
            },
            Deoxys => Evolution {
                base: Species::Deoxys,
                baby: None,
                paths: &[]
            },
            Turtwig => Evolution {
                base: Species::Turtwig,
                baby: None,
                paths: &[EvolutionPath { to: Species::Grotle, trigger: EvolutionTrigger::AtLevel(18) }]
            },
            Grotle => Evolution {
                base: Species::Turtwig,
                baby: None,
                paths: &[EvolutionPath { to: Species::Torterra, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Torterra => Evolution {
                base: Species::Turtwig,
                baby: None,
                paths: &[]
            },
            Chimchar => Evolution {
                base: Species::Chimchar,
                baby: None,
                paths: &[EvolutionPath { to: Species::Monferno, trigger: EvolutionTrigger::AtLevel(14) }]
            },
            Monferno => Evolution {
                base: Species::Chimchar,
                baby: None,
                paths: &[EvolutionPath { to: Species::Infernape, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Infernape => Evolution {
                base: Species::Chimchar,
                baby: None,
                paths: &[]
            },
            Piplup => Evolution {
                base: Species::Piplup,
                baby: None,
                paths: &[EvolutionPath { to: Species::Prinplup, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Prinplup => Evolution {
                base: Species::Piplup,
                baby: None,
                paths: &[EvolutionPath { to: Species::Empoleon, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Empoleon => Evolution {
                base: Species::Piplup,
                baby: None,
                paths: &[]
            },
            Starly => Evolution {
                base: Species::Starly,
                baby: None,
                paths: &[EvolutionPath { to: Species::Staravia, trigger: EvolutionTrigger::AtLevel(14) }]
            },
            Staravia => Evolution {
                base: Species::Starly,
                baby: None,
                paths: &[EvolutionPath { to: Species::Staraptor, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Staraptor => Evolution {
                base: Species::Starly,
                baby: None,
                paths: &[]
            },
            Bidoof => Evolution {
                base: Species::Bidoof,
                baby: None,
                paths: &[EvolutionPath { to: Species::Bibarel, trigger: EvolutionTrigger::AtLevel(15) }]
            },
            Bibarel => Evolution {
                base: Species::Bidoof,
                baby: None,
                paths: &[]
            },
            Kricketot => Evolution {
                base: Species::Kricketot,
                baby: None,
                paths: &[EvolutionPath { to: Species::Kricketune, trigger: EvolutionTrigger::AtLevel(10) }]
            },
            Kricketune => Evolution {
                base: Species::Kricketot,
                baby: None,
                paths: &[]
            },
            Shinx => Evolution {
                base: Species::Shinx,
                baby: None,
                paths: &[EvolutionPath { to: Species::Luxio, trigger: EvolutionTrigger::AtLevel(15) }]
            },
            Luxio => Evolution {
                base: Species::Shinx,
                baby: None,
                paths: &[EvolutionPath { to: Species::Luxray, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Luxray => Evolution {
                base: Species::Shinx,
                baby: None,
                paths: &[]
            },
            Cranidos => Evolution {
                base: Species::Cranidos,
                baby: None,
                paths: &[EvolutionPath { to: Species::Rampardos, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Rampardos => Evolution {
                base: Species::Cranidos,
                baby: None,
                paths: &[]
            },
            Shieldon => Evolution {
                base: Species::Shieldon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Bastiodon, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Bastiodon => Evolution {
                base: Species::Shieldon,
                baby: None,
                paths: &[]
            },
            Burmy => Evolution {
                base: Species::Burmy,
                baby: None,
                paths: &[EvolutionPath { to: Species::Wormadam, trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Female) }, EvolutionPath { to: Species::Mothim, trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Male) }]
            },
            Wormadam => Evolution {
                base: Species::Burmy,
                baby: None,
                paths: &[]
            },
            Mothim => Evolution {
                base: Species::Burmy,
                baby: None,
                paths: &[]
            },
            Combee => Evolution {
                base: Species::Combee,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vespiquen, trigger: EvolutionTrigger::AtLevelWithGender(21, EvolutionTriggerGender::Female) }]
            },
            Vespiquen => Evolution {
                base: Species::Combee,
                baby: None,
                paths: &[]
            },
            Pachirisu => Evolution {
                base: Species::Pachirisu,
                baby: None,
                paths: &[]
            },
            Buizel => Evolution {
                base: Species::Buizel,
                baby: None,
                paths: &[EvolutionPath { to: Species::Floatzel, trigger: EvolutionTrigger::AtLevel(26) }]
            },
            Floatzel => Evolution {
                base: Species::Buizel,
                baby: None,
                paths: &[]
            },
            Cherubi => Evolution {
                base: Species::Cherubi,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cherrim, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Cherrim => Evolution {
                base: Species::Cherubi,
                baby: None,
                paths: &[]
            },
            Shellos => Evolution {
                base: Species::Shellos,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gastrodon, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Gastrodon => Evolution {
                base: Species::Shellos,
                baby: None,
                paths: &[]
            },
            Drifloon => Evolution {
                base: Species::Drifloon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Drifblim, trigger: EvolutionTrigger::AtLevel(28) }]
            },
            Drifblim => Evolution {
                base: Species::Drifloon,
                baby: None,
                paths: &[]
            },
            Buneary => Evolution {
                base: Species::Buneary,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lopunny, trigger: EvolutionTrigger::HighFriendship }]
            },
            Lopunny => Evolution {
                base: Species::Buneary,
                baby: None,
                paths: &[]
            },
            Glameow => Evolution {
                base: Species::Glameow,
                baby: None,
                paths: &[EvolutionPath { to: Species::Purugly, trigger: EvolutionTrigger::AtLevel(38) }]
            },
            Purugly => Evolution {
                base: Species::Glameow,
                baby: None,
                paths: &[]
            },
            Stunky => Evolution {
                base: Species::Stunky,
                baby: None,
                paths: &[EvolutionPath { to: Species::Skuntank, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Skuntank => Evolution {
                base: Species::Stunky,
                baby: None,
                paths: &[]
            },
            Bronzor => Evolution {
                base: Species::Bronzor,
                baby: None,
                paths: &[EvolutionPath { to: Species::Bronzong, trigger: EvolutionTrigger::AtLevel(33) }]
            },
            Bronzong => Evolution {
                base: Species::Bronzor,
                baby: None,
                paths: &[]
            },
            Chatot => Evolution {
                base: Species::Chatot,
                baby: None,
                paths: &[]
            },
            Spiritomb => Evolution {
                base: Species::Spiritomb,
                baby: None,
                paths: &[]
            },
            Gible => Evolution {
                base: Species::Gible,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gabite, trigger: EvolutionTrigger::AtLevel(24) }]
            },
            Gabite => Evolution {
                base: Species::Gible,
                baby: None,
                paths: &[EvolutionPath { to: Species::Garchomp, trigger: EvolutionTrigger::AtLevel(48) }]
            },
            Garchomp => Evolution {
                base: Species::Gible,
                baby: None,
                paths: &[]
            },
            Riolu => Evolution {
                base: Species::Riolu,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lucario, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Day) }]
            },
            Lucario => Evolution {
                base: Species::Riolu,
                baby: None,
                paths: &[]
            },
            Hippopotas => Evolution {
                base: Species::Hippopotas,
                baby: None,
                paths: &[EvolutionPath { to: Species::Hippowdon, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Hippowdon => Evolution {
                base: Species::Hippopotas,
                baby: None,
                paths: &[]
            },
            Skorupi => Evolution {
                base: Species::Skorupi,
                baby: None,
                paths: &[EvolutionPath { to: Species::Drapion, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Drapion => Evolution {
                base: Species::Skorupi,
                baby: None,
                paths: &[]
            },
            Croagunk => Evolution {
                base: Species::Croagunk,
                baby: None,
                paths: &[EvolutionPath { to: Species::Toxicroak, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Toxicroak => Evolution {
                base: Species::Croagunk,
                baby: None,
                paths: &[]
            },
            Carnivine => Evolution {
                base: Species::Carnivine,
                baby: None,
                paths: &[]
            },
            Finneon => Evolution {
                base: Species::Finneon,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lumineon, trigger: EvolutionTrigger::AtLevel(31) }]
            },
            Lumineon => Evolution {
                base: Species::Finneon,
                baby: None,
                paths: &[]
            },
            Snover => Evolution {
                base: Species::Snover,
                baby: None,
                paths: &[EvolutionPath { to: Species::Abomasnow, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Abomasnow => Evolution {
                base: Species::Snover,
                baby: None,
                paths: &[]
            },
            Rotom => Evolution {
                base: Species::Rotom,
                baby: None,
                paths: &[]
            },
            Uxie => Evolution {
                base: Species::Uxie,
                baby: None,
                paths: &[]
            },
            Mesprit => Evolution {
                base: Species::Mesprit,
                baby: None,
                paths: &[]
            },
            Azelf => Evolution {
                base: Species::Azelf,
                baby: None,
                paths: &[]
            },
            Dialga => Evolution {
                base: Species::Dialga,
                baby: None,
                paths: &[]
            },
            Palkia => Evolution {
                base: Species::Palkia,
                baby: None,
                paths: &[]
            },
            Heatran => Evolution {
                base: Species::Heatran,
                baby: None,
                paths: &[]
            },
            Regigigas => Evolution {
                base: Species::Regigigas,
                baby: None,
                paths: &[]
            },
            Giratina => Evolution {
                base: Species::Giratina,
                baby: None,
                paths: &[]
            },
            Cresselia => Evolution {
                base: Species::Cresselia,
                baby: None,
                paths: &[]
            },
            Phione => Evolution {
                base: Species::Phione,
                baby: None,
                paths: &[]
            },
            Manaphy => Evolution {
                base: Species::Phione,
                baby: None,
                paths: &[]
            },
            Darkrai => Evolution {
                base: Species::Darkrai,
                baby: None,
                paths: &[]
            },
            Shaymin => Evolution {
                base: Species::Shaymin,
                baby: None,
                paths: &[]
            },
            Arceus => Evolution {
                base: Species::Arceus,
                baby: None,
                paths: &[]
            },
            Victini => Evolution {
                base: Species::Victini,
                baby: None,
                paths: &[]
            },
            Snivy => Evolution {
                base: Species::Snivy,
                baby: None,
                paths: &[EvolutionPath { to: Species::Servine, trigger: EvolutionTrigger::AtLevel(17) }]
            },
            Servine => Evolution {
                base: Species::Snivy,
                baby: None,
                paths: &[EvolutionPath { to: Species::Serperior, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Serperior => Evolution {
                base: Species::Snivy,
                baby: None,
                paths: &[]
            },
            Tepig => Evolution {
                base: Species::Tepig,
                baby: None,
                paths: &[EvolutionPath { to: Species::Pignite, trigger: EvolutionTrigger::AtLevel(17) }]
            },
            Pignite => Evolution {
                base: Species::Tepig,
                baby: None,
                paths: &[EvolutionPath { to: Species::Emboar, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Emboar => Evolution {
                base: Species::Tepig,
                baby: None,
                paths: &[]
            },
            Oshawott => Evolution {
                base: Species::Oshawott,
                baby: None,
                paths: &[EvolutionPath { to: Species::Dewott, trigger: EvolutionTrigger::AtLevel(17) }]
            },
            Dewott => Evolution {
                base: Species::Oshawott,
                baby: None,
                paths: &[EvolutionPath { to: Species::Samurott, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Samurott => Evolution {
                base: Species::Oshawott,
                baby: None,
                paths: &[]
            },
            Patrat => Evolution {
                base: Species::Patrat,
                baby: None,
                paths: &[EvolutionPath { to: Species::Watchog, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Watchog => Evolution {
                base: Species::Patrat,
                baby: None,
                paths: &[]
            },
            Lillipup => Evolution {
                base: Species::Lillipup,
                baby: None,
                paths: &[EvolutionPath { to: Species::Herdier, trigger: EvolutionTrigger::AtLevel(16) }]
            },
            Herdier => Evolution {
                base: Species::Lillipup,
                baby: None,
                paths: &[EvolutionPath { to: Species::Stoutland, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Stoutland => Evolution {
                base: Species::Lillipup,
                baby: None,
                paths: &[]
            },
            Purrloin => Evolution {
                base: Species::Purrloin,
                baby: None,
                paths: &[EvolutionPath { to: Species::Liepard, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Liepard => Evolution {
                base: Species::Purrloin,
                baby: None,
                paths: &[]
            },
            Pansage => Evolution {
                base: Species::Pansage,
                baby: None,
                paths: &[EvolutionPath { to: Species::Simisage, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
            },
            Simisage => Evolution {
                base: Species::Pansage,
                baby: None,
                paths: &[]
            },
            Pansear => Evolution {
                base: Species::Pansear,
                baby: None,
                paths: &[EvolutionPath { to: Species::Simisear, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }]
            },
            Simisear => Evolution {
                base: Species::Pansear,
                baby: None,
                paths: &[]
            },
            Panpour => Evolution {
                base: Species::Panpour,
                baby: None,
                paths: &[EvolutionPath { to: Species::Simipour, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
            },
            Simipour => Evolution {
                base: Species::Panpour,
                baby: None,
                paths: &[]
            },
            Munna => Evolution {
                base: Species::Munna,
                baby: None,
                paths: &[EvolutionPath { to: Species::Musharna, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
            },
            Musharna => Evolution {
                base: Species::Munna,
                baby: None,
                paths: &[]
            },
            Pidove => Evolution {
                base: Species::Pidove,
                baby: None,
                paths: &[EvolutionPath { to: Species::Tranquill, trigger: EvolutionTrigger::AtLevel(21) }]
            },
            Tranquill => Evolution {
                base: Species::Pidove,
                baby: None,
                paths: &[EvolutionPath { to: Species::Unfezant, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Unfezant => Evolution {
                base: Species::Pidove,
                baby: None,
                paths: &[]
            },
            Blitzle => Evolution {
                base: Species::Blitzle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Zebstrika, trigger: EvolutionTrigger::AtLevel(27) }]
            },
            Zebstrika => Evolution {
                base: Species::Blitzle,
                baby: None,
                paths: &[]
            },
            Roggenrola => Evolution {
                base: Species::Roggenrola,
                baby: None,
                paths: &[EvolutionPath { to: Species::Boldore, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Boldore => Evolution {
                base: Species::Roggenrola,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gigalith, trigger: EvolutionTrigger::Trading }]
            },
            Gigalith => Evolution {
                base: Species::Roggenrola,
                baby: None,
                paths: &[]
            },
            Woobat => Evolution {
                base: Species::Woobat,
                baby: None,
                paths: &[EvolutionPath { to: Species::Swoobat, trigger: EvolutionTrigger::HighFriendship }]
            },
            Swoobat => Evolution {
                base: Species::Woobat,
                baby: None,
                paths: &[]
            },
            Drilbur => Evolution {
                base: Species::Drilbur,
                baby: None,
                paths: &[EvolutionPath { to: Species::Excadrill, trigger: EvolutionTrigger::AtLevel(31) }]
            },
            Excadrill => Evolution {
                base: Species::Drilbur,
                baby: None,
                paths: &[]
            },
            Audino => Evolution {
                base: Species::Audino,
                baby: None,
                paths: &[]
            },
            Timburr => Evolution {
                base: Species::Timburr,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gurdurr, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Gurdurr => Evolution {
                base: Species::Timburr,
                baby: None,
                paths: &[EvolutionPath { to: Species::Conkeldurr, trigger: EvolutionTrigger::Trading }]
            },
            Conkeldurr => Evolution {
                base: Species::Timburr,
                baby: None,
                paths: &[]
            },
            Tympole => Evolution {
                base: Species::Tympole,
                baby: None,
                paths: &[EvolutionPath { to: Species::Palpitoad, trigger: EvolutionTrigger::AtLevel(25) }]
            },
            Palpitoad => Evolution {
                base: Species::Tympole,
                baby: None,
                paths: &[EvolutionPath { to: Species::Seismitoad, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Seismitoad => Evolution {
                base: Species::Tympole,
                baby: None,
                paths: &[]
            },
            Throh => Evolution {
                base: Species::Throh,
                baby: None,
                paths: &[]
            },
            Sawk => Evolution {
                base: Species::Sawk,
                baby: None,
                paths: &[]
            },
            Sewaddle => Evolution {
                base: Species::Sewaddle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Swadloon, trigger: EvolutionTrigger::AtLevel(20) }]
            },
            Swadloon => Evolution {
                base: Species::Sewaddle,
                baby: None,
                paths: &[EvolutionPath { to: Species::Leavanny, trigger: EvolutionTrigger::HighFriendship }]
            },
            Leavanny => Evolution {
                base: Species::Sewaddle,
                baby: None,
                paths: &[]
            },
            Venipede => Evolution {
                base: Species::Venipede,
                baby: None,
                paths: &[EvolutionPath { to: Species::Whirlipede, trigger: EvolutionTrigger::AtLevel(22) }]
            },
            Whirlipede => Evolution {
                base: Species::Venipede,
                baby: None,
                paths: &[EvolutionPath { to: Species::Scolipede, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Scolipede => Evolution {
                base: Species::Venipede,
                baby: None,
                paths: &[]
            },
            Cottonee => Evolution {
                base: Species::Cottonee,
                baby: None,
                paths: &[EvolutionPath { to: Species::Whimsicott, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
            },
            Whimsicott => Evolution {
                base: Species::Cottonee,
                baby: None,
                paths: &[]
            },
            Petilil => Evolution {
                base: Species::Petilil,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lilligant, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
            },
            Lilligant => Evolution {
                base: Species::Petilil,
                baby: None,
                paths: &[]
            },
            Basculin => Evolution {
                base: Species::Basculin,
                baby: None,
                paths: &[]
            },
            Sandile => Evolution {
                base: Species::Sandile,
                baby: None,
                paths: &[EvolutionPath { to: Species::Krokorok, trigger: EvolutionTrigger::AtLevel(29) }]
            },
            Krokorok => Evolution {
                base: Species::Sandile,
                baby: None,
                paths: &[EvolutionPath { to: Species::Krookodile, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Krookodile => Evolution {
                base: Species::Sandile,
                baby: None,
                paths: &[]
            },
            Darumaka => Evolution {
                base: Species::Darumaka,
                baby: None,
                paths: &[EvolutionPath { to: Species::Darmanitan, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Darmanitan => Evolution {
                base: Species::Darumaka,
                baby: None,
                paths: &[]
            },
            Maractus => Evolution {
                base: Species::Maractus,
                baby: None,
                paths: &[]
            },
            Dwebble => Evolution {
                base: Species::Dwebble,
                baby: None,
                paths: &[EvolutionPath { to: Species::Crustle, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Crustle => Evolution {
                base: Species::Dwebble,
                baby: None,
                paths: &[]
            },
            Scraggy => Evolution {
                base: Species::Scraggy,
                baby: None,
                paths: &[EvolutionPath { to: Species::Scrafty, trigger: EvolutionTrigger::AtLevel(39) }]
            },
            Scrafty => Evolution {
                base: Species::Scraggy,
                baby: None,
                paths: &[]
            },
            Sigilyph => Evolution {
                base: Species::Sigilyph,
                baby: None,
                paths: &[]
            },
            Yamask => Evolution {
                base: Species::Yamask,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cofagrigus, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Cofagrigus => Evolution {
                base: Species::Yamask,
                baby: None,
                paths: &[]
            },
            Runerigus => Evolution {
                base: Species::Yamask,
                baby: None,
                paths: &[]
            },
            Tirtouga => Evolution {
                base: Species::Tirtouga,
                baby: None,
                paths: &[EvolutionPath { to: Species::Carracosta, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Carracosta => Evolution {
                base: Species::Tirtouga,
                baby: None,
                paths: &[]
            },
            Archen => Evolution {
                base: Species::Archen,
                baby: None,
                paths: &[EvolutionPath { to: Species::Archeops, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Archeops => Evolution {
                base: Species::Archen,
                baby: None,
                paths: &[]
            },
            Trubbish => Evolution {
                base: Species::Trubbish,
                baby: None,
                paths: &[EvolutionPath { to: Species::Garbodor, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Garbodor => Evolution {
                base: Species::Trubbish,
                baby: None,
                paths: &[]
            },
            Zorua => Evolution {
                base: Species::Zorua,
                baby: None,
                paths: &[EvolutionPath { to: Species::Zoroark, trigger: EvolutionTrigger::AtLevel(30) }]
            },
            Zoroark => Evolution {
                base: Species::Zorua,
                baby: None,
                paths: &[]
            },
            Minccino => Evolution {
                base: Species::Minccino,
                baby: None,
                paths: &[EvolutionPath { to: Species::Cinccino, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ShinyStone) }]
            },
            Cinccino => Evolution {
                base: Species::Minccino,
                baby: None,
                paths: &[]
            },
            Gothita => Evolution {
                base: Species::Gothita,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gothorita, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Gothorita => Evolution {
                base: Species::Gothita,
                baby: None,
                paths: &[EvolutionPath { to: Species::Gothitelle, trigger: EvolutionTrigger::AtLevel(41) }]
            },
            Gothitelle => Evolution {
                base: Species::Gothita,
                baby: None,
                paths: &[]
            },
            Solosis => Evolution {
                base: Species::Solosis,
                baby: None,
                paths: &[EvolutionPath { to: Species::Duosion, trigger: EvolutionTrigger::AtLevel(32) }]
            },
            Duosion => Evolution {
                base: Species::Solosis,
                baby: None,
                paths: &[EvolutionPath { to: Species::Reuniclus, trigger: EvolutionTrigger::AtLevel(41) }]
            },
            Reuniclus => Evolution {
                base: Species::Solosis,
                baby: None,
                paths: &[]
            },
            Ducklett => Evolution {
                base: Species::Ducklett,
                baby: None,
                paths: &[EvolutionPath { to: Species::Swanna, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Swanna => Evolution {
                base: Species::Ducklett,
                baby: None,
                paths: &[]
            },
            Vanillite => Evolution {
                base: Species::Vanillite,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vanillish, trigger: EvolutionTrigger::AtLevel(35) }]
            },
            Vanillish => Evolution {
                base: Species::Vanillite,
                baby: None,
                paths: &[EvolutionPath { to: Species::Vanilluxe, trigger: EvolutionTrigger::AtLevel(47) }]
            },
            Vanilluxe => Evolution {
                base: Species::Vanillite,
                baby: None,
                paths: &[]
            },
            Deerling => Evolution {
                base: Species::Deerling,
                baby: None,
                paths: &[EvolutionPath { to: Species::Sawsbuck, trigger: EvolutionTrigger::AtLevel(34) }]
            },
            Sawsbuck => Evolution {
                base: Species::Deerling,
                baby: None,
                paths: &[]
            },
            Emolga => Evolution {
                base: Species::Emolga,
                baby: None,
                paths: &[]
            },
            Karrablast => Evolution {
                base: Species::Karrablast,
                baby: None,
                paths: &[EvolutionPath { to: Species::Escavalier, trigger: EvolutionTrigger::TradingForPokemon(Species::Shelmet) }]
            },
            Escavalier => Evolution {
                base: Species::Karrablast,
                baby: None,
                paths: &[]
            },
            Foongus => Evolution {
                base: Species::Foongus,
                baby: None,
                paths: &[EvolutionPath { to: Species::Amoonguss, trigger: EvolutionTrigger::AtLevel(39) }]
            },
            Amoonguss => Evolution {
                base: Species::Foongus,
                baby: None,
                paths: &[]
            },
            Frillish => Evolution {
                base: Species::Frillish,
                baby: None,
                paths: &[EvolutionPath { to: Species::Jellicent, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Jellicent => Evolution {
                base: Species::Frillish,
                baby: None,
                paths: &[]
            },
            Alomomola => Evolution {
                base: Species::Alomomola,
                baby: None,
                paths: &[]
            },
            Joltik => Evolution {
                base: Species::Joltik,
                baby: None,
                paths: &[EvolutionPath { to: Species::Galvantula, trigger: EvolutionTrigger::AtLevel(36) }]
            },
            Galvantula => Evolution {
                base: Species::Joltik,
                baby: None,
                paths: &[]
            },
            Ferroseed => Evolution {
                base: Species::Ferroseed,
                baby: None,
                paths: &[EvolutionPath { to: Species::Ferrothorn, trigger: EvolutionTrigger::AtLevel(40) }]
            },
            Ferrothorn => Evolution {
                base: Species::Ferroseed,
                baby: None,
                paths: &[]
            },
            Klink => Evolution {
                base: Species::Klink,
                baby: None,
                paths: &[EvolutionPath { to: Species::Klang, trigger: EvolutionTrigger::AtLevel(38) }]
            },
            Klang => Evolution {
                base: Species::Klink,
                baby: None,
                paths: &[EvolutionPath { to: Species::Klinklang, trigger: EvolutionTrigger::AtLevel(49) }]
            },
            Klinklang => Evolution {
                base: Species::Klink,
                baby: None,
                paths: &[]
            },
            Tynamo => Evolution {
                base: Species::Tynamo,
                baby: None,
                paths: &[EvolutionPath { to: Species::Eelektrik, trigger: EvolutionTrigger::AtLevel(39) }]
            },
            Eelektrik => Evolution {
                base: Species::Tynamo,
                baby: None,
                paths: &[EvolutionPath { to: Species::Eelektross, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
            },
            Eelektross => Evolution {
                base: Species::Tynamo,
                baby: None,
                paths: &[]
            },
            Elgyem => Evolution {
                base: Species::Elgyem,
                baby: None,
                paths: &[EvolutionPath { to: Species::Beheeyem, trigger: EvolutionTrigger::AtLevel(42) }]
            },
            Beheeyem => Evolution {
                base: Species::Elgyem,
                baby: None,
                paths: &[]
            },
            Litwick => Evolution {
                base: Species::Litwick,
                baby: None,
                paths: &[EvolutionPath { to: Species::Lampent, trigger: EvolutionTrigger::AtLevel(41) }]
            },
            Lampent => Evolution {
                base: Species::Litwick,
                baby: None,
                paths: &[EvolutionPath { to: Species::Chandelure, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::DuskStone) }]
            },
            Chandelure => Evolution {
                base: Species::Litwick,
                baby: None,
                paths: &[]
            },
            Axew => Evolution {
                base: Species::Axew,
                baby: None,
                paths: &[EvolutionPath { to: Species::Fraxure, trigger: EvolutionTrigger::AtLevel(38) }]
            },
            Fraxure => Evolution {
                base: Species::Axew,
                baby: None,
                paths: &[EvolutionPath { to: Species::Haxorus, trigger: EvolutionTrigger::AtLevel(48) }]
            },
            Haxorus => Evolution {
                base: Species::Axew,
                baby: None,
                paths: &[]
            },
            Cubchoo => Evolution {
                base: Species::Cubchoo,
                baby: None,
                paths: &[EvolutionPath { to: Species::Beartic, trigger: EvolutionTrigger::AtLevel(37) }]
            },
            Beartic => Evolution {
                base: Species::Cubchoo,
                baby: None,
                paths: &[]
            },
            Cryogonal => Evolution {
                base: Species::Cryogonal,
                baby: None,
                paths: &[]
            },
            Shelmet => Evolution {
                base: Species::Shelmet,
                baby: None,
                paths: &[EvolutionPath { to: Species::Accelgor, trigger: EvolutionTrigger::TradingForPokemon(Species::Karrablast) }]
            },
            Accelgor => Evolution {
                base: Species::Shelmet,
                baby: None,
                paths: &[]
            },
            Stunfisk => Evolution {
                base: Species::Stunfisk,
                baby: None,
                paths: &[]
            },
            Mienfoo => Evolution {
                base: Species::Mienfoo,
                baby: None,
                paths: &[EvolutionPath { to: Species::Mienshao, trigger: EvolutionTrigger::AtLevel(50) }]
            },
            Mienshao => Evolution {
                base: Species::Mienfoo,
                baby: None,
                paths: &[]
            },
            Druddigon => Evolution {
                base: Species::Druddigon,
                baby: None,
                paths: &[]
            },
            Golett => Evolution {
                base: Species::Golett,
                baby: None,
                paths: &[EvolutionPath { to: Species::Golurk, trigger: EvolutionTrigger::AtLevel(43) }]
            },
            Golurk => Evolution {
                base: Species::Golett,
                baby: None,
                paths: &[]
            },
            Pawniard => Evolution {
                base: Species::Pawniard,
                baby: None,
                paths: &[EvolutionPath { to: Species::Bisharp, trigger: EvolutionTrigger::AtLevel(52) }]
            },
            Bisharp => Evolution {
                base: Species::Pawniard,
                baby: None,
                paths: &[]
            },
            Bouffalant => Evolution {
                base: Species::Bouffalant,
                baby: None,
                paths: &[]
            },
            Rufflet => Evolution {
                base: Species::Rufflet,
                baby: None,
                paths: &[EvolutionPath { to: Species::Braviary, trigger: EvolutionTrigger::AtLevel(54) }]
            },
            Braviary => Evolution {
                base: Species::Rufflet,
                baby: None,
                paths: &[]
            },
            Vullaby => Evolution {
                base: Species::Vullaby,
                baby: None,
                paths: &[EvolutionPath { to: Species::Mandibuzz, trigger: EvolutionTrigger::AtLevel(54) }]
            },
            Mandibuzz => Evolution {
                base: Species::Vullaby,
                baby: None,
                paths: &[]
            },
            Heatmor => Evolution {
                base: Species::Heatmor,
                baby: None,
                paths: &[]
            },
            Durant => Evolution {
                base: Species::Durant,
                baby: None,
                paths: &[]
            },
            Deino => Evolution {
                base: Species::Deino,
                baby: None,
                paths: &[EvolutionPath { to: Species::Zweilous, trigger: EvolutionTrigger::AtLevel(50) }]
            },
            Zweilous => Evolution {
                base: Species::Deino,
                baby: None,
                paths: &[EvolutionPath { to: Species::Hydreigon, trigger: EvolutionTrigger::AtLevel(64) }]
            },
            Hydreigon => Evolution {
                base: Species::Deino,
                baby: None,
                paths: &[]
            },
            Larvesta => Evolution {
                base: Species::Larvesta,
                baby: None,
                paths: &[EvolutionPath { to: Species::Volcarona, trigger: EvolutionTrigger::AtLevel(59) }]
            },
            Volcarona => Evolution {
                base: Species::Larvesta,
                baby: None,
                paths: &[]
            },
            Cobalion => Evolution {
                base: Species::Cobalion,
                baby: None,
                paths: &[]
            },
            Terrakion => Evolution {
                base: Species::Terrakion,
                baby: None,
                paths: &[]
            },
            Virizion => Evolution {
                base: Species::Virizion,
                baby: None,
                paths: &[]
            },
            Tornadus => Evolution {
                base: Species::Tornadus,
                baby: None,
                paths: &[]
            },
            Thundurus => Evolution {
                base: Species::Thundurus,
                baby: None,
                paths: &[]
            },
            Reshiram => Evolution {
                base: Species::Reshiram,
                baby: None,
                paths: &[]
            },
            Zekrom => Evolution {
                base: Species::Zekrom,
                baby: None,
                paths: &[]
            },
            Landorus => Evolution {
                base: Species::Landorus,
                baby: None,
                paths: &[]
            },
            Kyurem => Evolution {
                base: Species::Kyurem,
                baby: None,
                paths: &[]
            },
            Keldeo => Evolution {
                base: Species::Keldeo,
                baby: None,
                paths: &[]
            },
            Meloetta => Evolution {
                base: Species::Meloetta,
                baby: None,
                paths: &[]
            },
            Genesect => Evolution {
                base: Species::Genesect,
                baby: None,
                paths: &[]
            }
        }
    }
}