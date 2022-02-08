#![allow(non_upper_case_globals)]
use serde::{Serialize, Deserialize};

use crate::attack::{Move, MoveData};
use crate::contest::Condition;
use crate::core::{GlazedTime, Player, TimeOfDay};
use crate::species::Species;
use crate::item::{EvolutionHeldItem, EvolutionStone, Incense, Item};
use crate::lookups::Lookup;
use crate::pokemon::{Gender, Pokemon};
use crate::types::Type;

/// Represents evolution + breeding data for a species
/// Breeding rules:
/// 1. if `baby`, and parent is holding `baby.incense`, offspring is `baby.species`
/// 2. else, offspring is `base`
pub struct Evolution {
    pub base: Species,
    pub baby: Option<IncenseBaby>,
    pub paths: Option<Vec<EvolutionPath>>
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
#[derive(Debug, Deserialize)]
pub enum EvolutionTrigger {
    /// Evolves when a zero or more conditions are met at level up
    OnLevelUp(Vec<EvolutionCondition>),

    /// Evolves when an evolutionary stone is used on it
    EvolutionStone {
        stone: EvolutionStone,
        #[serde(default)]
        conditions: Option<Vec<EvolutionCondition>>
    },

    /// Evolves after the Pokemon was traded and all conditions are met
    Trading {
        #[serde(default)]
        conditions: Option<Vec<EvolutionCondition>>
    },
    /// Evolves after the Pokemon was traded, if the opposite Pokemon was the specific species
    TradingForPokemon(Species)
}

/// Represents any number of sub-conditions that must be fulfilled on reaching a certain level
#[derive(Debug, Deserialize)]
pub enum EvolutionCondition {
    /// Is the Pokemon this level (or greater)?
    Level(u8),
    /// Is the Pokemon this gender?
    Gender(Gender),
    /// Is it current this time of day?
    TimeOfDay(TimeOfDay),
    /// Does the Pokemon have a high friendship? (>= 220)
    HighFriendship,
    /// Does the Pokemon have a high condition? (>= 170)
    HighCondition(Condition),
    /// Is the Pokemon holding this held item?
    HoldingItem(EvolutionHeldItem),
    /// Does the Pokemon know this move?
    KnowsMove(Move),
    /// Does the Pokemon know a move of this type?
    KnowsMoveOfType(Type),
    /// Does the party include a Pokemon of this species?
    WithPartyPokemon(Species),
    /// Is the Pokemon in a specific location?
    AtPlace(EvolutionTriggerLocation),
    /// Does the Pokemon have a low Personality Value?
    LowPersonality,
    /// Does the Pokemon have a high Personality Value?
    HighPersonality,
    /// Is the Pokemon's attack stat greater than its defense?
    HigherAttackThanDefense,
    /// Is the Pokemon's defense stat greater than its attack?
    HigherDefenseThanAttack,
    /// Are the Pokemon's attack and defense stats identical?
    EqualAttackAndDefense
}
impl EvolutionCondition {
    /// Check if a Pokemon meets the Evolution Condition
    pub fn meets_condition(&self, pkmn: &Pokemon, player: &Player) -> bool {
        match self {
            EvolutionCondition::Level(l) => pkmn.level >= *l,
            EvolutionCondition::Gender(g) => pkmn.gender == *g,
            EvolutionCondition::TimeOfDay(d) => GlazedTime::get_time_of_day() == *d,
            EvolutionCondition::HighFriendship => pkmn.friendship >= 220,
            EvolutionCondition::HighCondition(c) => pkmn.contest.get_condition(c) >= 170,
            EvolutionCondition::HoldingItem(h) => pkmn.is_holding(Item::from(*h)),
            EvolutionCondition::KnowsMove(m) => pkmn.knows_move(*m),
            EvolutionCondition::KnowsMoveOfType(t) => {
                pkmn.get_moves().iter()
                    .map(|m| MoveData::lookup(m))
                    .any(|md| md._type == *t)
            }
            EvolutionCondition::WithPartyPokemon(p) => player.party.iter().any(|pm| pm.species == *p),
            EvolutionCondition::AtPlace(_) => false,
            EvolutionCondition::LowPersonality => (pkmn.personality >> 16) & 10 < 5,
            EvolutionCondition::HighPersonality => (pkmn.personality >> 16) & 10 >= 5,
            EvolutionCondition::HigherAttackThanDefense => pkmn.attack.value > pkmn.defense.value,
            EvolutionCondition::HigherDefenseThanAttack => pkmn.attack.value < pkmn.defense.value,
            EvolutionCondition::EqualAttackAndDefense => pkmn.attack.value == pkmn.defense.value
        }
    }
}

/// Represents a type of location that can trigger Evolution
#[derive(Debug, Deserialize)]
pub enum EvolutionTriggerLocation {
    MossRock,
    IceRock,
    MagneticField
}

// #[allow(non_upper_case_globals)]
// impl Species {
//     pub fn get_evolutions(&self) -> &'static Evolution {
//         match self {
//             Species::Bulbasaur => &Bulbasaur,
//             Species::Ivysaur => &Ivysaur,
//             Species::Venusaur => &Venusaur,
//             Species::Charmander => &Charmander,
//             Species::Charmeleon => &Charmeleon,
//             Species::Charizard => &Charizard,
//             Species::Squirtle => &Squirtle,
//             Species::Wartortle => &Wartortle,
//             Species::Blastoise => &Blastoise,
//             Species::Caterpie => &Caterpie,
//             Species::Metapod => &Metapod,
//             Species::Butterfree => &Butterfree,
//             Species::Weedle => &Weedle,
//             Species::Kakuna => &Kakuna,
//             Species::Beedrill => &Beedrill,
//             Species::Pidgey => &Pidgey,
//             Species::Pidgeotto => &Pidgeotto,
//             Species::Pidgeot => &Pidgeot,
//             Species::Rattata => &Rattata,
//             Species::Raticate => &Raticate,
//             Species::Spearow => &Spearow,
//             Species::Fearow => &Fearow,
//             Species::Ekans => &Ekans,
//             Species::Arbok => &Arbok,
//             Species::Pichu => &Pichu,
//             Species::Pikachu => &Pikachu,
//             Species::Raichu => &Raichu,
//             Species::Sandshrew => &Sandshrew,
//             Species::Sandslash => &Sandslash,
//             Species::NidoranF => &NidoranF,
//             Species::Nidorina => &Nidorina,
//             Species::Nidoqueen => &Nidoqueen,
//             Species::NidoranM => &NidoranM,
//             Species::Nidorino => &Nidorino,
//             Species::Nidoking => &Nidoking,
//             Species::Cleffa => &Cleffa,
//             Species::Clefairy => &Clefairy,
//             Species::Clefable => &Clefable,
//             Species::Vulpix => &Vulpix,
//             Species::Ninetales => &Ninetales,
//             Species::Igglybuff => &Igglybuff,
//             Species::Jigglypuff => &Jigglypuff,
//             Species::Wigglytuff => &Wigglytuff,
//             Species::Zubat => &Zubat,
//             Species::Golbat => &Golbat,
//             Species::Crobat => &Crobat,
//             Species::Oddish => &Oddish,
//             Species::Gloom => &Gloom,
//             Species::Vileplume => &Vileplume,
//             Species::Bellossom => &Bellossom,
//             Species::Paras => &Paras,
//             Species::Parasect => &Parasect,
//             Species::Venonat => &Venonat,
//             Species::Venomoth => &Venomoth,
//             Species::Diglett => &Diglett,
//             Species::Dugtrio => &Dugtrio,
//             Species::Meowth => &Meowth,
//             Species::Persian => &Persian,
//             Species::Psyduck => &Psyduck,
//             Species::Golduck => &Golduck,
//             Species::Mankey => &Mankey,
//             Species::Primeape => &Primeape,
//             Species::Growlithe => &Growlithe,
//             Species::Arcanine => &Arcanine,
//             Species::Poliwag => &Poliwag,
//             Species::Poliwhirl => &Poliwhirl,
//             Species::Poliwrath => &Poliwrath,
//             Species::Politoed => &Politoed,
//             Species::Abra => &Abra,
//             Species::Kadabra => &Kadabra,
//             Species::Alakazam => &Alakazam,
//             Species::Machop => &Machop,
//             Species::Machoke => &Machoke,
//             Species::Machamp => &Machamp,
//             Species::Bellsprout => &Bellsprout,
//             Species::Weepinbell => &Weepinbell,
//             Species::Victreebel => &Victreebel,
//             Species::Tentacool => &Tentacool,
//             Species::Tentacruel => &Tentacruel,
//             Species::Geodude => &Geodude,
//             Species::Graveler => &Graveler,
//             Species::Golem => &Golem,
//             Species::Ponyta => &Ponyta,
//             Species::Rapidash => &Rapidash,
//             Species::Slowpoke => &Slowpoke,
//             Species::Slowbro => &Slowbro,
//             Species::Slowking => &Slowking,
//             Species::Magnemite => &Magnemite,
//             Species::Magneton => &Magneton,
//             Species::Magnezone => &Magnezone,
//             Species::Farfetchd => &Farfetchd,
//             Species::Doduo => &Doduo,
//             Species::Dodrio => &Dodrio,
//             Species::Seel => &Seel,
//             Species::Dewgong => &Dewgong,
//             Species::Grimer => &Grimer,
//             Species::Muk => &Muk,
//             Species::Shellder => &Shellder,
//             Species::Cloyster => &Cloyster,
//             Species::Gastly => &Gastly,
//             Species::Haunter => &Haunter,
//             Species::Gengar => &Gengar,
//             Species::Onix => &Onix,
//             Species::Steelix => &Steelix,
//             Species::Drowzee => &Drowzee,
//             Species::Hypno => &Hypno,
//             Species::Krabby => &Krabby,
//             Species::Kingler => &Kingler,
//             Species::Voltorb => &Voltorb,
//             Species::Electrode => &Electrode,
//             Species::Exeggcute => &Exeggcute,
//             Species::Exeggutor => &Exeggutor,
//             Species::Cubone => &Cubone,
//             Species::Marowak => &Marowak,
//             Species::Tyrogue => &Tyrogue,
//             Species::Hitmonlee => &Hitmonlee,
//             Species::Hitmonchan => &Hitmonchan,
//             Species::Hitmontop => &Hitmontop,
//             Species::Lickitung => &Lickitung,
//             Species::Lickilicky => &Lickilicky,
//             Species::Koffing => &Koffing,
//             Species::Weezing => &Weezing,
//             Species::Rhyhorn => &Rhyhorn,
//             Species::Rhydon => &Rhydon,
//             Species::Rhyperior => &Rhyperior,
//             Species::Happiny => &Happiny,
//             Species::Chansey => &Chansey,
//             Species::Blissey => &Blissey,
//             Species::Tangela => &Tangela,
//             Species::Tangrowth => &Tangrowth,
//             Species::Kangaskhan => &Kangaskhan,
//             Species::Horsea => &Horsea,
//             Species::Seadra => &Seadra,
//             Species::Kingdra => &Kingdra,
//             Species::Goldeen => &Goldeen,
//             Species::Seaking => &Seaking,
//             Species::Staryu => &Staryu,
//             Species::Starmie => &Starmie,
//             Species::MimeJr => &MimeJr,
//             Species::MrMime => &MrMime,
//             Species::Scyther => &Scyther,
//             Species::Scizor => &Scizor,
//             Species::Smoochum => &Smoochum,
//             Species::Jynx => &Jynx,
//             Species::Elekid => &Elekid,
//             Species::Electabuzz => &Electabuzz,
//             Species::Electivire => &Electivire,
//             Species::Magby => &Magby,
//             Species::Magmar => &Magmar,
//             Species::Magmortar => &Magmortar,
//             Species::Pinsir => &Pinsir,
//             Species::Tauros => &Tauros,
//             Species::Magikarp => &Magikarp,
//             Species::Gyarados => &Gyarados,
//             Species::Lapras => &Lapras,
//             Species::Ditto => &Ditto,
//             Species::Eevee => &Eevee,
//             Species::Vaporeon => &Vaporeon,
//             Species::Jolteon => &Jolteon,
//             Species::Flareon => &Flareon,
//             Species::Espeon => &Espeon,
//             Species::Umbreon => &Umbreon,
//             Species::Leafeon => &Leafeon,
//             Species::Glaceon => &Glaceon,
//             //Species::Sylveon => &Sylveon,
//             Species::Porygon => &Porygon,
//             Species::Porygon2 => &Porygon2,
//             Species::PorygonZ => &PorygonZ,
//             Species::Omanyte => &Omanyte,
//             Species::Omastar => &Omastar,
//             Species::Kabuto => &Kabuto,
//             Species::Kabutops => &Kabutops,
//             Species::Aerodactyl => &Aerodactyl,
//             Species::Munchlax => &Munchlax,
//             Species::Snorlax => &Snorlax,
//             Species::Articuno => &Articuno,
//             Species::Zapdos => &Zapdos,
//             Species::Moltres => &Moltres,
//             Species::Dratini => &Dratini,
//             Species::Dragonair => &Dragonair,
//             Species::Dragonite => &Dragonite,
//             Species::Mewtwo => &Mewtwo,
//             Species::Mew => &Mew,
//             Species::Chikorita => &Chikorita,
//             Species::Bayleef => &Bayleef,
//             Species::Meganium => &Meganium,
//             Species::Cyndaquil => &Cyndaquil,
//             Species::Quilava => &Quilava,
//             Species::Typhlosion => &Typhlosion,
//             Species::Totodile => &Totodile,
//             Species::Croconaw => &Croconaw,
//             Species::Feraligatr => &Feraligatr,
//             Species::Sentret => &Sentret,
//             Species::Furret => &Furret,
//             Species::Hoothoot => &Hoothoot,
//             Species::Noctowl => &Noctowl,
//             Species::Ledyba => &Ledyba,
//             Species::Ledian => &Ledian,
//             Species::Spinarak => &Spinarak,
//             Species::Ariados => &Ariados,
//             Species::Chinchou => &Chinchou,
//             Species::Lanturn => &Lanturn,
//             Species::Togepi => &Togepi,
//             Species::Togetic => &Togetic,
//             Species::Togekiss => &Togekiss,
//             Species::Natu => &Natu,
//             Species::Xatu => &Xatu,
//             Species::Mareep => &Mareep,
//             Species::Flaaffy => &Flaaffy,
//             Species::Ampharos => &Ampharos,
//             Species::Azurill => &Azurill,
//             Species::Marill => &Marill,
//             Species::Azumarill => &Azumarill,
//             Species::Bonsly => &Bonsly,
//             Species::Sudowoodo => &Sudowoodo,
//             Species::Hoppip => &Hoppip,
//             Species::Skiploom => &Skiploom,
//             Species::Jumpluff => &Jumpluff,
//             Species::Aipom => &Aipom,
//             Species::Ambipom => &Ambipom,
//             Species::Sunkern => &Sunkern,
//             Species::Sunflora => &Sunflora,
//             Species::Yanma => &Yanma,
//             Species::Yanmega => &Yanmega,
//             Species::Wooper => &Wooper,
//             Species::Quagsire => &Quagsire,
//             Species::Murkrow => &Murkrow,
//             Species::Honchkrow => &Honchkrow,
//             Species::Misdreavus => &Misdreavus,
//             Species::Mismagius => &Mismagius,
//             Species::Unown(_) => &Unown,
//             Species::Wynaut => &Wynaut,
//             Species::Wobbuffet => &Wobbuffet,
//             Species::Girafarig => &Girafarig,
//             Species::Pineco => &Pineco,
//             Species::Forretress => &Forretress,
//             Species::Dunsparce => &Dunsparce,
//             Species::Gligar => &Gligar,
//             Species::Gliscor => &Gliscor,
//             Species::Snubbull => &Snubbull,
//             Species::Granbull => &Granbull,
//             Species::Qwilfish => &Qwilfish,
//             Species::Shuckle => &Shuckle,
//             Species::Heracross => &Heracross,
//             Species::Sneasel => &Sneasel,
//             Species::Weavile => &Weavile,
//             Species::Teddiursa => &Teddiursa,
//             Species::Ursaring => &Ursaring,
//             Species::Slugma => &Slugma,
//             Species::Magcargo => &Magcargo,
//             Species::Swinub => &Swinub,
//             Species::Piloswine => &Piloswine,
//             Species::Mamoswine => &Mamoswine,
//             Species::Corsola => &Corsola,
//             Species::Remoraid => &Remoraid,
//             Species::Octillery => &Octillery,
//             Species::Delibird => &Delibird,
//             Species::Mantyke => &Mantyke,
//             Species::Mantine => &Mantine,
//             Species::Skarmory => &Skarmory,
//             Species::Houndour => &Houndour,
//             Species::Houndoom => &Houndoom,
//             Species::Phanpy => &Phanpy,
//             Species::Donphan => &Donphan,
//             Species::Stantler => &Stantler,
//             Species::Smeargle => &Smeargle,
//             Species::Miltank => &Miltank,
//             Species::Raikou => &Raikou,
//             Species::Entei => &Entei,
//             Species::Suicune => &Suicune,
//             Species::Larvitar => &Larvitar,
//             Species::Pupitar => &Pupitar,
//             Species::Tyranitar => &Tyranitar,
//             Species::Lugia => &Lugia,
//             Species::HoOh => &HoOh,
//             Species::Celebi => &Celebi,
//             Species::Treecko => &Treecko,
//             Species::Grovyle => &Grovyle,
//             Species::Sceptile => &Sceptile,
//             Species::Torchic => &Torchic,
//             Species::Combusken => &Combusken,
//             Species::Blaziken => &Blaziken,
//             Species::Mudkip => &Mudkip,
//             Species::Marshtomp => &Marshtomp,
//             Species::Swampert => &Swampert,
//             Species::Poochyena => &Poochyena,
//             Species::Mightyena => &Mightyena,
//             Species::Zigzagoon => &Zigzagoon,
//             Species::Linoone => &Linoone,
//             Species::Wurmple => &Wurmple,
//             Species::Silcoon => &Silcoon,
//             Species::Beautifly => &Beautifly,
//             Species::Cascoon => &Cascoon,
//             Species::Dustox => &Dustox,
//             Species::Lotad => &Lotad,
//             Species::Lombre => &Lombre,
//             Species::Ludicolo => &Ludicolo,
//             Species::Seedot => &Seedot,
//             Species::Nuzleaf => &Nuzleaf,
//             Species::Shiftry => &Shiftry,
//             Species::Taillow => &Taillow,
//             Species::Swellow => &Swellow,
//             Species::Wingull => &Wingull,
//             Species::Pelipper => &Pelipper,
//             Species::Ralts => &Ralts,
//             Species::Kirlia => &Kirlia,
//             Species::Gardevoir => &Gardevoir,
//             Species::Gallade => &Gallade,
//             Species::Surskit => &Surskit,
//             Species::Masquerain => &Masquerain,
//             Species::Shroomish => &Shroomish,
//             Species::Breloom => &Breloom,
//             Species::Slakoth => &Slakoth,
//             Species::Vigoroth => &Vigoroth,
//             Species::Slaking => &Slaking,
//             Species::Nincada => &Nincada,
//             Species::Ninjask => &Ninjask,
//             Species::Shedinja => &Shedinja,
//             Species::Whismur => &Whismur,
//             Species::Loudred => &Loudred,
//             Species::Exploud => &Exploud,
//             Species::Makuhita => &Makuhita,
//             Species::Hariyama => &Hariyama,
//             Species::Nosepass => &Nosepass,
//             Species::Probopass => &Probopass,
//             Species::Skitty => &Skitty,
//             Species::Delcatty => &Delcatty,
//             Species::Sableye => &Sableye,
//             Species::Mawile => &Mawile,
//             Species::Aron => &Aron,
//             Species::Lairon => &Lairon,
//             Species::Aggron => &Aggron,
//             Species::Meditite => &Meditite,
//             Species::Medicham => &Medicham,
//             Species::Electrike => &Electrike,
//             Species::Manectric => &Manectric,
//             Species::Plusle => &Plusle,
//             Species::Minun => &Minun,
//             Species::Volbeat => &Volbeat,
//             Species::Illumise => &Illumise,
//             Species::Budew => &Budew,
//             Species::Roselia => &Roselia,
//             Species::Roserade => &Roserade,
//             Species::Gulpin => &Gulpin,
//             Species::Swalot => &Swalot,
//             Species::Carvanha => &Carvanha,
//             Species::Sharpedo => &Sharpedo,
//             Species::Wailmer => &Wailmer,
//             Species::Wailord => &Wailord,
//             Species::Numel => &Numel,
//             Species::Camerupt => &Camerupt,
//             Species::Torkoal => &Torkoal,
//             Species::Spoink => &Spoink,
//             Species::Grumpig => &Grumpig,
//             Species::Spinda => &Spinda,
//             Species::Trapinch => &Trapinch,
//             Species::Vibrava => &Vibrava,
//             Species::Flygon => &Flygon,
//             Species::Cacnea => &Cacnea,
//             Species::Cacturne => &Cacturne,
//             Species::Swablu => &Swablu,
//             Species::Altaria => &Altaria,
//             Species::Zangoose => &Zangoose,
//             Species::Seviper => &Seviper,
//             Species::Lunatone => &Lunatone,
//             Species::Solrock => &Solrock,
//             Species::Barboach => &Barboach,
//             Species::Whiscash => &Whiscash,
//             Species::Corphish => &Corphish,
//             Species::Crawdaunt => &Crawdaunt,
//             Species::Baltoy => &Baltoy,
//             Species::Claydol => &Claydol,
//             Species::Lileep => &Lileep,
//             Species::Cradily => &Cradily,
//             Species::Anorith => &Anorith,
//             Species::Armaldo => &Armaldo,
//             Species::Feebas => &Feebas,
//             Species::Milotic => &Milotic,
//             Species::Castform(_) => &Castform,
//             Species::Kecleon => &Kecleon,
//             Species::Shuppet => &Shuppet,
//             Species::Banette => &Banette,
//             Species::Duskull => &Duskull,
//             Species::Dusclops => &Dusclops,
//             Species::Dusknoir => &Dusknoir,
//             Species::Tropius => &Tropius,
//             Species::Chingling => &Chingling,
//             Species::Chimecho => &Chimecho,
//             Species::Absol => &Absol,
//             Species::Snorunt => &Snorunt,
//             Species::Glalie => &Glalie,
//             Species::Froslass => &Froslass,
//             Species::Spheal => &Spheal,
//             Species::Sealeo => &Sealeo,
//             Species::Walrein => &Walrein,
//             Species::Clamperl => &Clamperl,
//             Species::Huntail => &Huntail,
//             Species::Gorebyss => &Gorebyss,
//             Species::Relicanth => &Relicanth,
//             Species::Luvdisc => &Luvdisc,
//             Species::Bagon => &Bagon,
//             Species::Shelgon => &Shelgon,
//             Species::Salamence => &Salamence,
//             Species::Beldum => &Beldum,
//             Species::Metang => &Metang,
//             Species::Metagross => &Metagross,
//             Species::Regirock => &Regirock,
//             Species::Regice => &Regice,
//             Species::Registeel => &Registeel,
//             Species::Latias => &Latias,
//             Species::Latios => &Latios,
//             Species::Kyogre => &Kyogre,
//             Species::Groudon => &Groudon,
//             Species::Rayquaza => &Rayquaza,
//             Species::Jirachi => &Jirachi,
//             Species::Deoxys(_) => &Deoxys,
//             Species::Turtwig => &Turtwig,
//             Species::Grotle => &Grotle,
//             Species::Torterra => &Torterra,
//             Species::Chimchar => &Chimchar,
//             Species::Monferno => &Monferno,
//             Species::Infernape => &Infernape,
//             Species::Piplup => &Piplup,
//             Species::Prinplup => &Prinplup,
//             Species::Empoleon => &Empoleon,
//             Species::Starly => &Starly,
//             Species::Staravia => &Staravia,
//             Species::Staraptor => &Staraptor,
//             Species::Bidoof => &Bidoof,
//             Species::Bibarel => &Bibarel,
//             Species::Kricketot => &Kricketot,
//             Species::Kricketune => &Kricketune,
//             Species::Shinx => &Shinx,
//             Species::Luxio => &Luxio,
//             Species::Luxray => &Luxray,
//             Species::Cranidos => &Cranidos,
//             Species::Rampardos => &Rampardos,
//             Species::Shieldon => &Shieldon,
//             Species::Bastiodon => &Bastiodon,
//             Species::Burmy(BurmyWormadamForm::Plant) => &Burmy_Plant,
//             Species::Burmy(BurmyWormadamForm::Sandy) => &Burmy_Sandy,
//             Species::Burmy(BurmyWormadamForm::Trash) => &Burmy_Trash,
//             Species::Wormadam(BurmyWormadamForm::Plant) => &Wormadam_Plant,
//             Species::Wormadam(BurmyWormadamForm::Sandy) => &Wormadam_Sandy,
//             Species::Wormadam(BurmyWormadamForm::Trash) => &Wormadam_Trash,
//             Species::Mothim => &Mothim,
//             Species::Combee => &Combee,
//             Species::Vespiquen => &Vespiquen,
//             Species::Pachirisu => &Pachirisu,
//             Species::Buizel => &Buizel,
//             Species::Floatzel => &Floatzel,
//             Species::Cherubi => &Cherubi,
//             Species::Cherrim(_) => &Cherrim,
//             Species::Shellos(ShellosGastrodonForm::WestSea) => &Shellos_WestSea,
//             Species::Shellos(ShellosGastrodonForm::EastSea) => &Shellos_EastSea,
//             Species::Gastrodon(ShellosGastrodonForm::WestSea) => &Gastrodon_WestSea,
//             Species::Gastrodon(ShellosGastrodonForm::EastSea) => &Gastrodon_EastSea,
//             Species::Drifloon => &Drifloon,
//             Species::Drifblim => &Drifblim,
//             Species::Buneary => &Buneary,
//             Species::Lopunny => &Lopunny,
//             Species::Glameow => &Glameow,
//             Species::Purugly => &Purugly,
//             Species::Stunky => &Stunky,
//             Species::Skuntank => &Skuntank,
//             Species::Bronzor => &Bronzor,
//             Species::Bronzong => &Bronzong,
//             Species::Chatot => &Chatot,
//             Species::Spiritomb => &Spiritomb,
//             Species::Gible => &Gible,
//             Species::Gabite => &Gabite,
//             Species::Garchomp => &Garchomp,
//             Species::Riolu => &Riolu,
//             Species::Lucario => &Lucario,
//             Species::Hippopotas => &Hippopotas,
//             Species::Hippowdon => &Hippowdon,
//             Species::Skorupi => &Skorupi,
//             Species::Drapion => &Drapion,
//             Species::Croagunk => &Croagunk,
//             Species::Toxicroak => &Toxicroak,
//             Species::Carnivine => &Carnivine,
//             Species::Finneon => &Finneon,
//             Species::Lumineon => &Lumineon,
//             Species::Snover => &Snover,
//             Species::Abomasnow => &Abomasnow,
//             Species::Rotom(_) => &Rotom,
//             Species::Uxie => &Uxie,
//             Species::Mesprit => &Mesprit,
//             Species::Azelf => &Azelf,
//             Species::Dialga => &Dialga,
//             Species::Palkia => &Palkia,
//             Species::Heatran => &Heatran,
//             Species::Regigigas => &Regigigas,
//             Species::Giratina(_) => &Giratina,
//             Species::Cresselia => &Cresselia,
//             Species::Phione => &Phione,
//             Species::Manaphy => &Manaphy,
//             Species::Darkrai => &Darkrai,
//             Species::Shaymin(_) => &Shaymin,
//             Species::Arceus(_) => &Arceus,
//             Species::Victini => &Victini,
//             Species::Snivy => &Snivy,
//             Species::Servine => &Servine,
//             Species::Serperior => &Serperior,
//             Species::Tepig => &Tepig,
//             Species::Pignite => &Pignite,
//             Species::Emboar => &Emboar,
//             Species::Oshawott => &Oshawott,
//             Species::Dewott => &Dewott,
//             Species::Samurott => &Samurott,
//             Species::Patrat => &Patrat,
//             Species::Watchog => &Watchog,
//             Species::Lillipup => &Lillipup,
//             Species::Herdier => &Herdier,
//             Species::Stoutland => &Stoutland,
//             Species::Purrloin => &Purrloin,
//             Species::Liepard => &Liepard,
//             Species::Pansage => &Pansage,
//             Species::Simisage => &Simisage,
//             Species::Pansear => &Pansear,
//             Species::Simisear => &Simisear,
//             Species::Panpour => &Panpour,
//             Species::Simipour => &Simipour,
//             Species::Munna => &Munna,
//             Species::Musharna => &Musharna,
//             Species::Pidove => &Pidove,
//             Species::Tranquill => &Tranquill,
//             Species::Unfezant => &Unfezant,
//             Species::Blitzle => &Blitzle,
//             Species::Zebstrika => &Zebstrika,
//             Species::Roggenrola => &Roggenrola,
//             Species::Boldore => &Boldore,
//             Species::Gigalith => &Gigalith,
//             Species::Woobat => &Woobat,
//             Species::Swoobat => &Swoobat,
//             Species::Drilbur => &Drilbur,
//             Species::Excadrill => &Excadrill,
//             Species::Audino => &Audino,
//             Species::Timburr => &Timburr,
//             Species::Gurdurr => &Gurdurr,
//             Species::Conkeldurr => &Conkeldurr,
//             Species::Tympole => &Tympole,
//             Species::Palpitoad => &Palpitoad,
//             Species::Seismitoad => &Seismitoad,
//             Species::Throh => &Throh,
//             Species::Sawk => &Sawk,
//             Species::Sewaddle => &Sewaddle,
//             Species::Swadloon => &Swadloon,
//             Species::Leavanny => &Leavanny,
//             Species::Venipede => &Venipede,
//             Species::Whirlipede => &Whirlipede,
//             Species::Scolipede => &Scolipede,
//             Species::Cottonee => &Cottonee,
//             Species::Whimsicott => &Whimsicott,
//             Species::Petilil => &Petilil,
//             Species::Lilligant => &Lilligant,
//             Species::Basculin(BasculinForm::RedStriped) => &Basculin_RedStriped,
//             Species::Basculin(BasculinForm::BlueStriped) => &Basculin_BlueStriped,
//             Species::Sandile => &Sandile,
//             Species::Krokorok => &Krokorok,
//             Species::Krookodile => &Krookodile,
//             Species::Darumaka => &Darumaka,
//             Species::Darmanitan(_) => &Darmanitan,
//             Species::Maractus => &Maractus,
//             Species::Dwebble => &Dwebble,
//             Species::Crustle => &Crustle,
//             Species::Scraggy => &Scraggy,
//             Species::Scrafty => &Scrafty,
//             Species::Sigilyph => &Sigilyph,
//             Species::Yamask => &Yamask,
//             Species::Cofagrigus => &Cofagrigus,
//             Species::Tirtouga => &Tirtouga,
//             Species::Carracosta => &Carracosta,
//             Species::Archen => &Archen,
//             Species::Archeops => &Archeops,
//             Species::Trubbish => &Trubbish,
//             Species::Garbodor => &Garbodor,
//             Species::Zorua => &Zorua,
//             Species::Zoroark => &Zoroark,
//             Species::Minccino => &Minccino,
//             Species::Cinccino => &Cinccino,
//             Species::Gothita => &Gothita,
//             Species::Gothorita => &Gothorita,
//             Species::Gothitelle => &Gothitelle,
//             Species::Solosis => &Solosis,
//             Species::Duosion => &Duosion,
//             Species::Reuniclus => &Reuniclus,
//             Species::Ducklett => &Ducklett,
//             Species::Swanna => &Swanna,
//             Species::Vanillite => &Vanillite,
//             Species::Vanillish => &Vanillish,
//             Species::Vanilluxe => &Vanilluxe,
//             Species::Deerling(Season::Spring) => &Deerling_Spring,
//             Species::Sawsbuck(Season::Spring) => &Sawsbuck_Spring,
//             Species::Deerling(Season::Summer) => &Deerling_Summer,
//             Species::Sawsbuck(Season::Summer) => &Sawsbuck_Summer,
//             Species::Deerling(Season::Autumn) => &Deerling_Fall,
//             Species::Sawsbuck(Season::Autumn) => &Sawsbuck_Fall,
//             Species::Deerling(Season::Winter) => &Deerling_Winter,
//             Species::Sawsbuck(Season::Winter) => &Sawsbuck_Winter,
//             Species::Emolga => &Emolga,
//             Species::Karrablast => &Karrablast,
//             Species::Escavalier => &Escavalier,
//             Species::Foongus => &Foongus,
//             Species::Amoonguss => &Amoonguss,
//             Species::Frillish => &Frillish,
//             Species::Jellicent => &Jellicent,
//             Species::Alomomola => &Alomomola,
//             Species::Joltik => &Joltik,
//             Species::Galvantula => &Galvantula,
//             Species::Ferroseed => &Ferroseed,
//             Species::Ferrothorn => &Ferrothorn,
//             Species::Klink => &Klink,
//             Species::Klang => &Klang,
//             Species::Klinklang => &Klinklang,
//             Species::Tynamo => &Tynamo,
//             Species::Eelektrik => &Eelektrik,
//             Species::Eelektross => &Eelektross,
//             Species::Elgyem => &Elgyem,
//             Species::Beheeyem => &Beheeyem,
//             Species::Litwick => &Litwick,
//             Species::Lampent => &Lampent,
//             Species::Chandelure => &Chandelure,
//             Species::Axew => &Axew,
//             Species::Fraxure => &Fraxure,
//             Species::Haxorus => &Haxorus,
//             Species::Cubchoo => &Cubchoo,
//             Species::Beartic => &Beartic,
//             Species::Cryogonal => &Cryogonal,
//             Species::Shelmet => &Shelmet,
//             Species::Accelgor => &Accelgor,
//             Species::Stunfisk => &Stunfisk,
//             Species::Mienfoo => &Mienfoo,
//             Species::Mienshao => &Mienshao,
//             Species::Druddigon => &Druddigon,
//             Species::Golett => &Golett,
//             Species::Golurk => &Golurk,
//             Species::Pawniard => &Pawniard,
//             Species::Bisharp => &Bisharp,
//             Species::Bouffalant => &Bouffalant,
//             Species::Rufflet => &Rufflet,
//             Species::Braviary => &Braviary,
//             Species::Vullaby => &Vullaby,
//             Species::Mandibuzz => &Mandibuzz,
//             Species::Heatmor => &Heatmor,
//             Species::Durant => &Durant,
//             Species::Deino => &Deino,
//             Species::Zweilous => &Zweilous,
//             Species::Hydreigon => &Hydreigon,
//             Species::Larvesta => &Larvesta,
//             Species::Volcarona => &Volcarona,
//             Species::Cobalion => &Cobalion,
//             Species::Terrakion => &Terrakion,
//             Species::Virizion => &Virizion,
//             Species::Tornadus(_) => &Tornadus,
//             Species::Thundurus(_) => &Thundurus,
//             Species::Reshiram => &Reshiram,
//             Species::Zekrom => &Zekrom,
//             Species::Landorus(_) => &Landorus,
//             Species::Kyurem(_) => &Kyurem,
//             Species::Keldeo(_) => &Keldeo,
//             Species::Meloetta(_) => &Meloetta,
//             Species::Genesect(_) => &Genesect
//         }
//     }
//
//     /// Check if this Pokemon is fully evolved or not
//     pub fn is_fully_evolved(&self) -> bool {
//         self.get_evolutions().paths.len() == 0
//     }
// }
//
// //region Evolution Constants
// pub static Bulbasaur: Evolution = Evolution {
// 	base: Species::Bulbasaur,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ivysaur, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Ivysaur: Evolution = Evolution {
// 	base: Species::Bulbasaur,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Venusaur, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Venusaur: Evolution = Evolution {
// 	base: Species::Bulbasaur,
// 	baby: None,
// 	paths: &[]
// };
// pub static Charmander: Evolution = Evolution {
// 	base: Species::Charmander,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Charmeleon, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Charmeleon: Evolution = Evolution {
// 	base: Species::Charmander,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Charizard, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Charizard: Evolution = Evolution {
// 	base: Species::Charmander,
// 	baby: None,
// 	paths: &[]
// };
// pub static Squirtle: Evolution = Evolution {
// 	base: Species::Squirtle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Wartortle, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Wartortle: Evolution = Evolution {
// 	base: Species::Squirtle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Blastoise, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Blastoise: Evolution = Evolution {
// 	base: Species::Squirtle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Caterpie: Evolution = Evolution {
// 	base: Species::Caterpie,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Metapod, trigger: EvolutionTrigger::AtLevel(7) }]
// };
// pub static Metapod: Evolution = Evolution {
// 	base: Species::Caterpie,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Butterfree, trigger: EvolutionTrigger::AtLevel(10) }]
// };
// pub static Butterfree: Evolution = Evolution {
// 	base: Species::Caterpie,
// 	baby: None,
// 	paths: &[]
// };
// pub static Weedle: Evolution = Evolution {
// 	base: Species::Weedle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kakuna, trigger: EvolutionTrigger::AtLevel(7) }]
// };
// pub static Kakuna: Evolution = Evolution {
// 	base: Species::Weedle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Beedrill, trigger: EvolutionTrigger::AtLevel(10) }]
// };
// pub static Beedrill: Evolution = Evolution {
// 	base: Species::Weedle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pidgey: Evolution = Evolution {
// 	base: Species::Pidgey,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Pidgeotto, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Pidgeotto: Evolution = Evolution {
// 	base: Species::Pidgey,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Pidgeot, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Pidgeot: Evolution = Evolution {
// 	base: Species::Pidgey,
// 	baby: None,
// 	paths: &[]
// };
// pub static Rattata: Evolution = Evolution {
// 	base: Species::Rattata,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Raticate, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Raticate: Evolution = Evolution {
// 	base: Species::Rattata,
// 	baby: None,
// 	paths: &[]
// };
// pub static Spearow: Evolution = Evolution {
// 	base: Species::Spearow,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Fearow, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Fearow: Evolution = Evolution {
// 	base: Species::Spearow,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ekans: Evolution = Evolution {
// 	base: Species::Ekans,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Arbok, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Arbok: Evolution = Evolution {
// 	base: Species::Ekans,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pichu: Evolution = Evolution {
// 	base: Species::Pichu,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Pikachu, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Pikachu: Evolution = Evolution {
// 	base: Species::Pichu,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Raichu, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
// };
// pub static Raichu: Evolution = Evolution {
// 	base: Species::Pichu,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sandshrew: Evolution = Evolution {
// 	base: Species::Sandshrew,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sandslash, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Sandslash: Evolution = Evolution {
// 	base: Species::Sandshrew,
// 	baby: None,
// 	paths: &[]
// };
// pub static NidoranF: Evolution = Evolution {
// 	base: Species::NidoranF,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Nidorina, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Nidorina: Evolution = Evolution {
// 	base: Species::NidoranF,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Nidoqueen, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
// };
// pub static Nidoqueen: Evolution = Evolution {
// 	base: Species::NidoranF,
// 	baby: None,
// 	paths: &[]
// };
// pub static NidoranM: Evolution = Evolution {
// 	base: Species::NidoranM,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Nidorino, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Nidorino: Evolution = Evolution {
// 	base: Species::NidoranM,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Nidoking, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
// };
// pub static Nidoking: Evolution = Evolution {
// 	base: Species::NidoranM,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cleffa: Evolution = Evolution {
// 	base: Species::Cleffa,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Clefairy, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Clefairy: Evolution = Evolution {
// 	base: Species::Cleffa,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Clefable, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
// };
// pub static Clefable: Evolution = Evolution {
// 	base: Species::Cleffa,
// 	baby: None,
// 	paths: &[]
// };
// pub static Vulpix: Evolution = Evolution {
// 	base: Species::Vulpix,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ninetales, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }]
// };
// pub static Ninetales: Evolution = Evolution {
// 	base: Species::Vulpix,
// 	baby: None,
// 	paths: &[]
// };
// pub static Igglybuff: Evolution = Evolution {
// 	base: Species::Igglybuff,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Jigglypuff, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Jigglypuff: Evolution = Evolution {
// 	base: Species::Igglybuff,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Wigglytuff, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
// };
// pub static Wigglytuff: Evolution = Evolution {
// 	base: Species::Igglybuff,
// 	baby: None,
// 	paths: &[]
// };
// pub static Zubat: Evolution = Evolution {
// 	base: Species::Zubat,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Golbat, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Golbat: Evolution = Evolution {
// 	base: Species::Zubat,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Crobat, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Crobat: Evolution = Evolution {
// 	base: Species::Zubat,
// 	baby: None,
// 	paths: &[]
// };
// pub static Oddish: Evolution = Evolution {
// 	base: Species::Oddish,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gloom, trigger: EvolutionTrigger::AtLevel(21) }]
// };
// pub static Gloom: Evolution = Evolution {
// 	base: Species::Oddish,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vileplume, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }, EvolutionPath { to: Species::Bellossom, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
// };
// pub static Vileplume: Evolution = Evolution {
// 	base: Species::Oddish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Bellossom: Evolution = Evolution {
// 	base: Species::Oddish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Paras: Evolution = Evolution {
// 	base: Species::Paras,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Parasect, trigger: EvolutionTrigger::AtLevel(24) }]
// };
// pub static Parasect: Evolution = Evolution {
// 	base: Species::Paras,
// 	baby: None,
// 	paths: &[]
// };
// pub static Venonat: Evolution = Evolution {
// 	base: Species::Venonat,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Venomoth, trigger: EvolutionTrigger::AtLevel(31) }]
// };
// pub static Venomoth: Evolution = Evolution {
// 	base: Species::Venonat,
// 	baby: None,
// 	paths: &[]
// };
// pub static Diglett: Evolution = Evolution {
// 	base: Species::Diglett,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dugtrio, trigger: EvolutionTrigger::AtLevel(26) }]
// };
// pub static Dugtrio: Evolution = Evolution {
// 	base: Species::Diglett,
// 	baby: None,
// 	paths: &[]
// };
// pub static Meowth: Evolution = Evolution {
// 	base: Species::Meowth,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Persian, trigger: EvolutionTrigger::AtLevel(28) }]
// };
// pub static Persian: Evolution = Evolution {
// 	base: Species::Meowth,
// 	baby: None,
// 	paths: &[]
// };
// pub static Psyduck: Evolution = Evolution {
// 	base: Species::Psyduck,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Golduck, trigger: EvolutionTrigger::AtLevel(33) }]
// };
// pub static Golduck: Evolution = Evolution {
// 	base: Species::Psyduck,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mankey: Evolution = Evolution {
// 	base: Species::Mankey,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Primeape, trigger: EvolutionTrigger::AtLevel(28) }]
// };
// pub static Primeape: Evolution = Evolution {
// 	base: Species::Mankey,
// 	baby: None,
// 	paths: &[]
// };
// pub static Growlithe: Evolution = Evolution {
// 	base: Species::Growlithe,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Arcanine, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }]
// };
// pub static Arcanine: Evolution = Evolution {
// 	base: Species::Growlithe,
// 	baby: None,
// 	paths: &[]
// };
// pub static Poliwag: Evolution = Evolution {
// 	base: Species::Poliwag,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Poliwhirl, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Poliwhirl: Evolution = Evolution {
// 	base: Species::Poliwag,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Poliwrath, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }, EvolutionPath { to: Species::Politoed, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::KingsRock) }]
// };
// pub static Poliwrath: Evolution = Evolution {
// 	base: Species::Poliwag,
// 	baby: None,
// 	paths: &[]
// };
// pub static Politoed: Evolution = Evolution {
// 	base: Species::Poliwag,
// 	baby: None,
// 	paths: &[]
// };
// pub static Abra: Evolution = Evolution {
// 	base: Species::Abra,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kadabra, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Kadabra: Evolution = Evolution {
// 	base: Species::Abra,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Alakazam, trigger: EvolutionTrigger::Trading }]
// };
// pub static Alakazam: Evolution = Evolution {
// 	base: Species::Abra,
// 	baby: None,
// 	paths: &[]
// };
// pub static Machop: Evolution = Evolution {
// 	base: Species::Machop,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Machoke, trigger: EvolutionTrigger::AtLevel(28) }]
// };
// pub static Machoke: Evolution = Evolution {
// 	base: Species::Machop,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Machamp, trigger: EvolutionTrigger::Trading }]
// };
// pub static Machamp: Evolution = Evolution {
// 	base: Species::Machop,
// 	baby: None,
// 	paths: &[]
// };
// pub static Bellsprout: Evolution = Evolution {
// 	base: Species::Bellsprout,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Weepinbell, trigger: EvolutionTrigger::AtLevel(21) }]
// };
// pub static Weepinbell: Evolution = Evolution {
// 	base: Species::Bellsprout,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Victreebel, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
// };
// pub static Victreebel: Evolution = Evolution {
// 	base: Species::Bellsprout,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tentacool: Evolution = Evolution {
// 	base: Species::Tentacool,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Tentacruel, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Tentacruel: Evolution = Evolution {
// 	base: Species::Tentacool,
// 	baby: None,
// 	paths: &[]
// };
// pub static Geodude: Evolution = Evolution {
// 	base: Species::Geodude,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Graveler, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Graveler: Evolution = Evolution {
// 	base: Species::Geodude,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Golem, trigger: EvolutionTrigger::Trading }]
// };
// pub static Golem: Evolution = Evolution {
// 	base: Species::Geodude,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ponyta: Evolution = Evolution {
// 	base: Species::Ponyta,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Rapidash, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Rapidash: Evolution = Evolution {
// 	base: Species::Ponyta,
// 	baby: None,
// 	paths: &[]
// };
// pub static Slowpoke: Evolution = Evolution {
// 	base: Species::Slowpoke,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Slowbro, trigger: EvolutionTrigger::AtLevel(37) }, EvolutionPath { to: Species::Slowking, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::KingsRock) }]
// };
// pub static Slowbro: Evolution = Evolution {
// 	base: Species::Slowpoke,
// 	baby: None,
// 	paths: &[]
// };
// pub static Slowking: Evolution = Evolution {
// 	base: Species::Slowpoke,
// 	baby: None,
// 	paths: &[]
// };
// pub static Magnemite: Evolution = Evolution {
// 	base: Species::Magnemite,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Magneton, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Magneton: Evolution = Evolution {
// 	base: Species::Magnemite,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Magnezone, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::MagneticField) }, EvolutionPath { to: Species::Magnezone, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
// };
// pub static Magnezone: Evolution = Evolution {
// 	base: Species::Magnemite,
// 	baby: None,
// 	paths: &[]
// };
// pub static Farfetchd: Evolution = Evolution {
// 	base: Species::Farfetchd,
// 	baby: None,
// 	paths: &[]
// };
// pub static Doduo: Evolution = Evolution {
// 	base: Species::Doduo,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dodrio, trigger: EvolutionTrigger::AtLevel(31) }]
// };
// pub static Dodrio: Evolution = Evolution {
// 	base: Species::Doduo,
// 	baby: None,
// 	paths: &[]
// };
// pub static Seel: Evolution = Evolution {
// 	base: Species::Seel,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dewgong, trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Dewgong: Evolution = Evolution {
// 	base: Species::Seel,
// 	baby: None,
// 	paths: &[]
// };
// pub static Grimer: Evolution = Evolution {
// 	base: Species::Grimer,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Muk, trigger: EvolutionTrigger::AtLevel(38) }]
// };
// pub static Muk: Evolution = Evolution {
// 	base: Species::Grimer,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shellder: Evolution = Evolution {
// 	base: Species::Shellder,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Cloyster, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
// };
// pub static Cloyster: Evolution = Evolution {
// 	base: Species::Shellder,
// 	baby: None,
// 	paths: &[]
// };
// pub static Gastly: Evolution = Evolution {
// 	base: Species::Gastly,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Haunter, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Haunter: Evolution = Evolution {
// 	base: Species::Gastly,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gengar, trigger: EvolutionTrigger::Trading }]
// };
// pub static Gengar: Evolution = Evolution {
// 	base: Species::Gastly,
// 	baby: None,
// 	paths: &[]
// };
// pub static Onix: Evolution = Evolution {
// 	base: Species::Onix,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Steelix, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::MetalCoat) }]
// };
// pub static Steelix: Evolution = Evolution {
// 	base: Species::Onix,
// 	baby: None,
// 	paths: &[]
// };
// pub static Drowzee: Evolution = Evolution {
// 	base: Species::Drowzee,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Hypno, trigger: EvolutionTrigger::AtLevel(26) }]
// };
// pub static Hypno: Evolution = Evolution {
// 	base: Species::Drowzee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Krabby: Evolution = Evolution {
// 	base: Species::Krabby,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kingler, trigger: EvolutionTrigger::AtLevel(28) }]
// };
// pub static Kingler: Evolution = Evolution {
// 	base: Species::Krabby,
// 	baby: None,
// 	paths: &[]
// };
// pub static Voltorb: Evolution = Evolution {
// 	base: Species::Voltorb,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Electrode, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Electrode: Evolution = Evolution {
// 	base: Species::Voltorb,
// 	baby: None,
// 	paths: &[]
// };
// pub static Exeggcute: Evolution = Evolution {
// 	base: Species::Exeggcute,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Exeggutor, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
// };
// pub static Exeggutor: Evolution = Evolution {
// 	base: Species::Exeggcute,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cubone: Evolution = Evolution {
// 	base: Species::Cubone,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Marowak, trigger: EvolutionTrigger::AtLevel(28) }]
// };
// pub static Marowak: Evolution = Evolution {
// 	base: Species::Cubone,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tyrogue: Evolution = Evolution {
// 	base: Species::Tyrogue,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Hitmonlee, trigger: EvolutionTrigger::AtLevelWithHighStat(20, EvolutionTriggerStats::Attack) }, EvolutionPath { to: Species::Hitmonchan, trigger: EvolutionTrigger::AtLevelWithHighStat(20, EvolutionTriggerStats::Defense) }, EvolutionPath { to: Species::Hitmontop, trigger: EvolutionTrigger::AtLevelWithHighStat(20, EvolutionTriggerStats::Equal) }]
// };
// pub static Hitmonlee: Evolution = Evolution {
// 	base: Species::Tyrogue,
// 	baby: None,
// 	paths: &[]
// };
// pub static Hitmonchan: Evolution = Evolution {
// 	base: Species::Tyrogue,
// 	baby: None,
// 	paths: &[]
// };
// pub static Hitmontop: Evolution = Evolution {
// 	base: Species::Tyrogue,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lickitung: Evolution = Evolution {
// 	base: Species::Lickitung,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lickilicky, trigger: EvolutionTrigger::KnowsMove(Move::Rollout) }]
// };
// pub static Lickilicky: Evolution = Evolution {
// 	base: Species::Lickitung,
// 	baby: None,
// 	paths: &[]
// };
// pub static Koffing: Evolution = Evolution {
// 	base: Species::Koffing,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Weezing, trigger: EvolutionTrigger::AtLevel(35) }]
// };
// pub static Weezing: Evolution = Evolution {
// 	base: Species::Koffing,
// 	baby: None,
// 	paths: &[]
// };
// pub static Rhyhorn: Evolution = Evolution {
// 	base: Species::Rhyhorn,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Rhydon, trigger: EvolutionTrigger::AtLevel(42) }]
// };
// pub static Rhydon: Evolution = Evolution {
// 	base: Species::Rhyhorn,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Rhyperior, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::Protector) }]
// };
// pub static Rhyperior: Evolution = Evolution {
// 	base: Species::Rhyhorn,
// 	baby: None,
// 	paths: &[]
// };
// pub static Happiny: Evolution = Evolution {
// 	base: Species::Chansey,
// 	baby: Some(IncenseBaby {species: Species::Happiny, incense: Incense::LuckIncense}),
// 	paths: &[EvolutionPath { to: Species::Chansey, trigger: EvolutionTrigger::HoldingItemAtTime(EvolutionHeldItem::OvalStone, EvolutionTriggerTime::Day) }]
// };
// pub static Chansey: Evolution = Evolution {
// 	base: Species::Chansey,
// 	baby: Some(IncenseBaby {species: Species::Happiny, incense: Incense::LuckIncense}),
// 	paths: &[EvolutionPath { to: Species::Blissey, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Blissey: Evolution = Evolution {
// 	base: Species::Chansey,
// 	baby: Some(IncenseBaby {species: Species::Happiny, incense: Incense::LuckIncense}),
// 	paths: &[]
// };
// pub static Tangela: Evolution = Evolution {
// 	base: Species::Tangela,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Tangrowth, trigger: EvolutionTrigger::KnowsMove(Move::AncientPower) }]
// };
// pub static Tangrowth: Evolution = Evolution {
// 	base: Species::Tangela,
// 	baby: None,
// 	paths: &[]
// };
// pub static Kangaskhan: Evolution = Evolution {
// 	base: Species::Kangaskhan,
// 	baby: None,
// 	paths: &[]
// };
// pub static Horsea: Evolution = Evolution {
// 	base: Species::Horsea,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Seadra, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Seadra: Evolution = Evolution {
// 	base: Species::Horsea,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kingdra, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::DragonScale) }]
// };
// pub static Kingdra: Evolution = Evolution {
// 	base: Species::Horsea,
// 	baby: None,
// 	paths: &[]
// };
// pub static Goldeen: Evolution = Evolution {
// 	base: Species::Goldeen,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Seaking, trigger: EvolutionTrigger::AtLevel(33) }]
// };
// pub static Seaking: Evolution = Evolution {
// 	base: Species::Goldeen,
// 	baby: None,
// 	paths: &[]
// };
// pub static Staryu: Evolution = Evolution {
// 	base: Species::Staryu,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Starmie, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
// };
// pub static Starmie: Evolution = Evolution {
// 	base: Species::Staryu,
// 	baby: None,
// 	paths: &[]
// };
// pub static MimeJr: Evolution = Evolution {
// 	base: Species::MrMime,
// 	baby: Some(IncenseBaby {species: Species::MimeJr, incense: Incense::OddIncense}),
// 	paths: &[EvolutionPath { to: Species::MrMime, trigger: EvolutionTrigger::KnowsMove(Move::Mimic) }]
// };
// pub static MrMime: Evolution = Evolution {
// 	base: Species::MrMime,
// 	baby: Some(IncenseBaby {species: Species::MimeJr, incense: Incense::OddIncense}),
// 	paths: &[]
// };
// pub static Scyther: Evolution = Evolution {
// 	base: Species::Scyther,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Scizor, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::MetalCoat) }]
// };
// pub static Scizor: Evolution = Evolution {
// 	base: Species::Scyther,
// 	baby: None,
// 	paths: &[]
// };
// pub static Smoochum: Evolution = Evolution {
// 	base: Species::Smoochum,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Jynx, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Jynx: Evolution = Evolution {
// 	base: Species::Smoochum,
// 	baby: None,
// 	paths: &[]
// };
// pub static Elekid: Evolution = Evolution {
// 	base: Species::Elekid,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Electabuzz, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Electabuzz: Evolution = Evolution {
// 	base: Species::Elekid,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Electivire, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::Electirizer) }]
// };
// pub static Electivire: Evolution = Evolution {
// 	base: Species::Elekid,
// 	baby: None,
// 	paths: &[]
// };
// pub static Magby: Evolution = Evolution {
// 	base: Species::Magby,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Magmar, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Magmar: Evolution = Evolution {
// 	base: Species::Magby,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Magmortar, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::Magmarizer) }]
// };
// pub static Magmortar: Evolution = Evolution {
// 	base: Species::Magby,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pinsir: Evolution = Evolution {
// 	base: Species::Pinsir,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tauros: Evolution = Evolution {
// 	base: Species::Tauros,
// 	baby: None,
// 	paths: &[]
// };
// pub static Magikarp: Evolution = Evolution {
// 	base: Species::Magikarp,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gyarados, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Gyarados: Evolution = Evolution {
// 	base: Species::Magikarp,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lapras: Evolution = Evolution {
// 	base: Species::Lapras,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ditto: Evolution = Evolution {
// 	base: Species::Ditto,
// 	baby: None,
// 	paths: &[]
// };
// pub static Eevee: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vaporeon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }, EvolutionPath { to: Species::Jolteon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }, EvolutionPath { to: Species::Flareon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }, EvolutionPath { to: Species::Espeon, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Day) }, EvolutionPath { to: Species::Umbreon, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Night) }, EvolutionPath { to: Species::Leafeon, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::MossRock) }, EvolutionPath { to: Species::Leafeon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }, EvolutionPath { to: Species::Glaceon, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::IceRock) }, EvolutionPath { to: Species::Glaceon, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::IceStone) }]
// };
// pub static Vaporeon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Jolteon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Flareon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Espeon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Umbreon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Leafeon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Glaceon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sylveon: Evolution = Evolution {
// 	base: Species::Eevee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Porygon: Evolution = Evolution {
// 	base: Species::Porygon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Porygon2, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::UpGrade) }]
// };
// pub static Porygon2: Evolution = Evolution {
// 	base: Species::Porygon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::PorygonZ, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::DubiousDisk) }]
// };
// pub static PorygonZ: Evolution = Evolution {
// 	base: Species::Porygon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Omanyte: Evolution = Evolution {
// 	base: Species::Omanyte,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Omastar, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Omastar: Evolution = Evolution {
// 	base: Species::Omanyte,
// 	baby: None,
// 	paths: &[]
// };
// pub static Kabuto: Evolution = Evolution {
// 	base: Species::Kabuto,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kabutops, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Kabutops: Evolution = Evolution {
// 	base: Species::Kabuto,
// 	baby: None,
// 	paths: &[]
// };
// pub static Aerodactyl: Evolution = Evolution {
// 	base: Species::Aerodactyl,
// 	baby: None,
// 	paths: &[]
// };
// pub static Munchlax: Evolution = Evolution {
// 	base: Species::Snorlax,
// 	baby: Some(IncenseBaby {species: Species::Munchlax, incense: Incense::FullIncense}),
// 	paths: &[EvolutionPath { to: Species::Snorlax, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Snorlax: Evolution = Evolution {
// 	base: Species::Snorlax,
// 	baby: Some(IncenseBaby {species: Species::Munchlax, incense: Incense::FullIncense}),
// 	paths: &[]
// };
// pub static Articuno: Evolution = Evolution {
// 	base: Species::Articuno,
// 	baby: None,
// 	paths: &[]
// };
// pub static Zapdos: Evolution = Evolution {
// 	base: Species::Zapdos,
// 	baby: None,
// 	paths: &[]
// };
// pub static Moltres: Evolution = Evolution {
// 	base: Species::Moltres,
// 	baby: None,
// 	paths: &[]
// };
// pub static Dratini: Evolution = Evolution {
// 	base: Species::Dratini,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dragonair, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Dragonair: Evolution = Evolution {
// 	base: Species::Dratini,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dragonite, trigger: EvolutionTrigger::AtLevel(55) }]
// };
// pub static Dragonite: Evolution = Evolution {
// 	base: Species::Dratini,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mewtwo: Evolution = Evolution {
// 	base: Species::Mewtwo,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mew: Evolution = Evolution {
// 	base: Species::Mew,
// 	baby: None,
// 	paths: &[]
// };
// pub static Chikorita: Evolution = Evolution {
// 	base: Species::Chikorita,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Bayleef, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Bayleef: Evolution = Evolution {
// 	base: Species::Chikorita,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Meganium, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Meganium: Evolution = Evolution {
// 	base: Species::Chikorita,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cyndaquil: Evolution = Evolution {
// 	base: Species::Cyndaquil,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Quilava, trigger: EvolutionTrigger::AtLevel(14) }]
// };
// pub static Quilava: Evolution = Evolution {
// 	base: Species::Cyndaquil,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Typhlosion, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Typhlosion: Evolution = Evolution {
// 	base: Species::Cyndaquil,
// 	baby: None,
// 	paths: &[]
// };
// pub static Totodile: Evolution = Evolution {
// 	base: Species::Totodile,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Croconaw, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Croconaw: Evolution = Evolution {
// 	base: Species::Totodile,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Feraligatr, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Feraligatr: Evolution = Evolution {
// 	base: Species::Totodile,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sentret: Evolution = Evolution {
// 	base: Species::Sentret,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Furret, trigger: EvolutionTrigger::AtLevel(15) }]
// };
// pub static Furret: Evolution = Evolution {
// 	base: Species::Sentret,
// 	baby: None,
// 	paths: &[]
// };
// pub static Hoothoot: Evolution = Evolution {
// 	base: Species::Hoothoot,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Noctowl, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Noctowl: Evolution = Evolution {
// 	base: Species::Hoothoot,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ledyba: Evolution = Evolution {
// 	base: Species::Ledyba,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ledian, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Ledian: Evolution = Evolution {
// 	base: Species::Ledyba,
// 	baby: None,
// 	paths: &[]
// };
// pub static Spinarak: Evolution = Evolution {
// 	base: Species::Spinarak,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ariados, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Ariados: Evolution = Evolution {
// 	base: Species::Spinarak,
// 	baby: None,
// 	paths: &[]
// };
// pub static Chinchou: Evolution = Evolution {
// 	base: Species::Chinchou,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lanturn, trigger: EvolutionTrigger::AtLevel(27) }]
// };
// pub static Lanturn: Evolution = Evolution {
// 	base: Species::Chinchou,
// 	baby: None,
// 	paths: &[]
// };
// pub static Togepi: Evolution = Evolution {
// 	base: Species::Togepi,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Togetic, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Togetic: Evolution = Evolution {
// 	base: Species::Togepi,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Togekiss, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ShinyStone) }]
// };
// pub static Togekiss: Evolution = Evolution {
// 	base: Species::Togepi,
// 	baby: None,
// 	paths: &[]
// };
// pub static Natu: Evolution = Evolution {
// 	base: Species::Natu,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Xatu, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Xatu: Evolution = Evolution {
// 	base: Species::Natu,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mareep: Evolution = Evolution {
// 	base: Species::Mareep,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Flaaffy, trigger: EvolutionTrigger::AtLevel(15) }]
// };
// pub static Flaaffy: Evolution = Evolution {
// 	base: Species::Mareep,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ampharos, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Ampharos: Evolution = Evolution {
// 	base: Species::Mareep,
// 	baby: None,
// 	paths: &[]
// };
// pub static Azurill: Evolution = Evolution {
// 	base: Species::Marill,
// 	baby: Some(IncenseBaby {species: Species::Azurill, incense: Incense::SeaIncense}),
// 	paths: &[EvolutionPath { to: Species::Marill, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Marill: Evolution = Evolution {
// 	base: Species::Marill,
// 	baby: Some(IncenseBaby {species: Species::Azurill, incense: Incense::SeaIncense}),
// 	paths: &[EvolutionPath { to: Species::Azumarill, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Azumarill: Evolution = Evolution {
// 	base: Species::Marill,
// 	baby: Some(IncenseBaby {species: Species::Azurill, incense: Incense::SeaIncense}),
// 	paths: &[]
// };
// pub static Bonsly: Evolution = Evolution {
// 	base: Species::Sudowoodo,
// 	baby: Some(IncenseBaby {species: Species::Bonsly, incense: Incense::RockIncense}),
// 	paths: &[EvolutionPath { to: Species::Sudowoodo, trigger: EvolutionTrigger::KnowsMove(Move::Mimic) }]
// };
// pub static Sudowoodo: Evolution = Evolution {
// 	base: Species::Sudowoodo,
// 	baby: Some(IncenseBaby {species: Species::Bonsly, incense: Incense::RockIncense}),
// 	paths: &[]
// };
// pub static Hoppip: Evolution = Evolution {
// 	base: Species::Hoppip,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Skiploom, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Skiploom: Evolution = Evolution {
// 	base: Species::Hoppip,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Jumpluff, trigger: EvolutionTrigger::AtLevel(27) }]
// };
// pub static Jumpluff: Evolution = Evolution {
// 	base: Species::Hoppip,
// 	baby: None,
// 	paths: &[]
// };
// pub static Aipom: Evolution = Evolution {
// 	base: Species::Aipom,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ambipom, trigger: EvolutionTrigger::KnowsMove(Move::DoubleHit) }]
// };
// pub static Ambipom: Evolution = Evolution {
// 	base: Species::Aipom,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sunkern: Evolution = Evolution {
// 	base: Species::Sunkern,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sunflora, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
// };
// pub static Sunflora: Evolution = Evolution {
// 	base: Species::Sunkern,
// 	baby: None,
// 	paths: &[]
// };
// pub static Yanma: Evolution = Evolution {
// 	base: Species::Yanma,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Yanmega, trigger: EvolutionTrigger::KnowsMove(Move::AncientPower) }]
// };
// pub static Yanmega: Evolution = Evolution {
// 	base: Species::Yanma,
// 	baby: None,
// 	paths: &[]
// };
// pub static Wooper: Evolution = Evolution {
// 	base: Species::Wooper,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Quagsire, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Quagsire: Evolution = Evolution {
// 	base: Species::Wooper,
// 	baby: None,
// 	paths: &[]
// };
// pub static Murkrow: Evolution = Evolution {
// 	base: Species::Murkrow,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Honchkrow, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::DuskStone) }]
// };
// pub static Honchkrow: Evolution = Evolution {
// 	base: Species::Murkrow,
// 	baby: None,
// 	paths: &[]
// };
// pub static Misdreavus: Evolution = Evolution {
// 	base: Species::Misdreavus,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Mismagius, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::DuskStone) }]
// };
// pub static Mismagius: Evolution = Evolution {
// 	base: Species::Misdreavus,
// 	baby: None,
// 	paths: &[]
// };
// pub static Unown: Evolution = Evolution {
// 	base: Species::Unown(UnownForm::A),
// 	baby: None,
// 	paths: &[]
// };
// pub static Wynaut: Evolution = Evolution {
// 	base: Species::Wobbuffet,
// 	baby: Some(IncenseBaby {species: Species::Wynaut, incense: Incense::LaxIncense}),
// 	paths: &[EvolutionPath { to: Species::Wobbuffet, trigger: EvolutionTrigger::AtLevel(15) }]
// };
// pub static Wobbuffet: Evolution = Evolution {
// 	base: Species::Wobbuffet,
// 	baby: Some(IncenseBaby {species: Species::Wynaut, incense: Incense::LaxIncense}),
// 	paths: &[]
// };
// pub static Girafarig: Evolution = Evolution {
// 	base: Species::Girafarig,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pineco: Evolution = Evolution {
// 	base: Species::Pineco,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Forretress, trigger: EvolutionTrigger::AtLevel(31) }]
// };
// pub static Forretress: Evolution = Evolution {
// 	base: Species::Pineco,
// 	baby: None,
// 	paths: &[]
// };
// pub static Dunsparce: Evolution = Evolution {
// 	base: Species::Dunsparce,
// 	baby: None,
// 	paths: &[]
// };
// pub static Gligar: Evolution = Evolution {
// 	base: Species::Gligar,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gliscor, trigger: EvolutionTrigger::HoldingItemAtTime(EvolutionHeldItem::RazorFang, EvolutionTriggerTime::Night) }]
// };
// pub static Gliscor: Evolution = Evolution {
// 	base: Species::Gligar,
// 	baby: None,
// 	paths: &[]
// };
// pub static Snubbull: Evolution = Evolution {
// 	base: Species::Snubbull,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Granbull, trigger: EvolutionTrigger::AtLevel(23) }]
// };
// pub static Granbull: Evolution = Evolution {
// 	base: Species::Snubbull,
// 	baby: None,
// 	paths: &[]
// };
// pub static Qwilfish: Evolution = Evolution {
// 	base: Species::Qwilfish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shuckle: Evolution = Evolution {
// 	base: Species::Shuckle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Heracross: Evolution = Evolution {
// 	base: Species::Heracross,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sneasel: Evolution = Evolution {
// 	base: Species::Sneasel,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Weavile, trigger: EvolutionTrigger::HoldingItemAtTime(EvolutionHeldItem::RazorClaw, EvolutionTriggerTime::Night) }]
// };
// pub static Weavile: Evolution = Evolution {
// 	base: Species::Sneasel,
// 	baby: None,
// 	paths: &[]
// };
// pub static Teddiursa: Evolution = Evolution {
// 	base: Species::Teddiursa,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ursaring, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Ursaring: Evolution = Evolution {
// 	base: Species::Teddiursa,
// 	baby: None,
// 	paths: &[]
// };
// pub static Slugma: Evolution = Evolution {
// 	base: Species::Slugma,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Magcargo, trigger: EvolutionTrigger::AtLevel(38) }]
// };
// pub static Magcargo: Evolution = Evolution {
// 	base: Species::Slugma,
// 	baby: None,
// 	paths: &[]
// };
// pub static Swinub: Evolution = Evolution {
// 	base: Species::Swinub,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Piloswine, trigger: EvolutionTrigger::AtLevel(33) }]
// };
// pub static Piloswine: Evolution = Evolution {
// 	base: Species::Swinub,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Mamoswine, trigger: EvolutionTrigger::KnowsMove(Move::AncientPower) }]
// };
// pub static Mamoswine: Evolution = Evolution {
// 	base: Species::Swinub,
// 	baby: None,
// 	paths: &[]
// };
// pub static Corsola: Evolution = Evolution {
// 	base: Species::Corsola,
// 	baby: None,
// 	paths: &[]
// };
// pub static Remoraid: Evolution = Evolution {
// 	base: Species::Remoraid,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Octillery, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Octillery: Evolution = Evolution {
// 	base: Species::Remoraid,
// 	baby: None,
// 	paths: &[]
// };
// pub static Delibird: Evolution = Evolution {
// 	base: Species::Delibird,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mantyke: Evolution = Evolution {
// 	base: Species::Mantine,
// 	baby: Some(IncenseBaby {species: Species::Mantyke, incense: Incense::WaveIncense}),
// 	paths: &[EvolutionPath { to: Species::Mantine, trigger: EvolutionTrigger::WithPartyPokemon(Species::Remoraid) }]
// };
// pub static Mantine: Evolution = Evolution {
// 	base: Species::Mantine,
// 	baby: Some(IncenseBaby {species: Species::Mantyke, incense: Incense::WaveIncense}),
// 	paths: &[]
// };
// pub static Skarmory: Evolution = Evolution {
// 	base: Species::Skarmory,
// 	baby: None,
// 	paths: &[]
// };
// pub static Houndour: Evolution = Evolution {
// 	base: Species::Houndour,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Houndoom, trigger: EvolutionTrigger::AtLevel(24) }]
// };
// pub static Houndoom: Evolution = Evolution {
// 	base: Species::Houndour,
// 	baby: None,
// 	paths: &[]
// };
// pub static Phanpy: Evolution = Evolution {
// 	base: Species::Phanpy,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Donphan, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Donphan: Evolution = Evolution {
// 	base: Species::Phanpy,
// 	baby: None,
// 	paths: &[]
// };
// pub static Stantler: Evolution = Evolution {
// 	base: Species::Stantler,
// 	baby: None,
// 	paths: &[]
// };
// pub static Smeargle: Evolution = Evolution {
// 	base: Species::Smeargle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Miltank: Evolution = Evolution {
// 	base: Species::Miltank,
// 	baby: None,
// 	paths: &[]
// };
// pub static Raikou: Evolution = Evolution {
// 	base: Species::Raikou,
// 	baby: None,
// 	paths: &[]
// };
// pub static Entei: Evolution = Evolution {
// 	base: Species::Entei,
// 	baby: None,
// 	paths: &[]
// };
// pub static Suicune: Evolution = Evolution {
// 	base: Species::Suicune,
// 	baby: None,
// 	paths: &[]
// };
// pub static Larvitar: Evolution = Evolution {
// 	base: Species::Larvitar,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Pupitar, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Pupitar: Evolution = Evolution {
// 	base: Species::Larvitar,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Tyranitar, trigger: EvolutionTrigger::AtLevel(55) }]
// };
// pub static Tyranitar: Evolution = Evolution {
// 	base: Species::Larvitar,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lugia: Evolution = Evolution {
// 	base: Species::Lugia,
// 	baby: None,
// 	paths: &[]
// };
// pub static HoOh: Evolution = Evolution {
// 	base: Species::HoOh,
// 	baby: None,
// 	paths: &[]
// };
// pub static Celebi: Evolution = Evolution {
// 	base: Species::Celebi,
// 	baby: None,
// 	paths: &[]
// };
// pub static Treecko: Evolution = Evolution {
// 	base: Species::Treecko,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Grovyle, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Grovyle: Evolution = Evolution {
// 	base: Species::Treecko,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sceptile, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Sceptile: Evolution = Evolution {
// 	base: Species::Treecko,
// 	baby: None,
// 	paths: &[]
// };
// pub static Torchic: Evolution = Evolution {
// 	base: Species::Torchic,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Combusken, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Combusken: Evolution = Evolution {
// 	base: Species::Torchic,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Blaziken, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Blaziken: Evolution = Evolution {
// 	base: Species::Torchic,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mudkip: Evolution = Evolution {
// 	base: Species::Mudkip,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Marshtomp, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Marshtomp: Evolution = Evolution {
// 	base: Species::Mudkip,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Swampert, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Swampert: Evolution = Evolution {
// 	base: Species::Mudkip,
// 	baby: None,
// 	paths: &[]
// };
// pub static Poochyena: Evolution = Evolution {
// 	base: Species::Poochyena,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Mightyena, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Mightyena: Evolution = Evolution {
// 	base: Species::Poochyena,
// 	baby: None,
// 	paths: &[]
// };
// pub static Zigzagoon: Evolution = Evolution {
// 	base: Species::Zigzagoon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Linoone, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Linoone: Evolution = Evolution {
// 	base: Species::Zigzagoon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Obstagoon: Evolution = Evolution {
// 	base: Species::Zigzagoon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Wurmple: Evolution = Evolution {
// 	base: Species::Wurmple,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Silcoon, trigger: EvolutionTrigger::AtLevelRandomly(7, EvolutionTriggerRandom::Low) }, EvolutionPath { to: Species::Cascoon, trigger: EvolutionTrigger::AtLevelRandomly(7, EvolutionTriggerRandom::High) }]
// };
// pub static Silcoon: Evolution = Evolution {
// 	base: Species::Wurmple,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Beautifly, trigger: EvolutionTrigger::AtLevel(10) }]
// };
// pub static Beautifly: Evolution = Evolution {
// 	base: Species::Wurmple,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cascoon: Evolution = Evolution {
// 	base: Species::Wurmple,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dustox, trigger: EvolutionTrigger::AtLevel(10) }]
// };
// pub static Dustox: Evolution = Evolution {
// 	base: Species::Wurmple,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lotad: Evolution = Evolution {
// 	base: Species::Lotad,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lombre, trigger: EvolutionTrigger::AtLevel(14) }]
// };
// pub static Lombre: Evolution = Evolution {
// 	base: Species::Lotad,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ludicolo, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
// };
// pub static Ludicolo: Evolution = Evolution {
// 	base: Species::Lotad,
// 	baby: None,
// 	paths: &[]
// };
// pub static Seedot: Evolution = Evolution {
// 	base: Species::Seedot,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Nuzleaf, trigger: EvolutionTrigger::AtLevel(14) }]
// };
// pub static Nuzleaf: Evolution = Evolution {
// 	base: Species::Seedot,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Shiftry, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
// };
// pub static Shiftry: Evolution = Evolution {
// 	base: Species::Seedot,
// 	baby: None,
// 	paths: &[]
// };
// pub static Taillow: Evolution = Evolution {
// 	base: Species::Taillow,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Swellow, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Swellow: Evolution = Evolution {
// 	base: Species::Taillow,
// 	baby: None,
// 	paths: &[]
// };
// pub static Wingull: Evolution = Evolution {
// 	base: Species::Wingull,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Pelipper, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Pelipper: Evolution = Evolution {
// 	base: Species::Wingull,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ralts: Evolution = Evolution {
// 	base: Species::Ralts,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kirlia, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Kirlia: Evolution = Evolution {
// 	base: Species::Ralts,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gardevoir, trigger: EvolutionTrigger::AtLevel(30) }, EvolutionPath { to: Species::Gallade, trigger: EvolutionTrigger::EvolutionStoneWithGender(EvolutionStone::DawnStone, EvolutionTriggerGender::Male) }]
// };
// pub static Gardevoir: Evolution = Evolution {
// 	base: Species::Ralts,
// 	baby: None,
// 	paths: &[]
// };
// pub static Gallade: Evolution = Evolution {
// 	base: Species::Ralts,
// 	baby: None,
// 	paths: &[]
// };
// pub static Surskit: Evolution = Evolution {
// 	base: Species::Surskit,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Masquerain, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Masquerain: Evolution = Evolution {
// 	base: Species::Surskit,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shroomish: Evolution = Evolution {
// 	base: Species::Shroomish,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Breloom, trigger: EvolutionTrigger::AtLevel(23) }]
// };
// pub static Breloom: Evolution = Evolution {
// 	base: Species::Shroomish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Slakoth: Evolution = Evolution {
// 	base: Species::Slakoth,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vigoroth, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Vigoroth: Evolution = Evolution {
// 	base: Species::Slakoth,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Slaking, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Slaking: Evolution = Evolution {
// 	base: Species::Slakoth,
// 	baby: None,
// 	paths: &[]
// };
// pub static Nincada: Evolution = Evolution {
// 	base: Species::Nincada,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ninjask, trigger: EvolutionTrigger::AtLevelSpawn(20, Species::Shedinja) }]
// };
// pub static Ninjask: Evolution = Evolution {
// 	base: Species::Nincada,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shedinja: Evolution = Evolution {
// 	base: Species::Nincada,
// 	baby: None,
// 	paths: &[]
// };
// pub static Whismur: Evolution = Evolution {
// 	base: Species::Whismur,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Loudred, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Loudred: Evolution = Evolution {
// 	base: Species::Whismur,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Exploud, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Exploud: Evolution = Evolution {
// 	base: Species::Whismur,
// 	baby: None,
// 	paths: &[]
// };
// pub static Makuhita: Evolution = Evolution {
// 	base: Species::Makuhita,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Hariyama, trigger: EvolutionTrigger::AtLevel(24) }]
// };
// pub static Hariyama: Evolution = Evolution {
// 	base: Species::Makuhita,
// 	baby: None,
// 	paths: &[]
// };
// pub static Nosepass: Evolution = Evolution {
// 	base: Species::Nosepass,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Probopass, trigger: EvolutionTrigger::AtLocation(EvolutionTriggerLocation::MagneticField) }, EvolutionPath { to: Species::Probopass, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
// };
// pub static Probopass: Evolution = Evolution {
// 	base: Species::Nosepass,
// 	baby: None,
// 	paths: &[]
// };
// pub static Skitty: Evolution = Evolution {
// 	base: Species::Skitty,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Delcatty, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
// };
// pub static Delcatty: Evolution = Evolution {
// 	base: Species::Skitty,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sableye: Evolution = Evolution {
// 	base: Species::Sableye,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mawile: Evolution = Evolution {
// 	base: Species::Mawile,
// 	baby: None,
// 	paths: &[]
// };
// pub static Aron: Evolution = Evolution {
// 	base: Species::Aron,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lairon, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Lairon: Evolution = Evolution {
// 	base: Species::Aron,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Aggron, trigger: EvolutionTrigger::AtLevel(42) }]
// };
// pub static Aggron: Evolution = Evolution {
// 	base: Species::Aron,
// 	baby: None,
// 	paths: &[]
// };
// pub static Meditite: Evolution = Evolution {
// 	base: Species::Meditite,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Medicham, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Medicham: Evolution = Evolution {
// 	base: Species::Meditite,
// 	baby: None,
// 	paths: &[]
// };
// pub static Electrike: Evolution = Evolution {
// 	base: Species::Electrike,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Manectric, trigger: EvolutionTrigger::AtLevel(26) }]
// };
// pub static Manectric: Evolution = Evolution {
// 	base: Species::Electrike,
// 	baby: None,
// 	paths: &[]
// };
// pub static Plusle: Evolution = Evolution {
// 	base: Species::Plusle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Minun: Evolution = Evolution {
// 	base: Species::Minun,
// 	baby: None,
// 	paths: &[]
// };
// pub static Volbeat: Evolution = Evolution {
// 	base: Species::Volbeat,
// 	baby: None,
// 	paths: &[]
// };
// pub static Illumise: Evolution = Evolution {
// 	base: Species::Illumise,
// 	baby: None,
// 	paths: &[]
// };
// pub static Budew: Evolution = Evolution {
// 	base: Species::Roselia,
// 	baby: Some(IncenseBaby {species: Species::Budew, incense: Incense::RoseIncense}),
// 	paths: &[EvolutionPath { to: Species::Roselia, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Day) }]
// };
// pub static Roselia: Evolution = Evolution {
// 	base: Species::Roselia,
// 	baby: Some(IncenseBaby {species: Species::Budew, incense: Incense::RoseIncense}),
// 	paths: &[EvolutionPath { to: Species::Roserade, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ShinyStone) }]
// };
// pub static Roserade: Evolution = Evolution {
// 	base: Species::Roselia,
// 	baby: Some(IncenseBaby {species: Species::Budew, incense: Incense::RoseIncense}),
// 	paths: &[]
// };
// pub static Gulpin: Evolution = Evolution {
// 	base: Species::Gulpin,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Swalot, trigger: EvolutionTrigger::AtLevel(26) }]
// };
// pub static Swalot: Evolution = Evolution {
// 	base: Species::Gulpin,
// 	baby: None,
// 	paths: &[]
// };
// pub static Carvanha: Evolution = Evolution {
// 	base: Species::Carvanha,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sharpedo, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Sharpedo: Evolution = Evolution {
// 	base: Species::Carvanha,
// 	baby: None,
// 	paths: &[]
// };
// pub static Wailmer: Evolution = Evolution {
// 	base: Species::Wailmer,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Wailord, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Wailord: Evolution = Evolution {
// 	base: Species::Wailmer,
// 	baby: None,
// 	paths: &[]
// };
// pub static Numel: Evolution = Evolution {
// 	base: Species::Numel,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Camerupt, trigger: EvolutionTrigger::AtLevel(33) }]
// };
// pub static Camerupt: Evolution = Evolution {
// 	base: Species::Numel,
// 	baby: None,
// 	paths: &[]
// };
// pub static Torkoal: Evolution = Evolution {
// 	base: Species::Torkoal,
// 	baby: None,
// 	paths: &[]
// };
// pub static Spoink: Evolution = Evolution {
// 	base: Species::Spoink,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Grumpig, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Grumpig: Evolution = Evolution {
// 	base: Species::Spoink,
// 	baby: None,
// 	paths: &[]
// };
// pub static Spinda: Evolution = Evolution {
// 	base: Species::Spinda,
// 	baby: None,
// 	paths: &[]
// };
// pub static Trapinch: Evolution = Evolution {
// 	base: Species::Trapinch,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vibrava, trigger: EvolutionTrigger::AtLevel(35) }]
// };
// pub static Vibrava: Evolution = Evolution {
// 	base: Species::Trapinch,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Flygon, trigger: EvolutionTrigger::AtLevel(45) }]
// };
// pub static Flygon: Evolution = Evolution {
// 	base: Species::Trapinch,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cacnea: Evolution = Evolution {
// 	base: Species::Cacnea,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Cacturne, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Cacturne: Evolution = Evolution {
// 	base: Species::Cacnea,
// 	baby: None,
// 	paths: &[]
// };
// pub static Swablu: Evolution = Evolution {
// 	base: Species::Swablu,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Altaria, trigger: EvolutionTrigger::AtLevel(35) }]
// };
// pub static Altaria: Evolution = Evolution {
// 	base: Species::Swablu,
// 	baby: None,
// 	paths: &[]
// };
// pub static Zangoose: Evolution = Evolution {
// 	base: Species::Zangoose,
// 	baby: None,
// 	paths: &[]
// };
// pub static Seviper: Evolution = Evolution {
// 	base: Species::Seviper,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lunatone: Evolution = Evolution {
// 	base: Species::Lunatone,
// 	baby: None,
// 	paths: &[]
// };
// pub static Solrock: Evolution = Evolution {
// 	base: Species::Solrock,
// 	baby: None,
// 	paths: &[]
// };
// pub static Barboach: Evolution = Evolution {
// 	base: Species::Barboach,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Whiscash, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Whiscash: Evolution = Evolution {
// 	base: Species::Barboach,
// 	baby: None,
// 	paths: &[]
// };
// pub static Corphish: Evolution = Evolution {
// 	base: Species::Corphish,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Crawdaunt, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Crawdaunt: Evolution = Evolution {
// 	base: Species::Corphish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Baltoy: Evolution = Evolution {
// 	base: Species::Baltoy,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Claydol, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Claydol: Evolution = Evolution {
// 	base: Species::Baltoy,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lileep: Evolution = Evolution {
// 	base: Species::Lileep,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Cradily, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Cradily: Evolution = Evolution {
// 	base: Species::Lileep,
// 	baby: None,
// 	paths: &[]
// };
// pub static Anorith: Evolution = Evolution {
// 	base: Species::Anorith,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Armaldo, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Armaldo: Evolution = Evolution {
// 	base: Species::Anorith,
// 	baby: None,
// 	paths: &[]
// };
// pub static Feebas: Evolution = Evolution {
// 	base: Species::Feebas,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Milotic, trigger: EvolutionTrigger::HighBeauty }, EvolutionPath { to: Species::Milotic, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::PrismScale) }]
// };
// pub static Milotic: Evolution = Evolution {
// 	base: Species::Feebas,
// 	baby: None,
// 	paths: &[]
// };
// pub static Castform: Evolution = Evolution {
// 	base: Species::Castform(CastformForm::Normal),
// 	baby: None,
// 	paths: &[]
// };
// pub static Kecleon: Evolution = Evolution {
// 	base: Species::Kecleon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shuppet: Evolution = Evolution {
// 	base: Species::Shuppet,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Banette, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Banette: Evolution = Evolution {
// 	base: Species::Shuppet,
// 	baby: None,
// 	paths: &[]
// };
// pub static Duskull: Evolution = Evolution {
// 	base: Species::Duskull,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dusclops, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Dusclops: Evolution = Evolution {
// 	base: Species::Duskull,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dusknoir, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::ReaperCloth) }]
// };
// pub static Dusknoir: Evolution = Evolution {
// 	base: Species::Duskull,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tropius: Evolution = Evolution {
// 	base: Species::Tropius,
// 	baby: None,
// 	paths: &[]
// };
// pub static Chingling: Evolution = Evolution {
// 	base: Species::Chimecho,
// 	baby: Some(IncenseBaby {species: Species::Chingling, incense: Incense::PureIncense}),
// 	paths: &[EvolutionPath { to: Species::Chimecho, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Night) }]
// };
// pub static Chimecho: Evolution = Evolution {
// 	base: Species::Chimecho,
// 	baby: Some(IncenseBaby {species: Species::Chingling, incense: Incense::PureIncense}),
// 	paths: &[]
// };
// pub static Absol: Evolution = Evolution {
// 	base: Species::Absol,
// 	baby: None,
// 	paths: &[]
// };
// pub static Snorunt: Evolution = Evolution {
// 	base: Species::Snorunt,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Glalie, trigger: EvolutionTrigger::AtLevel(42) }, EvolutionPath { to: Species::Froslass, trigger: EvolutionTrigger::EvolutionStoneWithGender(EvolutionStone::DawnStone, EvolutionTriggerGender::Female) }]
// };
// pub static Glalie: Evolution = Evolution {
// 	base: Species::Snorunt,
// 	baby: None,
// 	paths: &[]
// };
// pub static Froslass: Evolution = Evolution {
// 	base: Species::Snorunt,
// 	baby: None,
// 	paths: &[]
// };
// pub static Spheal: Evolution = Evolution {
// 	base: Species::Spheal,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sealeo, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Sealeo: Evolution = Evolution {
// 	base: Species::Spheal,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Walrein, trigger: EvolutionTrigger::AtLevel(44) }]
// };
// pub static Walrein: Evolution = Evolution {
// 	base: Species::Spheal,
// 	baby: None,
// 	paths: &[]
// };
// pub static Clamperl: Evolution = Evolution {
// 	base: Species::Clamperl,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Huntail, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::DeepSeaTooth) }, EvolutionPath { to: Species::Gorebyss, trigger: EvolutionTrigger::TradingWithItem(EvolutionHeldItem::DeepSeaScale) }]
// };
// pub static Huntail: Evolution = Evolution {
// 	base: Species::Clamperl,
// 	baby: None,
// 	paths: &[]
// };
// pub static Gorebyss: Evolution = Evolution {
// 	base: Species::Clamperl,
// 	baby: None,
// 	paths: &[]
// };
// pub static Relicanth: Evolution = Evolution {
// 	base: Species::Relicanth,
// 	baby: None,
// 	paths: &[]
// };
// pub static Luvdisc: Evolution = Evolution {
// 	base: Species::Luvdisc,
// 	baby: None,
// 	paths: &[]
// };
// pub static Bagon: Evolution = Evolution {
// 	base: Species::Bagon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Shelgon, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Shelgon: Evolution = Evolution {
// 	base: Species::Bagon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Salamence, trigger: EvolutionTrigger::AtLevel(50) }]
// };
// pub static Salamence: Evolution = Evolution {
// 	base: Species::Bagon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Beldum: Evolution = Evolution {
// 	base: Species::Beldum,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Metang, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Metang: Evolution = Evolution {
// 	base: Species::Beldum,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Metagross, trigger: EvolutionTrigger::AtLevel(45) }]
// };
// pub static Metagross: Evolution = Evolution {
// 	base: Species::Beldum,
// 	baby: None,
// 	paths: &[]
// };
// pub static Regirock: Evolution = Evolution {
// 	base: Species::Regirock,
// 	baby: None,
// 	paths: &[]
// };
// pub static Regice: Evolution = Evolution {
// 	base: Species::Regice,
// 	baby: None,
// 	paths: &[]
// };
// pub static Registeel: Evolution = Evolution {
// 	base: Species::Registeel,
// 	baby: None,
// 	paths: &[]
// };
// pub static Latias: Evolution = Evolution {
// 	base: Species::Latias,
// 	baby: None,
// 	paths: &[]
// };
// pub static Latios: Evolution = Evolution {
// 	base: Species::Latios,
// 	baby: None,
// 	paths: &[]
// };
// pub static Kyogre: Evolution = Evolution {
// 	base: Species::Kyogre,
// 	baby: None,
// 	paths: &[]
// };
// pub static Groudon: Evolution = Evolution {
// 	base: Species::Groudon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Rayquaza: Evolution = Evolution {
// 	base: Species::Rayquaza,
// 	baby: None,
// 	paths: &[]
// };
// pub static Jirachi: Evolution = Evolution {
// 	base: Species::Jirachi,
// 	baby: None,
// 	paths: &[]
// };
// pub static Deoxys: Evolution = Evolution {
// 	base: Species::Deoxys(DeoxysForm::Normal),
// 	baby: None,
// 	paths: &[]
// };
// pub static Turtwig: Evolution = Evolution {
// 	base: Species::Turtwig,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Grotle, trigger: EvolutionTrigger::AtLevel(18) }]
// };
// pub static Grotle: Evolution = Evolution {
// 	base: Species::Turtwig,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Torterra, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Torterra: Evolution = Evolution {
// 	base: Species::Turtwig,
// 	baby: None,
// 	paths: &[]
// };
// pub static Chimchar: Evolution = Evolution {
// 	base: Species::Chimchar,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Monferno, trigger: EvolutionTrigger::AtLevel(14) }]
// };
// pub static Monferno: Evolution = Evolution {
// 	base: Species::Chimchar,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Infernape, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Infernape: Evolution = Evolution {
// 	base: Species::Chimchar,
// 	baby: None,
// 	paths: &[]
// };
// pub static Piplup: Evolution = Evolution {
// 	base: Species::Piplup,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Prinplup, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Prinplup: Evolution = Evolution {
// 	base: Species::Piplup,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Empoleon, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Empoleon: Evolution = Evolution {
// 	base: Species::Piplup,
// 	baby: None,
// 	paths: &[]
// };
// pub static Starly: Evolution = Evolution {
// 	base: Species::Starly,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Staravia, trigger: EvolutionTrigger::AtLevel(14) }]
// };
// pub static Staravia: Evolution = Evolution {
// 	base: Species::Starly,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Staraptor, trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Staraptor: Evolution = Evolution {
// 	base: Species::Starly,
// 	baby: None,
// 	paths: &[]
// };
// pub static Bidoof: Evolution = Evolution {
// 	base: Species::Bidoof,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Bibarel, trigger: EvolutionTrigger::AtLevel(15) }]
// };
// pub static Bibarel: Evolution = Evolution {
// 	base: Species::Bidoof,
// 	baby: None,
// 	paths: &[]
// };
// pub static Kricketot: Evolution = Evolution {
// 	base: Species::Kricketot,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Kricketune, trigger: EvolutionTrigger::AtLevel(10) }]
// };
// pub static Kricketune: Evolution = Evolution {
// 	base: Species::Kricketot,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shinx: Evolution = Evolution {
// 	base: Species::Shinx,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Luxio, trigger: EvolutionTrigger::AtLevel(15) }]
// };
// pub static Luxio: Evolution = Evolution {
// 	base: Species::Shinx,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Luxray, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Luxray: Evolution = Evolution {
// 	base: Species::Shinx,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cranidos: Evolution = Evolution {
// 	base: Species::Cranidos,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Rampardos, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Rampardos: Evolution = Evolution {
// 	base: Species::Cranidos,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shieldon: Evolution = Evolution {
// 	base: Species::Shieldon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Bastiodon, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Bastiodon: Evolution = Evolution {
// 	base: Species::Shieldon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Burmy_Plant: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Plant),
// 	baby: None,
// 	paths: &[
// EvolutionPath { to: Species::Wormadam(BurmyWormadamForm::Plant), trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Female) },
// EvolutionPath { to: Species::Mothim, trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Male) }]
// };
// pub static Burmy_Sandy: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Plant),
// 	baby: None,
// 	paths: &[
// EvolutionPath { to: Species::Wormadam(BurmyWormadamForm::Sandy), trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Female) },
// EvolutionPath { to: Species::Mothim, trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Male) }]
// };
// pub static Burmy_Trash: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Plant),
// 	baby: None,
// 	paths: &[
// EvolutionPath { to: Species::Wormadam(BurmyWormadamForm::Trash), trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Female) },
// EvolutionPath { to: Species::Mothim, trigger: EvolutionTrigger::AtLevelWithGender(20, EvolutionTriggerGender::Male) }]
// };
// pub static Wormadam_Plant: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Plant),
// 	baby: None,
// 	paths: &[]
// };
// pub static Wormadam_Sandy: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Sandy),
// 	baby: None,
// 	paths: &[]
// };
// pub static Wormadam_Trash: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Trash),
// 	baby: None,
// 	paths: &[]
// };
// pub static Mothim: Evolution = Evolution {
// 	base: Species::Burmy(BurmyWormadamForm::Plant),
// 	baby: None,
// 	paths: &[]
// };
// pub static Combee: Evolution = Evolution {
// 	base: Species::Combee,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vespiquen, trigger: EvolutionTrigger::AtLevelWithGender(21, EvolutionTriggerGender::Female) }]
// };
// pub static Vespiquen: Evolution = Evolution {
// 	base: Species::Combee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pachirisu: Evolution = Evolution {
// 	base: Species::Pachirisu,
// 	baby: None,
// 	paths: &[]
// };
// pub static Buizel: Evolution = Evolution {
// 	base: Species::Buizel,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Floatzel, trigger: EvolutionTrigger::AtLevel(26) }]
// };
// pub static Floatzel: Evolution = Evolution {
// 	base: Species::Buizel,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cherubi: Evolution = Evolution {
// 	base: Species::Cherubi,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Cherrim(CherrimForm::Overcase), trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Cherrim: Evolution = Evolution {
// 	base: Species::Cherubi,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shellos_WestSea: Evolution = Evolution {
// 	base: Species::Shellos(ShellosGastrodonForm::WestSea),
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gastrodon(ShellosGastrodonForm::WestSea), trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Shellos_EastSea: Evolution = Evolution {
// 	base: Species::Shellos(ShellosGastrodonForm::EastSea),
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gastrodon(ShellosGastrodonForm::EastSea), trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Gastrodon_WestSea: Evolution = Evolution {
// 	base: Species::Shellos(ShellosGastrodonForm::WestSea),
// 	baby: None,
// 	paths: &[]
// };
// pub static Gastrodon_EastSea: Evolution = Evolution {
// 	base: Species::Shellos(ShellosGastrodonForm::EastSea),
// 	baby: None,
// 	paths: &[]
// };
// pub static Drifloon: Evolution = Evolution {
// 	base: Species::Drifloon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Drifblim, trigger: EvolutionTrigger::AtLevel(28) }]
// };
// pub static Drifblim: Evolution = Evolution {
// 	base: Species::Drifloon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Buneary: Evolution = Evolution {
// 	base: Species::Buneary,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lopunny, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Lopunny: Evolution = Evolution {
// 	base: Species::Buneary,
// 	baby: None,
// 	paths: &[]
// };
// pub static Glameow: Evolution = Evolution {
// 	base: Species::Glameow,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Purugly, trigger: EvolutionTrigger::AtLevel(38) }]
// };
// pub static Purugly: Evolution = Evolution {
// 	base: Species::Glameow,
// 	baby: None,
// 	paths: &[]
// };
// pub static Stunky: Evolution = Evolution {
// 	base: Species::Stunky,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Skuntank, trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Skuntank: Evolution = Evolution {
// 	base: Species::Stunky,
// 	baby: None,
// 	paths: &[]
// };
// pub static Bronzor: Evolution = Evolution {
// 	base: Species::Bronzor,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Bronzong, trigger: EvolutionTrigger::AtLevel(33) }]
// };
// pub static Bronzong: Evolution = Evolution {
// 	base: Species::Bronzor,
// 	baby: None,
// 	paths: &[]
// };
// pub static Chatot: Evolution = Evolution {
// 	base: Species::Chatot,
// 	baby: None,
// 	paths: &[]
// };
// pub static Spiritomb: Evolution = Evolution {
// 	base: Species::Spiritomb,
// 	baby: None,
// 	paths: &[]
// };
// pub static Gible: Evolution = Evolution {
// 	base: Species::Gible,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gabite, trigger: EvolutionTrigger::AtLevel(24) }]
// };
// pub static Gabite: Evolution = Evolution {
// 	base: Species::Gible,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Garchomp, trigger: EvolutionTrigger::AtLevel(48) }]
// };
// pub static Garchomp: Evolution = Evolution {
// 	base: Species::Gible,
// 	baby: None,
// 	paths: &[]
// };
// pub static Riolu: Evolution = Evolution {
// 	base: Species::Riolu,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lucario, trigger: EvolutionTrigger::HighFriendshipAtTime(EvolutionTriggerTime::Day) }]
// };
// pub static Lucario: Evolution = Evolution {
// 	base: Species::Riolu,
// 	baby: None,
// 	paths: &[]
// };
// pub static Hippopotas: Evolution = Evolution {
// 	base: Species::Hippopotas,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Hippowdon, trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Hippowdon: Evolution = Evolution {
// 	base: Species::Hippopotas,
// 	baby: None,
// 	paths: &[]
// };
// pub static Skorupi: Evolution = Evolution {
// 	base: Species::Skorupi,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Drapion, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Drapion: Evolution = Evolution {
// 	base: Species::Skorupi,
// 	baby: None,
// 	paths: &[]
// };
// pub static Croagunk: Evolution = Evolution {
// 	base: Species::Croagunk,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Toxicroak, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Toxicroak: Evolution = Evolution {
// 	base: Species::Croagunk,
// 	baby: None,
// 	paths: &[]
// };
// pub static Carnivine: Evolution = Evolution {
// 	base: Species::Carnivine,
// 	baby: None,
// 	paths: &[]
// };
// pub static Finneon: Evolution = Evolution {
// 	base: Species::Finneon,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lumineon, trigger: EvolutionTrigger::AtLevel(31) }]
// };
// pub static Lumineon: Evolution = Evolution {
// 	base: Species::Finneon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Snover: Evolution = Evolution {
// 	base: Species::Snover,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Abomasnow, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Abomasnow: Evolution = Evolution {
// 	base: Species::Snover,
// 	baby: None,
// 	paths: &[]
// };
// pub static Rotom: Evolution = Evolution {
// 	base: Species::Rotom(RotomForm::Normal),
// 	baby: None,
// 	paths: &[]
// };
// pub static Uxie: Evolution = Evolution {
// 	base: Species::Uxie,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mesprit: Evolution = Evolution {
// 	base: Species::Mesprit,
// 	baby: None,
// 	paths: &[]
// };
// pub static Azelf: Evolution = Evolution {
// 	base: Species::Azelf,
// 	baby: None,
// 	paths: &[]
// };
// pub static Dialga: Evolution = Evolution {
// 	base: Species::Dialga,
// 	baby: None,
// 	paths: &[]
// };
// pub static Palkia: Evolution = Evolution {
// 	base: Species::Palkia,
// 	baby: None,
// 	paths: &[]
// };
// pub static Heatran: Evolution = Evolution {
// 	base: Species::Heatran,
// 	baby: None,
// 	paths: &[]
// };
// pub static Regigigas: Evolution = Evolution {
// 	base: Species::Regigigas,
// 	baby: None,
// 	paths: &[]
// };
// pub static Giratina: Evolution = Evolution {
// 	base: Species::Giratina(GiratinaForm::Altered),
// 	baby: None,
// 	paths: &[]
// };
// pub static Cresselia: Evolution = Evolution {
// 	base: Species::Cresselia,
// 	baby: None,
// 	paths: &[]
// };
// pub static Phione: Evolution = Evolution {
// 	base: Species::Phione,
// 	baby: None,
// 	paths: &[]
// };
// pub static Manaphy: Evolution = Evolution {
// 	base: Species::Phione,
// 	baby: None,
// 	paths: &[]
// };
// pub static Darkrai: Evolution = Evolution {
// 	base: Species::Darkrai,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shaymin: Evolution = Evolution {
// 	base: Species::Shaymin(ShayminForm::Land),
// 	baby: None,
// 	paths: &[]
// };
// pub static Arceus: Evolution = Evolution {
// 	base: Species::Arceus(Type::Normal),
// 	baby: None,
// 	paths: &[]
// };
// pub static Victini: Evolution = Evolution {
// 	base: Species::Victini,
// 	baby: None,
// 	paths: &[]
// };
// pub static Snivy: Evolution = Evolution {
// 	base: Species::Snivy,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Servine, trigger: EvolutionTrigger::AtLevel(17) }]
// };
// pub static Servine: Evolution = Evolution {
// 	base: Species::Snivy,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Serperior, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Serperior: Evolution = Evolution {
// 	base: Species::Snivy,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tepig: Evolution = Evolution {
// 	base: Species::Tepig,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Pignite, trigger: EvolutionTrigger::AtLevel(17) }]
// };
// pub static Pignite: Evolution = Evolution {
// 	base: Species::Tepig,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Emboar, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Emboar: Evolution = Evolution {
// 	base: Species::Tepig,
// 	baby: None,
// 	paths: &[]
// };
// pub static Oshawott: Evolution = Evolution {
// 	base: Species::Oshawott,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Dewott, trigger: EvolutionTrigger::AtLevel(17) }]
// };
// pub static Dewott: Evolution = Evolution {
// 	base: Species::Oshawott,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Samurott, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Samurott: Evolution = Evolution {
// 	base: Species::Oshawott,
// 	baby: None,
// 	paths: &[]
// };
// pub static Patrat: Evolution = Evolution {
// 	base: Species::Patrat,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Watchog, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Watchog: Evolution = Evolution {
// 	base: Species::Patrat,
// 	baby: None,
// 	paths: &[]
// };
// pub static Lillipup: Evolution = Evolution {
// 	base: Species::Lillipup,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Herdier, trigger: EvolutionTrigger::AtLevel(16) }]
// };
// pub static Herdier: Evolution = Evolution {
// 	base: Species::Lillipup,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Stoutland, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Stoutland: Evolution = Evolution {
// 	base: Species::Lillipup,
// 	baby: None,
// 	paths: &[]
// };
// pub static Purrloin: Evolution = Evolution {
// 	base: Species::Purrloin,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Liepard, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Liepard: Evolution = Evolution {
// 	base: Species::Purrloin,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pansage: Evolution = Evolution {
// 	base: Species::Pansage,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Simisage, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::LeafStone) }]
// };
// pub static Simisage: Evolution = Evolution {
// 	base: Species::Pansage,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pansear: Evolution = Evolution {
// 	base: Species::Pansear,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Simisear, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::FireStone) }]
// };
// pub static Simisear: Evolution = Evolution {
// 	base: Species::Pansear,
// 	baby: None,
// 	paths: &[]
// };
// pub static Panpour: Evolution = Evolution {
// 	base: Species::Panpour,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Simipour, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::WaterStone) }]
// };
// pub static Simipour: Evolution = Evolution {
// 	base: Species::Panpour,
// 	baby: None,
// 	paths: &[]
// };
// pub static Munna: Evolution = Evolution {
// 	base: Species::Munna,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Musharna, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::MoonStone) }]
// };
// pub static Musharna: Evolution = Evolution {
// 	base: Species::Munna,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pidove: Evolution = Evolution {
// 	base: Species::Pidove,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Tranquill, trigger: EvolutionTrigger::AtLevel(21) }]
// };
// pub static Tranquill: Evolution = Evolution {
// 	base: Species::Pidove,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Unfezant, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Unfezant: Evolution = Evolution {
// 	base: Species::Pidove,
// 	baby: None,
// 	paths: &[]
// };
// pub static Blitzle: Evolution = Evolution {
// 	base: Species::Blitzle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Zebstrika, trigger: EvolutionTrigger::AtLevel(27) }]
// };
// pub static Zebstrika: Evolution = Evolution {
// 	base: Species::Blitzle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Roggenrola: Evolution = Evolution {
// 	base: Species::Roggenrola,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Boldore, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Boldore: Evolution = Evolution {
// 	base: Species::Roggenrola,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gigalith, trigger: EvolutionTrigger::Trading }]
// };
// pub static Gigalith: Evolution = Evolution {
// 	base: Species::Roggenrola,
// 	baby: None,
// 	paths: &[]
// };
// pub static Woobat: Evolution = Evolution {
// 	base: Species::Woobat,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Swoobat, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Swoobat: Evolution = Evolution {
// 	base: Species::Woobat,
// 	baby: None,
// 	paths: &[]
// };
// pub static Drilbur: Evolution = Evolution {
// 	base: Species::Drilbur,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Excadrill, trigger: EvolutionTrigger::AtLevel(31) }]
// };
// pub static Excadrill: Evolution = Evolution {
// 	base: Species::Drilbur,
// 	baby: None,
// 	paths: &[]
// };
// pub static Audino: Evolution = Evolution {
// 	base: Species::Audino,
// 	baby: None,
// 	paths: &[]
// };
// pub static Timburr: Evolution = Evolution {
// 	base: Species::Timburr,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gurdurr, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Gurdurr: Evolution = Evolution {
// 	base: Species::Timburr,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Conkeldurr, trigger: EvolutionTrigger::Trading }]
// };
// pub static Conkeldurr: Evolution = Evolution {
// 	base: Species::Timburr,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tympole: Evolution = Evolution {
// 	base: Species::Tympole,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Palpitoad, trigger: EvolutionTrigger::AtLevel(25) }]
// };
// pub static Palpitoad: Evolution = Evolution {
// 	base: Species::Tympole,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Seismitoad, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Seismitoad: Evolution = Evolution {
// 	base: Species::Tympole,
// 	baby: None,
// 	paths: &[]
// };
// pub static Throh: Evolution = Evolution {
// 	base: Species::Throh,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sawk: Evolution = Evolution {
// 	base: Species::Sawk,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sewaddle: Evolution = Evolution {
// 	base: Species::Sewaddle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Swadloon, trigger: EvolutionTrigger::AtLevel(20) }]
// };
// pub static Swadloon: Evolution = Evolution {
// 	base: Species::Sewaddle,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Leavanny, trigger: EvolutionTrigger::HighFriendship }]
// };
// pub static Leavanny: Evolution = Evolution {
// 	base: Species::Sewaddle,
// 	baby: None,
// 	paths: &[]
// };
// pub static Venipede: Evolution = Evolution {
// 	base: Species::Venipede,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Whirlipede, trigger: EvolutionTrigger::AtLevel(22) }]
// };
// pub static Whirlipede: Evolution = Evolution {
// 	base: Species::Venipede,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Scolipede, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Scolipede: Evolution = Evolution {
// 	base: Species::Venipede,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cottonee: Evolution = Evolution {
// 	base: Species::Cottonee,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Whimsicott, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
// };
// pub static Whimsicott: Evolution = Evolution {
// 	base: Species::Cottonee,
// 	baby: None,
// 	paths: &[]
// };
// pub static Petilil: Evolution = Evolution {
// 	base: Species::Petilil,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lilligant, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::SunStone) }]
// };
// pub static Lilligant: Evolution = Evolution {
// 	base: Species::Petilil,
// 	baby: None,
// 	paths: &[]
// };
// pub static Basculin_RedStriped: Evolution = Evolution {
// 	base: Species::Basculin(BasculinForm::RedStriped),
// 	baby: None,
// 	paths: &[]
// };
// pub static Basculin_BlueStriped: Evolution = Evolution {
// 	base: Species::Basculin(BasculinForm::BlueStriped),
// 	baby: None,
// 	paths: &[]
// };
// pub static Sandile: Evolution = Evolution {
// 	base: Species::Sandile,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Krokorok, trigger: EvolutionTrigger::AtLevel(29) }]
// };
// pub static Krokorok: Evolution = Evolution {
// 	base: Species::Sandile,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Krookodile, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Krookodile: Evolution = Evolution {
// 	base: Species::Sandile,
// 	baby: None,
// 	paths: &[]
// };
// pub static Darumaka: Evolution = Evolution {
// 	base: Species::Darumaka,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Darmanitan(DarmanitanForm::Standard), trigger: EvolutionTrigger::AtLevel(35) }]
// };
// pub static Darmanitan: Evolution = Evolution {
// 	base: Species::Darumaka,
// 	baby: None,
// 	paths: &[]
// };
// pub static Maractus: Evolution = Evolution {
// 	base: Species::Maractus,
// 	baby: None,
// 	paths: &[]
// };
// pub static Dwebble: Evolution = Evolution {
// 	base: Species::Dwebble,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Crustle, trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Crustle: Evolution = Evolution {
// 	base: Species::Dwebble,
// 	baby: None,
// 	paths: &[]
// };
// pub static Scraggy: Evolution = Evolution {
// 	base: Species::Scraggy,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Scrafty, trigger: EvolutionTrigger::AtLevel(39) }]
// };
// pub static Scrafty: Evolution = Evolution {
// 	base: Species::Scraggy,
// 	baby: None,
// 	paths: &[]
// };
// pub static Sigilyph: Evolution = Evolution {
// 	base: Species::Sigilyph,
// 	baby: None,
// 	paths: &[]
// };
// pub static Yamask: Evolution = Evolution {
// 	base: Species::Yamask,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Cofagrigus, trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Cofagrigus: Evolution = Evolution {
// 	base: Species::Yamask,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tirtouga: Evolution = Evolution {
// 	base: Species::Tirtouga,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Carracosta, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Carracosta: Evolution = Evolution {
// 	base: Species::Tirtouga,
// 	baby: None,
// 	paths: &[]
// };
// pub static Archen: Evolution = Evolution {
// 	base: Species::Archen,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Archeops, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Archeops: Evolution = Evolution {
// 	base: Species::Archen,
// 	baby: None,
// 	paths: &[]
// };
// pub static Trubbish: Evolution = Evolution {
// 	base: Species::Trubbish,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Garbodor, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Garbodor: Evolution = Evolution {
// 	base: Species::Trubbish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Zorua: Evolution = Evolution {
// 	base: Species::Zorua,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Zoroark, trigger: EvolutionTrigger::AtLevel(30) }]
// };
// pub static Zoroark: Evolution = Evolution {
// 	base: Species::Zorua,
// 	baby: None,
// 	paths: &[]
// };
// pub static Minccino: Evolution = Evolution {
// 	base: Species::Minccino,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Cinccino, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ShinyStone) }]
// };
// pub static Cinccino: Evolution = Evolution {
// 	base: Species::Minccino,
// 	baby: None,
// 	paths: &[]
// };
// pub static Gothita: Evolution = Evolution {
// 	base: Species::Gothita,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gothorita, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Gothorita: Evolution = Evolution {
// 	base: Species::Gothita,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Gothitelle, trigger: EvolutionTrigger::AtLevel(41) }]
// };
// pub static Gothitelle: Evolution = Evolution {
// 	base: Species::Gothita,
// 	baby: None,
// 	paths: &[]
// };
// pub static Solosis: Evolution = Evolution {
// 	base: Species::Solosis,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Duosion, trigger: EvolutionTrigger::AtLevel(32) }]
// };
// pub static Duosion: Evolution = Evolution {
// 	base: Species::Solosis,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Reuniclus, trigger: EvolutionTrigger::AtLevel(41) }]
// };
// pub static Reuniclus: Evolution = Evolution {
// 	base: Species::Solosis,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ducklett: Evolution = Evolution {
// 	base: Species::Ducklett,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Swanna, trigger: EvolutionTrigger::AtLevel(35) }]
// };
// pub static Swanna: Evolution = Evolution {
// 	base: Species::Ducklett,
// 	baby: None,
// 	paths: &[]
// };
// pub static Vanillite: Evolution = Evolution {
// 	base: Species::Vanillite,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vanillish, trigger: EvolutionTrigger::AtLevel(35) }]
// };
// pub static Vanillish: Evolution = Evolution {
// 	base: Species::Vanillite,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Vanilluxe, trigger: EvolutionTrigger::AtLevel(47) }]
// };
// pub static Vanilluxe: Evolution = Evolution {
// 	base: Species::Vanillite,
// 	baby: None,
// 	paths: &[]
// };
// pub static Deerling_Spring: Evolution = Evolution {
// 	base: Species::Deerling(Season::Spring),
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sawsbuck(Season::Spring), trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Sawsbuck_Spring: Evolution = Evolution {
// 	base: Species::Deerling(Season::Spring),
// 	baby: None,
// 	paths: &[]
// };
// pub static Deerling_Summer: Evolution = Evolution {
// 	base: Species::Deerling(Season::Summer),
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sawsbuck(Season::Summer), trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Sawsbuck_Summer: Evolution = Evolution {
// 	base: Species::Deerling(Season::Summer),
// 	baby: None,
// 	paths: &[]
// };
// pub static Deerling_Fall: Evolution = Evolution {
// 	base: Species::Deerling(Season::Autumn),
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sawsbuck(Season::Autumn), trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Sawsbuck_Fall: Evolution = Evolution {
// 	base: Species::Deerling(Season::Autumn),
// 	baby: None,
// 	paths: &[]
// };
// pub static Deerling_Winter: Evolution = Evolution {
// 	base: Species::Deerling(Season::Winter),
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Sawsbuck(Season::Winter), trigger: EvolutionTrigger::AtLevel(34) }]
// };
// pub static Sawsbuck_Winter: Evolution = Evolution {
// 	base: Species::Deerling(Season::Winter),
// 	baby: None,
// 	paths: &[]
// };
// pub static Emolga: Evolution = Evolution {
// 	base: Species::Emolga,
// 	baby: None,
// 	paths: &[]
// };
// pub static Karrablast: Evolution = Evolution {
// 	base: Species::Karrablast,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Escavalier, trigger: EvolutionTrigger::TradingForPokemon(Species::Shelmet) }]
// };
// pub static Escavalier: Evolution = Evolution {
// 	base: Species::Karrablast,
// 	baby: None,
// 	paths: &[]
// };
// pub static Foongus: Evolution = Evolution {
// 	base: Species::Foongus,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Amoonguss, trigger: EvolutionTrigger::AtLevel(39) }]
// };
// pub static Amoonguss: Evolution = Evolution {
// 	base: Species::Foongus,
// 	baby: None,
// 	paths: &[]
// };
// pub static Frillish: Evolution = Evolution {
// 	base: Species::Frillish,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Jellicent, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Jellicent: Evolution = Evolution {
// 	base: Species::Frillish,
// 	baby: None,
// 	paths: &[]
// };
// pub static Alomomola: Evolution = Evolution {
// 	base: Species::Alomomola,
// 	baby: None,
// 	paths: &[]
// };
// pub static Joltik: Evolution = Evolution {
// 	base: Species::Joltik,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Galvantula, trigger: EvolutionTrigger::AtLevel(36) }]
// };
// pub static Galvantula: Evolution = Evolution {
// 	base: Species::Joltik,
// 	baby: None,
// 	paths: &[]
// };
// pub static Ferroseed: Evolution = Evolution {
// 	base: Species::Ferroseed,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Ferrothorn, trigger: EvolutionTrigger::AtLevel(40) }]
// };
// pub static Ferrothorn: Evolution = Evolution {
// 	base: Species::Ferroseed,
// 	baby: None,
// 	paths: &[]
// };
// pub static Klink: Evolution = Evolution {
// 	base: Species::Klink,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Klang, trigger: EvolutionTrigger::AtLevel(38) }]
// };
// pub static Klang: Evolution = Evolution {
// 	base: Species::Klink,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Klinklang, trigger: EvolutionTrigger::AtLevel(49) }]
// };
// pub static Klinklang: Evolution = Evolution {
// 	base: Species::Klink,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tynamo: Evolution = Evolution {
// 	base: Species::Tynamo,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Eelektrik, trigger: EvolutionTrigger::AtLevel(39) }]
// };
// pub static Eelektrik: Evolution = Evolution {
// 	base: Species::Tynamo,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Eelektross, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::ThunderStone) }]
// };
// pub static Eelektross: Evolution = Evolution {
// 	base: Species::Tynamo,
// 	baby: None,
// 	paths: &[]
// };
// pub static Elgyem: Evolution = Evolution {
// 	base: Species::Elgyem,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Beheeyem, trigger: EvolutionTrigger::AtLevel(42) }]
// };
// pub static Beheeyem: Evolution = Evolution {
// 	base: Species::Elgyem,
// 	baby: None,
// 	paths: &[]
// };
// pub static Litwick: Evolution = Evolution {
// 	base: Species::Litwick,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Lampent, trigger: EvolutionTrigger::AtLevel(41) }]
// };
// pub static Lampent: Evolution = Evolution {
// 	base: Species::Litwick,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Chandelure, trigger: EvolutionTrigger::EvolutionStone(EvolutionStone::DuskStone) }]
// };
// pub static Chandelure: Evolution = Evolution {
// 	base: Species::Litwick,
// 	baby: None,
// 	paths: &[]
// };
// pub static Axew: Evolution = Evolution {
// 	base: Species::Axew,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Fraxure, trigger: EvolutionTrigger::AtLevel(38) }]
// };
// pub static Fraxure: Evolution = Evolution {
// 	base: Species::Axew,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Haxorus, trigger: EvolutionTrigger::AtLevel(48) }]
// };
// pub static Haxorus: Evolution = Evolution {
// 	base: Species::Axew,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cubchoo: Evolution = Evolution {
// 	base: Species::Cubchoo,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Beartic, trigger: EvolutionTrigger::AtLevel(37) }]
// };
// pub static Beartic: Evolution = Evolution {
// 	base: Species::Cubchoo,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cryogonal: Evolution = Evolution {
// 	base: Species::Cryogonal,
// 	baby: None,
// 	paths: &[]
// };
// pub static Shelmet: Evolution = Evolution {
// 	base: Species::Shelmet,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Accelgor, trigger: EvolutionTrigger::TradingForPokemon(Species::Karrablast) }]
// };
// pub static Accelgor: Evolution = Evolution {
// 	base: Species::Shelmet,
// 	baby: None,
// 	paths: &[]
// };
// pub static Stunfisk: Evolution = Evolution {
// 	base: Species::Stunfisk,
// 	baby: None,
// 	paths: &[]
// };
// pub static Mienfoo: Evolution = Evolution {
// 	base: Species::Mienfoo,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Mienshao, trigger: EvolutionTrigger::AtLevel(50) }]
// };
// pub static Mienshao: Evolution = Evolution {
// 	base: Species::Mienfoo,
// 	baby: None,
// 	paths: &[]
// };
// pub static Druddigon: Evolution = Evolution {
// 	base: Species::Druddigon,
// 	baby: None,
// 	paths: &[]
// };
// pub static Golett: Evolution = Evolution {
// 	base: Species::Golett,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Golurk, trigger: EvolutionTrigger::AtLevel(43) }]
// };
// pub static Golurk: Evolution = Evolution {
// 	base: Species::Golett,
// 	baby: None,
// 	paths: &[]
// };
// pub static Pawniard: Evolution = Evolution {
// 	base: Species::Pawniard,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Bisharp, trigger: EvolutionTrigger::AtLevel(52) }]
// };
// pub static Bisharp: Evolution = Evolution {
// 	base: Species::Pawniard,
// 	baby: None,
// 	paths: &[]
// };
// pub static Bouffalant: Evolution = Evolution {
// 	base: Species::Bouffalant,
// 	baby: None,
// 	paths: &[]
// };
// pub static Rufflet: Evolution = Evolution {
// 	base: Species::Rufflet,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Braviary, trigger: EvolutionTrigger::AtLevel(54) }]
// };
// pub static Braviary: Evolution = Evolution {
// 	base: Species::Rufflet,
// 	baby: None,
// 	paths: &[]
// };
// pub static Vullaby: Evolution = Evolution {
// 	base: Species::Vullaby,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Mandibuzz, trigger: EvolutionTrigger::AtLevel(54) }]
// };
// pub static Mandibuzz: Evolution = Evolution {
// 	base: Species::Vullaby,
// 	baby: None,
// 	paths: &[]
// };
// pub static Heatmor: Evolution = Evolution {
// 	base: Species::Heatmor,
// 	baby: None,
// 	paths: &[]
// };
// pub static Durant: Evolution = Evolution {
// 	base: Species::Durant,
// 	baby: None,
// 	paths: &[]
// };
// pub static Deino: Evolution = Evolution {
// 	base: Species::Deino,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Zweilous, trigger: EvolutionTrigger::AtLevel(50) }]
// };
// pub static Zweilous: Evolution = Evolution {
// 	base: Species::Deino,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Hydreigon, trigger: EvolutionTrigger::AtLevel(64) }]
// };
// pub static Hydreigon: Evolution = Evolution {
// 	base: Species::Deino,
// 	baby: None,
// 	paths: &[]
// };
// pub static Larvesta: Evolution = Evolution {
// 	base: Species::Larvesta,
// 	baby: None,
// 	paths: &[EvolutionPath { to: Species::Volcarona, trigger: EvolutionTrigger::AtLevel(59) }]
// };
// pub static Volcarona: Evolution = Evolution {
// 	base: Species::Larvesta,
// 	baby: None,
// 	paths: &[]
// };
// pub static Cobalion: Evolution = Evolution {
// 	base: Species::Cobalion,
// 	baby: None,
// 	paths: &[]
// };
// pub static Terrakion: Evolution = Evolution {
// 	base: Species::Terrakion,
// 	baby: None,
// 	paths: &[]
// };
// pub static Virizion: Evolution = Evolution {
// 	base: Species::Virizion,
// 	baby: None,
// 	paths: &[]
// };
// pub static Tornadus: Evolution = Evolution {
// 	base: Species::Tornadus(ForcesOfNatureForm::Incarnate),
// 	baby: None,
// 	paths: &[]
// };
// pub static Thundurus: Evolution = Evolution {
// 	base: Species::Thundurus(ForcesOfNatureForm::Incarnate),
// 	baby: None,
// 	paths: &[]
// };
// pub static Reshiram: Evolution = Evolution {
// 	base: Species::Reshiram,
// 	baby: None,
// 	paths: &[]
// };
// pub static Zekrom: Evolution = Evolution {
// 	base: Species::Zekrom,
// 	baby: None,
// 	paths: &[]
// };
// pub static Landorus: Evolution = Evolution {
// 	base: Species::Landorus(ForcesOfNatureForm::Incarnate),
// 	baby: None,
// 	paths: &[]
// };
// pub static Kyurem: Evolution = Evolution {
// 	base: Species::Kyurem(KyuremForm::Normal),
// 	baby: None,
// 	paths: &[]
// };
// pub static Keldeo: Evolution = Evolution {
// 	base: Species::Keldeo(KeldeoForm::Ordinary),
// 	baby: None,
// 	paths: &[]
// };
// pub static Meloetta: Evolution = Evolution {
// 	base: Species::Meloetta(MeloettaForm::Aria),
// 	baby: None,
// 	paths: &[]
// };
// pub static Genesect: Evolution = Evolution {
// 	base: Species::Genesect(GenesectForm::Normal),
// 	baby: None,
// 	paths: &[]
// };
// //endregion