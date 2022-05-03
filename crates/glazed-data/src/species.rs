#![allow(non_upper_case_globals)]

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, IntoStaticStr};

use crate::time::{GlazedTime, Season};
use crate::types::Type;

//region Pokemon Species Enums
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, EnumDiscriminants)]
#[strum_discriminants(derive(IntoStaticStr))]
pub enum Species {
    Bulbasaur,
    Ivysaur,
    Venusaur,
    Charmander,
    Charmeleon,
    Charizard,
    Squirtle,
    Wartortle,
    Blastoise,
    Caterpie,
    Metapod,
    Butterfree,
    Weedle,
    Kakuna,
    Beedrill,
    Pidgey,
    Pidgeotto,
    Pidgeot,
    Rattata,
    Raticate,
    Spearow,
    Fearow,
    Ekans,
    Arbok,
    Pikachu,
    Raichu,
    Sandshrew,
    Sandslash,
    NidoranF,
    Nidorina,
    Nidoqueen,
    NidoranM,
    Nidorino,
    Nidoking,
    Clefairy,
    Clefable,
    Vulpix,
    Ninetales,
    Jigglypuff,
    Wigglytuff,
    Zubat,
    Golbat,
    Oddish,
    Gloom,
    Vileplume,
    Paras,
    Parasect,
    Venonat,
    Venomoth,
    Diglett,
    Dugtrio,
    Meowth,
    Persian,
    Psyduck,
    Golduck,
    Mankey,
    Primeape,
    Growlithe,
    Arcanine,
    Poliwag,
    Poliwhirl,
    Poliwrath,
    Abra,
    Kadabra,
    Alakazam,
    Machop,
    Machoke,
    Machamp,
    Bellsprout,
    Weepinbell,
    Victreebel,
    Tentacool,
    Tentacruel,
    Geodude,
    Graveler,
    Golem,
    Ponyta,
    Rapidash,
    Slowpoke,
    Slowbro,
    Magnemite,
    Magneton,
    Farfetchd,
    Doduo,
    Dodrio,
    Seel,
    Dewgong,
    Grimer,
    Muk,
    Shellder,
    Cloyster,
    Gastly,
    Haunter,
    Gengar,
    Onix,
    Drowzee,
    Hypno,
    Krabby,
    Kingler,
    Voltorb,
    Electrode,
    Exeggcute,
    Exeggutor,
    Cubone,
    Marowak,
    Hitmonlee,
    Hitmonchan,
    Lickitung,
    Koffing,
    Weezing,
    Rhyhorn,
    Rhydon,
    Chansey,
    Tangela,
    Kangaskhan,
    Horsea,
    Seadra,
    Goldeen,
    Seaking,
    Staryu,
    Starmie,
    MrMime,
    Scyther,
    Jynx,
    Electabuzz,
    Magmar,
    Pinsir,
    Tauros,
    Magikarp,
    Gyarados,
    Lapras,
    Ditto,
    Eevee,
    Vaporeon,
    Jolteon,
    Flareon,
    Porygon,
    Omanyte,
    Omastar,
    Kabuto,
    Kabutops,
    Aerodactyl,
    Snorlax,
    Articuno,
    Zapdos,
    Moltres,
    Dratini,
    Dragonair,
    Dragonite,
    Mewtwo,
    Mew,
    Chikorita,
    Bayleef,
    Meganium,
    Cyndaquil,
    Quilava,
    Typhlosion,
    Totodile,
    Croconaw,
    Feraligatr,
    Sentret,
    Furret,
    Hoothoot,
    Noctowl,
    Ledyba,
    Ledian,
    Spinarak,
    Ariados,
    Crobat,
    Chinchou,
    Lanturn,
    Pichu,
    Cleffa,
    Igglybuff,
    Togepi,
    Togetic,
    Natu,
    Xatu,
    Mareep,
    Flaaffy,
    Ampharos,
    Bellossom,
    Marill,
    Azumarill,
    Sudowoodo,
    Politoed,
    Hoppip,
    Skiploom,
    Jumpluff,
    Aipom,
    Sunkern,
    Sunflora,
    Yanma,
    Wooper,
    Quagsire,
    Espeon,
    Umbreon,
    Murkrow,
    Slowking,
    Misdreavus,
    Unown(UnownForm),
    Wobbuffet,
    Girafarig,
    Pineco,
    Forretress,
    Dunsparce,
    Gligar,
    Steelix,
    Snubbull,
    Granbull,
    Qwilfish,
    Scizor,
    Shuckle,
    Heracross,
    Sneasel,
    Teddiursa,
    Ursaring,
    Slugma,
    Magcargo,
    Swinub,
    Piloswine,
    Corsola,
    Remoraid,
    Octillery,
    Delibird,
    Mantine,
    Skarmory,
    Houndour,
    Houndoom,
    Kingdra,
    Phanpy,
    Donphan,
    Porygon2,
    Stantler,
    Smeargle,
    Tyrogue,
    Hitmontop,
    Smoochum,
    Elekid,
    Magby,
    Miltank,
    Blissey,
    Raikou,
    Entei,
    Suicune,
    Larvitar,
    Pupitar,
    Tyranitar,
    Lugia,
    HoOh,
    Celebi,
    Treecko,
    Grovyle,
    Sceptile,
    Torchic,
    Combusken,
    Blaziken,
    Mudkip,
    Marshtomp,
    Swampert,
    Poochyena,
    Mightyena,
    Zigzagoon,
    Linoone,
    Wurmple,
    Silcoon,
    Beautifly,
    Cascoon,
    Dustox,
    Lotad,
    Lombre,
    Ludicolo,
    Seedot,
    Nuzleaf,
    Shiftry,
    Taillow,
    Swellow,
    Wingull,
    Pelipper,
    Ralts,
    Kirlia,
    Gardevoir,
    Surskit,
    Masquerain,
    Shroomish,
    Breloom,
    Slakoth,
    Vigoroth,
    Slaking,
    Nincada,
    Ninjask,
    Shedinja,
    Whismur,
    Loudred,
    Exploud,
    Makuhita,
    Hariyama,
    Azurill,
    Nosepass,
    Skitty,
    Delcatty,
    Sableye,
    Mawile,
    Aron,
    Lairon,
    Aggron,
    Meditite,
    Medicham,
    Electrike,
    Manectric,
    Plusle,
    Minun,
    Volbeat,
    Illumise,
    Roselia,
    Gulpin,
    Swalot,
    Carvanha,
    Sharpedo,
    Wailmer,
    Wailord,
    Numel,
    Camerupt,
    Torkoal,
    Spoink,
    Grumpig,
    Spinda,
    Trapinch,
    Vibrava,
    Flygon,
    Cacnea,
    Cacturne,
    Swablu,
    Altaria,
    Zangoose,
    Seviper,
    Lunatone,
    Solrock,
    Barboach,
    Whiscash,
    Corphish,
    Crawdaunt,
    Baltoy,
    Claydol,
    Lileep,
    Cradily,
    Anorith,
    Armaldo,
    Feebas,
    Milotic,
    Castform(CastformForm),
    Kecleon,
    Shuppet,
    Banette,
    Duskull,
    Dusclops,
    Tropius,
    Chimecho,
    Absol,
    Wynaut,
    Snorunt,
    Glalie,
    Spheal,
    Sealeo,
    Walrein,
    Clamperl,
    Huntail,
    Gorebyss,
    Relicanth,
    Luvdisc,
    Bagon,
    Shelgon,
    Salamence,
    Beldum,
    Metang,
    Metagross,
    Regirock,
    Regice,
    Registeel,
    Latias,
    Latios,
    Kyogre,
    Groudon,
    Rayquaza,
    Jirachi,
    Deoxys(DeoxysForm),
    Turtwig,
    Grotle,
    Torterra,
    Chimchar,
    Monferno,
    Infernape,
    Piplup,
    Prinplup,
    Empoleon,
    Starly,
    Staravia,
    Staraptor,
    Bidoof,
    Bibarel,
    Kricketot,
    Kricketune,
    Shinx,
    Luxio,
    Luxray,
    Budew,
    Roserade,
    Cranidos,
    Rampardos,
    Shieldon,
    Bastiodon,
    Burmy(BurmyWormadamForm),
    Wormadam(BurmyWormadamForm),
    Mothim,
    Combee,
    Vespiquen,
    Pachirisu,
    Buizel,
    Floatzel,
    Cherubi,
    Cherrim(CherrimForm),
    Shellos(ShellosGastrodonForm),
    Gastrodon(ShellosGastrodonForm),
    Ambipom,
    Drifloon,
    Drifblim,
    Buneary,
    Lopunny,
    Mismagius,
    Honchkrow,
    Glameow,
    Purugly,
    Chingling,
    Stunky,
    Skuntank,
    Bronzor,
    Bronzong,
    Bonsly,
    MimeJr,
    Happiny,
    Chatot,
    Spiritomb,
    Gible,
    Gabite,
    Garchomp,
    Munchlax,
    Riolu,
    Lucario,
    Hippopotas,
    Hippowdon,
    Skorupi,
    Drapion,
    Croagunk,
    Toxicroak,
    Carnivine,
    Finneon,
    Lumineon,
    Mantyke,
    Snover,
    Abomasnow,
    Weavile,
    Magnezone,
    Lickilicky,
    Rhyperior,
    Tangrowth,
    Electivire,
    Magmortar,
    Togekiss,
    Yanmega,
    Leafeon,
    Glaceon,
    Gliscor,
    Mamoswine,
    PorygonZ,
    Gallade,
    Probopass,
    Dusknoir,
    Froslass,
    Rotom(RotomForm),
    Uxie,
    Mesprit,
    Azelf,
    Dialga,
    Palkia,
    Heatran,
    Regigigas,
    Giratina(GiratinaForm),
    Cresselia,
    Phione,
    Manaphy,
    Darkrai,
    Shaymin(ShayminForm),
    Arceus(Type),
    Victini,
    Snivy,
    Servine,
    Serperior,
    Tepig,
    Pignite,
    Emboar,
    Oshawott,
    Dewott,
    Samurott,
    Patrat,
    Watchog,
    Lillipup,
    Herdier,
    Stoutland,
    Purrloin,
    Liepard,
    Pansage,
    Simisage,
    Pansear,
    Simisear,
    Panpour,
    Simipour,
    Munna,
    Musharna,
    Pidove,
    Tranquill,
    Unfezant,
    Blitzle,
    Zebstrika,
    Roggenrola,
    Boldore,
    Gigalith,
    Woobat,
    Swoobat,
    Drilbur,
    Excadrill,
    Audino,
    Timburr,
    Gurdurr,
    Conkeldurr,
    Tympole,
    Palpitoad,
    Seismitoad,
    Throh,
    Sawk,
    Sewaddle,
    Swadloon,
    Leavanny,
    Venipede,
    Whirlipede,
    Scolipede,
    Cottonee,
    Whimsicott,
    Petilil,
    Lilligant,
    Basculin(BasculinForm),
    Sandile,
    Krokorok,
    Krookodile,
    Darumaka,
    Darmanitan(DarmanitanForm),
    Maractus,
    Dwebble,
    Crustle,
    Scraggy,
    Scrafty,
    Sigilyph,
    Yamask,
    Cofagrigus,
    Tirtouga,
    Carracosta,
    Archen,
    Archeops,
    Trubbish,
    Garbodor,
    Zorua,
    Zoroark,
    Minccino,
    Cinccino,
    Gothita,
    Gothorita,
    Gothitelle,
    Solosis,
    Duosion,
    Reuniclus,
    Ducklett,
    Swanna,
    Vanillite,
    Vanillish,
    Vanilluxe,
    Deerling(Season),
    Sawsbuck(Season),
    Emolga,
    Karrablast,
    Escavalier,
    Foongus,
    Amoonguss,
    Frillish,
    Jellicent,
    Alomomola,
    Joltik,
    Galvantula,
    Ferroseed,
    Ferrothorn,
    Klink,
    Klang,
    Klinklang,
    Tynamo,
    Eelektrik,
    Eelektross,
    Elgyem,
    Beheeyem,
    Litwick,
    Lampent,
    Chandelure,
    Axew,
    Fraxure,
    Haxorus,
    Cubchoo,
    Beartic,
    Cryogonal,
    Shelmet,
    Accelgor,
    Stunfisk,
    Mienfoo,
    Mienshao,
    Druddigon,
    Golett,
    Golurk,
    Pawniard,
    Bisharp,
    Bouffalant,
    Rufflet,
    Braviary,
    Vullaby,
    Mandibuzz,
    Heatmor,
    Durant,
    Deino,
    Zweilous,
    Hydreigon,
    Larvesta,
    Volcarona,
    Cobalion,
    Terrakion,
    Virizion,
    Tornadus(ForcesOfNatureForm),
    Thundurus(ForcesOfNatureForm),
    Reshiram,
    Zekrom,
    Landorus(ForcesOfNatureForm),
    Kyurem(KyuremForm),
    Keldeo(KeldeoForm),
    Meloetta(MeloettaForm),
    Genesect(GenesectForm)
}
impl Default for Species {
    fn default() -> Self {
        Species::Bulbasaur
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum UnownForm {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ExclamationMark,
    QuestionMark
}
impl Distribution<UnownForm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnownForm {
        match rng.gen_range(0..=27) {
            0 => UnownForm::A,
            1 => UnownForm::B,
            2 => UnownForm::C,
            3 => UnownForm::D,
            4 => UnownForm::E,
            5 => UnownForm::F,
            6 => UnownForm::G,
            7 => UnownForm::H,
            8 => UnownForm::I,
            9 => UnownForm::J,
            10 => UnownForm::K,
            11 => UnownForm::L,
            12 => UnownForm::M,
            13 => UnownForm::N,
            14 => UnownForm::O,
            15 => UnownForm::P,
            16 => UnownForm::Q,
            17 => UnownForm::R,
            18 => UnownForm::S,
            19 => UnownForm::T,
            20 => UnownForm::U,
            21 => UnownForm::V,
            22 => UnownForm::W,
            23 => UnownForm::X,
            24 => UnownForm::Y,
            25 => UnownForm::Z,
            26 => UnownForm::ExclamationMark,
            27 => UnownForm::QuestionMark,
            _ => panic!("Invalid Unown Form -> Expected 0-27")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum CastformForm {
    Normal, Sunny, Rainy, Snowy
}
impl Default for CastformForm {
    fn default() -> Self { CastformForm::Normal }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum DeoxysForm {
    Normal, Attack, Defense, Speed
}
impl Default for DeoxysForm {
    fn default() -> Self { DeoxysForm::Normal }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum BurmyWormadamForm {
    Plant, Sandy, Trash
}
impl Default for BurmyWormadamForm {
    fn default() -> Self { BurmyWormadamForm::Plant }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum CherrimForm {
    Overcast, Sunshine
}
impl Default for CherrimForm {
    fn default() -> Self { CherrimForm::Overcast }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum ShellosGastrodonForm {
    WestSea, EastSea
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum RotomForm {
    Normal, Heat, Wash, Frost, Fan, Mow
}
impl Default for RotomForm {
    fn default() -> Self { RotomForm::Normal }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum GiratinaForm {
    Altered, Origin
}
impl Default for GiratinaForm {
    fn default() -> Self { GiratinaForm::Altered }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum ShayminForm {
    Land, Sky
}
impl Default for ShayminForm {
    fn default() -> Self { ShayminForm::Sky }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum BasculinForm {
    RedStriped, BlueStriped
}
impl Distribution<BasculinForm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BasculinForm {
        if rng.gen_bool(0.5) {
            BasculinForm::RedStriped
        } else {
            BasculinForm::BlueStriped
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum DarmanitanForm {
    Standard, Zen
}
impl Default for DarmanitanForm {
    fn default() -> Self { DarmanitanForm::Standard }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum ForcesOfNatureForm {
    Incarnate, Therian
}
impl Default for ForcesOfNatureForm {
    fn default() -> Self { ForcesOfNatureForm::Incarnate }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum KyuremForm {
    Normal, White, Black
}
impl Default for KyuremForm {
    fn default() -> Self { KyuremForm::Normal }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum KeldeoForm {
    Ordinary, Resolute
}
impl Default for KeldeoForm {
    fn default() -> Self { KeldeoForm::Ordinary }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum MeloettaForm {
    Aria, Pirouette
}
impl Default for MeloettaForm {
    fn default() -> Self { MeloettaForm::Aria }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum GenesectForm {
    Normal, Shock, Burn, Chill, Douse
}
impl Default for GenesectForm {
    fn default() -> Self { GenesectForm::Normal }
}

/// Specific flavors of Species Generation based on some form of internal context
#[derive(Debug, Serialize, Deserialize)]
pub enum SpeciesGenerator {
    /// Generates an Unown of a random flavor, evenly distributed
    RandomUnown,
    /// Generates a Deerling corresponding to the current season
    SeasonalDeerling,
    /// Generates a Sawsbuck corresponding to the current season
    SeasonalSawsbuck,
    /// Generates a random species, evenly distributed
    Random(Vec<Species>)
}
impl Into<Species> for SpeciesGenerator {
    fn into(self) -> Species {
        match self {
            SpeciesGenerator::RandomUnown => Species::Unown(rand::thread_rng().gen()),
            SpeciesGenerator::Random(choices) => {
                let idx = rand::thread_rng().gen_range(0..choices.len());
                choices[idx]
            }
            SpeciesGenerator::SeasonalDeerling => Species::Deerling(GlazedTime::get_season()),
            SpeciesGenerator::SeasonalSawsbuck => Species::Sawsbuck(GlazedTime::get_season()),
        }
    }
}