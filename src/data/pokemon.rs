use strum_macros;
use strum::EnumProperty;

use crate::data::abilities::{Ability, PokemonAbility};
use crate::data::attack::Move;
use crate::data::core::Season;
use crate::data::types::{PokemonType, Type};

//region Pokemon Species Enums
#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnownForm {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ExclamationMark,
    QuestionMark
}

#[derive(Debug)]
pub enum CastformForm {
    Normal, Sunny, Rainy, Snowy
}

#[derive(Debug)]
pub enum DeoxysForm {
    Normal, Attack, Defense, Speed
}

#[derive(Debug)]
pub enum BurmyWormadamForm {
    Plant, Sandy, Trash
}

#[derive(Debug)]
pub enum CherrimForm {
    Overcase, Sunshine
}

#[derive(Debug)]
pub enum ShellosGastrodonForm {
    WestSea, EastSea
}

#[derive(Debug)]
pub enum RotomForm {
    Normal, Heat, Wash, Frost, Fan, Mow
}

#[derive(Debug)]
pub enum GiratinaForm {
    Altered, Origin
}

#[derive(Debug)]
pub enum ShayminForm {
    Land, Sky
}

#[derive(Debug)]
pub enum BasculinForm {
    RedStriped, BlueStriped
}

#[derive(Debug)]
pub enum DarmanitanForm {
    Standard, Zen
}

#[derive(Debug)]
pub enum ForcesOfNatureForm {
    Incarnate, Therian
}

#[derive(Debug)]
pub enum KyuremForm {
    Normal, White, Black
}

#[derive(Debug)]
pub enum KeldeoForm {
    Ordinary, Resolute
}

#[derive(Debug)]
pub enum MeloettaForm {
    Aria, Pirouette
}

#[derive(Debug)]
pub enum GenesectForm {
    Normal, Shock, Burn, Chill, Douse
}
//endregion

#[derive(Debug)]
pub enum GenderRatio {
    None,
    Proportion(u8, u8)
}

#[derive(Debug)]
pub enum EggGroup {
    Monster,
    Water1,
    Water2,
    Water3,
    Humanshape,
    Bug,
    Mineral,
    Flying,
    Indeterminate,
    Ground,
    Fairy,
    Ditto,
    Plant,
    Dragon
}

#[derive(Debug)]
pub enum PokemonEggGroup {
    None,
    One(EggGroup),
    Two(EggGroup, EggGroup)
}

#[derive(Debug)]
pub enum LevelRate {
    Erratic, Fast, MediumFast, MediumSlow, Slow, Fluctuating
}

#[derive(Debug)]
pub enum Color {
    Red, Blue, Yellow, Green, Black, Brown, Purple, Gray, White, Pink
}

#[derive(Debug)]
pub struct Stats(Stat, Stat, Stat, Stat, Stat, Stat);

#[derive(Debug)]
pub struct Stat {
    base_stat: u8,
    ev: u8
}
impl Stat {
    pub const fn new(base: u8, ev: u8) -> Stat {
        Stat {
            base_stat: base,
            ev
        }
    }
}

#[derive(Debug)]
pub struct PokemonData {
    _type: PokemonType,
    ability: PokemonAbility,
    hidden_ability: Option<Ability>,
    gender_ratio: GenderRatio,
    catch_rate: u8,
    egg_group: PokemonEggGroup,
    egg_cycles: u8,
    height: u8,
    weight: u16,
    base_exp_yield: u16,
    level_rate: LevelRate,
    stats: Stats,
    base_friendship: u8,
    level_up_moves: &'static[(u8, Move)]
}

//region Pokemon Constants Data
impl Species {
    fn data(&self) -> PokemonData {
        match self {
            Species::Bulbasaur => PokemonData::Bulbasaur,
            Species::Ivysaur => PokemonData::Ivysaur,
            Species::Venusaur => PokemonData::Venusaur,
            Species::Charmander => PokemonData::Charmander,
            Species::Charmeleon => PokemonData::Charmeleon,
            Species::Charizard => PokemonData::Charizard,
            Species::Squirtle => PokemonData::Squirtle,
            Species::Wartortle => PokemonData::Wartortle,
            Species::Blastoise => PokemonData::Blastoise,
            Species::Caterpie => PokemonData::Caterpie,
            Species::Metapod => PokemonData::Metapod,
            Species::Butterfree => PokemonData::Butterfree,
            Species::Weedle => PokemonData::Weedle,
            Species::Kakuna => PokemonData::Kakuna,
            Species::Beedrill => PokemonData::Beedrill,
            Species::Pidgey => PokemonData::Pidgey,
            Species::Pidgeotto => PokemonData::Pidgeotto,
            Species::Pidgeot => PokemonData::Pidgeot,
            Species::Rattata => PokemonData::Rattata,
            Species::Raticate => PokemonData::Raticate,
            Species::Spearow => PokemonData::Spearow,
            Species::Fearow => PokemonData::Fearow,
            Species::Ekans => PokemonData::Ekans,
            Species::Arbok => PokemonData::Arbok,
            Species::Pikachu => PokemonData::Pikachu,
            Species::Raichu => PokemonData::Raichu,
            Species::Sandshrew => PokemonData::Sandshrew,
            Species::Sandslash => PokemonData::Sandslash,
            Species::NidoranF => PokemonData::NidoranF,
            Species::Nidorina => PokemonData::Nidorina,
            Species::Nidoqueen => PokemonData::Nidoqueen,
            Species::NidoranM => PokemonData::NidoranM,
            Species::Nidorino => PokemonData::Nidorino,
            Species::Nidoking => PokemonData::Nidoking,
            Species::Clefairy => PokemonData::Clefairy,
            Species::Clefable => PokemonData::Clefable,
            Species::Vulpix => PokemonData::Vulpix,
            Species::Ninetales => PokemonData::Ninetales,
            Species::Jigglypuff => PokemonData::Jigglypuff,
            Species::Wigglytuff => PokemonData::Wigglytuff,
            Species::Zubat => PokemonData::Zubat,
            Species::Golbat => PokemonData::Golbat,
            Species::Oddish => PokemonData::Oddish,
            Species::Gloom => PokemonData::Gloom,
            Species::Vileplume => PokemonData::Vileplume,
            Species::Paras => PokemonData::Paras,
            Species::Parasect => PokemonData::Parasect,
            Species::Venonat => PokemonData::Venonat,
            Species::Venomoth => PokemonData::Venomoth,
            Species::Diglett => PokemonData::Diglett,
            Species::Dugtrio => PokemonData::Dugtrio,
            Species::Meowth => PokemonData::Meowth,
            Species::Persian => PokemonData::Persian,
            Species::Psyduck => PokemonData::Psyduck,
            Species::Golduck => PokemonData::Golduck,
            Species::Mankey => PokemonData::Mankey,
            Species::Primeape => PokemonData::Primeape,
            Species::Growlithe => PokemonData::Growlithe,
            Species::Arcanine => PokemonData::Arcanine,
            Species::Poliwag => PokemonData::Poliwag,
            Species::Poliwhirl => PokemonData::Poliwhirl,
            Species::Poliwrath => PokemonData::Poliwrath,
            Species::Abra => PokemonData::Abra,
            Species::Kadabra => PokemonData::Kadabra,
            Species::Alakazam => PokemonData::Alakazam,
            Species::Machop => PokemonData::Machop,
            Species::Machoke => PokemonData::Machoke,
            Species::Machamp => PokemonData::Machamp,
            Species::Bellsprout => PokemonData::Bellsprout,
            Species::Weepinbell => PokemonData::Weepinbell,
            Species::Victreebel => PokemonData::Victreebel,
            Species::Tentacool => PokemonData::Tentacool,
            Species::Tentacruel => PokemonData::Tentacruel,
            Species::Geodude => PokemonData::Geodude,
            Species::Graveler => PokemonData::Graveler,
            Species::Golem => PokemonData::Golem,
            Species::Ponyta => PokemonData::Ponyta,
            Species::Rapidash => PokemonData::Rapidash,
            Species::Slowpoke => PokemonData::Slowpoke,
            Species::Slowbro => PokemonData::Slowbro,
            Species::Magnemite => PokemonData::Magnemite,
            Species::Magneton => PokemonData::Magneton,
            Species::Farfetchd => PokemonData::Farfetchd,
            Species::Doduo => PokemonData::Doduo,
            Species::Dodrio => PokemonData::Dodrio,
            Species::Seel => PokemonData::Seel,
            Species::Dewgong => PokemonData::Dewgong,
            Species::Grimer => PokemonData::Grimer,
            Species::Muk => PokemonData::Muk,
            Species::Shellder => PokemonData::Shellder,
            Species::Cloyster => PokemonData::Cloyster,
            Species::Gastly => PokemonData::Gastly,
            Species::Haunter => PokemonData::Haunter,
            Species::Gengar => PokemonData::Gengar,
            Species::Onix => PokemonData::Onix,
            Species::Drowzee => PokemonData::Drowzee,
            Species::Hypno => PokemonData::Hypno,
            Species::Krabby => PokemonData::Krabby,
            Species::Kingler => PokemonData::Kingler,
            Species::Voltorb => PokemonData::Voltorb,
            Species::Electrode => PokemonData::Electrode,
            Species::Exeggcute => PokemonData::Exeggcute,
            Species::Exeggutor => PokemonData::Exeggutor,
            Species::Cubone => PokemonData::Cubone,
            Species::Marowak => PokemonData::Marowak,
            Species::Hitmonlee => PokemonData::Hitmonlee,
            Species::Hitmonchan => PokemonData::Hitmonchan,
            Species::Lickitung => PokemonData::Lickitung,
            Species::Koffing => PokemonData::Koffing,
            Species::Weezing => PokemonData::Weezing,
            Species::Rhyhorn => PokemonData::Rhyhorn,
            Species::Rhydon => PokemonData::Rhydon,
            Species::Chansey => PokemonData::Chansey,
            Species::Tangela => PokemonData::Tangela,
            Species::Kangaskhan => PokemonData::Kangaskhan,
            Species::Horsea => PokemonData::Horsea,
            Species::Seadra => PokemonData::Seadra,
            Species::Goldeen => PokemonData::Goldeen,
            Species::Seaking => PokemonData::Seaking,
            Species::Staryu => PokemonData::Staryu,
            Species::Starmie => PokemonData::Starmie,
            Species::MrMime => PokemonData::MrMime,
            Species::Scyther => PokemonData::Scyther,
            Species::Jynx => PokemonData::Jynx,
            Species::Electabuzz => PokemonData::Electabuzz,
            Species::Magmar => PokemonData::Magmar,
            Species::Pinsir => PokemonData::Pinsir,
            Species::Tauros => PokemonData::Tauros,
            Species::Magikarp => PokemonData::Magikarp,
            Species::Gyarados => PokemonData::Gyarados,
            Species::Lapras => PokemonData::Lapras,
            Species::Ditto => PokemonData::Ditto,
            Species::Eevee => PokemonData::Eevee,
            Species::Vaporeon => PokemonData::Vaporeon,
            Species::Jolteon => PokemonData::Jolteon,
            Species::Flareon => PokemonData::Flareon,
            Species::Porygon => PokemonData::Porygon,
            Species::Omanyte => PokemonData::Omanyte,
            Species::Omastar => PokemonData::Omastar,
            Species::Kabuto => PokemonData::Kabuto,
            Species::Kabutops => PokemonData::Kabutops,
            Species::Aerodactyl => PokemonData::Aerodactyl,
            Species::Snorlax => PokemonData::Snorlax,
            Species::Articuno => PokemonData::Articuno,
            Species::Zapdos => PokemonData::Zapdos,
            Species::Moltres => PokemonData::Moltres,
            Species::Dratini => PokemonData::Dratini,
            Species::Dragonair => PokemonData::Dragonair,
            Species::Dragonite => PokemonData::Dragonite,
            Species::Mewtwo => PokemonData::Mewtwo,
            Species::Mew => PokemonData::Mew,
            Species::Chikorita => PokemonData::Chikorita,
            Species::Bayleef => PokemonData::Bayleef,
            Species::Meganium => PokemonData::Meganium,
            Species::Cyndaquil => PokemonData::Cyndaquil,
            Species::Quilava => PokemonData::Quilava,
            Species::Typhlosion => PokemonData::Typhlosion,
            Species::Totodile => PokemonData::Totodile,
            Species::Croconaw => PokemonData::Croconaw,
            Species::Feraligatr => PokemonData::Feraligatr,
            Species::Sentret => PokemonData::Sentret,
            Species::Furret => PokemonData::Furret,
            Species::Hoothoot => PokemonData::Hoothoot,
            Species::Noctowl => PokemonData::Noctowl,
            Species::Ledyba => PokemonData::Ledyba,
            Species::Ledian => PokemonData::Ledian,
            Species::Spinarak => PokemonData::Spinarak,
            Species::Ariados => PokemonData::Ariados,
            Species::Crobat => PokemonData::Crobat,
            Species::Chinchou => PokemonData::Chinchou,
            Species::Lanturn => PokemonData::Lanturn,
            Species::Pichu => PokemonData::Pichu,
            Species::Cleffa => PokemonData::Cleffa,
            Species::Igglybuff => PokemonData::Igglybuff,
            Species::Togepi => PokemonData::Togepi,
            Species::Togetic => PokemonData::Togetic,
            Species::Natu => PokemonData::Natu,
            Species::Xatu => PokemonData::Xatu,
            Species::Mareep => PokemonData::Mareep,
            Species::Flaaffy => PokemonData::Flaaffy,
            Species::Ampharos => PokemonData::Ampharos,
            Species::Bellossom => PokemonData::Bellossom,
            Species::Marill => PokemonData::Marill,
            Species::Azumarill => PokemonData::Azumarill,
            Species::Sudowoodo => PokemonData::Sudowoodo,
            Species::Politoed => PokemonData::Politoed,
            Species::Hoppip => PokemonData::Hoppip,
            Species::Skiploom => PokemonData::Skiploom,
            Species::Jumpluff => PokemonData::Jumpluff,
            Species::Aipom => PokemonData::Aipom,
            Species::Sunkern => PokemonData::Sunkern,
            Species::Sunflora => PokemonData::Sunflora,
            Species::Yanma => PokemonData::Yanma,
            Species::Wooper => PokemonData::Wooper,
            Species::Quagsire => PokemonData::Quagsire,
            Species::Espeon => PokemonData::Espeon,
            Species::Umbreon => PokemonData::Umbreon,
            Species::Murkrow => PokemonData::Murkrow,
            Species::Slowking => PokemonData::Slowking,
            Species::Misdreavus => PokemonData::Misdreavus,
            Species::Unown(_) => PokemonData::Unown,
            Species::Wobbuffet => PokemonData::Wobbuffet,
            Species::Girafarig => PokemonData::Girafarig,
            Species::Pineco => PokemonData::Pineco,
            Species::Forretress => PokemonData::Forretress,
            Species::Dunsparce => PokemonData::Dunsparce,
            Species::Gligar => PokemonData::Gligar,
            Species::Steelix => PokemonData::Steelix,
            Species::Snubbull => PokemonData::Snubbull,
            Species::Granbull => PokemonData::Granbull,
            Species::Qwilfish => PokemonData::Qwilfish,
            Species::Scizor => PokemonData::Scizor,
            Species::Shuckle => PokemonData::Shuckle,
            Species::Heracross => PokemonData::Heracross,
            Species::Sneasel => PokemonData::Sneasel,
            Species::Teddiursa => PokemonData::Teddiursa,
            Species::Ursaring => PokemonData::Ursaring,
            Species::Slugma => PokemonData::Slugma,
            Species::Magcargo => PokemonData::Magcargo,
            Species::Swinub => PokemonData::Swinub,
            Species::Piloswine => PokemonData::Piloswine,
            Species::Corsola => PokemonData::Corsola,
            Species::Remoraid => PokemonData::Remoraid,
            Species::Octillery => PokemonData::Octillery,
            Species::Delibird => PokemonData::Delibird,
            Species::Mantine => PokemonData::Mantine,
            Species::Skarmory => PokemonData::Skarmory,
            Species::Houndour => PokemonData::Houndour,
            Species::Houndoom => PokemonData::Houndoom,
            Species::Kingdra => PokemonData::Kingdra,
            Species::Phanpy => PokemonData::Phanpy,
            Species::Donphan => PokemonData::Donphan,
            Species::Porygon2 => PokemonData::Porygon2,
            Species::Stantler => PokemonData::Stantler,
            Species::Smeargle => PokemonData::Smeargle,
            Species::Tyrogue => PokemonData::Tyrogue,
            Species::Hitmontop => PokemonData::Hitmontop,
            Species::Smoochum => PokemonData::Smoochum,
            Species::Elekid => PokemonData::Elekid,
            Species::Magby => PokemonData::Magby,
            Species::Miltank => PokemonData::Miltank,
            Species::Blissey => PokemonData::Blissey,
            Species::Raikou => PokemonData::Raikou,
            Species::Entei => PokemonData::Entei,
            Species::Suicune => PokemonData::Suicune,
            Species::Larvitar => PokemonData::Larvitar,
            Species::Pupitar => PokemonData::Pupitar,
            Species::Tyranitar => PokemonData::Tyranitar,
            Species::Lugia => PokemonData::Lugia,
            Species::HoOh => PokemonData::HoOh,
            Species::Celebi => PokemonData::Celebi,
            Species::Treecko => PokemonData::Treecko,
            Species::Grovyle => PokemonData::Grovyle,
            Species::Sceptile => PokemonData::Sceptile,
            Species::Torchic => PokemonData::Torchic,
            Species::Combusken => PokemonData::Combusken,
            Species::Blaziken => PokemonData::Blaziken,
            Species::Mudkip => PokemonData::Mudkip,
            Species::Marshtomp => PokemonData::Marshtomp,
            Species::Swampert => PokemonData::Swampert,
            Species::Poochyena => PokemonData::Poochyena,
            Species::Mightyena => PokemonData::Mightyena,
            Species::Zigzagoon => PokemonData::Zigzagoon,
            Species::Linoone => PokemonData::Linoone,
            Species::Wurmple => PokemonData::Wurmple,
            Species::Silcoon => PokemonData::Silcoon,
            Species::Beautifly => PokemonData::Beautifly,
            Species::Cascoon => PokemonData::Cascoon,
            Species::Dustox => PokemonData::Dustox,
            Species::Lotad => PokemonData::Lotad,
            Species::Lombre => PokemonData::Lombre,
            Species::Ludicolo => PokemonData::Ludicolo,
            Species::Seedot => PokemonData::Seedot,
            Species::Nuzleaf => PokemonData::Nuzleaf,
            Species::Shiftry => PokemonData::Shiftry,
            Species::Taillow => PokemonData::Taillow,
            Species::Swellow => PokemonData::Swellow,
            Species::Wingull => PokemonData::Wingull,
            Species::Pelipper => PokemonData::Pelipper,
            Species::Ralts => PokemonData::Ralts,
            Species::Kirlia => PokemonData::Kirlia,
            Species::Gardevoir => PokemonData::Gardevoir,
            Species::Surskit => PokemonData::Surskit,
            Species::Masquerain => PokemonData::Masquerain,
            Species::Shroomish => PokemonData::Shroomish,
            Species::Breloom => PokemonData::Breloom,
            Species::Slakoth => PokemonData::Slakoth,
            Species::Vigoroth => PokemonData::Vigoroth,
            Species::Slaking => PokemonData::Slaking,
            Species::Nincada => PokemonData::Nincada,
            Species::Ninjask => PokemonData::Ninjask,
            Species::Shedinja => PokemonData::Shedinja,
            Species::Whismur => PokemonData::Whismur,
            Species::Loudred => PokemonData::Loudred,
            Species::Exploud => PokemonData::Exploud,
            Species::Makuhita => PokemonData::Makuhita,
            Species::Hariyama => PokemonData::Hariyama,
            Species::Azurill => PokemonData::Azurill,
            Species::Nosepass => PokemonData::Nosepass,
            Species::Skitty => PokemonData::Skitty,
            Species::Delcatty => PokemonData::Delcatty,
            Species::Sableye => PokemonData::Sableye,
            Species::Mawile => PokemonData::Mawile,
            Species::Aron => PokemonData::Aron,
            Species::Lairon => PokemonData::Lairon,
            Species::Aggron => PokemonData::Aggron,
            Species::Meditite => PokemonData::Meditite,
            Species::Medicham => PokemonData::Medicham,
            Species::Electrike => PokemonData::Electrike,
            Species::Manectric => PokemonData::Manectric,
            Species::Plusle => PokemonData::Plusle,
            Species::Minun => PokemonData::Minun,
            Species::Volbeat => PokemonData::Volbeat,
            Species::Illumise => PokemonData::Illumise,
            Species::Roselia => PokemonData::Roselia,
            Species::Gulpin => PokemonData::Gulpin,
            Species::Swalot => PokemonData::Swalot,
            Species::Carvanha => PokemonData::Carvanha,
            Species::Sharpedo => PokemonData::Sharpedo,
            Species::Wailmer => PokemonData::Wailmer,
            Species::Wailord => PokemonData::Wailord,
            Species::Numel => PokemonData::Numel,
            Species::Camerupt => PokemonData::Camerupt,
            Species::Torkoal => PokemonData::Torkoal,
            Species::Spoink => PokemonData::Spoink,
            Species::Grumpig => PokemonData::Grumpig,
            Species::Spinda => PokemonData::Spinda,
            Species::Trapinch => PokemonData::Trapinch,
            Species::Vibrava => PokemonData::Vibrava,
            Species::Flygon => PokemonData::Flygon,
            Species::Cacnea => PokemonData::Cacnea,
            Species::Cacturne => PokemonData::Cacturne,
            Species::Swablu => PokemonData::Swablu,
            Species::Altaria => PokemonData::Altaria,
            Species::Zangoose => PokemonData::Zangoose,
            Species::Seviper => PokemonData::Seviper,
            Species::Lunatone => PokemonData::Lunatone,
            Species::Solrock => PokemonData::Solrock,
            Species::Barboach => PokemonData::Barboach,
            Species::Whiscash => PokemonData::Whiscash,
            Species::Corphish => PokemonData::Corphish,
            Species::Crawdaunt => PokemonData::Crawdaunt,
            Species::Baltoy => PokemonData::Baltoy,
            Species::Claydol => PokemonData::Claydol,
            Species::Lileep => PokemonData::Lileep,
            Species::Cradily => PokemonData::Cradily,
            Species::Anorith => PokemonData::Anorith,
            Species::Armaldo => PokemonData::Armaldo,
            Species::Feebas => PokemonData::Feebas,
            Species::Milotic => PokemonData::Milotic,
            Species::Castform(_) => PokemonData::Castform,
            Species::Kecleon => PokemonData::Kecleon,
            Species::Shuppet => PokemonData::Shuppet,
            Species::Banette => PokemonData::Banette,
            Species::Duskull => PokemonData::Duskull,
            Species::Dusclops => PokemonData::Dusclops,
            Species::Tropius => PokemonData::Tropius,
            Species::Chimecho => PokemonData::Chimecho,
            Species::Absol => PokemonData::Absol,
            Species::Wynaut => PokemonData::Wynaut,
            Species::Snorunt => PokemonData::Snorunt,
            Species::Glalie => PokemonData::Glalie,
            Species::Spheal => PokemonData::Spheal,
            Species::Sealeo => PokemonData::Sealeo,
            Species::Walrein => PokemonData::Walrein,
            Species::Clamperl => PokemonData::Clamperl,
            Species::Huntail => PokemonData::Huntail,
            Species::Gorebyss => PokemonData::Gorebyss,
            Species::Relicanth => PokemonData::Relicanth,
            Species::Luvdisc => PokemonData::Luvdisc,
            Species::Bagon => PokemonData::Bagon,
            Species::Shelgon => PokemonData::Shelgon,
            Species::Salamence => PokemonData::Salamence,
            Species::Beldum => PokemonData::Beldum,
            Species::Metang => PokemonData::Metang,
            Species::Metagross => PokemonData::Metagross,
            Species::Regirock => PokemonData::Regirock,
            Species::Regice => PokemonData::Regice,
            Species::Registeel => PokemonData::Registeel,
            Species::Latias => PokemonData::Latias,
            Species::Latios => PokemonData::Latios,
            Species::Kyogre => PokemonData::Kyogre,
            Species::Groudon => PokemonData::Groudon,
            Species::Rayquaza => PokemonData::Rayquaza,
            Species::Jirachi => PokemonData::Jirachi,
            Species::Deoxys(_) => PokemonData::DeoxysNormal,
            Species::Turtwig => PokemonData::Turtwig,
            Species::Grotle => PokemonData::Grotle,
            Species::Torterra => PokemonData::Torterra,
            Species::Chimchar => PokemonData::Chimchar,
            Species::Monferno => PokemonData::Monferno,
            Species::Infernape => PokemonData::Infernape,
            Species::Piplup => PokemonData::Piplup,
            Species::Prinplup => PokemonData::Prinplup,
            Species::Empoleon => PokemonData::Empoleon,
            Species::Starly => PokemonData::Starly,
            Species::Staravia => PokemonData::Staravia,
            Species::Staraptor => PokemonData::Staraptor,
            Species::Bidoof => PokemonData::Bidoof,
            Species::Bibarel => PokemonData::Bibarel,
            Species::Kricketot => PokemonData::Kricketot,
            Species::Kricketune => PokemonData::Kricketune,
            Species::Shinx => PokemonData::Shinx,
            Species::Luxio => PokemonData::Luxio,
            Species::Luxray => PokemonData::Luxray,
            Species::Budew => PokemonData::Budew,
            Species::Roserade => PokemonData::Roserade,
            Species::Cranidos => PokemonData::Cranidos,
            Species::Rampardos => PokemonData::Rampardos,
            Species::Shieldon => PokemonData::Shieldon,
            Species::Bastiodon => PokemonData::Bastiodon,
            Species::Burmy(_) => PokemonData::Burmy,
            Species::Wormadam(_) => PokemonData::WormadamPlant,
            Species::Mothim => PokemonData::Mothim,
            Species::Combee => PokemonData::Combee,
            Species::Vespiquen => PokemonData::Vespiquen,
            Species::Pachirisu => PokemonData::Pachirisu,
            Species::Buizel => PokemonData::Buizel,
            Species::Floatzel => PokemonData::Floatzel,
            Species::Cherubi => PokemonData::Cherubi,
            Species::Cherrim(_) => PokemonData::Cherrim,
            Species::Shellos(_) => PokemonData::Shellos,
            Species::Gastrodon(_) => PokemonData::Gastrodon,
            Species::Ambipom => PokemonData::Ambipom,
            Species::Drifloon => PokemonData::Drifloon,
            Species::Drifblim => PokemonData::Drifblim,
            Species::Buneary => PokemonData::Buneary,
            Species::Lopunny => PokemonData::Lopunny,
            Species::Mismagius => PokemonData::Mismagius,
            Species::Honchkrow => PokemonData::Honchkrow,
            Species::Glameow => PokemonData::Glameow,
            Species::Purugly => PokemonData::Purugly,
            Species::Chingling => PokemonData::Chingling,
            Species::Stunky => PokemonData::Stunky,
            Species::Skuntank => PokemonData::Skuntank,
            Species::Bronzor => PokemonData::Bronzor,
            Species::Bronzong => PokemonData::Bronzong,
            Species::Bonsly => PokemonData::Bonsly,
            Species::MimeJr => PokemonData::MimeJr,
            Species::Happiny => PokemonData::Happiny,
            Species::Chatot => PokemonData::Chatot,
            Species::Spiritomb => PokemonData::Spiritomb,
            Species::Gible => PokemonData::Gible,
            Species::Gabite => PokemonData::Gabite,
            Species::Garchomp => PokemonData::Garchomp,
            Species::Munchlax => PokemonData::Munchlax,
            Species::Riolu => PokemonData::Riolu,
            Species::Lucario => PokemonData::Lucario,
            Species::Hippopotas => PokemonData::Hippopotas,
            Species::Hippowdon => PokemonData::Hippowdon,
            Species::Skorupi => PokemonData::Skorupi,
            Species::Drapion => PokemonData::Drapion,
            Species::Croagunk => PokemonData::Croagunk,
            Species::Toxicroak => PokemonData::Toxicroak,
            Species::Carnivine => PokemonData::Carnivine,
            Species::Finneon => PokemonData::Finneon,
            Species::Lumineon => PokemonData::Lumineon,
            Species::Mantyke => PokemonData::Mantyke,
            Species::Snover => PokemonData::Snover,
            Species::Abomasnow => PokemonData::Abomasnow,
            Species::Weavile => PokemonData::Weavile,
            Species::Magnezone => PokemonData::Magnezone,
            Species::Lickilicky => PokemonData::Lickilicky,
            Species::Rhyperior => PokemonData::Rhyperior,
            Species::Tangrowth => PokemonData::Tangrowth,
            Species::Electivire => PokemonData::Electivire,
            Species::Magmortar => PokemonData::Magmortar,
            Species::Togekiss => PokemonData::Togekiss,
            Species::Yanmega => PokemonData::Yanmega,
            Species::Leafeon => PokemonData::Leafeon,
            Species::Glaceon => PokemonData::Glaceon,
            Species::Gliscor => PokemonData::Gliscor,
            Species::Mamoswine => PokemonData::Mamoswine,
            Species::PorygonZ => PokemonData::PorygonZ,
            Species::Gallade => PokemonData::Gallade,
            Species::Probopass => PokemonData::Probopass,
            Species::Dusknoir => PokemonData::Dusknoir,
            Species::Froslass => PokemonData::Froslass,
            Species::Rotom(_) => PokemonData::Rotom,
            Species::Uxie => PokemonData::Uxie,
            Species::Mesprit => PokemonData::Mesprit,
            Species::Azelf => PokemonData::Azelf,
            Species::Dialga => PokemonData::Dialga,
            Species::Palkia => PokemonData::Palkia,
            Species::Heatran => PokemonData::Heatran,
            Species::Regigigas => PokemonData::Regigigas,
            Species::Giratina(_) => PokemonData::GiratinaAltered,
            Species::Cresselia => PokemonData::Cresselia,
            Species::Phione => PokemonData::Phione,
            Species::Manaphy => PokemonData::Manaphy,
            Species::Darkrai => PokemonData::Darkrai,
            Species::Shaymin(_) => PokemonData::ShayminLand,
            Species::Arceus(_) => PokemonData::Arceus,
            Species::Victini => PokemonData::Victini,
            Species::Snivy => PokemonData::Snivy,
            Species::Servine => PokemonData::Servine,
            Species::Serperior => PokemonData::Serperior,
            Species::Tepig => PokemonData::Tepig,
            Species::Pignite => PokemonData::Pignite,
            Species::Emboar => PokemonData::Emboar,
            Species::Oshawott => PokemonData::Oshawott,
            Species::Dewott => PokemonData::Dewott,
            Species::Samurott => PokemonData::Samurott,
            Species::Patrat => PokemonData::Patrat,
            Species::Watchog => PokemonData::Watchog,
            Species::Lillipup => PokemonData::Lillipup,
            Species::Herdier => PokemonData::Herdier,
            Species::Stoutland => PokemonData::Stoutland,
            Species::Purrloin => PokemonData::Purrloin,
            Species::Liepard => PokemonData::Liepard,
            Species::Pansage => PokemonData::Pansage,
            Species::Simisage => PokemonData::Simisage,
            Species::Pansear => PokemonData::Pansear,
            Species::Simisear => PokemonData::Simisear,
            Species::Panpour => PokemonData::Panpour,
            Species::Simipour => PokemonData::Simipour,
            Species::Munna => PokemonData::Munna,
            Species::Musharna => PokemonData::Musharna,
            Species::Pidove => PokemonData::Pidove,
            Species::Tranquill => PokemonData::Tranquill,
            Species::Unfezant => PokemonData::Unfezant,
            Species::Blitzle => PokemonData::Blitzle,
            Species::Zebstrika => PokemonData::Zebstrika,
            Species::Roggenrola => PokemonData::Roggenrola,
            Species::Boldore => PokemonData::Boldore,
            Species::Gigalith => PokemonData::Gigalith,
            Species::Woobat => PokemonData::Woobat,
            Species::Swoobat => PokemonData::Swoobat,
            Species::Drilbur => PokemonData::Drilbur,
            Species::Excadrill => PokemonData::Excadrill,
            Species::Audino => PokemonData::Audino,
            Species::Timburr => PokemonData::Timburr,
            Species::Gurdurr => PokemonData::Gurdurr,
            Species::Conkeldurr => PokemonData::Conkeldurr,
            Species::Tympole => PokemonData::Tympole,
            Species::Palpitoad => PokemonData::Palpitoad,
            Species::Seismitoad => PokemonData::Seismitoad,
            Species::Throh => PokemonData::Throh,
            Species::Sawk => PokemonData::Sawk,
            Species::Sewaddle => PokemonData::Sewaddle,
            Species::Swadloon => PokemonData::Swadloon,
            Species::Leavanny => PokemonData::Leavanny,
            Species::Venipede => PokemonData::Venipede,
            Species::Whirlipede => PokemonData::Whirlipede,
            Species::Scolipede => PokemonData::Scolipede,
            Species::Cottonee => PokemonData::Cottonee,
            Species::Whimsicott => PokemonData::Whimsicott,
            Species::Petilil => PokemonData::Petilil,
            Species::Lilligant => PokemonData::Lilligant,
            Species::Basculin(_) => PokemonData::BasculinRedStriped,
            Species::Sandile => PokemonData::Sandile,
            Species::Krokorok => PokemonData::Krokorok,
            Species::Krookodile => PokemonData::Krookodile,
            Species::Darumaka => PokemonData::Darumaka,
            Species::Darmanitan(_) => PokemonData::DarmanitanStandard,
            Species::Maractus => PokemonData::Maractus,
            Species::Dwebble => PokemonData::Dwebble,
            Species::Crustle => PokemonData::Crustle,
            Species::Scraggy => PokemonData::Scraggy,
            Species::Scrafty => PokemonData::Scrafty,
            Species::Sigilyph => PokemonData::Sigilyph,
            Species::Yamask => PokemonData::Yamask,
            Species::Cofagrigus => PokemonData::Cofagrigus,
            Species::Tirtouga => PokemonData::Tirtouga,
            Species::Carracosta => PokemonData::Carracosta,
            Species::Archen => PokemonData::Archen,
            Species::Archeops => PokemonData::Archeops,
            Species::Trubbish => PokemonData::Trubbish,
            Species::Garbodor => PokemonData::Garbodor,
            Species::Zorua => PokemonData::Zorua,
            Species::Zoroark => PokemonData::Zoroark,
            Species::Minccino => PokemonData::Minccino,
            Species::Cinccino => PokemonData::Cinccino,
            Species::Gothita => PokemonData::Gothita,
            Species::Gothorita => PokemonData::Gothorita,
            Species::Gothitelle => PokemonData::Gothitelle,
            Species::Solosis => PokemonData::Solosis,
            Species::Duosion => PokemonData::Duosion,
            Species::Reuniclus => PokemonData::Reuniclus,
            Species::Ducklett => PokemonData::Ducklett,
            Species::Swanna => PokemonData::Swanna,
            Species::Vanillite => PokemonData::Vanillite,
            Species::Vanillish => PokemonData::Vanillish,
            Species::Vanilluxe => PokemonData::Vanilluxe,
            Species::Deerling(_) => PokemonData::Deerling,
            Species::Sawsbuck(_) => PokemonData::Sawsbuck,
            Species::Emolga => PokemonData::Emolga,
            Species::Karrablast => PokemonData::Karrablast,
            Species::Escavalier => PokemonData::Escavalier,
            Species::Foongus => PokemonData::Foongus,
            Species::Amoonguss => PokemonData::Amoonguss,
            Species::Frillish => PokemonData::Frillish,
            Species::Jellicent => PokemonData::Jellicent,
            Species::Alomomola => PokemonData::Alomomola,
            Species::Joltik => PokemonData::Joltik,
            Species::Galvantula => PokemonData::Galvantula,
            Species::Ferroseed => PokemonData::Ferroseed,
            Species::Ferrothorn => PokemonData::Ferrothorn,
            Species::Klink => PokemonData::Klink,
            Species::Klang => PokemonData::Klang,
            Species::Klinklang => PokemonData::Klinklang,
            Species::Tynamo => PokemonData::Tynamo,
            Species::Eelektrik => PokemonData::Eelektrik,
            Species::Eelektross => PokemonData::Eelektross,
            Species::Elgyem => PokemonData::Elgyem,
            Species::Beheeyem => PokemonData::Beheeyem,
            Species::Litwick => PokemonData::Litwick,
            Species::Lampent => PokemonData::Lampent,
            Species::Chandelure => PokemonData::Chandelure,
            Species::Axew => PokemonData::Axew,
            Species::Fraxure => PokemonData::Fraxure,
            Species::Haxorus => PokemonData::Haxorus,
            Species::Cubchoo => PokemonData::Cubchoo,
            Species::Beartic => PokemonData::Beartic,
            Species::Cryogonal => PokemonData::Cryogonal,
            Species::Shelmet => PokemonData::Shelmet,
            Species::Accelgor => PokemonData::Accelgor,
            Species::Stunfisk => PokemonData::Stunfisk,
            Species::Mienfoo => PokemonData::Mienfoo,
            Species::Mienshao => PokemonData::Mienshao,
            Species::Druddigon => PokemonData::Druddigon,
            Species::Golett => PokemonData::Golett,
            Species::Golurk => PokemonData::Golurk,
            Species::Pawniard => PokemonData::Pawniard,
            Species::Bisharp => PokemonData::Bisharp,
            Species::Bouffalant => PokemonData::Bouffalant,
            Species::Rufflet => PokemonData::Rufflet,
            Species::Braviary => PokemonData::Braviary,
            Species::Vullaby => PokemonData::Vullaby,
            Species::Mandibuzz => PokemonData::Mandibuzz,
            Species::Heatmor => PokemonData::Heatmor,
            Species::Durant => PokemonData::Durant,
            Species::Deino => PokemonData::Deino,
            Species::Zweilous => PokemonData::Zweilous,
            Species::Hydreigon => PokemonData::Hydreigon,
            Species::Larvesta => PokemonData::Larvesta,
            Species::Volcarona => PokemonData::Volcarona,
            Species::Cobalion => PokemonData::Cobalion,
            Species::Terrakion => PokemonData::Terrakion,
            Species::Virizion => PokemonData::Virizion,
            Species::Tornadus(_) => PokemonData::TornadusIncarnate,
            Species::Thundurus(_) => PokemonData::ThundurusIncarnate,
            Species::Reshiram => PokemonData::Reshiram,
            Species::Zekrom => PokemonData::Zekrom,
            Species::Landorus(_) => PokemonData::LandorusIncarnate,
            Species::Kyurem(_) => PokemonData::Kyurem,
            Species::Keldeo(_) => PokemonData::KeldeoOrdinary,
            Species::Meloetta(_) => PokemonData::MeloettaAria,
            Species::Genesect(_) => PokemonData::Genesect
        }
    }
}

#[allow(non_upper_case_globals)]
impl PokemonData {
    pub const Bulbasaur: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Chlorophyll),
        height: 7,
        weight: 69,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(49, 0),
            Stat::new(49, 0),
            Stat::new(65, 1),
            Stat::new(65, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::Growl),
            (7, Move::LeechSeed),
            (9, Move::VineWhip),
            (13, Move::PoisonPowder),
            (13, Move::SleepPowder),
            (15, Move::TakeDown),
            (19, Move::RazorLeaf),
            (21, Move::SweetScent),
            (25, Move::Growth),
            (27, Move::DoubleEdge),
            (31, Move::WorrySeed),
            (33, Move::Synthesis),
            (37, Move::SeedBomb),
        ]
    };
    pub const Ivysaur: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Chlorophyll),
        height: 10,
        weight: 130,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(62, 0),
            Stat::new(63, 0),
            Stat::new(80, 1),
            Stat::new(80, 1),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::LeechSeed),
            (9, Move::VineWhip),
            (13, Move::PoisonPowder),
            (13, Move::SleepPowder),
            (15, Move::TakeDown),
            (20, Move::RazorLeaf),
            (23, Move::SweetScent),
            (28, Move::Growth),
            (31, Move::DoubleEdge),
            (36, Move::WorrySeed),
            (39, Move::Synthesis),
            (44, Move::SolarBeam),
        ]
    };
    pub const Venusaur: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Chlorophyll),
        height: 20,
        weight: 1000,
        base_exp_yield: 236,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(82, 0),
            Stat::new(83, 0),
            Stat::new(100, 2),
            Stat::new(100, 1),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::VineWhip),
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::LeechSeed),
            (13, Move::PoisonPowder),
            (13, Move::SleepPowder),
            (15, Move::TakeDown),
            (20, Move::RazorLeaf),
            (23, Move::SweetScent),
            (28, Move::Growth),
            (31, Move::DoubleEdge),
            (32, Move::PetalDance),
            (39, Move::WorrySeed),
            (45, Move::Synthesis),
            (53, Move::SolarBeam),
        ]
    };
    pub const Charmander: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::SolarPower),
        height: 6,
        weight: 85,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(39, 0),
            Stat::new(52, 0),
            Stat::new(43, 0),
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (7, Move::Ember),
            (10, Move::Smokescreen),
            (16, Move::DragonRage),
            (19, Move::ScaryFace),
            (25, Move::FireFang),
            (28, Move::FlameBurst),
            (34, Move::Slash),
            (37, Move::Flamethrower),
            (43, Move::FireSpin),
            (46, Move::Inferno),
        ]
    };
    pub const Charmeleon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::SolarPower),
        height: 11,
        weight: 190,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(58, 0),
            Stat::new(64, 0),
            Stat::new(58, 0),
            Stat::new(80, 1),
            Stat::new(65, 0),
            Stat::new(80, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (1, Move::Ember),
            (10, Move::Smokescreen),
            (17, Move::DragonRage),
            (21, Move::ScaryFace),
            (28, Move::FireFang),
            (32, Move::FlameBurst),
            (39, Move::Slash),
            (43, Move::Flamethrower),
            (50, Move::FireSpin),
            (54, Move::Inferno),
        ]
    };
    pub const Charizard: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Flying),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::SolarPower),
        height: 17,
        weight: 905,
        base_exp_yield: 240,
        stats: Stats(
            Stat::new(78, 0),
            Stat::new(84, 0),
            Stat::new(78, 0),
            Stat::new(109, 3),
            Stat::new(85, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (1, Move::Ember),
            (1, Move::Smokescreen),
            (1, Move::DragonClaw),
            (1, Move::AirSlash),
            (1, Move::ShadowClaw),
            (17, Move::DragonRage),
            (21, Move::ScaryFace),
            (28, Move::FireFang),
            (32, Move::FlameBurst),
            (36, Move::WingAttack),
            (41, Move::Slash),
            (47, Move::Flamethrower),
            (56, Move::FireSpin),
            (62, Move::Inferno),
            (71, Move::HeatWave),
            (77, Move::FlareBlitz),
        ]
    };
    pub const Squirtle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::RainDish),
        height: 5,
        weight: 90,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(44, 0),
            Stat::new(48, 0),
            Stat::new(65, 1),
            Stat::new(50, 0),
            Stat::new(64, 0),
            Stat::new(43, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::TailWhip),
            (7, Move::Bubble),
            (10, Move::Withdraw),
            (13, Move::WaterGun),
            (16, Move::Bite),
            (19, Move::RapidSpin),
            (22, Move::Protect),
            (25, Move::WaterPulse),
            (28, Move::AquaTail),
            (31, Move::SkullBash),
            (34, Move::IronDefense),
            (37, Move::RainDance),
            (40, Move::HydroPump),
        ]
    };
    pub const Wartortle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::RainDish),
        height: 10,
        weight: 225,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(59, 0),
            Stat::new(63, 0),
            Stat::new(80, 1),
            Stat::new(65, 0),
            Stat::new(80, 1),
            Stat::new(58, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Bubble),
            (10, Move::Withdraw),
            (13, Move::WaterGun),
            (16, Move::Bite),
            (20, Move::RapidSpin),
            (24, Move::Protect),
            (28, Move::WaterPulse),
            (32, Move::AquaTail),
            (36, Move::SkullBash),
            (40, Move::IronDefense),
            (44, Move::RainDance),
            (48, Move::HydroPump),
        ]
    };
    pub const Blastoise: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::RainDish),
        height: 16,
        weight: 855,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(79, 0),
            Stat::new(83, 0),
            Stat::new(100, 0),
            Stat::new(85, 0),
            Stat::new(105, 3),
            Stat::new(78, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Withdraw),
            (1, Move::Bubble),
            (1, Move::FlashCannon),
            (13, Move::WaterGun),
            (16, Move::Bite),
            (20, Move::RapidSpin),
            (24, Move::Protect),
            (28, Move::WaterPulse),
            (32, Move::AquaTail),
            (39, Move::SkullBash),
            (46, Move::IronDefense),
            (53, Move::RainDance),
            (60, Move::HydroPump),
        ]
    };
    pub const Caterpie: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShieldDust),
        hidden_ability: Some(Ability::RunAway),
        height: 3,
        weight: 29,
        base_exp_yield: 39,
        stats: Stats(
            Stat::new(45, 1),
            Stat::new(30, 0),
            Stat::new(35, 0),
            Stat::new(20, 0),
            Stat::new(20, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::StringShot),
            (15, Move::BugBite),
        ]
    };
    pub const Metapod: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: None,
        height: 7,
        weight: 99,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(20, 0),
            Stat::new(55, 2),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
        ]
    };
    pub const Butterfree: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::CompoundEyes),
        hidden_ability: Some(Ability::TintedLens),
        height: 11,
        weight: 320,
        base_exp_yield: 178,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(90, 2),
            Stat::new(80, 1),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (12, Move::PoisonPowder),
            (12, Move::StunSpore),
            (12, Move::SleepPowder),
            (16, Move::Gust),
            (18, Move::Supersonic),
            (22, Move::Whirlwind),
            (24, Move::Psybeam),
            (28, Move::SilverWind),
            (30, Move::Tailwind),
            (34, Move::RagePowder),
            (36, Move::Safeguard),
            (40, Move::Captivate),
            (42, Move::BugBuzz),
            (46, Move::QuiverDance),
        ]
    };
    pub const Weedle: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::One(Ability::ShieldDust),
        hidden_ability: Some(Ability::RunAway),
        height: 3,
        weight: 32,
        base_exp_yield: 39,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(30, 0),
            Stat::new(20, 0),
            Stat::new(20, 0),
            Stat::new(50, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::StringShot),
            (15, Move::BugBite),
        ]
    };
    pub const Kakuna: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: None,
        height: 6,
        weight: 100,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(25, 0),
            Stat::new(50, 2),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
        ]
    };
    pub const Beedrill: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::One(Ability::Swarm),
        hidden_ability: Some(Ability::Sniper),
        height: 10,
        weight: 295,
        base_exp_yield: 178,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(90, 2),
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(80, 1),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FuryAttack),
            (13, Move::FocusEnergy),
            (16, Move::Twineedle),
            (19, Move::Rage),
            (22, Move::Pursuit),
            (25, Move::ToxicSpikes),
            (28, Move::PinMissile),
            (31, Move::Agility),
            (34, Move::Assurance),
            (37, Move::PoisonJab),
            (40, Move::Endeavor),
        ]
    };
    pub const Pidgey: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::TangledFeet),
        hidden_ability: Some(Ability::BigPecks),
        height: 3,
        weight: 18,
        base_exp_yield: 50,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(56, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::SandAttack),
            (9, Move::Gust),
            (13, Move::QuickAttack),
            (17, Move::Whirlwind),
            (21, Move::Twister),
            (25, Move::FeatherDance),
            (29, Move::Agility),
            (33, Move::WingAttack),
            (37, Move::Roost),
            (41, Move::Tailwind),
            (45, Move::MirrorMove),
            (49, Move::AirSlash),
            (53, Move::Hurricane),
        ]
    };
    pub const Pidgeotto: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::TangledFeet),
        hidden_ability: Some(Ability::BigPecks),
        height: 11,
        weight: 300,
        base_exp_yield: 122,
        stats: Stats(
            Stat::new(63, 0),
            Stat::new(60, 0),
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(71, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (13, Move::QuickAttack),
            (17, Move::Whirlwind),
            (22, Move::Twister),
            (27, Move::FeatherDance),
            (32, Move::Agility),
            (37, Move::WingAttack),
            (42, Move::Roost),
            (47, Move::Tailwind),
            (52, Move::MirrorMove),
            (57, Move::AirSlash),
            (62, Move::Hurricane),
        ]
    };
    pub const Pidgeot: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::TangledFeet),
        hidden_ability: Some(Ability::BigPecks),
        height: 15,
        weight: 395,
        base_exp_yield: 216,
        stats: Stats(
            Stat::new(83, 0),
            Stat::new(80, 0),
            Stat::new(75, 0),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(101, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (1, Move::QuickAttack),
            (17, Move::Whirlwind),
            (22, Move::Twister),
            (27, Move::FeatherDance),
            (32, Move::Agility),
            (38, Move::WingAttack),
            (44, Move::Roost),
            (50, Move::Tailwind),
            (56, Move::MirrorMove),
            (62, Move::AirSlash),
            (68, Move::Hurricane),
        ]
    };
    pub const Rattata: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::Guts),
        hidden_ability: Some(Ability::Hustle),
        height: 3,
        weight: 35,
        base_exp_yield: 51,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(56, 0),
            Stat::new(35, 0),
            Stat::new(25, 0),
            Stat::new(35, 0),
            Stat::new(72, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (4, Move::QuickAttack),
            (7, Move::FocusEnergy),
            (10, Move::Bite),
            (13, Move::Pursuit),
            (16, Move::HyperFang),
            (19, Move::SuckerPunch),
            (22, Move::Crunch),
            (25, Move::Assurance),
            (28, Move::SuperFang),
            (31, Move::DoubleEdge),
            (34, Move::Endeavor),
        ]
    };
    pub const Raticate: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::Guts),
        hidden_ability: Some(Ability::Hustle),
        height: 7,
        weight: 185,
        base_exp_yield: 145,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(81, 0),
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(70, 0),
            Stat::new(97, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SwordsDance),
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::QuickAttack),
            (1, Move::FocusEnergy),
            (10, Move::Bite),
            (13, Move::Pursuit),
            (16, Move::HyperFang),
            (19, Move::SuckerPunch),
            (20, Move::ScaryFace),
            (24, Move::Crunch),
            (29, Move::Assurance),
            (34, Move::SuperFang),
            (39, Move::DoubleEdge),
            (44, Move::Endeavor),
        ]
    };
    pub const Spearow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::KeenEye),
        hidden_ability: Some(Ability::Sniper),
        height: 3,
        weight: 20,
        base_exp_yield: 52,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(30, 0),
            Stat::new(31, 0),
            Stat::new(31, 0),
            Stat::new(70, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Peck),
            (5, Move::Leer),
            (9, Move::FuryAttack),
            (13, Move::Pursuit),
            (17, Move::AerialAce),
            (21, Move::MirrorMove),
            (25, Move::Agility),
            (29, Move::Assurance),
            (33, Move::Roost),
            (37, Move::DrillPeck),
        ]
    };
    pub const Fearow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::KeenEye),
        hidden_ability: Some(Ability::Sniper),
        height: 12,
        weight: 380,
        base_exp_yield: 155,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(90, 0),
            Stat::new(65, 0),
            Stat::new(61, 0),
            Stat::new(61, 0),
            Stat::new(100, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FuryAttack),
            (1, Move::Leer),
            (1, Move::Growl),
            (1, Move::Peck),
            (1, Move::Pluck),
            (13, Move::Pursuit),
            (17, Move::AerialAce),
            (23, Move::MirrorMove),
            (29, Move::Agility),
            (35, Move::Assurance),
            (41, Move::Roost),
            (47, Move::DrillPeck),
            (53, Move::DrillRun),
        ]
    };
    pub const Ekans: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::ShedSkin),
        hidden_ability: Some(Ability::Unnerve),
        height: 20,
        weight: 69,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(60, 1),
            Stat::new(44, 0),
            Stat::new(40, 0),
            Stat::new(54, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::Leer),
            (4, Move::PoisonSting),
            (9, Move::Bite),
            (12, Move::Glare),
            (17, Move::Screech),
            (20, Move::Acid),
            (25, Move::Stockpile),
            (25, Move::SpitUp),
            (25, Move::Swallow),
            (28, Move::AcidSpray),
            (33, Move::MudBomb),
            (36, Move::GastroAcid),
            (41, Move::Haze),
            (44, Move::Coil),
            (49, Move::GunkShot),
        ]
    };
    pub const Arbok: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::ShedSkin),
        hidden_ability: Some(Ability::Unnerve),
        height: 35,
        weight: 650,
        base_exp_yield: 157,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(95, 2),
            Stat::new(69, 0),
            Stat::new(65, 0),
            Stat::new(79, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::PoisonSting),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (12, Move::Glare),
            (17, Move::Screech),
            (20, Move::Acid),
            (22, Move::Crunch),
            (27, Move::Stockpile),
            (27, Move::SpitUp),
            (27, Move::Swallow),
            (32, Move::AcidSpray),
            (39, Move::MudBomb),
            (44, Move::GastroAcid),
            (51, Move::Haze),
            (56, Move::Coil),
            (63, Move::GunkShot),
        ]
    };
    pub const Pikachu: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::LightningRod),
        height: 4,
        weight: 60,
        base_exp_yield: 112,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(90, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::ThunderShock),
            (5, Move::TailWhip),
            (10, Move::ThunderWave),
            (13, Move::QuickAttack),
            (18, Move::ElectroBall),
            (21, Move::DoubleTeam),
            (26, Move::Slam),
            (29, Move::Thunderbolt),
            (34, Move::Feint),
            (37, Move::Agility),
            (42, Move::Discharge),
            (45, Move::LightScreen),
            (50, Move::Thunder),
        ]
    };
    pub const Raichu: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::LightningRod),
        height: 8,
        weight: 300,
        base_exp_yield: 218,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(90, 0),
            Stat::new(55, 0),
            Stat::new(90, 0),
            Stat::new(80, 0),
            Stat::new(110, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::ThunderShock),
            (1, Move::Thunderbolt),
            (1, Move::QuickAttack),
        ]
    };
    pub const Sandshrew: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::SandRush),
        height: 6,
        weight: 120,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(75, 0),
            Stat::new(85, 1),
            Stat::new(20, 0),
            Stat::new(30, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::DefenseCurl),
            (3, Move::SandAttack),
            (5, Move::PoisonSting),
            (7, Move::Rollout),
            (9, Move::RapidSpin),
            (11, Move::Swift),
            (14, Move::FuryCutter),
            (17, Move::Magnitude),
            (20, Move::FurySwipes),
            (23, Move::SandTomb),
            (26, Move::Slash),
            (30, Move::Dig),
            (34, Move::GyroBall),
            (38, Move::SwordsDance),
            (42, Move::Sandstorm),
            (46, Move::Earthquake),
        ]
    };
    pub const Sandslash: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::SandRush),
        height: 10,
        weight: 295,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(100, 0),
            Stat::new(110, 2),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::SandAttack),
            (1, Move::PoisonSting),
            (1, Move::DefenseCurl),
            (7, Move::Rollout),
            (9, Move::RapidSpin),
            (11, Move::Swift),
            (14, Move::FuryCutter),
            (17, Move::Magnitude),
            (20, Move::FurySwipes),
            (22, Move::CrushClaw),
            (23, Move::SandTomb),
            (26, Move::Slash),
            (30, Move::Dig),
            (34, Move::GyroBall),
            (38, Move::SwordsDance),
            (42, Move::Sandstorm),
            (46, Move::Earthquake),
        ]
    };
    pub const NidoranF: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Rivalry),
        hidden_ability: Some(Ability::Hustle),
        height: 4,
        weight: 70,
        base_exp_yield: 55,
        stats: Stats(
            Stat::new(55, 1),
            Stat::new(47, 0),
            Stat::new(52, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(41, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (7, Move::TailWhip),
            (9, Move::DoubleKick),
            (13, Move::PoisonSting),
            (19, Move::FurySwipes),
            (21, Move::Bite),
            (25, Move::HelpingHand),
            (31, Move::ToxicSpikes),
            (33, Move::Flatter),
            (37, Move::Crunch),
            (43, Move::Captivate),
            (45, Move::PoisonFang),
        ]
    };
    pub const Nidorina: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Rivalry),
        hidden_ability: Some(Ability::Hustle),
        height: 8,
        weight: 200,
        base_exp_yield: 128,
        stats: Stats(
            Stat::new(70, 2),
            Stat::new(62, 0),
            Stat::new(67, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(56, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (7, Move::TailWhip),
            (9, Move::DoubleKick),
            (13, Move::PoisonSting),
            (20, Move::FurySwipes),
            (23, Move::Bite),
            (28, Move::HelpingHand),
            (35, Move::ToxicSpikes),
            (38, Move::Flatter),
            (43, Move::Crunch),
            (50, Move::Captivate),
            (58, Move::PoisonFang),
        ]
    };
    pub const Nidoqueen: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Ground),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Rivalry),
        hidden_ability: Some(Ability::SheerForce),
        height: 13,
        weight: 600,
        base_exp_yield: 227,
        stats: Stats(
            Stat::new(90, 3),
            Stat::new(92, 0),
            Stat::new(87, 0),
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(76, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::DoubleKick),
            (1, Move::TailWhip),
            (1, Move::PoisonSting),
            (23, Move::ChipAway),
            (35, Move::BodySlam),
            (43, Move::EarthPower),
            (58, Move::Superpower),
        ]
    };
    pub const NidoranM: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Rivalry),
        hidden_ability: Some(Ability::Hustle),
        height: 5,
        weight: 90,
        base_exp_yield: 55,
        stats: Stats(
            Stat::new(46, 0),
            Stat::new(57, 1),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Peck),
            (7, Move::FocusEnergy),
            (9, Move::DoubleKick),
            (13, Move::PoisonSting),
            (19, Move::FuryAttack),
            (21, Move::HornAttack),
            (25, Move::HelpingHand),
            (31, Move::ToxicSpikes),
            (33, Move::Flatter),
            (37, Move::PoisonJab),
            (43, Move::Captivate),
            (45, Move::HornDrill),
        ]
    };
    pub const Nidorino: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Rivalry),
        hidden_ability: Some(Ability::Hustle),
        height: 9,
        weight: 195,
        base_exp_yield: 128,
        stats: Stats(
            Stat::new(61, 0),
            Stat::new(72, 2),
            Stat::new(57, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Peck),
            (7, Move::FocusEnergy),
            (9, Move::DoubleKick),
            (13, Move::PoisonSting),
            (20, Move::FuryAttack),
            (23, Move::HornAttack),
            (28, Move::HelpingHand),
            (35, Move::ToxicSpikes),
            (38, Move::Flatter),
            (43, Move::PoisonJab),
            (50, Move::Captivate),
            (58, Move::HornDrill),
        ]
    };
    pub const Nidoking: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Ground),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Rivalry),
        hidden_ability: Some(Ability::SheerForce),
        height: 14,
        weight: 620,
        base_exp_yield: 227,
        stats: Stats(
            Stat::new(81, 0),
            Stat::new(102, 3),
            Stat::new(77, 0),
            Stat::new(85, 0),
            Stat::new(75, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleKick),
            (1, Move::PoisonSting),
            (1, Move::Peck),
            (1, Move::FocusEnergy),
            (23, Move::ChipAway),
            (35, Move::Thrash),
            (43, Move::EarthPower),
            (58, Move::Megahorn),
        ]
    };
    pub const Clefairy: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fairy),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::MagicGuard),
        hidden_ability: Some(Ability::FriendGuard),
        height: 6,
        weight: 75,
        base_exp_yield: 113,
        stats: Stats(
            Stat::new(70, 2),
            Stat::new(45, 0),
            Stat::new(48, 0),
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Growl),
            (4, Move::Encore),
            (7, Move::Sing),
            (10, Move::DoubleSlap),
            (13, Move::DefenseCurl),
            (16, Move::FollowMe),
            (19, Move::Bestow),
            (22, Move::WakeUpSlap),
            (25, Move::Minimize),
            (28, Move::StoredPower),
            (31, Move::Metronome),
            (34, Move::CosmicPower),
            (37, Move::LuckyChant),
            (40, Move::BodySlam),
            (43, Move::Moonlight),
            (46, Move::LightScreen),
            (49, Move::Gravity),
            (52, Move::MeteorMash),
            (55, Move::HealingWish),
            (58, Move::AfterYou),
        ]
    };
    pub const Clefable: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fairy),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::MagicGuard),
        hidden_ability: Some(Ability::Unaware),
        height: 13,
        weight: 400,
        base_exp_yield: 217,
        stats: Stats(
            Stat::new(95, 3),
            Stat::new(70, 0),
            Stat::new(73, 0),
            Stat::new(95, 0),
            Stat::new(90, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleSlap),
            (1, Move::Sing),
            (1, Move::Minimize),
            (1, Move::Metronome),
        ]
    };
    pub const Vulpix: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::FlashFire),
        hidden_ability: Some(Ability::Drought),
        height: 6,
        weight: 99,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(38, 0),
            Stat::new(41, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(65, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (4, Move::TailWhip),
            (7, Move::Roar),
            (10, Move::QuickAttack),
            (12, Move::FireSpin),
            (15, Move::ConfuseRay),
            (18, Move::Imprison),
            (20, Move::FeintAttack),
            (23, Move::FlameBurst),
            (26, Move::WillOWisp),
            (28, Move::Hex),
            (31, Move::Payback),
            (34, Move::Flamethrower),
            (36, Move::Safeguard),
            (39, Move::Extrasensory),
            (42, Move::FireBlast),
            (44, Move::Grudge),
            (47, Move::Captivate),
            (50, Move::Inferno),
        ]
    };
    pub const Ninetales: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::FlashFire),
        hidden_ability: Some(Ability::Drought),
        height: 11,
        weight: 199,
        base_exp_yield: 177,
        stats: Stats(
            Stat::new(73, 0),
            Stat::new(76, 0),
            Stat::new(75, 0),
            Stat::new(81, 0),
            Stat::new(100, 1),
            Stat::new(100, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (1, Move::QuickAttack),
            (1, Move::ConfuseRay),
            (1, Move::Safeguard),
            (1, Move::NastyPlot),
        ]
    };
    pub const Jigglypuff: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Fairy),
        ability: PokemonAbility::One(Ability::CuteCharm),
        hidden_ability: Some(Ability::FriendGuard),
        height: 5,
        weight: 55,
        base_exp_yield: 95,
        stats: Stats(
            Stat::new(115, 2),
            Stat::new(45, 0),
            Stat::new(20, 0),
            Stat::new(45, 0),
            Stat::new(25, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Sing),
            (5, Move::DefenseCurl),
            (9, Move::Pound),
            (13, Move::Disable),
            (17, Move::Round),
            (21, Move::Rollout),
            (25, Move::DoubleSlap),
            (29, Move::Rest),
            (33, Move::BodySlam),
            (37, Move::GyroBall),
            (41, Move::WakeUpSlap),
            (45, Move::Mimic),
            (49, Move::HyperVoice),
            (53, Move::DoubleEdge),
        ]
    };
    pub const Wigglytuff: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Fairy),
        ability: PokemonAbility::One(Ability::CuteCharm),
        hidden_ability: Some(Ability::Frisk),
        height: 10,
        weight: 120,
        base_exp_yield: 196,
        stats: Stats(
            Stat::new(140, 3),
            Stat::new(70, 0),
            Stat::new(45, 0),
            Stat::new(85, 0),
            Stat::new(50, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleSlap),
            (1, Move::Sing),
            (1, Move::Disable),
            (1, Move::DefenseCurl),
        ]
    };
    pub const Zubat: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Flying),
        ability: PokemonAbility::One(Ability::InnerFocus),
        hidden_ability: Some(Ability::Infiltrator),
        height: 8,
        weight: 75,
        base_exp_yield: 49,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(55, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::LeechLife),
            (4, Move::Supersonic),
            (8, Move::Astonish),
            (12, Move::Bite),
            (15, Move::WingAttack),
            (19, Move::ConfuseRay),
            (23, Move::Swift),
            (26, Move::AirCutter),
            (30, Move::Acrobatics),
            (34, Move::MeanLook),
            (37, Move::PoisonFang),
            (41, Move::Haze),
            (45, Move::AirSlash),
        ]
    };
    pub const Golbat: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Flying),
        ability: PokemonAbility::One(Ability::InnerFocus),
        hidden_ability: Some(Ability::Infiltrator),
        height: 16,
        weight: 550,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(75, 0),
            Stat::new(90, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Supersonic),
            (1, Move::Screech),
            (1, Move::LeechLife),
            (1, Move::Astonish),
            (12, Move::Bite),
            (15, Move::WingAttack),
            (19, Move::ConfuseRay),
            (24, Move::Swift),
            (28, Move::AirCutter),
            (33, Move::Acrobatics),
            (38, Move::MeanLook),
            (42, Move::PoisonFang),
            (47, Move::Haze),
            (52, Move::AirSlash),
        ]
    };
    pub const Oddish: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::RunAway),
        height: 5,
        weight: 54,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(75, 1),
            Stat::new(65, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (5, Move::SweetScent),
            (9, Move::Acid),
            (13, Move::PoisonPowder),
            (15, Move::StunSpore),
            (17, Move::SleepPowder),
            (21, Move::MegaDrain),
            (25, Move::LuckyChant),
            (29, Move::NaturalGift),
            (33, Move::Moonlight),
            (37, Move::GigaDrain),
            (41, Move::PetalDance),
        ]
    };
    pub const Gloom: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Stench),
        height: 8,
        weight: 86,
        base_exp_yield: 138,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(85, 2),
            Stat::new(75, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Acid),
            (1, Move::Absorb),
            (1, Move::SweetScent),
            (13, Move::PoisonPowder),
            (15, Move::StunSpore),
            (17, Move::SleepPowder),
            (23, Move::MegaDrain),
            (29, Move::LuckyChant),
            (35, Move::NaturalGift),
            (41, Move::Moonlight),
            (47, Move::GigaDrain),
            (53, Move::PetalDance),
        ]
    };
    pub const Vileplume: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::EffectSpore),
        height: 12,
        weight: 186,
        base_exp_yield: 221,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(80, 0),
            Stat::new(85, 0),
            Stat::new(110, 3),
            Stat::new(90, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MegaDrain),
            (1, Move::PoisonPowder),
            (1, Move::StunSpore),
            (1, Move::Aromatherapy),
            (53, Move::PetalDance),
            (65, Move::SolarBeam),
        ]
    };
    pub const Paras: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Grass),
        ability: PokemonAbility::Two(Ability::EffectSpore, Ability::DrySkin),
        hidden_ability: Some(Ability::Damp),
        height: 3,
        weight: 54,
        base_exp_yield: 57,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(70, 1),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (6, Move::PoisonPowder),
            (6, Move::StunSpore),
            (11, Move::LeechLife),
            (17, Move::FuryCutter),
            (22, Move::Spore),
            (27, Move::Slash),
            (33, Move::Growth),
            (38, Move::GigaDrain),
            (43, Move::Aromatherapy),
            (49, Move::RagePowder),
            (54, Move::XScissor),
        ]
    };
    pub const Parasect: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Grass),
        ability: PokemonAbility::Two(Ability::EffectSpore, Ability::DrySkin),
        hidden_ability: Some(Ability::Damp),
        height: 10,
        weight: 295,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(95, 2),
            Stat::new(80, 1),
            Stat::new(60, 0),
            Stat::new(80, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::PoisonPowder),
            (1, Move::StunSpore),
            (1, Move::LeechLife),
            (1, Move::CrossPoison),
            (17, Move::FuryCutter),
            (22, Move::Spore),
            (29, Move::Slash),
            (37, Move::Growth),
            (44, Move::GigaDrain),
            (51, Move::Aromatherapy),
            (59, Move::RagePowder),
            (66, Move::XScissor),
        ]
    };
    pub const Venonat: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::CompoundEyes, Ability::TintedLens),
        hidden_ability: Some(Ability::RunAway),
        height: 10,
        weight: 300,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(55, 1),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Disable),
            (1, Move::Foresight),
            (5, Move::Supersonic),
            (11, Move::Confusion),
            (13, Move::PoisonPowder),
            (17, Move::LeechLife),
            (23, Move::StunSpore),
            (25, Move::Psybeam),
            (29, Move::SleepPowder),
            (35, Move::SignalBeam),
            (37, Move::ZenHeadbutt),
            (41, Move::PoisonFang),
            (47, Move::Psychic),
        ]
    };
    pub const Venomoth: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::ShieldDust, Ability::TintedLens),
        hidden_ability: Some(Ability::WonderSkin),
        height: 15,
        weight: 125,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(90, 1),
            Stat::new(75, 0),
            Stat::new(90, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Supersonic),
            (1, Move::Disable),
            (1, Move::Foresight),
            (1, Move::SilverWind),
            (11, Move::Confusion),
            (13, Move::PoisonPowder),
            (17, Move::LeechLife),
            (23, Move::StunSpore),
            (25, Move::Psybeam),
            (29, Move::SleepPowder),
            (31, Move::Gust),
            (37, Move::SignalBeam),
            (41, Move::ZenHeadbutt),
            (47, Move::PoisonFang),
            (55, Move::Psychic),
            (59, Move::BugBuzz),
            (63, Move::QuiverDance),
        ]
    };
    pub const Diglett: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::Two(Ability::SandVeil, Ability::ArenaTrap),
        hidden_ability: Some(Ability::SandForce),
        height: 2,
        weight: 8,
        base_exp_yield: 53,
        stats: Stats(
            Stat::new(10, 0),
            Stat::new(55, 0),
            Stat::new(25, 0),
            Stat::new(35, 0),
            Stat::new(45, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::SandAttack),
            (4, Move::Growl),
            (7, Move::Astonish),
            (12, Move::MudSlap),
            (15, Move::Magnitude),
            (18, Move::Bulldoze),
            (23, Move::SuckerPunch),
            (26, Move::MudBomb),
            (29, Move::EarthPower),
            (34, Move::Dig),
            (37, Move::Slash),
            (40, Move::Earthquake),
            (45, Move::Fissure),
        ]
    };
    pub const Dugtrio: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::Two(Ability::SandVeil, Ability::ArenaTrap),
        hidden_ability: Some(Ability::SandForce),
        height: 7,
        weight: 333,
        base_exp_yield: 149,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(100, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(70, 0),
            Stat::new(120, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::SandAttack),
            (1, Move::Growl),
            (1, Move::TriAttack),
            (1, Move::NightSlash),
            (7, Move::Astonish),
            (12, Move::MudSlap),
            (15, Move::Magnitude),
            (18, Move::Bulldoze),
            (23, Move::SuckerPunch),
            (26, Move::SandTomb),
            (28, Move::MudBomb),
            (33, Move::EarthPower),
            (40, Move::Dig),
            (45, Move::Slash),
            (50, Move::Earthquake),
            (57, Move::Fissure),
        ]
    };
    pub const Meowth: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Pickup, Ability::Technician),
        hidden_ability: Some(Ability::Unnerve),
        height: 4,
        weight: 42,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(90, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (6, Move::Bite),
            (9, Move::FakeOut),
            (14, Move::FurySwipes),
            (17, Move::Screech),
            (22, Move::FeintAttack),
            (25, Move::Taunt),
            (30, Move::PayDay),
            (33, Move::Slash),
            (38, Move::NastyPlot),
            (41, Move::Assurance),
            (46, Move::Captivate),
            (49, Move::NightSlash),
            (54, Move::Feint),
        ]
    };
    pub const Persian: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Limber, Ability::Technician),
        hidden_ability: Some(Ability::Unnerve),
        height: 10,
        weight: 320,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(115, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Bite),
            (1, Move::Growl),
            (1, Move::FakeOut),
            (1, Move::Switcheroo),
            (14, Move::FurySwipes),
            (17, Move::Screech),
            (22, Move::FeintAttack),
            (25, Move::Taunt),
            (28, Move::Swift),
            (32, Move::PowerGem),
            (37, Move::Slash),
            (44, Move::NastyPlot),
            (49, Move::Assurance),
            (56, Move::Captivate),
            (61, Move::NightSlash),
            (68, Move::Feint),
        ]
    };
    pub const Psyduck: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::Damp, Ability::CloudNine),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 8,
        weight: 196,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(52, 0),
            Stat::new(48, 0),
            Stat::new(65, 1),
            Stat::new(50, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::WaterSport),
            (4, Move::TailWhip),
            (8, Move::WaterGun),
            (11, Move::Disable),
            (15, Move::Confusion),
            (18, Move::WaterPulse),
            (22, Move::FurySwipes),
            (25, Move::Screech),
            (29, Move::ZenHeadbutt),
            (32, Move::AquaTail),
            (36, Move::Soak),
            (39, Move::PsychUp),
            (43, Move::Amnesia),
            (46, Move::HydroPump),
            (50, Move::WonderRoom),
        ]
    };
    pub const Golduck: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::Damp, Ability::CloudNine),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 17,
        weight: 766,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(82, 0),
            Stat::new(78, 0),
            Stat::new(95, 2),
            Stat::new(80, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::TailWhip),
            (1, Move::WaterGun),
            (1, Move::WaterSport),
            (1, Move::AquaJet),
            (11, Move::Disable),
            (15, Move::Confusion),
            (18, Move::WaterPulse),
            (22, Move::FurySwipes),
            (25, Move::Screech),
            (29, Move::ZenHeadbutt),
            (32, Move::AquaTail),
            (38, Move::Soak),
            (43, Move::PsychUp),
            (49, Move::Amnesia),
            (54, Move::HydroPump),
            (60, Move::WonderRoom),
        ]
    };
    pub const Mankey: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::VitalSpirit, Ability::AngerPoint),
        hidden_ability: Some(Ability::Defiant),
        height: 5,
        weight: 280,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(80, 1),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(45, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::LowKick),
            (1, Move::FocusEnergy),
            (1, Move::Covet),
            (9, Move::FurySwipes),
            (13, Move::KarateChop),
            (17, Move::SeismicToss),
            (21, Move::Screech),
            (25, Move::Assurance),
            (33, Move::Swagger),
            (37, Move::CrossChop),
            (41, Move::Thrash),
            (45, Move::Punishment),
            (49, Move::CloseCombat),
            (53, Move::FinalGambit),
        ]
    };
    pub const Primeape: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::VitalSpirit, Ability::AngerPoint),
        hidden_ability: Some(Ability::Defiant),
        height: 10,
        weight: 320,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(105, 2),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::LowKick),
            (1, Move::FocusEnergy),
            (1, Move::Fling),
            (9, Move::FurySwipes),
            (13, Move::KarateChop),
            (17, Move::SeismicToss),
            (21, Move::Screech),
            (25, Move::Assurance),
            (28, Move::Rage),
            (35, Move::Swagger),
            (41, Move::CrossChop),
            (47, Move::Thrash),
            (53, Move::Punishment),
            (59, Move::CloseCombat),
            (63, Move::FinalGambit),
        ]
    };
    pub const Growlithe: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::FlashFire),
        hidden_ability: Some(Ability::Justified),
        height: 7,
        weight: 190,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(70, 1),
            Stat::new(45, 0),
            Stat::new(70, 0),
            Stat::new(50, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bite),
            (1, Move::Roar),
            (6, Move::Ember),
            (8, Move::Leer),
            (10, Move::OdorSleuth),
            (12, Move::HelpingHand),
            (17, Move::FlameWheel),
            (19, Move::Reversal),
            (21, Move::FireFang),
            (23, Move::TakeDown),
            (28, Move::FlameBurst),
            (30, Move::Agility),
            (32, Move::Retaliate),
            (34, Move::Flamethrower),
            (39, Move::Crunch),
            (41, Move::HeatWave),
            (43, Move::Outrage),
            (45, Move::FlareBlitz),
        ]
    };
    pub const Arcanine: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::FlashFire),
        hidden_ability: Some(Ability::Justified),
        height: 19,
        weight: 1550,
        base_exp_yield: 194,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(110, 2),
            Stat::new(80, 0),
            Stat::new(100, 0),
            Stat::new(80, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bite),
            (1, Move::Roar),
            (1, Move::OdorSleuth),
            (1, Move::ThunderFang),
            (1, Move::FireFang),
            (34, Move::ExtremeSpeed),
        ]
    };
    pub const Poliwag: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::Damp),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 6,
        weight: 124,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(90, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterSport),
            (5, Move::Bubble),
            (8, Move::Hypnosis),
            (11, Move::WaterGun),
            (15, Move::DoubleSlap),
            (18, Move::RainDance),
            (21, Move::BodySlam),
            (25, Move::BubbleBeam),
            (28, Move::MudShot),
            (31, Move::BellyDrum),
            (35, Move::WakeUpSlap),
            (38, Move::HydroPump),
            (41, Move::MudBomb),
        ]
    };
    pub const Poliwhirl: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::Damp),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 10,
        weight: 200,
        base_exp_yield: 135,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(90, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Hypnosis),
            (1, Move::Bubble),
            (1, Move::WaterSport),
            (11, Move::WaterGun),
            (15, Move::DoubleSlap),
            (18, Move::RainDance),
            (21, Move::BodySlam),
            (27, Move::BubbleBeam),
            (32, Move::MudShot),
            (37, Move::BellyDrum),
            (43, Move::WakeUpSlap),
            (48, Move::HydroPump),
            (53, Move::MudBomb),
        ]
    };
    pub const Poliwrath: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Fighting),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::Damp),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 13,
        weight: 540,
        base_exp_yield: 230,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(95, 0),
            Stat::new(95, 3),
            Stat::new(70, 0),
            Stat::new(90, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleSlap),
            (1, Move::BubbleBeam),
            (1, Move::Submission),
            (1, Move::Hypnosis),
            (32, Move::DynamicPunch),
            (43, Move::MindReader),
            (53, Move::CircleThrow),
        ]
    };
    pub const Abra: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::InnerFocus),
        hidden_ability: Some(Ability::MagicGuard),
        height: 9,
        weight: 195,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(25, 0),
            Stat::new(20, 0),
            Stat::new(15, 0),
            Stat::new(105, 1),
            Stat::new(55, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Teleport),
        ]
    };
    pub const Kadabra: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::InnerFocus),
        hidden_ability: Some(Ability::MagicGuard),
        height: 13,
        weight: 565,
        base_exp_yield: 140,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(30, 0),
            Stat::new(120, 2),
            Stat::new(70, 0),
            Stat::new(105, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Teleport),
            (1, Move::Kinesis),
            (18, Move::Disable),
            (22, Move::MiracleEye),
            (24, Move::AllySwitch),
            (28, Move::Psybeam),
            (30, Move::Reflect),
            (34, Move::Telekinesis),
            (36, Move::Recover),
            (40, Move::PsychoCut),
            (42, Move::RolePlay),
            (46, Move::Psychic),
            (48, Move::FutureSight),
            (52, Move::Trick),
        ]
    };
    pub const Alakazam: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::InnerFocus),
        hidden_ability: Some(Ability::MagicGuard),
        height: 15,
        weight: 480,
        base_exp_yield: 225,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(135, 3),
            Stat::new(95, 0),
            Stat::new(120, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Teleport),
            (1, Move::Kinesis),
            (18, Move::Disable),
            (22, Move::MiracleEye),
            (24, Move::AllySwitch),
            (28, Move::Psybeam),
            (30, Move::Reflect),
            (34, Move::Telekinesis),
            (36, Move::Recover),
            (40, Move::PsychoCut),
            (42, Move::CalmMind),
            (46, Move::Psychic),
            (48, Move::FutureSight),
            (52, Move::Trick),
        ]
    };
    pub const Machop: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::NoGuard),
        hidden_ability: Some(Ability::Steadfast),
        height: 8,
        weight: 195,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(80, 1),
            Stat::new(50, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::LowKick),
            (7, Move::FocusEnergy),
            (10, Move::KarateChop),
            (13, Move::LowSweep),
            (19, Move::Foresight),
            (22, Move::SeismicToss),
            (25, Move::Revenge),
            (31, Move::VitalThrow),
            (34, Move::Submission),
            (37, Move::WakeUpSlap),
            (43, Move::CrossChop),
            (46, Move::ScaryFace),
            (49, Move::DynamicPunch),
        ]
    };
    pub const Machoke: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::NoGuard),
        hidden_ability: Some(Ability::Steadfast),
        height: 15,
        weight: 705,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(100, 2),
            Stat::new(70, 0),
            Stat::new(50, 0),
            Stat::new(60, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::KarateChop),
            (1, Move::Leer),
            (1, Move::LowKick),
            (1, Move::FocusEnergy),
            (13, Move::LowSweep),
            (19, Move::Foresight),
            (22, Move::SeismicToss),
            (25, Move::Revenge),
            (32, Move::VitalThrow),
            (36, Move::Submission),
            (40, Move::WakeUpSlap),
            (44, Move::CrossChop),
            (51, Move::ScaryFace),
            (55, Move::DynamicPunch),
        ]
    };
    pub const Machamp: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::NoGuard),
        hidden_ability: Some(Ability::Steadfast),
        height: 16,
        weight: 1300,
        base_exp_yield: 227,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(130, 3),
            Stat::new(80, 0),
            Stat::new(65, 0),
            Stat::new(85, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::KarateChop),
            (1, Move::Leer),
            (1, Move::LowKick),
            (1, Move::FocusEnergy),
            (1, Move::WideGuard),
            (13, Move::LowSweep),
            (19, Move::Foresight),
            (22, Move::SeismicToss),
            (25, Move::Revenge),
            (32, Move::VitalThrow),
            (36, Move::Submission),
            (40, Move::WakeUpSlap),
            (44, Move::CrossChop),
            (51, Move::ScaryFace),
            (55, Move::DynamicPunch),
        ]
    };
    pub const Bellsprout: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Gluttony),
        height: 7,
        weight: 40,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(75, 1),
            Stat::new(35, 0),
            Stat::new(70, 0),
            Stat::new(30, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::VineWhip),
            (7, Move::Growth),
            (11, Move::Wrap),
            (13, Move::SleepPowder),
            (15, Move::PoisonPowder),
            (17, Move::StunSpore),
            (23, Move::Acid),
            (27, Move::KnockOff),
            (29, Move::SweetScent),
            (35, Move::GastroAcid),
            (39, Move::RazorLeaf),
            (41, Move::Slam),
            (47, Move::WringOut),
        ]
    };
    pub const Weepinbell: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Gluttony),
        height: 10,
        weight: 64,
        base_exp_yield: 137,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(90, 2),
            Stat::new(50, 0),
            Stat::new(85, 0),
            Stat::new(45, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::VineWhip),
            (1, Move::Wrap),
            (1, Move::Growth),
            (13, Move::SleepPowder),
            (15, Move::PoisonPowder),
            (17, Move::StunSpore),
            (23, Move::Acid),
            (27, Move::KnockOff),
            (29, Move::SweetScent),
            (35, Move::GastroAcid),
            (39, Move::RazorLeaf),
            (41, Move::Slam),
            (47, Move::WringOut),
        ]
    };
    pub const Victreebel: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Gluttony),
        height: 17,
        weight: 155,
        base_exp_yield: 221,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(105, 3),
            Stat::new(65, 0),
            Stat::new(100, 0),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::VineWhip),
            (1, Move::RazorLeaf),
            (1, Move::SleepPowder),
            (1, Move::SweetScent),
            (1, Move::Stockpile),
            (1, Move::SpitUp),
            (1, Move::Swallow),
            (27, Move::LeafTornado),
            (47, Move::LeafBlade),
            (47, Move::LeafStorm),
        ]
    };
    pub const Tentacool: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Poison),
        ability: PokemonAbility::Two(Ability::ClearBody, Ability::LiquidOoze),
        hidden_ability: Some(Ability::RainDish),
        height: 9,
        weight: 455,
        base_exp_yield: 67,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(50, 0),
            Stat::new(100, 1),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (5, Move::Supersonic),
            (8, Move::Constrict),
            (12, Move::Acid),
            (15, Move::ToxicSpikes),
            (19, Move::BubbleBeam),
            (22, Move::Wrap),
            (26, Move::AcidSpray),
            (29, Move::Barrier),
            (33, Move::WaterPulse),
            (36, Move::PoisonJab),
            (40, Move::Screech),
            (43, Move::Hex),
            (47, Move::HydroPump),
            (50, Move::SludgeWave),
            (54, Move::WringOut),
        ]
    };
    pub const Tentacruel: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Poison),
        ability: PokemonAbility::Two(Ability::ClearBody, Ability::LiquidOoze),
        hidden_ability: Some(Ability::RainDish),
        height: 16,
        weight: 550,
        base_exp_yield: 180,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(80, 0),
            Stat::new(120, 2),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Supersonic),
            (1, Move::Constrict),
            (12, Move::Acid),
            (15, Move::ToxicSpikes),
            (19, Move::BubbleBeam),
            (22, Move::Wrap),
            (26, Move::AcidSpray),
            (29, Move::Barrier),
            (34, Move::WaterPulse),
            (38, Move::PoisonJab),
            (43, Move::Screech),
            (47, Move::Hex),
            (52, Move::HydroPump),
            (56, Move::SludgeWave),
            (61, Move::WringOut),
        ]
    };
    pub const Geodude: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::Sturdy),
        hidden_ability: Some(Ability::SandVeil),
        height: 4,
        weight: 200,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(80, 0),
            Stat::new(100, 1),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::DefenseCurl),
            (4, Move::MudSport),
            (8, Move::RockPolish),
            (11, Move::RockThrow),
            (15, Move::Magnitude),
            (18, Move::Rollout),
            (22, Move::RockBlast),
            (25, Move::SmackDown),
            (29, Move::SelfDestruct),
            (32, Move::Bulldoze),
            (36, Move::StealthRock),
            (39, Move::Earthquake),
            (43, Move::Explosion),
            (46, Move::DoubleEdge),
            (50, Move::StoneEdge),
        ]
    };
    pub const Graveler: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::Sturdy),
        hidden_ability: Some(Ability::SandVeil),
        height: 10,
        weight: 1050,
        base_exp_yield: 137,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(95, 0),
            Stat::new(115, 2),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::DefenseCurl),
            (1, Move::MudSport),
            (1, Move::RockPolish),
            (11, Move::RockThrow),
            (15, Move::Magnitude),
            (18, Move::Rollout),
            (22, Move::RockBlast),
            (27, Move::SmackDown),
            (31, Move::SelfDestruct),
            (36, Move::Bulldoze),
            (42, Move::StealthRock),
            (47, Move::Earthquake),
            (53, Move::Explosion),
            (58, Move::DoubleEdge),
            (64, Move::StoneEdge),
        ]
    };
    pub const Golem: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::Sturdy),
        hidden_ability: Some(Ability::SandVeil),
        height: 14,
        weight: 3000,
        base_exp_yield: 223,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(120, 0),
            Stat::new(130, 3),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::DefenseCurl),
            (1, Move::MudSport),
            (1, Move::RockPolish),
            (11, Move::RockThrow),
            (15, Move::Magnitude),
            (18, Move::Steamroller),
            (22, Move::RockBlast),
            (27, Move::SmackDown),
            (31, Move::SelfDestruct),
            (36, Move::Bulldoze),
            (42, Move::StealthRock),
            (47, Move::Earthquake),
            (53, Move::Explosion),
            (58, Move::DoubleEdge),
            (64, Move::StoneEdge),
            (69, Move::HeavySlam),
        ]
    };
    pub const Ponyta: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::FlashFire),
        hidden_ability: Some(Ability::FlameBody),
        height: 10,
        weight: 300,
        base_exp_yield: 82,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(85, 0),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(90, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (4, Move::TailWhip),
            (9, Move::Ember),
            (13, Move::FlameWheel),
            (17, Move::Stomp),
            (21, Move::FlameCharge),
            (25, Move::FireSpin),
            (29, Move::TakeDown),
            (33, Move::Inferno),
            (37, Move::Agility),
            (41, Move::FireBlast),
            (45, Move::Bounce),
            (49, Move::FlareBlitz),
        ]
    };
    pub const Rapidash: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::FlashFire),
        hidden_ability: Some(Ability::FlameBody),
        height: 17,
        weight: 950,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(100, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(105, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::Growl),
            (1, Move::Ember),
            (1, Move::QuickAttack),
            (1, Move::Megahorn),
            (1, Move::PoisonJab),
            (13, Move::FlameWheel),
            (17, Move::Stomp),
            (21, Move::FlameCharge),
            (25, Move::FireSpin),
            (29, Move::TakeDown),
            (33, Move::Inferno),
            (37, Move::Agility),
            (40, Move::FuryAttack),
            (41, Move::FireBlast),
            (45, Move::Bounce),
            (49, Move::FlareBlitz),
        ]
    };
    pub const Slowpoke: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::OwnTempo),
        hidden_ability: Some(Ability::Regenerator),
        height: 12,
        weight: 360,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(90, 1),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Curse),
            (1, Move::Yawn),
            (5, Move::Growl),
            (9, Move::WaterGun),
            (14, Move::Confusion),
            (19, Move::Disable),
            (23, Move::Headbutt),
            (28, Move::WaterPulse),
            (32, Move::ZenHeadbutt),
            (36, Move::SlackOff),
            (41, Move::Amnesia),
            (45, Move::Psychic),
            (49, Move::RainDance),
            (54, Move::PsychUp),
            (58, Move::HealPulse),
        ]
    };
    pub const Slowbro: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::OwnTempo),
        hidden_ability: Some(Ability::Regenerator),
        height: 16,
        weight: 785,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(75, 0),
            Stat::new(110, 2),
            Stat::new(100, 0),
            Stat::new(80, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Curse),
            (1, Move::Yawn),
            (9, Move::WaterGun),
            (14, Move::Confusion),
            (19, Move::Disable),
            (23, Move::Headbutt),
            (28, Move::WaterPulse),
            (32, Move::ZenHeadbutt),
            (36, Move::SlackOff),
            (37, Move::Withdraw),
            (43, Move::Amnesia),
            (49, Move::Psychic),
            (55, Move::RainDance),
            (62, Move::PsychUp),
            (68, Move::HealPulse),
        ]
    };
    pub const Magnemite: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Steel),
        ability: PokemonAbility::Two(Ability::MagnetPull, Ability::Sturdy),
        hidden_ability: Some(Ability::Analytic),
        height: 3,
        weight: 60,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(25, 0),
            Stat::new(35, 0),
            Stat::new(70, 0),
            Stat::new(95, 1),
            Stat::new(55, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Supersonic),
            (7, Move::ThunderShock),
            (11, Move::SonicBoom),
            (15, Move::ThunderWave),
            (18, Move::MagnetBomb),
            (21, Move::Spark),
            (25, Move::MirrorShot),
            (29, Move::MetalSound),
            (32, Move::ElectroBall),
            (35, Move::FlashCannon),
            (39, Move::Screech),
            (43, Move::Discharge),
            (46, Move::LockOn),
            (49, Move::MagnetRise),
            (53, Move::GyroBall),
            (57, Move::ZapCannon),
        ]
    };
    pub const Magneton: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Steel),
        ability: PokemonAbility::Two(Ability::MagnetPull, Ability::Sturdy),
        hidden_ability: Some(Ability::Analytic),
        height: 10,
        weight: 600,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(60, 0),
            Stat::new(95, 0),
            Stat::new(120, 2),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Supersonic),
            (1, Move::SonicBoom),
            (1, Move::ThunderShock),
            (1, Move::TriAttack),
            (15, Move::ThunderWave),
            (18, Move::MagnetBomb),
            (21, Move::Spark),
            (25, Move::MirrorShot),
            (29, Move::MetalSound),
            (34, Move::ElectroBall),
            (39, Move::FlashCannon),
            (45, Move::Screech),
            (51, Move::Discharge),
            (56, Move::LockOn),
            (62, Move::MagnetRise),
            (67, Move::GyroBall),
            (73, Move::ZapCannon),
        ]
    };
    pub const Farfetchd: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::InnerFocus),
        hidden_ability: Some(Ability::Defiant),
        height: 8,
        weight: 150,
        base_exp_yield: 132,
        stats: Stats(
            Stat::new(52, 0),
            Stat::new(90, 1),
            Stat::new(55, 0),
            Stat::new(58, 0),
            Stat::new(62, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Leer),
            (1, Move::Peck),
            (1, Move::FuryCutter),
            (1, Move::PoisonJab),
            (7, Move::FuryAttack),
            (9, Move::KnockOff),
            (13, Move::AerialAce),
            (19, Move::Slash),
            (21, Move::AirCutter),
            (25, Move::SwordsDance),
            (31, Move::Agility),
            (33, Move::NightSlash),
            (37, Move::Acrobatics),
            (43, Move::Feint),
            (45, Move::FalseSwipe),
            (49, Move::AirSlash),
            (55, Move::BraveBird),
        ]
    };
    pub const Doduo: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::EarlyBird),
        hidden_ability: Some(Ability::TangledFeet),
        height: 14,
        weight: 392,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(85, 1),
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Peck),
            (5, Move::QuickAttack),
            (10, Move::Rage),
            (14, Move::FuryAttack),
            (19, Move::Pursuit),
            (23, Move::Uproar),
            (28, Move::Acupressure),
            (32, Move::DoubleHit),
            (37, Move::Agility),
            (41, Move::DrillPeck),
            (46, Move::Endeavor),
            (50, Move::Thrash),
        ]
    };
    pub const Dodrio: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::EarlyBird),
        hidden_ability: Some(Ability::TangledFeet),
        height: 18,
        weight: 852,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(110, 2),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Peck),
            (1, Move::QuickAttack),
            (1, Move::Rage),
            (1, Move::Pluck),
            (14, Move::FuryAttack),
            (19, Move::Pursuit),
            (23, Move::Uproar),
            (28, Move::Acupressure),
            (34, Move::TriAttack),
            (41, Move::Agility),
            (47, Move::DrillPeck),
            (54, Move::Endeavor),
            (60, Move::Thrash),
        ]
    };
    pub const Seel: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::Hydration),
        hidden_ability: Some(Ability::IceBody),
        height: 11,
        weight: 900,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(70, 1),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (3, Move::Growl),
            (7, Move::WaterSport),
            (11, Move::IcyWind),
            (13, Move::Encore),
            (17, Move::IceShard),
            (21, Move::Rest),
            (23, Move::AquaRing),
            (27, Move::AuroraBeam),
            (31, Move::AquaJet),
            (33, Move::Brine),
            (37, Move::TakeDown),
            (41, Move::Dive),
            (43, Move::AquaTail),
            (47, Move::IceBeam),
            (51, Move::Safeguard),
            (53, Move::Hail),
        ]
    };
    pub const Dewgong: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ice),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::Hydration),
        hidden_ability: Some(Ability::IceBody),
        height: 17,
        weight: 1200,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(95, 2),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Growl),
            (1, Move::IcyWind),
            (1, Move::SignalBeam),
            (13, Move::Encore),
            (17, Move::IceShard),
            (21, Move::Rest),
            (23, Move::AquaRing),
            (27, Move::AuroraBeam),
            (31, Move::AquaJet),
            (33, Move::Brine),
            (34, Move::SheerCold),
            (39, Move::TakeDown),
            (45, Move::Dive),
            (49, Move::AquaTail),
            (55, Move::IceBeam),
            (61, Move::Safeguard),
            (65, Move::Hail),
        ]
    };
    pub const Grimer: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::Stench, Ability::StickyHold),
        hidden_ability: Some(Ability::PoisonTouch),
        height: 9,
        weight: 300,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(80, 1),
            Stat::new(80, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::PoisonGas),
            (4, Move::Harden),
            (7, Move::MudSlap),
            (12, Move::Disable),
            (15, Move::Sludge),
            (18, Move::Minimize),
            (21, Move::MudBomb),
            (26, Move::SludgeBomb),
            (29, Move::Fling),
            (32, Move::Screech),
            (37, Move::SludgeWave),
            (40, Move::AcidArmor),
            (43, Move::GunkShot),
            (48, Move::Memento),
        ]
    };
    pub const Muk: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::Stench, Ability::StickyHold),
        hidden_ability: Some(Ability::PoisonTouch),
        height: 12,
        weight: 300,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(105, 1),
            Stat::new(105, 1),
            Stat::new(75, 0),
            Stat::new(65, 0),
            Stat::new(100, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Harden),
            (1, Move::PoisonGas),
            (1, Move::MudSlap),
            (12, Move::Disable),
            (15, Move::Sludge),
            (18, Move::Minimize),
            (21, Move::MudBomb),
            (26, Move::SludgeBomb),
            (29, Move::Fling),
            (32, Move::Screech),
            (37, Move::SludgeWave),
            (43, Move::AcidArmor),
            (49, Move::GunkShot),
            (57, Move::Memento),
        ]
    };
    pub const Shellder: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::ShellArmor, Ability::SkillLink),
        hidden_ability: Some(Ability::Overcoat),
        height: 3,
        weight: 40,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(65, 0),
            Stat::new(100, 1),
            Stat::new(45, 0),
            Stat::new(25, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Withdraw),
            (8, Move::Supersonic),
            (13, Move::IcicleSpear),
            (16, Move::Protect),
            (20, Move::Leer),
            (25, Move::Clamp),
            (28, Move::IceShard),
            (32, Move::RazorShell),
            (37, Move::AuroraBeam),
            (40, Move::Whirlpool),
            (44, Move::Brine),
            (49, Move::IronDefense),
            (52, Move::IceBeam),
            (56, Move::ShellSmash),
            (61, Move::HydroPump),
        ]
    };
    pub const Cloyster: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ice),
        ability: PokemonAbility::Two(Ability::ShellArmor, Ability::SkillLink),
        hidden_ability: Some(Ability::Overcoat),
        height: 15,
        weight: 1325,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(95, 0),
            Stat::new(180, 2),
            Stat::new(85, 0),
            Stat::new(45, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Supersonic),
            (1, Move::AuroraBeam),
            (1, Move::Withdraw),
            (1, Move::Protect),
            (1, Move::ToxicSpikes),
            (13, Move::SpikeCannon),
            (28, Move::Spikes),
            (52, Move::IcicleCrash),
        ]
    };
    pub const Gastly: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Poison),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 13,
        weight: 1,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(35, 0),
            Stat::new(30, 0),
            Stat::new(100, 1),
            Stat::new(35, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Hypnosis),
            (1, Move::Lick),
            (5, Move::Spite),
            (8, Move::MeanLook),
            (12, Move::Curse),
            (15, Move::NightShade),
            (19, Move::ConfuseRay),
            (22, Move::SuckerPunch),
            (26, Move::Payback),
            (29, Move::ShadowBall),
            (33, Move::DreamEater),
            (36, Move::DarkPulse),
            (40, Move::DestinyBond),
            (43, Move::Hex),
            (47, Move::Nightmare),
        ]
    };
    pub const Haunter: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Poison),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 16,
        weight: 1,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(115, 2),
            Stat::new(55, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Hypnosis),
            (1, Move::Lick),
            (1, Move::Spite),
            (8, Move::MeanLook),
            (12, Move::Curse),
            (15, Move::NightShade),
            (19, Move::ConfuseRay),
            (22, Move::SuckerPunch),
            (25, Move::ShadowPunch),
            (28, Move::Payback),
            (33, Move::ShadowBall),
            (39, Move::DreamEater),
            (44, Move::DarkPulse),
            (50, Move::DestinyBond),
            (55, Move::Hex),
            (61, Move::Nightmare),
        ]
    };
    pub const Gengar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Poison),
        ability: PokemonAbility::One(Ability::CursedBody),
        hidden_ability: None,
        height: 15,
        weight: 405,
        base_exp_yield: 225,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(130, 3),
            Stat::new(75, 0),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Hypnosis),
            (1, Move::Lick),
            (1, Move::Spite),
            (8, Move::MeanLook),
            (12, Move::Curse),
            (15, Move::NightShade),
            (19, Move::ConfuseRay),
            (22, Move::SuckerPunch),
            (25, Move::ShadowPunch),
            (28, Move::Payback),
            (33, Move::ShadowBall),
            (39, Move::DreamEater),
            (44, Move::DarkPulse),
            (50, Move::DestinyBond),
            (55, Move::Hex),
            (61, Move::Nightmare),
        ]
    };
    pub const Onix: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::Sturdy),
        hidden_ability: Some(Ability::WeakArmor),
        height: 88,
        weight: 2100,
        base_exp_yield: 77,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(45, 0),
            Stat::new(160, 1),
            Stat::new(30, 0),
            Stat::new(45, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bind),
            (1, Move::Tackle),
            (1, Move::Harden),
            (1, Move::MudSport),
            (4, Move::Curse),
            (7, Move::RockThrow),
            (10, Move::Rage),
            (13, Move::RockTomb),
            (16, Move::StealthRock),
            (19, Move::RockPolish),
            (22, Move::SmackDown),
            (25, Move::DragonBreath),
            (28, Move::Slam),
            (31, Move::Screech),
            (34, Move::RockSlide),
            (37, Move::SandTomb),
            (40, Move::IronTail),
            (43, Move::Dig),
            (46, Move::StoneEdge),
            (49, Move::DoubleEdge),
            (52, Move::Sandstorm),
        ]
    };
    pub const Drowzee: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::Forewarn),
        hidden_ability: Some(Ability::InnerFocus),
        height: 10,
        weight: 324,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(48, 0),
            Stat::new(45, 0),
            Stat::new(43, 0),
            Stat::new(90, 1),
            Stat::new(42, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Hypnosis),
            (5, Move::Disable),
            (9, Move::Confusion),
            (13, Move::Headbutt),
            (17, Move::PoisonGas),
            (21, Move::Meditate),
            (25, Move::Psybeam),
            (33, Move::PsychUp),
            (37, Move::Synchronoise),
            (41, Move::ZenHeadbutt),
            (45, Move::Swagger),
            (49, Move::Psychic),
            (53, Move::NastyPlot),
            (57, Move::Psyshock),
            (61, Move::FutureSight),
        ]
    };
    pub const Hypno: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::Forewarn),
        hidden_ability: Some(Ability::InnerFocus),
        height: 16,
        weight: 756,
        base_exp_yield: 169,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(73, 0),
            Stat::new(70, 0),
            Stat::new(73, 0),
            Stat::new(115, 2),
            Stat::new(67, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Disable),
            (1, Move::Confusion),
            (1, Move::Hypnosis),
            (1, Move::Nightmare),
            (1, Move::Switcheroo),
            (13, Move::Headbutt),
            (17, Move::PoisonGas),
            (21, Move::Meditate),
            (25, Move::Psybeam),
            (33, Move::PsychUp),
            (37, Move::Synchronoise),
            (41, Move::ZenHeadbutt),
            (45, Move::Swagger),
            (49, Move::Psychic),
            (53, Move::NastyPlot),
            (57, Move::Psyshock),
            (61, Move::FutureSight),
        ]
    };
    pub const Krabby: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::ShellArmor),
        hidden_ability: Some(Ability::SheerForce),
        height: 4,
        weight: 65,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(105, 1),
            Stat::new(90, 0),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (1, Move::MudSport),
            (5, Move::ViseGrip),
            (9, Move::Leer),
            (11, Move::Harden),
            (15, Move::BubbleBeam),
            (19, Move::MudShot),
            (21, Move::MetalClaw),
            (25, Move::Stomp),
            (29, Move::Protect),
            (31, Move::Guillotine),
            (35, Move::Slam),
            (39, Move::Brine),
            (41, Move::Crabhammer),
            (45, Move::Flail),
        ]
    };
    pub const Kingler: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::ShellArmor),
        hidden_ability: Some(Ability::SheerForce),
        height: 13,
        weight: 600,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(130, 2),
            Stat::new(115, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (1, Move::Leer),
            (1, Move::Bubble),
            (1, Move::MudSport),
            (1, Move::WideGuard),
            (11, Move::Harden),
            (15, Move::BubbleBeam),
            (19, Move::MudShot),
            (21, Move::MetalClaw),
            (25, Move::Stomp),
            (32, Move::Protect),
            (37, Move::Guillotine),
            (44, Move::Slam),
            (51, Move::Brine),
            (56, Move::Crabhammer),
            (63, Move::Flail),
        ]
    };
    pub const Voltorb: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Soundproof, Ability::Static),
        hidden_ability: Some(Ability::Aftermath),
        height: 5,
        weight: 104,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(100, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Charge),
            (5, Move::Tackle),
            (8, Move::SonicBoom),
            (12, Move::Spark),
            (15, Move::Rollout),
            (19, Move::Screech),
            (22, Move::ChargeBeam),
            (26, Move::LightScreen),
            (29, Move::ElectroBall),
            (33, Move::SelfDestruct),
            (36, Move::Swift),
            (40, Move::MagnetRise),
            (43, Move::GyroBall),
            (47, Move::Explosion),
            (50, Move::MirrorCoat),
        ]
    };
    pub const Electrode: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Soundproof, Ability::Static),
        hidden_ability: Some(Ability::Aftermath),
        height: 12,
        weight: 666,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(150, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::SonicBoom),
            (1, Move::Spark),
            (1, Move::Charge),
            (15, Move::Rollout),
            (19, Move::Screech),
            (22, Move::ChargeBeam),
            (26, Move::LightScreen),
            (29, Move::ElectroBall),
            (35, Move::SelfDestruct),
            (40, Move::Swift),
            (46, Move::MagnetRise),
            (51, Move::GyroBall),
            (57, Move::Explosion),
            (62, Move::MirrorCoat),
        ]
    };
    pub const Exeggcute: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Psychic),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Harvest),
        height: 4,
        weight: 25,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(80, 1),
            Stat::new(60, 0),
            Stat::new(45, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Hypnosis),
            (1, Move::Barrage),
            (1, Move::Uproar),
            (7, Move::Reflect),
            (11, Move::LeechSeed),
            (17, Move::BulletSeed),
            (19, Move::StunSpore),
            (21, Move::PoisonPowder),
            (23, Move::SleepPowder),
            (27, Move::Confusion),
            (33, Move::WorrySeed),
            (37, Move::NaturalGift),
            (43, Move::SolarBeam),
            (47, Move::Extrasensory),
            (53, Move::Bestow),
        ]
    };
    pub const Exeggutor: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Psychic),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Harvest),
        height: 20,
        weight: 1200,
        base_exp_yield: 186,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(95, 0),
            Stat::new(85, 0),
            Stat::new(125, 2),
            Stat::new(75, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Stomp),
            (1, Move::Confusion),
            (1, Move::Hypnosis),
            (1, Move::Barrage),
            (1, Move::SeedBomb),
            (17, Move::Psyshock),
            (27, Move::EggBomb),
            (37, Move::WoodHammer),
            (47, Move::LeafStorm),
        ]
    };
    pub const Cubone: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::LightningRod),
        hidden_ability: Some(Ability::BattleArmor),
        height: 4,
        weight: 65,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(95, 1),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (3, Move::TailWhip),
            (7, Move::BoneClub),
            (11, Move::Headbutt),
            (13, Move::Leer),
            (17, Move::FocusEnergy),
            (21, Move::Bonemerang),
            (23, Move::Rage),
            (27, Move::FalseSwipe),
            (31, Move::Thrash),
            (33, Move::Fling),
            (37, Move::BoneRush),
            (41, Move::Endeavor),
            (43, Move::DoubleEdge),
            (47, Move::Retaliate),
        ]
    };
    pub const Marowak: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::LightningRod),
        hidden_ability: Some(Ability::BattleArmor),
        height: 10,
        weight: 450,
        base_exp_yield: 149,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(80, 0),
            Stat::new(110, 2),
            Stat::new(50, 0),
            Stat::new(80, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::TailWhip),
            (1, Move::Growl),
            (1, Move::BoneClub),
            (13, Move::Leer),
            (17, Move::FocusEnergy),
            (21, Move::Bonemerang),
            (23, Move::Rage),
            (27, Move::FalseSwipe),
            (33, Move::Thrash),
            (37, Move::Fling),
            (43, Move::BoneRush),
            (49, Move::Endeavor),
            (53, Move::DoubleEdge),
            (59, Move::Retaliate),
        ]
    };
    pub const Hitmonlee: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Limber, Ability::Reckless),
        hidden_ability: Some(Ability::Unburden),
        height: 15,
        weight: 498,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(120, 2),
            Stat::new(53, 0),
            Stat::new(35, 0),
            Stat::new(110, 0),
            Stat::new(87, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleKick),
            (1, Move::Revenge),
            (5, Move::Meditate),
            (9, Move::RollingKick),
            (13, Move::JumpKick),
            (17, Move::BrickBreak),
            (21, Move::FocusEnergy),
            (25, Move::Feint),
            (29, Move::HighJumpKick),
            (33, Move::MindReader),
            (37, Move::Foresight),
            (41, Move::WideGuard),
            (45, Move::BlazeKick),
            (49, Move::Endure),
            (53, Move::MegaKick),
            (57, Move::CloseCombat),
            (61, Move::Reversal),
        ]
    };
    pub const Hitmonchan: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::IronFist),
        hidden_ability: Some(Ability::InnerFocus),
        height: 14,
        weight: 502,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(105, 0),
            Stat::new(79, 0),
            Stat::new(35, 0),
            Stat::new(110, 2),
            Stat::new(76, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::CometPunch),
            (1, Move::Revenge),
            (6, Move::Agility),
            (11, Move::Pursuit),
            (16, Move::MachPunch),
            (16, Move::BulletPunch),
            (21, Move::Feint),
            (26, Move::VacuumWave),
            (31, Move::QuickGuard),
            (36, Move::FirePunch),
            (36, Move::IcePunch),
            (36, Move::ThunderPunch),
            (41, Move::SkyUppercut),
            (46, Move::MegaPunch),
            (51, Move::Detect),
            (56, Move::FocusPunch),
            (61, Move::Counter),
            (66, Move::CloseCombat),
        ]
    };
    pub const Lickitung: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::OwnTempo, Ability::Oblivious),
        hidden_ability: Some(Ability::CloudNine),
        height: 12,
        weight: 655,
        base_exp_yield: 77,
        stats: Stats(
            Stat::new(90, 2),
            Stat::new(55, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Lick),
            (5, Move::Supersonic),
            (9, Move::DefenseCurl),
            (13, Move::KnockOff),
            (17, Move::Wrap),
            (21, Move::Stomp),
            (25, Move::Disable),
            (29, Move::Slam),
            (33, Move::Rollout),
            (37, Move::ChipAway),
            (41, Move::MeFirst),
            (45, Move::Refresh),
            (49, Move::Screech),
            (53, Move::PowerWhip),
            (57, Move::WringOut),
        ]
    };
    pub const Koffing: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 6,
        weight: 10,
        base_exp_yield: 68,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(65, 0),
            Stat::new(95, 1),
            Stat::new(60, 0),
            Stat::new(45, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::PoisonGas),
            (4, Move::Smog),
            (7, Move::Smokescreen),
            (12, Move::Assurance),
            (15, Move::ClearSmog),
            (18, Move::Sludge),
            (23, Move::SelfDestruct),
            (26, Move::Haze),
            (29, Move::GyroBall),
            (34, Move::SludgeBomb),
            (37, Move::Explosion),
            (40, Move::DestinyBond),
            (45, Move::Memento),
        ]
    };
    pub const Weezing: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 12,
        weight: 95,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(90, 0),
            Stat::new(120, 2),
            Stat::new(85, 0),
            Stat::new(70, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Smokescreen),
            (1, Move::Smog),
            (1, Move::PoisonGas),
            (12, Move::Assurance),
            (15, Move::ClearSmog),
            (18, Move::Sludge),
            (23, Move::SelfDestruct),
            (26, Move::Haze),
            (29, Move::DoubleHit),
            (34, Move::SludgeBomb),
            (40, Move::Explosion),
            (46, Move::DestinyBond),
            (54, Move::Memento),
        ]
    };
    pub const Rhyhorn: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Rock),
        ability: PokemonAbility::Two(Ability::LightningRod, Ability::RockHead),
        hidden_ability: Some(Ability::Reckless),
        height: 10,
        weight: 1150,
        base_exp_yield: 69,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(85, 0),
            Stat::new(95, 1),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::HornAttack),
            (1, Move::TailWhip),
            (8, Move::Stomp),
            (12, Move::FuryAttack),
            (19, Move::ScaryFace),
            (23, Move::RockBlast),
            (30, Move::Bulldoze),
            (34, Move::ChipAway),
            (41, Move::TakeDown),
            (45, Move::DrillRun),
            (52, Move::StoneEdge),
            (56, Move::Earthquake),
            (63, Move::HornDrill),
            (67, Move::Megahorn),
        ]
    };
    pub const Rhydon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Rock),
        ability: PokemonAbility::Two(Ability::LightningRod, Ability::RockHead),
        hidden_ability: Some(Ability::Reckless),
        height: 19,
        weight: 1200,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(105, 0),
            Stat::new(130, 2),
            Stat::new(120, 0),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Stomp),
            (1, Move::HornAttack),
            (1, Move::FuryAttack),
            (1, Move::TailWhip),
            (19, Move::ScaryFace),
            (23, Move::RockBlast),
            (30, Move::Bulldoze),
            (34, Move::ChipAway),
            (41, Move::TakeDown),
            (42, Move::HammerArm),
            (47, Move::DrillRun),
            (56, Move::StoneEdge),
            (62, Move::Earthquake),
            (71, Move::HornDrill),
            (77, Move::Megahorn),
        ]
    };
    pub const Chansey: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::NaturalCure, Ability::SereneGrace),
        hidden_ability: Some(Ability::Healer),
        height: 11,
        weight: 346,
        base_exp_yield: 395,
        stats: Stats(
            Stat::new(250, 2),
            Stat::new(5, 0),
            Stat::new(5, 0),
            Stat::new(35, 0),
            Stat::new(105, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Growl),
            (1, Move::DefenseCurl),
            (5, Move::TailWhip),
            (9, Move::Refresh),
            (12, Move::DoubleSlap),
            (16, Move::SoftBoiled),
            (20, Move::Bestow),
            (23, Move::Minimize),
            (27, Move::TakeDown),
            (31, Move::Sing),
            (34, Move::Fling),
            (38, Move::HealPulse),
            (42, Move::EggBomb),
            (46, Move::LightScreen),
            (50, Move::HealingWish),
            (54, Move::DoubleEdge),
        ]
    };
    pub const Tangela: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::LeafGuard),
        hidden_ability: Some(Ability::Regenerator),
        height: 10,
        weight: 350,
        base_exp_yield: 87,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(115, 1),
            Stat::new(100, 0),
            Stat::new(40, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Constrict),
            (1, Move::Ingrain),
            (4, Move::SleepPowder),
            (7, Move::VineWhip),
            (10, Move::Absorb),
            (14, Move::PoisonPowder),
            (17, Move::Bind),
            (20, Move::Growth),
            (23, Move::MegaDrain),
            (27, Move::KnockOff),
            (30, Move::StunSpore),
            (33, Move::NaturalGift),
            (36, Move::GigaDrain),
            (40, Move::AncientPower),
            (43, Move::Slam),
            (46, Move::Tickle),
            (49, Move::WringOut),
            (53, Move::PowerWhip),
        ]
    };
    pub const Kangaskhan: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::EarlyBird, Ability::Scrappy),
        hidden_ability: Some(Ability::InnerFocus),
        height: 22,
        weight: 800,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(105, 2),
            Stat::new(95, 0),
            Stat::new(80, 0),
            Stat::new(40, 0),
            Stat::new(80, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::CometPunch),
            (1, Move::Leer),
            (7, Move::FakeOut),
            (10, Move::TailWhip),
            (13, Move::Bite),
            (19, Move::DoubleHit),
            (22, Move::Rage),
            (25, Move::MegaPunch),
            (31, Move::ChipAway),
            (34, Move::DizzyPunch),
            (37, Move::Crunch),
            (43, Move::Endure),
            (46, Move::Outrage),
            (49, Move::SuckerPunch),
            (55, Move::Reversal),
        ]
    };
    pub const Horsea: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::Sniper),
        hidden_ability: Some(Ability::Damp),
        height: 4,
        weight: 80,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(70, 0),
            Stat::new(70, 1),
            Stat::new(25, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (4, Move::Smokescreen),
            (8, Move::Leer),
            (11, Move::WaterGun),
            (14, Move::FocusEnergy),
            (18, Move::BubbleBeam),
            (23, Move::Agility),
            (26, Move::Twister),
            (30, Move::Brine),
            (35, Move::HydroPump),
            (38, Move::DragonDance),
            (42, Move::DragonPulse),
        ]
    };
    pub const Seadra: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Sniper),
        hidden_ability: Some(Ability::Damp),
        height: 12,
        weight: 250,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(95, 1),
            Stat::new(95, 1),
            Stat::new(45, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::WaterGun),
            (1, Move::Smokescreen),
            (1, Move::Bubble),
            (14, Move::FocusEnergy),
            (18, Move::BubbleBeam),
            (23, Move::Agility),
            (26, Move::Twister),
            (30, Move::Brine),
            (40, Move::HydroPump),
            (48, Move::DragonDance),
            (57, Move::DragonPulse),
        ]
    };
    pub const Goldeen: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::WaterVeil),
        hidden_ability: Some(Ability::LightningRod),
        height: 6,
        weight: 150,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(67, 1),
            Stat::new(60, 0),
            Stat::new(35, 0),
            Stat::new(50, 0),
            Stat::new(63, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::Peck),
            (1, Move::WaterSport),
            (7, Move::Supersonic),
            (11, Move::HornAttack),
            (17, Move::WaterPulse),
            (21, Move::Flail),
            (27, Move::AquaRing),
            (31, Move::FuryAttack),
            (37, Move::Waterfall),
            (41, Move::HornDrill),
            (47, Move::Agility),
            (51, Move::Soak),
            (57, Move::Megahorn),
        ]
    };
    pub const Seaking: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::WaterVeil),
        hidden_ability: Some(Ability::LightningRod),
        height: 13,
        weight: 390,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(92, 2),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(80, 0),
            Stat::new(68, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::Supersonic),
            (1, Move::Peck),
            (1, Move::WaterSport),
            (1, Move::PoisonJab),
            (11, Move::HornAttack),
            (17, Move::WaterPulse),
            (21, Move::Flail),
            (27, Move::AquaRing),
            (31, Move::FuryAttack),
            (40, Move::Waterfall),
            (47, Move::HornDrill),
            (56, Move::Agility),
            (63, Move::Soak),
            (72, Move::Megahorn),
        ]
    };
    pub const Staryu: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::Illuminate, Ability::NaturalCure),
        hidden_ability: Some(Ability::Analytic),
        height: 8,
        weight: 345,
        base_exp_yield: 68,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Harden),
            (6, Move::WaterGun),
            (10, Move::RapidSpin),
            (12, Move::Recover),
            (15, Move::Camouflage),
            (18, Move::Swift),
            (22, Move::BubbleBeam),
            (25, Move::Minimize),
            (30, Move::GyroBall),
            (33, Move::LightScreen),
            (36, Move::Brine),
            (40, Move::ReflectType),
            (43, Move::PowerGem),
            (48, Move::CosmicPower),
            (52, Move::HydroPump),
        ]
    };
    pub const Starmie: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Illuminate, Ability::NaturalCure),
        hidden_ability: Some(Ability::Analytic),
        height: 11,
        weight: 800,
        base_exp_yield: 182,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(100, 0),
            Stat::new(85, 0),
            Stat::new(115, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (1, Move::Recover),
            (1, Move::Swift),
            (1, Move::RapidSpin),
            (22, Move::ConfuseRay),
        ]
    };
    pub const MrMime: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Soundproof, Ability::Filter),
        hidden_ability: Some(Ability::Technician),
        height: 13,
        weight: 545,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(65, 0),
            Stat::new(100, 0),
            Stat::new(120, 2),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Barrier),
            (1, Move::MagicalLeaf),
            (1, Move::PowerSwap),
            (1, Move::GuardSwap),
            (1, Move::WideGuard),
            (1, Move::QuickGuard),
            (4, Move::Copycat),
            (8, Move::Meditate),
            (11, Move::DoubleSlap),
            (15, Move::Mimic),
            (15, Move::Psywave),
            (18, Move::Encore),
            (22, Move::LightScreen),
            (22, Move::Reflect),
            (25, Move::Psybeam),
            (29, Move::Substitute),
            (32, Move::Recycle),
            (36, Move::Trick),
            (39, Move::Psychic),
            (43, Move::RolePlay),
            (46, Move::BatonPass),
            (50, Move::Safeguard),
        ]
    };
    pub const Scyther: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Technician),
        hidden_ability: Some(Ability::Steadfast),
        height: 15,
        weight: 560,
        base_exp_yield: 100,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(110, 1),
            Stat::new(80, 0),
            Stat::new(55, 0),
            Stat::new(80, 0),
            Stat::new(105, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (1, Move::VacuumWave),
            (5, Move::FocusEnergy),
            (9, Move::Pursuit),
            (13, Move::FalseSwipe),
            (17, Move::Agility),
            (21, Move::WingAttack),
            (25, Move::FuryCutter),
            (29, Move::Slash),
            (33, Move::RazorWind),
            (37, Move::DoubleTeam),
            (41, Move::XScissor),
            (45, Move::NightSlash),
            (49, Move::DoubleHit),
            (53, Move::AirSlash),
            (57, Move::SwordsDance),
            (61, Move::Feint),
        ]
    };
    pub const Jynx: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::Forewarn),
        hidden_ability: Some(Ability::DrySkin),
        height: 14,
        weight: 406,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(50, 0),
            Stat::new(35, 0),
            Stat::new(115, 2),
            Stat::new(95, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Lick),
            (1, Move::LovelyKiss),
            (1, Move::PowderSnow),
            (15, Move::DoubleSlap),
            (18, Move::IcePunch),
            (21, Move::HeartStamp),
            (25, Move::MeanLook),
            (28, Move::FakeTears),
            (33, Move::WakeUpSlap),
            (39, Move::Avalanche),
            (44, Move::BodySlam),
            (49, Move::WringOut),
            (55, Move::PerishSong),
            (60, Move::Blizzard),
        ]
    };
    pub const Electabuzz: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 11,
        weight: 300,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(83, 0),
            Stat::new(57, 0),
            Stat::new(95, 0),
            Stat::new(85, 0),
            Stat::new(105, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::ThunderShock),
            (1, Move::QuickAttack),
            (8, Move::LowKick),
            (12, Move::Swift),
            (15, Move::ShockWave),
            (19, Move::ThunderWave),
            (22, Move::ElectroBall),
            (26, Move::LightScreen),
            (29, Move::ThunderPunch),
            (36, Move::Discharge),
            (42, Move::Screech),
            (49, Move::Thunderbolt),
            (55, Move::Thunder),
        ]
    };
    pub const Magmar: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::FlameBody),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 13,
        weight: 445,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(95, 0),
            Stat::new(57, 0),
            Stat::new(100, 2),
            Stat::new(85, 0),
            Stat::new(93, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Ember),
            (1, Move::Smog),
            (8, Move::Smokescreen),
            (12, Move::FeintAttack),
            (15, Move::FireSpin),
            (19, Move::ClearSmog),
            (22, Move::FlameBurst),
            (26, Move::ConfuseRay),
            (29, Move::FirePunch),
            (36, Move::LavaPlume),
            (42, Move::SunnyDay),
            (49, Move::Flamethrower),
            (55, Move::FireBlast),
        ]
    };
    pub const Pinsir: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::MoldBreaker),
        hidden_ability: Some(Ability::Moxie),
        height: 15,
        weight: 550,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(125, 2),
            Stat::new(100, 0),
            Stat::new(55, 0),
            Stat::new(70, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (1, Move::FocusEnergy),
            (4, Move::Bind),
            (8, Move::SeismicToss),
            (11, Move::Harden),
            (15, Move::Revenge),
            (18, Move::BrickBreak),
            (22, Move::VitalThrow),
            (26, Move::Submission),
            (29, Move::XScissor),
            (33, Move::StormThrow),
            (36, Move::Thrash),
            (40, Move::SwordsDance),
            (43, Move::Superpower),
            (47, Move::Guillotine),
        ]
    };
    pub const Tauros: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::AngerPoint),
        hidden_ability: Some(Ability::SheerForce),
        height: 14,
        weight: 884,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(100, 1),
            Stat::new(95, 0),
            Stat::new(40, 0),
            Stat::new(70, 0),
            Stat::new(110, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::TailWhip),
            (5, Move::Rage),
            (8, Move::HornAttack),
            (11, Move::ScaryFace),
            (15, Move::Pursuit),
            (19, Move::Rest),
            (24, Move::Payback),
            (29, Move::WorkUp),
            (35, Move::ZenHeadbutt),
            (41, Move::TakeDown),
            (48, Move::Swagger),
            (55, Move::Thrash),
            (63, Move::GigaImpact),
        ]
    };
    pub const Magikarp: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::Rattled),
        height: 9,
        weight: 100,
        base_exp_yield: 40,
        stats: Stats(
            Stat::new(20, 0),
            Stat::new(10, 0),
            Stat::new(55, 0),
            Stat::new(15, 0),
            Stat::new(20, 0),
            Stat::new(80, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water2, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Splash),
            (15, Move::Tackle),
            (30, Move::Flail),
        ]
    };
    pub const Gyarados: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::One(Ability::Intimidate),
        hidden_ability: Some(Ability::Moxie),
        height: 65,
        weight: 2350,
        base_exp_yield: 189,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(125, 2),
            Stat::new(79, 0),
            Stat::new(60, 0),
            Stat::new(100, 0),
            Stat::new(81, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water2, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Thrash),
            (20, Move::Bite),
            (23, Move::DragonRage),
            (26, Move::Leer),
            (29, Move::Twister),
            (32, Move::IceFang),
            (35, Move::AquaTail),
            (38, Move::RainDance),
            (41, Move::HydroPump),
            (44, Move::DragonDance),
            (47, Move::HyperBeam),
        ]
    };
    pub const Lapras: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ice),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::ShellArmor),
        hidden_ability: Some(Ability::Hydration),
        height: 25,
        weight: 2200,
        base_exp_yield: 187,
        stats: Stats(
            Stat::new(130, 2),
            Stat::new(85, 0),
            Stat::new(80, 0),
            Stat::new(85, 0),
            Stat::new(95, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Sing),
            (1, Move::WaterGun),
            (4, Move::Mist),
            (7, Move::ConfuseRay),
            (10, Move::IceShard),
            (14, Move::WaterPulse),
            (18, Move::BodySlam),
            (22, Move::RainDance),
            (27, Move::PerishSong),
            (32, Move::IceBeam),
            (37, Move::Brine),
            (43, Move::Safeguard),
            (49, Move::HydroPump),
            (55, Move::SheerCold),
        ]
    };
    pub const Ditto: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Limber),
        hidden_ability: Some(Ability::Imposter),
        height: 3,
        weight: 40,
        base_exp_yield: 101,
        stats: Stats(
            Stat::new(48, 1),
            Stat::new(48, 0),
            Stat::new(48, 0),
            Stat::new(48, 0),
            Stat::new(48, 0),
            Stat::new(48, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ditto),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Transform),
        ]
    };
    pub const Eevee: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::Adaptability),
        hidden_ability: Some(Ability::Anticipation),
        height: 3,
        weight: 65,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(65, 1),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::Growl),
            (13, Move::QuickAttack),
            (17, Move::Bite),
            (21, Move::Covet),
            (25, Move::TakeDown),
            (29, Move::Charm),
            (33, Move::BatonPass),
            (37, Move::DoubleEdge),
            (41, Move::LastResort),
            (45, Move::TrumpCard),
        ]
    };
    pub const Vaporeon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::WaterAbsorb),
        hidden_ability: Some(Ability::Hydration),
        height: 10,
        weight: 290,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(130, 2),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(110, 0),
            Stat::new(95, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::WaterGun),
            (13, Move::QuickAttack),
            (17, Move::WaterPulse),
            (21, Move::AuroraBeam),
            (25, Move::AquaRing),
            (29, Move::AcidArmor),
            (33, Move::Haze),
            (37, Move::MuddyWater),
            (41, Move::LastResort),
            (45, Move::HydroPump),
        ]
    };
    pub const Jolteon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::VoltAbsorb),
        hidden_ability: Some(Ability::QuickFeet),
        height: 8,
        weight: 245,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(110, 0),
            Stat::new(95, 0),
            Stat::new(130, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::ThunderShock),
            (13, Move::QuickAttack),
            (17, Move::DoubleKick),
            (21, Move::ThunderFang),
            (25, Move::PinMissile),
            (29, Move::Agility),
            (33, Move::ThunderWave),
            (37, Move::Discharge),
            (41, Move::LastResort),
            (45, Move::Thunder),
        ]
    };
    pub const Flareon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::FlashFire),
        hidden_ability: Some(Ability::Guts),
        height: 9,
        weight: 250,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(130, 2),
            Stat::new(60, 0),
            Stat::new(95, 0),
            Stat::new(110, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::Ember),
            (13, Move::QuickAttack),
            (17, Move::Bite),
            (21, Move::FireFang),
            (25, Move::FireSpin),
            (29, Move::ScaryFace),
            (33, Move::Smog),
            (37, Move::LavaPlume),
            (41, Move::LastResort),
            (45, Move::FireBlast),
        ]
    };
    pub const Porygon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Trace, Ability::Download),
        hidden_ability: Some(Ability::Analytic),
        height: 8,
        weight: 365,
        base_exp_yield: 79,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(85, 1),
            Stat::new(75, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Sharpen),
            (1, Move::Conversion),
            (1, Move::Conversion2),
            (7, Move::Psybeam),
            (12, Move::Agility),
            (18, Move::Recover),
            (23, Move::MagnetRise),
            (29, Move::SignalBeam),
            (34, Move::Recycle),
            (40, Move::Discharge),
            (45, Move::LockOn),
            (51, Move::TriAttack),
            (56, Move::MagicCoat),
            (62, Move::ZapCannon),
        ]
    };
    pub const Omanyte: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::ShellArmor),
        hidden_ability: Some(Ability::WeakArmor),
        height: 4,
        weight: 75,
        base_exp_yield: 71,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(40, 0),
            Stat::new(100, 1),
            Stat::new(90, 0),
            Stat::new(55, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Withdraw),
            (1, Move::Constrict),
            (7, Move::Bite),
            (10, Move::WaterGun),
            (16, Move::Rollout),
            (19, Move::Leer),
            (25, Move::MudShot),
            (28, Move::Brine),
            (34, Move::Protect),
            (37, Move::AncientPower),
            (43, Move::Tickle),
            (46, Move::RockBlast),
            (52, Move::ShellSmash),
            (55, Move::HydroPump),
        ]
    };
    pub const Omastar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::ShellArmor),
        hidden_ability: Some(Ability::WeakArmor),
        height: 10,
        weight: 350,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(125, 2),
            Stat::new(115, 0),
            Stat::new(70, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bite),
            (1, Move::Withdraw),
            (1, Move::Constrict),
            (10, Move::WaterGun),
            (16, Move::Rollout),
            (19, Move::Leer),
            (25, Move::MudShot),
            (28, Move::Brine),
            (34, Move::Protect),
            (37, Move::AncientPower),
            (40, Move::SpikeCannon),
            (48, Move::Tickle),
            (56, Move::RockBlast),
            (67, Move::ShellSmash),
            (75, Move::HydroPump),
        ]
    };
    pub const Kabuto: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::BattleArmor),
        hidden_ability: Some(Ability::WeakArmor),
        height: 5,
        weight: 115,
        base_exp_yield: 71,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(80, 0),
            Stat::new(90, 1),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Harden),
            (6, Move::Absorb),
            (11, Move::Leer),
            (16, Move::MudShot),
            (21, Move::SandAttack),
            (26, Move::Endure),
            (31, Move::AquaJet),
            (36, Move::MegaDrain),
            (41, Move::MetalSound),
            (46, Move::AncientPower),
            (51, Move::WringOut),
        ]
    };
    pub const Kabutops: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::BattleArmor),
        hidden_ability: Some(Ability::WeakArmor),
        height: 13,
        weight: 405,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(115, 2),
            Stat::new(105, 0),
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Absorb),
            (1, Move::Harden),
            (1, Move::Feint),
            (16, Move::MudShot),
            (21, Move::SandAttack),
            (26, Move::Endure),
            (31, Move::AquaJet),
            (36, Move::MegaDrain),
            (40, Move::Slash),
            (45, Move::MetalSound),
            (54, Move::AncientPower),
            (63, Move::WringOut),
            (72, Move::NightSlash),
        ]
    };
    pub const Aerodactyl: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Flying),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::Pressure),
        hidden_ability: Some(Ability::Unnerve),
        height: 18,
        weight: 590,
        base_exp_yield: 180,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(105, 0),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(130, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Bite),
            (1, Move::Supersonic),
            (1, Move::ScaryFace),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (9, Move::Roar),
            (17, Move::Agility),
            (25, Move::AncientPower),
            (33, Move::Crunch),
            (41, Move::TakeDown),
            (49, Move::SkyDrop),
            (57, Move::IronHead),
            (65, Move::HyperBeam),
            (73, Move::RockSlide),
            (81, Move::GigaImpact),
        ]
    };
    pub const Snorlax: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Immunity, Ability::ThickFat),
        hidden_ability: Some(Ability::Gluttony),
        height: 21,
        weight: 4600,
        base_exp_yield: 189,
        stats: Stats(
            Stat::new(160, 2),
            Stat::new(110, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(110, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::DefenseCurl),
            (9, Move::Amnesia),
            (12, Move::Lick),
            (17, Move::BellyDrum),
            (20, Move::Yawn),
            (25, Move::ChipAway),
            (28, Move::Rest),
            (28, Move::Snore),
            (33, Move::SleepTalk),
            (36, Move::BodySlam),
            (41, Move::Block),
            (44, Move::Rollout),
            (49, Move::Crunch),
            (52, Move::HeavySlam),
            (57, Move::GigaImpact),
        ]
    };
    pub const Articuno: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Flying),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::SnowCloak),
        height: 17,
        weight: 554,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(85, 0),
            Stat::new(100, 0),
            Stat::new(95, 0),
            Stat::new(125, 3),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::PowderSnow),
            (8, Move::Mist),
            (15, Move::IceShard),
            (22, Move::MindReader),
            (29, Move::AncientPower),
            (36, Move::Agility),
            (43, Move::IceBeam),
            (50, Move::Reflect),
            (57, Move::Roost),
            (64, Move::Tailwind),
            (71, Move::Blizzard),
            (78, Move::SheerCold),
            (85, Move::Hail),
            (92, Move::Hurricane),
        ]
    };
    pub const Zapdos: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Flying),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Static),
        height: 16,
        weight: 526,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(90, 0),
            Stat::new(85, 0),
            Stat::new(125, 3),
            Stat::new(90, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (1, Move::ThunderShock),
            (8, Move::ThunderWave),
            (15, Move::Detect),
            (22, Move::Pluck),
            (29, Move::AncientPower),
            (36, Move::Charge),
            (43, Move::Agility),
            (50, Move::Discharge),
            (57, Move::Roost),
            (64, Move::LightScreen),
            (71, Move::DrillPeck),
            (78, Move::Thunder),
            (85, Move::RainDance),
            (92, Move::ZapCannon),
        ]
    };
    pub const Moltres: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Flying),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::FlameBody),
        height: 20,
        weight: 600,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(100, 0),
            Stat::new(90, 0),
            Stat::new(125, 3),
            Stat::new(85, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Ember),
            (8, Move::FireSpin),
            (15, Move::Agility),
            (22, Move::Endure),
            (29, Move::AncientPower),
            (36, Move::Flamethrower),
            (43, Move::Safeguard),
            (50, Move::AirSlash),
            (57, Move::Roost),
            (64, Move::HeatWave),
            (71, Move::SolarBeam),
            (78, Move::SkyAttack),
            (85, Move::SunnyDay),
            (92, Move::Hurricane),
        ]
    };
    pub const Dratini: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: Some(Ability::MarvelScale),
        height: 18,
        weight: 33,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(41, 0),
            Stat::new(64, 1),
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::Leer),
            (5, Move::ThunderWave),
            (11, Move::Twister),
            (15, Move::DragonRage),
            (21, Move::Slam),
            (25, Move::Agility),
            (31, Move::DragonTail),
            (35, Move::AquaTail),
            (41, Move::DragonRush),
            (45, Move::Safeguard),
            (51, Move::DragonDance),
            (55, Move::Outrage),
            (61, Move::HyperBeam),
        ]
    };
    pub const Dragonair: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: Some(Ability::MarvelScale),
        height: 40,
        weight: 165,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(61, 0),
            Stat::new(84, 2),
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::Leer),
            (1, Move::ThunderWave),
            (1, Move::Twister),
            (15, Move::DragonRage),
            (21, Move::Slam),
            (25, Move::Agility),
            (33, Move::DragonTail),
            (39, Move::AquaTail),
            (47, Move::DragonRush),
            (53, Move::Safeguard),
            (61, Move::DragonDance),
            (67, Move::Outrage),
            (75, Move::HyperBeam),
        ]
    };
    pub const Dragonite: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Flying),
        ability: PokemonAbility::One(Ability::InnerFocus),
        hidden_ability: Some(Ability::Multiscale),
        height: 22,
        weight: 2100,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(91, 0),
            Stat::new(134, 3),
            Stat::new(95, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::ThunderPunch),
            (1, Move::Wrap),
            (1, Move::Leer),
            (1, Move::ThunderWave),
            (1, Move::Twister),
            (1, Move::Roost),
            (15, Move::DragonRage),
            (21, Move::Slam),
            (25, Move::Agility),
            (33, Move::DragonTail),
            (39, Move::AquaTail),
            (47, Move::DragonRush),
            (53, Move::Safeguard),
            (55, Move::WingAttack),
            (61, Move::DragonDance),
            (67, Move::Outrage),
            (75, Move::HyperBeam),
            (81, Move::Hurricane),
        ]
    };
    pub const Mewtwo: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Unnerve),
        height: 20,
        weight: 1220,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(106, 0),
            Stat::new(110, 0),
            Stat::new(90, 0),
            Stat::new(154, 3),
            Stat::new(90, 0),
            Stat::new(130, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Disable),
            (1, Move::Confusion),
            (1, Move::Barrier),
            (8, Move::Swift),
            (15, Move::FutureSight),
            (22, Move::PsychUp),
            (29, Move::MiracleEye),
            (36, Move::Mist),
            (43, Move::PsychoCut),
            (50, Move::Amnesia),
            (57, Move::PowerSwap),
            (57, Move::GuardSwap),
            (64, Move::Psychic),
            (71, Move::MeFirst),
            (79, Move::Recover),
            (86, Move::Safeguard),
            (93, Move::AuraSphere),
            (100, Move::Psystrike),
        ]
    };
    pub const Mew: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Synchronize),
        hidden_ability: None,
        height: 4,
        weight: 40,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Transform),
            (1, Move::ReflectType),
            (10, Move::MegaPunch),
            (20, Move::Metronome),
            (30, Move::Psychic),
            (40, Move::Barrier),
            (50, Move::AncientPower),
            (60, Move::Amnesia),
            (70, Move::MeFirst),
            (80, Move::BatonPass),
            (90, Move::NastyPlot),
            (100, Move::AuraSphere),
        ]
    };
    pub const Chikorita: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::LeafGuard),
        height: 9,
        weight: 64,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(49, 0),
            Stat::new(65, 0),
            Stat::new(49, 0),
            Stat::new(65, 1),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (6, Move::RazorLeaf),
            (9, Move::PoisonPowder),
            (12, Move::Synthesis),
            (17, Move::Reflect),
            (20, Move::MagicalLeaf),
            (23, Move::NaturalGift),
            (28, Move::SweetScent),
            (31, Move::LightScreen),
            (34, Move::BodySlam),
            (39, Move::Safeguard),
            (42, Move::Aromatherapy),
            (45, Move::SolarBeam),
        ]
    };
    pub const Bayleef: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::LeafGuard),
        height: 12,
        weight: 158,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(62, 0),
            Stat::new(80, 1),
            Stat::new(63, 0),
            Stat::new(80, 1),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::RazorLeaf),
            (1, Move::PoisonPowder),
            (12, Move::Synthesis),
            (18, Move::Reflect),
            (22, Move::MagicalLeaf),
            (26, Move::NaturalGift),
            (32, Move::SweetScent),
            (36, Move::LightScreen),
            (40, Move::BodySlam),
            (46, Move::Safeguard),
            (50, Move::Aromatherapy),
            (54, Move::SolarBeam),
        ]
    };
    pub const Meganium: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::LeafGuard),
        height: 18,
        weight: 1005,
        base_exp_yield: 236,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(82, 0),
            Stat::new(100, 1),
            Stat::new(83, 0),
            Stat::new(100, 2),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::RazorLeaf),
            (1, Move::PoisonPowder),
            (12, Move::Synthesis),
            (18, Move::Reflect),
            (22, Move::MagicalLeaf),
            (26, Move::NaturalGift),
            (32, Move::PetalDance),
            (34, Move::SweetScent),
            (40, Move::LightScreen),
            (46, Move::BodySlam),
            (54, Move::Safeguard),
            (60, Move::Aromatherapy),
            (66, Move::SolarBeam),
        ]
    };
    pub const Cyndaquil: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::FlashFire),
        height: 5,
        weight: 79,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(39, 0),
            Stat::new(52, 0),
            Stat::new(43, 0),
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (6, Move::Smokescreen),
            (10, Move::Ember),
            (13, Move::QuickAttack),
            (19, Move::FlameWheel),
            (22, Move::DefenseCurl),
            (28, Move::FlameCharge),
            (31, Move::Swift),
            (37, Move::LavaPlume),
            (40, Move::Flamethrower),
            (46, Move::Inferno),
            (49, Move::Rollout),
            (55, Move::DoubleEdge),
            (58, Move::Eruption),
        ]
    };
    pub const Quilava: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::FlashFire),
        height: 9,
        weight: 190,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(58, 0),
            Stat::new(64, 0),
            Stat::new(58, 0),
            Stat::new(80, 1),
            Stat::new(65, 0),
            Stat::new(80, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Smokescreen),
            (10, Move::Ember),
            (13, Move::QuickAttack),
            (20, Move::FlameWheel),
            (24, Move::DefenseCurl),
            (31, Move::Swift),
            (35, Move::FlameCharge),
            (42, Move::LavaPlume),
            (46, Move::Flamethrower),
            (53, Move::Inferno),
            (57, Move::Rollout),
            (64, Move::DoubleEdge),
            (68, Move::Eruption),
        ]
    };
    pub const Typhlosion: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::FlashFire),
        height: 17,
        weight: 795,
        base_exp_yield: 240,
        stats: Stats(
            Stat::new(78, 0),
            Stat::new(84, 0),
            Stat::new(78, 0),
            Stat::new(109, 3),
            Stat::new(85, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Ember),
            (1, Move::Smokescreen),
            (1, Move::GyroBall),
            (13, Move::QuickAttack),
            (20, Move::FlameWheel),
            (24, Move::DefenseCurl),
            (31, Move::Swift),
            (35, Move::FlameCharge),
            (43, Move::LavaPlume),
            (48, Move::Flamethrower),
            (56, Move::Inferno),
            (61, Move::Rollout),
            (69, Move::DoubleEdge),
            (74, Move::Eruption),
        ]
    };
    pub const Totodile: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::SheerForce),
        height: 6,
        weight: 95,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(65, 1),
            Stat::new(64, 0),
            Stat::new(44, 0),
            Stat::new(48, 0),
            Stat::new(43, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (6, Move::WaterGun),
            (8, Move::Rage),
            (13, Move::Bite),
            (15, Move::ScaryFace),
            (20, Move::IceFang),
            (22, Move::Flail),
            (27, Move::Crunch),
            (29, Move::ChipAway),
            (34, Move::Slash),
            (36, Move::Screech),
            (41, Move::Thrash),
            (43, Move::AquaTail),
            (48, Move::Superpower),
            (50, Move::HydroPump),
        ]
    };
    pub const Croconaw: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::SheerForce),
        height: 11,
        weight: 250,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(80, 1),
            Stat::new(80, 1),
            Stat::new(59, 0),
            Stat::new(63, 0),
            Stat::new(58, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::WaterGun),
            (8, Move::Rage),
            (13, Move::Bite),
            (15, Move::ScaryFace),
            (21, Move::IceFang),
            (24, Move::Flail),
            (30, Move::Crunch),
            (33, Move::ChipAway),
            (39, Move::Slash),
            (42, Move::Screech),
            (48, Move::Thrash),
            (51, Move::AquaTail),
            (57, Move::Superpower),
            (60, Move::HydroPump),
        ]
    };
    pub const Feraligatr: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::SheerForce),
        height: 23,
        weight: 888,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(105, 2),
            Stat::new(100, 1),
            Stat::new(79, 0),
            Stat::new(83, 0),
            Stat::new(78, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::WaterGun),
            (1, Move::Rage),
            (13, Move::Bite),
            (15, Move::ScaryFace),
            (21, Move::IceFang),
            (24, Move::Flail),
            (30, Move::Agility),
            (32, Move::Crunch),
            (37, Move::ChipAway),
            (45, Move::Slash),
            (50, Move::Screech),
            (58, Move::Thrash),
            (63, Move::AquaTail),
            (71, Move::Superpower),
            (76, Move::HydroPump),
        ]
    };
    pub const Sentret: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::KeenEye),
        hidden_ability: Some(Ability::Frisk),
        height: 8,
        weight: 60,
        base_exp_yield: 43,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(46, 1),
            Stat::new(34, 0),
            Stat::new(35, 0),
            Stat::new(45, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Foresight),
            (4, Move::DefenseCurl),
            (7, Move::QuickAttack),
            (13, Move::FurySwipes),
            (16, Move::HelpingHand),
            (19, Move::FollowMe),
            (25, Move::Slam),
            (28, Move::Rest),
            (31, Move::SuckerPunch),
            (36, Move::Amnesia),
            (39, Move::BatonPass),
            (42, Move::MeFirst),
            (47, Move::HyperVoice),
        ]
    };
    pub const Furret: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::KeenEye),
        hidden_ability: Some(Ability::Frisk),
        height: 18,
        weight: 325,
        base_exp_yield: 145,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(76, 0),
            Stat::new(64, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(90, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::QuickAttack),
            (1, Move::DefenseCurl),
            (1, Move::Foresight),
            (13, Move::FurySwipes),
            (17, Move::HelpingHand),
            (21, Move::FollowMe),
            (28, Move::Slam),
            (32, Move::Rest),
            (36, Move::SuckerPunch),
            (42, Move::Amnesia),
            (46, Move::BatonPass),
            (50, Move::MeFirst),
            (56, Move::HyperVoice),
        ]
    };
    pub const Hoothoot: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::KeenEye),
        hidden_ability: Some(Ability::TintedLens),
        height: 7,
        weight: 212,
        base_exp_yield: 52,
        stats: Stats(
            Stat::new(60, 1),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(36, 0),
            Stat::new(56, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Foresight),
            (5, Move::Hypnosis),
            (9, Move::Peck),
            (13, Move::Uproar),
            (17, Move::Reflect),
            (21, Move::Confusion),
            (25, Move::EchoedVoice),
            (29, Move::TakeDown),
            (33, Move::AirSlash),
            (37, Move::ZenHeadbutt),
            (41, Move::Synchronoise),
            (45, Move::Extrasensory),
            (49, Move::PsychoShift),
            (53, Move::Roost),
            (57, Move::DreamEater),
        ]
    };
    pub const Noctowl: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::KeenEye),
        hidden_ability: Some(Ability::TintedLens),
        height: 16,
        weight: 408,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(100, 2),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(86, 0),
            Stat::new(96, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Hypnosis),
            (1, Move::SkyAttack),
            (1, Move::Foresight),
            (9, Move::Peck),
            (13, Move::Uproar),
            (17, Move::Reflect),
            (22, Move::Confusion),
            (27, Move::EchoedVoice),
            (32, Move::TakeDown),
            (37, Move::AirSlash),
            (42, Move::ZenHeadbutt),
            (47, Move::Synchronoise),
            (52, Move::Extrasensory),
            (57, Move::PsychoShift),
            (62, Move::Roost),
            (67, Move::DreamEater),
        ]
    };
    pub const Ledyba: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::EarlyBird),
        hidden_ability: Some(Ability::Rattled),
        height: 10,
        weight: 108,
        base_exp_yield: 53,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(20, 0),
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(80, 1),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (6, Move::Supersonic),
            (9, Move::CometPunch),
            (14, Move::LightScreen),
            (14, Move::Reflect),
            (14, Move::Safeguard),
            (17, Move::MachPunch),
            (22, Move::BatonPass),
            (25, Move::SilverWind),
            (30, Move::Agility),
            (33, Move::Swift),
            (38, Move::DoubleEdge),
            (41, Move::BugBuzz),
        ]
    };
    pub const Ledian: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::EarlyBird),
        hidden_ability: Some(Ability::IronFist),
        height: 14,
        weight: 356,
        base_exp_yield: 137,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(35, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(110, 2),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::CometPunch),
            (1, Move::Tackle),
            (1, Move::Supersonic),
            (14, Move::LightScreen),
            (14, Move::Reflect),
            (14, Move::Safeguard),
            (17, Move::MachPunch),
            (24, Move::BatonPass),
            (29, Move::SilverWind),
            (36, Move::Agility),
            (41, Move::Swift),
            (48, Move::DoubleEdge),
            (53, Move::BugBuzz),
        ]
    };
    pub const Spinarak: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Insomnia),
        hidden_ability: Some(Ability::Sniper),
        height: 5,
        weight: 85,
        base_exp_yield: 50,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(60, 1),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::StringShot),
            (5, Move::ScaryFace),
            (8, Move::Constrict),
            (12, Move::LeechLife),
            (15, Move::NightShade),
            (19, Move::ShadowSneak),
            (22, Move::FurySwipes),
            (26, Move::SuckerPunch),
            (29, Move::SpiderWeb),
            (33, Move::Agility),
            (36, Move::PinMissile),
            (40, Move::Psychic),
            (43, Move::PoisonJab),
            (47, Move::CrossPoison),
        ]
    };
    pub const Ariados: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Insomnia),
        hidden_ability: Some(Ability::Sniper),
        height: 11,
        weight: 335,
        base_exp_yield: 140,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(90, 2),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::StringShot),
            (1, Move::Constrict),
            (1, Move::ScaryFace),
            (1, Move::BugBite),
            (12, Move::LeechLife),
            (15, Move::NightShade),
            (19, Move::ShadowSneak),
            (23, Move::FurySwipes),
            (28, Move::SuckerPunch),
            (32, Move::SpiderWeb),
            (37, Move::Agility),
            (41, Move::PinMissile),
            (46, Move::Psychic),
            (50, Move::PoisonJab),
            (55, Move::CrossPoison),
        ]
    };
    pub const Crobat: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Flying),
        ability: PokemonAbility::One(Ability::InnerFocus),
        hidden_ability: Some(Ability::Infiltrator),
        height: 18,
        weight: 750,
        base_exp_yield: 241,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(90, 0),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(130, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Supersonic),
            (1, Move::Screech),
            (1, Move::LeechLife),
            (1, Move::Astonish),
            (1, Move::CrossPoison),
            (12, Move::Bite),
            (15, Move::WingAttack),
            (19, Move::ConfuseRay),
            (24, Move::Swift),
            (28, Move::AirCutter),
            (33, Move::Acrobatics),
            (38, Move::MeanLook),
            (42, Move::PoisonFang),
            (47, Move::Haze),
            (52, Move::AirSlash),
        ]
    };
    pub const Chinchou: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Electric),
        ability: PokemonAbility::Two(Ability::VoltAbsorb, Ability::Illuminate),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 5,
        weight: 120,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(75, 1),
            Stat::new(38, 0),
            Stat::new(38, 0),
            Stat::new(56, 0),
            Stat::new(56, 0),
            Stat::new(67, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Supersonic),
            (1, Move::Bubble),
            (6, Move::ThunderWave),
            (9, Move::Flail),
            (12, Move::ConfuseRay),
            (17, Move::WaterGun),
            (20, Move::Spark),
            (23, Move::TakeDown),
            (28, Move::ElectroBall),
            (31, Move::BubbleBeam),
            (34, Move::SignalBeam),
            (39, Move::Discharge),
            (42, Move::AquaRing),
            (45, Move::HydroPump),
            (50, Move::Charge),
        ]
    };
    pub const Lanturn: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Electric),
        ability: PokemonAbility::Two(Ability::VoltAbsorb, Ability::Illuminate),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 12,
        weight: 225,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(125, 2),
            Stat::new(58, 0),
            Stat::new(58, 0),
            Stat::new(76, 0),
            Stat::new(76, 0),
            Stat::new(67, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Supersonic),
            (1, Move::ThunderWave),
            (1, Move::Bubble),
            (9, Move::Flail),
            (12, Move::WaterGun),
            (17, Move::ConfuseRay),
            (20, Move::Spark),
            (23, Move::TakeDown),
            (27, Move::Stockpile),
            (27, Move::SpitUp),
            (27, Move::Swallow),
            (30, Move::ElectroBall),
            (35, Move::BubbleBeam),
            (40, Move::SignalBeam),
            (47, Move::Discharge),
            (52, Move::AquaRing),
            (57, Move::HydroPump),
            (64, Move::Charge),
        ]
    };
    pub const Pichu: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::LightningRod),
        height: 3,
        weight: 20,
        base_exp_yield: 41,
        stats: Stats(
            Stat::new(20, 0),
            Stat::new(40, 0),
            Stat::new(15, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(60, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ThunderShock),
            (1, Move::Charm),
            (5, Move::TailWhip),
            (10, Move::ThunderWave),
            (13, Move::SweetKiss),
            (18, Move::NastyPlot),
        ]
    };
    pub const Cleffa: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fairy),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::MagicGuard),
        hidden_ability: Some(Ability::FriendGuard),
        height: 3,
        weight: 30,
        base_exp_yield: 44,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(25, 0),
            Stat::new(28, 0),
            Stat::new(45, 0),
            Stat::new(55, 1),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Charm),
            (4, Move::Encore),
            (7, Move::Sing),
            (10, Move::SweetKiss),
            (13, Move::Copycat),
            (16, Move::MagicalLeaf),
        ]
    };
    pub const Igglybuff: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Fairy),
        ability: PokemonAbility::One(Ability::CuteCharm),
        hidden_ability: Some(Ability::FriendGuard),
        height: 3,
        weight: 10,
        base_exp_yield: 42,
        stats: Stats(
            Stat::new(90, 1),
            Stat::new(30, 0),
            Stat::new(15, 0),
            Stat::new(40, 0),
            Stat::new(20, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Sing),
            (1, Move::Charm),
            (5, Move::DefenseCurl),
            (9, Move::Pound),
            (13, Move::SweetKiss),
            (17, Move::Copycat),
        ]
    };
    pub const Togepi: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fairy),
        ability: PokemonAbility::Two(Ability::Hustle, Ability::SereneGrace),
        hidden_ability: Some(Ability::SuperLuck),
        height: 3,
        weight: 15,
        base_exp_yield: 49,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(20, 0),
            Stat::new(65, 0),
            Stat::new(40, 0),
            Stat::new(65, 1),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Charm),
            (5, Move::Metronome),
            (9, Move::SweetKiss),
            (13, Move::Yawn),
            (17, Move::Encore),
            (21, Move::FollowMe),
            (25, Move::Bestow),
            (29, Move::Wish),
            (33, Move::AncientPower),
            (37, Move::Safeguard),
            (41, Move::BatonPass),
            (45, Move::DoubleEdge),
            (49, Move::LastResort),
            (53, Move::AfterYou),
        ]
    };
    pub const Togetic: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fairy, Type::Flying),
        ability: PokemonAbility::Two(Ability::Hustle, Ability::SereneGrace),
        hidden_ability: Some(Ability::SuperLuck),
        height: 6,
        weight: 32,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(85, 0),
            Stat::new(80, 0),
            Stat::new(105, 2),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Metronome),
            (1, Move::SweetKiss),
            (1, Move::Charm),
            (1, Move::MagicalLeaf),
            (13, Move::Yawn),
            (17, Move::Encore),
            (21, Move::FollowMe),
            (25, Move::Bestow),
            (29, Move::Wish),
            (33, Move::AncientPower),
            (37, Move::Safeguard),
            (41, Move::BatonPass),
            (45, Move::DoubleEdge),
            (49, Move::LastResort),
            (53, Move::AfterYou),
        ]
    };
    pub const Natu: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Flying),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::EarlyBird),
        hidden_ability: Some(Ability::MagicBounce),
        height: 2,
        weight: 20,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(70, 1),
            Stat::new(45, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Peck),
            (6, Move::NightShade),
            (9, Move::Teleport),
            (12, Move::LuckyChant),
            (17, Move::MiracleEye),
            (20, Move::MeFirst),
            (23, Move::ConfuseRay),
            (28, Move::Wish),
            (33, Move::PsychoShift),
            (36, Move::FutureSight),
            (39, Move::StoredPower),
            (44, Move::OminousWind),
            (47, Move::PowerSwap),
            (47, Move::GuardSwap),
            (50, Move::Psychic),
        ]
    };
    pub const Xatu: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Flying),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::EarlyBird),
        hidden_ability: Some(Ability::MagicBounce),
        height: 15,
        weight: 150,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(75, 0),
            Stat::new(70, 0),
            Stat::new(95, 1),
            Stat::new(70, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Peck),
            (6, Move::NightShade),
            (9, Move::Teleport),
            (12, Move::LuckyChant),
            (17, Move::MiracleEye),
            (20, Move::MeFirst),
            (23, Move::ConfuseRay),
            (27, Move::Tailwind),
            (30, Move::Wish),
            (37, Move::PsychoShift),
            (42, Move::FutureSight),
            (47, Move::StoredPower),
            (54, Move::PowerSwap),
            (54, Move::OminousWind),
            (59, Move::GuardSwap),
            (66, Move::Psychic),
        ]
    };
    pub const Mareep: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::Plus),
        height: 6,
        weight: 78,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(65, 1),
            Stat::new(45, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (4, Move::ThunderWave),
            (8, Move::ThunderShock),
            (11, Move::CottonSpore),
            (15, Move::Charge),
            (18, Move::TakeDown),
            (22, Move::ElectroBall),
            (25, Move::ConfuseRay),
            (29, Move::PowerGem),
            (32, Move::Discharge),
            (36, Move::CottonGuard),
            (39, Move::SignalBeam),
            (43, Move::LightScreen),
            (46, Move::Thunder),
        ]
    };
    pub const Flaaffy: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::Plus),
        height: 8,
        weight: 133,
        base_exp_yield: 128,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(80, 2),
            Stat::new(60, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::ThunderShock),
            (1, Move::ThunderWave),
            (11, Move::CottonSpore),
            (16, Move::Charge),
            (20, Move::TakeDown),
            (25, Move::ElectroBall),
            (29, Move::ConfuseRay),
            (34, Move::PowerGem),
            (38, Move::Discharge),
            (43, Move::CottonGuard),
            (47, Move::SignalBeam),
            (52, Move::LightScreen),
            (56, Move::Thunder),
        ]
    };
    pub const Ampharos: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::Plus),
        height: 14,
        weight: 615,
        base_exp_yield: 230,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(115, 3),
            Stat::new(90, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::ThunderShock),
            (1, Move::ThunderWave),
            (11, Move::CottonSpore),
            (16, Move::Charge),
            (20, Move::TakeDown),
            (25, Move::ElectroBall),
            (29, Move::ConfuseRay),
            (30, Move::ThunderPunch),
            (35, Move::PowerGem),
            (40, Move::Discharge),
            (46, Move::CottonGuard),
            (51, Move::SignalBeam),
            (57, Move::LightScreen),
            (62, Move::Thunder),
        ]
    };
    pub const Bellossom: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: Some(Ability::Healer),
        height: 4,
        weight: 58,
        base_exp_yield: 221,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(80, 0),
            Stat::new(95, 0),
            Stat::new(90, 0),
            Stat::new(100, 3),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MegaDrain),
            (1, Move::StunSpore),
            (1, Move::SweetScent),
            (1, Move::SunnyDay),
            (1, Move::LeafBlade),
            (23, Move::MagicalLeaf),
            (53, Move::LeafStorm),
        ]
    };
    pub const Marill: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Fairy),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::HugePower),
        hidden_ability: Some(Ability::SapSipper),
        height: 4,
        weight: 85,
        base_exp_yield: 88,
        stats: Stats(
            Stat::new(70, 2),
            Stat::new(20, 0),
            Stat::new(50, 0),
            Stat::new(20, 0),
            Stat::new(50, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Bubble),
            (2, Move::TailWhip),
            (5, Move::WaterSport),
            (7, Move::WaterGun),
            (10, Move::DefenseCurl),
            (10, Move::Rollout),
            (13, Move::BubbleBeam),
            (16, Move::HelpingHand),
            (20, Move::AquaTail),
            (23, Move::DoubleEdge),
            (28, Move::AquaRing),
            (31, Move::RainDance),
            (37, Move::Superpower),
            (40, Move::HydroPump),
        ]
    };
    pub const Azumarill: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Fairy),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::HugePower),
        hidden_ability: Some(Ability::SapSipper),
        height: 8,
        weight: 285,
        base_exp_yield: 189,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(50, 0),
            Stat::new(80, 0),
            Stat::new(60, 0),
            Stat::new(80, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Bubble),
            (1, Move::WaterSport),
            (7, Move::WaterGun),
            (10, Move::DefenseCurl),
            (10, Move::Rollout),
            (13, Move::BubbleBeam),
            (16, Move::HelpingHand),
            (21, Move::AquaTail),
            (25, Move::DoubleEdge),
            (31, Move::AquaRing),
            (35, Move::RainDance),
            (42, Move::Superpower),
            (46, Move::HydroPump),
        ]
    };
    pub const Sudowoodo: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::RockHead),
        hidden_ability: Some(Ability::Rattled),
        height: 12,
        weight: 380,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(100, 0),
            Stat::new(115, 2),
            Stat::new(30, 0),
            Stat::new(65, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::LowKick),
            (1, Move::RockThrow),
            (1, Move::Flail),
            (1, Move::Copycat),
            (1, Move::WoodHammer),
            (15, Move::Slam),
            (15, Move::Mimic),
            (19, Move::FeintAttack),
            (22, Move::RockTomb),
            (26, Move::Block),
            (29, Move::RockSlide),
            (33, Move::Counter),
            (36, Move::SuckerPunch),
            (40, Move::DoubleEdge),
            (43, Move::StoneEdge),
            (47, Move::HammerArm),
        ]
    };
    pub const Politoed: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::Damp),
        hidden_ability: Some(Ability::Drizzle),
        height: 11,
        weight: 339,
        base_exp_yield: 225,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(90, 0),
            Stat::new(100, 3),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleSlap),
            (1, Move::BubbleBeam),
            (1, Move::Hypnosis),
            (1, Move::PerishSong),
            (27, Move::Swagger),
            (37, Move::Bounce),
            (48, Move::HyperVoice),
        ]
    };
    pub const Hoppip: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Flying),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::LeafGuard),
        hidden_ability: Some(Ability::Infiltrator),
        height: 4,
        weight: 5,
        base_exp_yield: 50,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(55, 1),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Splash),
            (4, Move::Synthesis),
            (7, Move::TailWhip),
            (10, Move::Tackle),
            (12, Move::PoisonPowder),
            (14, Move::StunSpore),
            (16, Move::SleepPowder),
            (19, Move::BulletSeed),
            (22, Move::LeechSeed),
            (25, Move::MegaDrain),
            (28, Move::Acrobatics),
            (31, Move::RagePowder),
            (34, Move::CottonSpore),
            (37, Move::UTurn),
            (40, Move::WorrySeed),
            (43, Move::GigaDrain),
            (46, Move::Bounce),
            (49, Move::Memento),
        ]
    };
    pub const Skiploom: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Flying),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::LeafGuard),
        hidden_ability: Some(Ability::Infiltrator),
        height: 6,
        weight: 10,
        base_exp_yield: 119,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(65, 0),
            Stat::new(80, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Splash),
            (1, Move::Synthesis),
            (12, Move::PoisonPowder),
            (14, Move::StunSpore),
            (16, Move::SleepPowder),
            (20, Move::BulletSeed),
            (24, Move::LeechSeed),
            (28, Move::MegaDrain),
            (32, Move::Acrobatics),
            (36, Move::RagePowder),
            (40, Move::CottonSpore),
            (44, Move::UTurn),
            (48, Move::WorrySeed),
            (52, Move::GigaDrain),
            (56, Move::Bounce),
            (60, Move::Memento),
        ]
    };
    pub const Jumpluff: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Flying),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::LeafGuard),
        hidden_ability: Some(Ability::Infiltrator),
        height: 8,
        weight: 30,
        base_exp_yield: 207,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(55, 0),
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(95, 0),
            Stat::new(110, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Splash),
            (1, Move::Synthesis),
            (12, Move::PoisonPowder),
            (14, Move::StunSpore),
            (16, Move::SleepPowder),
            (20, Move::BulletSeed),
            (24, Move::LeechSeed),
            (29, Move::MegaDrain),
            (34, Move::Acrobatics),
            (39, Move::RagePowder),
            (44, Move::CottonSpore),
            (49, Move::UTurn),
            (54, Move::WorrySeed),
            (59, Move::GigaDrain),
            (64, Move::Bounce),
            (69, Move::Memento),
        ]
    };
    pub const Aipom: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::Pickup),
        hidden_ability: Some(Ability::SkillLink),
        height: 8,
        weight: 115,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::TailWhip),
            (4, Move::SandAttack),
            (8, Move::Astonish),
            (11, Move::BatonPass),
            (15, Move::Tickle),
            (18, Move::FurySwipes),
            (22, Move::Swift),
            (25, Move::Screech),
            (29, Move::Agility),
            (32, Move::DoubleHit),
            (36, Move::Fling),
            (39, Move::NastyPlot),
            (43, Move::LastResort),
        ]
    };
    pub const Sunkern: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::SolarPower),
        hidden_ability: Some(Ability::EarlyBird),
        height: 3,
        weight: 18,
        base_exp_yield: 36,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(30, 1),
            Stat::new(30, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (1, Move::Growth),
            (4, Move::Ingrain),
            (7, Move::GrassWhistle),
            (10, Move::MegaDrain),
            (13, Move::LeechSeed),
            (16, Move::RazorLeaf),
            (19, Move::WorrySeed),
            (22, Move::GigaDrain),
            (25, Move::Endeavor),
            (28, Move::Synthesis),
            (31, Move::NaturalGift),
            (34, Move::SolarBeam),
            (37, Move::DoubleEdge),
            (40, Move::SunnyDay),
            (43, Move::SeedBomb),
        ]
    };
    pub const Sunflora: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::SolarPower),
        hidden_ability: Some(Ability::EarlyBird),
        height: 8,
        weight: 85,
        base_exp_yield: 149,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(55, 0),
            Stat::new(105, 2),
            Stat::new(85, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Absorb),
            (1, Move::Growth),
            (4, Move::Ingrain),
            (7, Move::GrassWhistle),
            (10, Move::MegaDrain),
            (13, Move::LeechSeed),
            (16, Move::RazorLeaf),
            (19, Move::WorrySeed),
            (22, Move::GigaDrain),
            (25, Move::BulletSeed),
            (28, Move::PetalDance),
            (31, Move::NaturalGift),
            (34, Move::SolarBeam),
            (37, Move::DoubleEdge),
            (40, Move::SunnyDay),
            (43, Move::LeafStorm),
        ]
    };
    pub const Yanma: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::Two(Ability::SpeedBoost, Ability::CompoundEyes),
        hidden_ability: Some(Ability::Frisk),
        height: 12,
        weight: 380,
        base_exp_yield: 78,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(45, 0),
            Stat::new(75, 0),
            Stat::new(45, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Foresight),
            (6, Move::QuickAttack),
            (11, Move::DoubleTeam),
            (14, Move::SonicBoom),
            (17, Move::Detect),
            (22, Move::Supersonic),
            (27, Move::Uproar),
            (30, Move::Pursuit),
            (33, Move::AncientPower),
            (38, Move::Hypnosis),
            (43, Move::WingAttack),
            (46, Move::Screech),
            (49, Move::UTurn),
            (54, Move::AirSlash),
            (57, Move::BugBuzz),
        ]
    };
    pub const Wooper: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::Damp, Ability::WaterAbsorb),
        hidden_ability: Some(Ability::Unaware),
        height: 4,
        weight: 85,
        base_exp_yield: 42,
        stats: Stats(
            Stat::new(55, 1),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::WaterGun),
            (5, Move::MudSport),
            (9, Move::MudShot),
            (15, Move::Slam),
            (19, Move::MudBomb),
            (23, Move::Amnesia),
            (29, Move::Yawn),
            (33, Move::Earthquake),
            (37, Move::RainDance),
            (43, Move::Mist),
            (43, Move::Haze),
            (47, Move::MuddyWater),
        ]
    };
    pub const Quagsire: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::Damp, Ability::WaterAbsorb),
        hidden_ability: Some(Ability::Unaware),
        height: 14,
        weight: 750,
        base_exp_yield: 151,
        stats: Stats(
            Stat::new(95, 2),
            Stat::new(85, 0),
            Stat::new(85, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::WaterGun),
            (1, Move::MudSport),
            (9, Move::MudShot),
            (15, Move::Slam),
            (19, Move::MudBomb),
            (24, Move::Amnesia),
            (31, Move::Yawn),
            (36, Move::Earthquake),
            (41, Move::RainDance),
            (48, Move::Mist),
            (48, Move::Haze),
            (53, Move::MuddyWater),
        ]
    };
    pub const Espeon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Synchronize),
        hidden_ability: Some(Ability::MagicBounce),
        height: 9,
        weight: 265,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(130, 2),
            Stat::new(95, 0),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::Confusion),
            (13, Move::QuickAttack),
            (17, Move::Swift),
            (21, Move::Psybeam),
            (25, Move::FutureSight),
            (29, Move::PsychUp),
            (33, Move::MorningSun),
            (37, Move::Psychic),
            (41, Move::LastResort),
            (45, Move::PowerSwap),
        ]
    };
    pub const Umbreon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::One(Ability::Synchronize),
        hidden_ability: Some(Ability::InnerFocus),
        height: 10,
        weight: 270,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(65, 0),
            Stat::new(110, 0),
            Stat::new(60, 0),
            Stat::new(130, 2),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::Pursuit),
            (13, Move::QuickAttack),
            (17, Move::ConfuseRay),
            (21, Move::FeintAttack),
            (25, Move::Assurance),
            (29, Move::Screech),
            (33, Move::Moonlight),
            (37, Move::MeanLook),
            (41, Move::LastResort),
            (45, Move::GuardSwap),
        ]
    };
    pub const Murkrow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Flying),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::SuperLuck),
        hidden_ability: Some(Ability::Prankster),
        height: 5,
        weight: 21,
        base_exp_yield: 81,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(85, 0),
            Stat::new(42, 0),
            Stat::new(85, 0),
            Stat::new(42, 0),
            Stat::new(91, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (1, Move::Astonish),
            (5, Move::Pursuit),
            (11, Move::Haze),
            (15, Move::WingAttack),
            (21, Move::NightShade),
            (25, Move::Assurance),
            (31, Move::Taunt),
            (35, Move::FeintAttack),
            (41, Move::MeanLook),
            (45, Move::FoulPlay),
            (51, Move::Tailwind),
            (55, Move::SuckerPunch),
            (61, Move::Torment),
            (65, Move::Quash),
        ]
    };
    pub const Slowking: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::OwnTempo),
        hidden_ability: Some(Ability::Regenerator),
        height: 20,
        weight: 795,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(75, 0),
            Stat::new(80, 0),
            Stat::new(100, 0),
            Stat::new(110, 3),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Curse),
            (1, Move::HiddenPower),
            (1, Move::Yawn),
            (1, Move::PowerGem),
            (5, Move::Growl),
            (9, Move::WaterGun),
            (14, Move::Confusion),
            (19, Move::Disable),
            (23, Move::Headbutt),
            (28, Move::WaterPulse),
            (32, Move::ZenHeadbutt),
            (36, Move::NastyPlot),
            (41, Move::Swagger),
            (45, Move::Psychic),
            (49, Move::TrumpCard),
            (54, Move::PsychUp),
            (58, Move::HealPulse),
        ]
    };
    pub const Misdreavus: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 7,
        weight: 10,
        base_exp_yield: 87,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(85, 0),
            Stat::new(85, 1),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Psywave),
            (5, Move::Spite),
            (10, Move::Astonish),
            (14, Move::ConfuseRay),
            (19, Move::MeanLook),
            (23, Move::Hex),
            (28, Move::Psybeam),
            (32, Move::PainSplit),
            (37, Move::Payback),
            (41, Move::ShadowBall),
            (46, Move::PerishSong),
            (50, Move::Grudge),
            (55, Move::PowerGem),
        ]
    };
    pub const Unown: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 5,
        weight: 50,
        base_exp_yield: 118,
        stats: Stats(
            Stat::new(48, 0),
            Stat::new(72, 1),
            Stat::new(48, 0),
            Stat::new(72, 1),
            Stat::new(48, 0),
            Stat::new(48, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::HiddenPower),
        ]
    };
    pub const Wobbuffet: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::ShadowTag),
        hidden_ability: Some(Ability::Telepathy),
        height: 13,
        weight: 285,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(190, 2),
            Stat::new(33, 0),
            Stat::new(58, 0),
            Stat::new(33, 0),
            Stat::new(58, 0),
            Stat::new(33, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Counter),
            (1, Move::DestinyBond),
            (1, Move::Safeguard),
            (1, Move::MirrorCoat),
        ]
    };
    pub const Girafarig: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Psychic),
        ability: PokemonAbility::Two(Ability::InnerFocus, Ability::EarlyBird),
        hidden_ability: Some(Ability::SapSipper),
        height: 15,
        weight: 415,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(65, 0),
            Stat::new(90, 2),
            Stat::new(65, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Confusion),
            (1, Move::Astonish),
            (1, Move::PowerSwap),
            (1, Move::GuardSwap),
            (5, Move::OdorSleuth),
            (10, Move::Stomp),
            (14, Move::Agility),
            (19, Move::Psybeam),
            (23, Move::BatonPass),
            (28, Move::Assurance),
            (32, Move::DoubleHit),
            (37, Move::Psychic),
            (41, Move::ZenHeadbutt),
            (46, Move::Crunch),
        ]
    };
    pub const Pineco: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::Sturdy),
        hidden_ability: Some(Ability::Overcoat),
        height: 6,
        weight: 72,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(65, 0),
            Stat::new(90, 1),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Protect),
            (6, Move::SelfDestruct),
            (9, Move::BugBite),
            (12, Move::TakeDown),
            (17, Move::RapidSpin),
            (20, Move::Bide),
            (23, Move::NaturalGift),
            (28, Move::Spikes),
            (31, Move::Payback),
            (34, Move::Explosion),
            (39, Move::IronDefense),
            (42, Move::GyroBall),
            (45, Move::DoubleEdge),
        ]
    };
    pub const Forretress: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Steel),
        ability: PokemonAbility::One(Ability::Sturdy),
        hidden_ability: Some(Ability::Overcoat),
        height: 12,
        weight: 1258,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(90, 0),
            Stat::new(140, 2),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::SelfDestruct),
            (1, Move::Protect),
            (1, Move::ToxicSpikes),
            (1, Move::BugBite),
            (12, Move::TakeDown),
            (17, Move::RapidSpin),
            (20, Move::Bide),
            (23, Move::NaturalGift),
            (28, Move::Spikes),
            (31, Move::MirrorShot),
            (32, Move::Autotomize),
            (36, Move::Payback),
            (42, Move::Explosion),
            (46, Move::IronDefense),
            (50, Move::GyroBall),
            (56, Move::DoubleEdge),
            (60, Move::MagnetRise),
            (64, Move::ZapCannon),
            (70, Move::HeavySlam),
        ]
    };
    pub const Dunsparce: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::SereneGrace, Ability::RunAway),
        hidden_ability: Some(Ability::Rattled),
        height: 15,
        weight: 140,
        base_exp_yield: 145,
        stats: Stats(
            Stat::new(100, 1),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Rage),
            (1, Move::DefenseCurl),
            (4, Move::Rollout),
            (7, Move::Spite),
            (10, Move::Pursuit),
            (13, Move::Screech),
            (16, Move::Yawn),
            (19, Move::AncientPower),
            (22, Move::TakeDown),
            (25, Move::Roost),
            (28, Move::Glare),
            (31, Move::Dig),
            (34, Move::DoubleEdge),
            (37, Move::Coil),
            (40, Move::Endure),
            (43, Move::DrillRun),
            (46, Move::Endeavor),
            (49, Move::Flail),
        ]
    };
    pub const Gligar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Flying),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::SandVeil),
        hidden_ability: Some(Ability::Immunity),
        height: 11,
        weight: 648,
        base_exp_yield: 86,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(75, 0),
            Stat::new(105, 1),
            Stat::new(35, 0),
            Stat::new(65, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (4, Move::SandAttack),
            (7, Move::Harden),
            (10, Move::KnockOff),
            (13, Move::QuickAttack),
            (16, Move::FuryCutter),
            (19, Move::FeintAttack),
            (22, Move::Acrobatics),
            (27, Move::Slash),
            (30, Move::UTurn),
            (35, Move::Screech),
            (40, Move::XScissor),
            (45, Move::SkyUppercut),
            (50, Move::SwordsDance),
            (55, Move::Guillotine),
        ]
    };
    pub const Steelix: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Ground),
        ability: PokemonAbility::Two(Ability::RockHead, Ability::Sturdy),
        hidden_ability: Some(Ability::SheerForce),
        height: 92,
        weight: 4000,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(200, 2),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bind),
            (1, Move::Tackle),
            (1, Move::Harden),
            (1, Move::MudSport),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (4, Move::Curse),
            (7, Move::RockThrow),
            (10, Move::Rage),
            (13, Move::RockTomb),
            (16, Move::StealthRock),
            (19, Move::Autotomize),
            (22, Move::SmackDown),
            (25, Move::DragonBreath),
            (28, Move::Slam),
            (31, Move::Screech),
            (34, Move::RockSlide),
            (37, Move::Crunch),
            (40, Move::IronTail),
            (43, Move::Dig),
            (46, Move::StoneEdge),
            (49, Move::DoubleEdge),
            (52, Move::Sandstorm),
        ]
    };
    pub const Snubbull: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fairy),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::RunAway),
        hidden_ability: Some(Ability::Rattled),
        height: 6,
        weight: 78,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(80, 1),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::ScaryFace),
            (1, Move::Charm),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (7, Move::Bite),
            (13, Move::Lick),
            (19, Move::Headbutt),
            (25, Move::Roar),
            (31, Move::Rage),
            (37, Move::TakeDown),
            (43, Move::Payback),
            (49, Move::Crunch),
        ]
    };
    pub const Granbull: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fairy),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::QuickFeet),
        hidden_ability: Some(Ability::Rattled),
        height: 14,
        weight: 487,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(120, 2),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::ScaryFace),
            (1, Move::Charm),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (7, Move::Bite),
            (13, Move::Lick),
            (19, Move::Headbutt),
            (27, Move::Roar),
            (35, Move::Rage),
            (43, Move::TakeDown),
            (51, Move::Payback),
            (59, Move::Crunch),
            (67, Move::Outrage),
        ]
    };
    pub const Qwilfish: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::SwiftSwim),
        hidden_ability: Some(Ability::Intimidate),
        height: 5,
        weight: 39,
        base_exp_yield: 88,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(95, 1),
            Stat::new(85, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::PoisonSting),
            (1, Move::Spikes),
            (9, Move::Harden),
            (9, Move::Minimize),
            (13, Move::WaterGun),
            (17, Move::Rollout),
            (21, Move::ToxicSpikes),
            (25, Move::Stockpile),
            (25, Move::SpitUp),
            (29, Move::Revenge),
            (33, Move::Brine),
            (37, Move::PinMissile),
            (41, Move::TakeDown),
            (45, Move::AquaTail),
            (49, Move::PoisonJab),
            (53, Move::DestinyBond),
            (57, Move::HydroPump),
        ]
    };
    pub const Scizor: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Steel),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Technician),
        hidden_ability: Some(Ability::LightMetal),
        height: 18,
        weight: 1180,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(130, 2),
            Stat::new(100, 0),
            Stat::new(55, 0),
            Stat::new(80, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (1, Move::BulletPunch),
            (5, Move::FocusEnergy),
            (9, Move::Pursuit),
            (13, Move::FalseSwipe),
            (17, Move::Agility),
            (21, Move::MetalClaw),
            (25, Move::FuryCutter),
            (29, Move::Slash),
            (33, Move::RazorWind),
            (37, Move::IronDefense),
            (41, Move::XScissor),
            (45, Move::NightSlash),
            (49, Move::DoubleHit),
            (53, Move::IronHead),
            (57, Move::SwordsDance),
            (61, Move::Feint),
        ]
    };
    pub const Shuckle: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::Gluttony),
        hidden_ability: Some(Ability::Contrary),
        height: 6,
        weight: 205,
        base_exp_yield: 177,
        stats: Stats(
            Stat::new(20, 0),
            Stat::new(10, 0),
            Stat::new(230, 1),
            Stat::new(10, 0),
            Stat::new(230, 1),
            Stat::new(5, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Withdraw),
            (1, Move::Bide),
            (1, Move::Constrict),
            (1, Move::Rollout),
            (5, Move::Encore),
            (9, Move::Wrap),
            (12, Move::StruggleBug),
            (16, Move::Safeguard),
            (20, Move::Rest),
            (23, Move::RockThrow),
            (27, Move::GastroAcid),
            (31, Move::PowerTrick),
            (34, Move::ShellSmash),
            (38, Move::RockSlide),
            (42, Move::BugBite),
            (45, Move::GuardSplit),
            (45, Move::PowerSplit),
            (49, Move::StoneEdge),
        ]
    };
    pub const Heracross: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Fighting),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Guts),
        hidden_ability: Some(Ability::Moxie),
        height: 15,
        weight: 540,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(125, 2),
            Stat::new(75, 0),
            Stat::new(40, 0),
            Stat::new(95, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::HornAttack),
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Endure),
            (1, Move::NightSlash),
            (7, Move::FuryAttack),
            (10, Move::AerialAce),
            (16, Move::ChipAway),
            (19, Move::Counter),
            (25, Move::BrickBreak),
            (28, Move::TakeDown),
            (34, Move::CloseCombat),
            (37, Move::Feint),
            (43, Move::Reversal),
            (46, Move::Megahorn),
        ]
    };
    pub const Sneasel: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Ice),
        ability: PokemonAbility::Two(Ability::InnerFocus, Ability::KeenEye),
        hidden_ability: Some(Ability::Pickpocket),
        height: 9,
        weight: 280,
        base_exp_yield: 86,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(95, 0),
            Stat::new(55, 0),
            Stat::new(35, 0),
            Stat::new(75, 0),
            Stat::new(115, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Taunt),
            (8, Move::QuickAttack),
            (10, Move::FeintAttack),
            (14, Move::IcyWind),
            (16, Move::FurySwipes),
            (20, Move::Agility),
            (22, Move::MetalClaw),
            (25, Move::HoneClaws),
            (28, Move::BeatUp),
            (32, Move::Screech),
            (35, Move::Slash),
            (40, Move::Snatch),
            (44, Move::Punishment),
            (47, Move::IceShard),
        ]
    };
    pub const Teddiursa: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Pickup, Ability::QuickFeet),
        hidden_ability: Some(Ability::HoneyGather),
        height: 6,
        weight: 88,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(80, 1),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Lick),
            (1, Move::FakeTears),
            (1, Move::Covet),
            (8, Move::FurySwipes),
            (15, Move::FeintAttack),
            (22, Move::SweetScent),
            (29, Move::Slash),
            (36, Move::Charm),
            (43, Move::Rest),
            (43, Move::Snore),
            (50, Move::Thrash),
            (57, Move::Fling),
        ]
    };
    pub const Ursaring: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Guts, Ability::QuickFeet),
        hidden_ability: Some(Ability::Unnerve),
        height: 18,
        weight: 1258,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(130, 2),
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Lick),
            (1, Move::FakeTears),
            (1, Move::Covet),
            (8, Move::FurySwipes),
            (15, Move::FeintAttack),
            (22, Move::SweetScent),
            (29, Move::Slash),
            (38, Move::ScaryFace),
            (47, Move::Rest),
            (49, Move::Snore),
            (58, Move::Thrash),
            (67, Move::HammerArm),
        ]
    };
    pub const Slugma: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::MagmaArmor, Ability::FlameBody),
        hidden_ability: Some(Ability::WeakArmor),
        height: 7,
        weight: 350,
        base_exp_yield: 50,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(70, 1),
            Stat::new(40, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Smog),
            (1, Move::Yawn),
            (5, Move::Ember),
            (10, Move::RockThrow),
            (14, Move::Harden),
            (19, Move::Recover),
            (23, Move::FlameBurst),
            (28, Move::AncientPower),
            (32, Move::Amnesia),
            (37, Move::LavaPlume),
            (41, Move::RockSlide),
            (46, Move::BodySlam),
            (50, Move::Flamethrower),
            (55, Move::EarthPower),
        ]
    };
    pub const Magcargo: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Rock),
        ability: PokemonAbility::Two(Ability::MagmaArmor, Ability::FlameBody),
        hidden_ability: Some(Ability::WeakArmor),
        height: 8,
        weight: 550,
        base_exp_yield: 151,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(120, 2),
            Stat::new(90, 0),
            Stat::new(80, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (1, Move::RockThrow),
            (1, Move::Smog),
            (1, Move::Yawn),
            (14, Move::Harden),
            (19, Move::Recover),
            (23, Move::FlameBurst),
            (28, Move::AncientPower),
            (32, Move::Amnesia),
            (37, Move::LavaPlume),
            (38, Move::ShellSmash),
            (44, Move::RockSlide),
            (52, Move::BodySlam),
            (59, Move::Flamethrower),
            (67, Move::EarthPower),
        ]
    };
    pub const Swinub: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Ground),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::SnowCloak),
        hidden_ability: Some(Ability::ThickFat),
        height: 4,
        weight: 65,
        base_exp_yield: 50,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(50, 1),
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::OdorSleuth),
            (5, Move::MudSport),
            (8, Move::PowderSnow),
            (11, Move::MudSlap),
            (14, Move::Endure),
            (18, Move::MudBomb),
            (21, Move::IcyWind),
            (24, Move::IceShard),
            (28, Move::TakeDown),
            (35, Move::Mist),
            (37, Move::Earthquake),
            (40, Move::Flail),
            (44, Move::Blizzard),
            (48, Move::Amnesia),
        ]
    };
    pub const Piloswine: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Ground),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::SnowCloak),
        hidden_ability: Some(Ability::ThickFat),
        height: 11,
        weight: 558,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(100, 1),
            Stat::new(100, 1),
            Stat::new(80, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (1, Move::PowderSnow),
            (1, Move::AncientPower),
            (1, Move::MudSport),
            (1, Move::OdorSleuth),
            (11, Move::MudSlap),
            (14, Move::Endure),
            (18, Move::MudBomb),
            (21, Move::IcyWind),
            (24, Move::IceFang),
            (28, Move::TakeDown),
            (33, Move::FuryAttack),
            (37, Move::Mist),
            (41, Move::Thrash),
            (46, Move::Earthquake),
            (52, Move::Blizzard),
            (58, Move::Amnesia),
        ]
    };
    pub const Corsola: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Rock),
        ability: PokemonAbility::Two(Ability::Hustle, Ability::NaturalCure),
        hidden_ability: Some(Ability::Regenerator),
        height: 6,
        weight: 50,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(95, 1),
            Stat::new(65, 0),
            Stat::new(95, 1),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Harden),
            (8, Move::Bubble),
            (10, Move::Recover),
            (13, Move::Refresh),
            (17, Move::BubbleBeam),
            (20, Move::AncientPower),
            (23, Move::LuckyChant),
            (27, Move::SpikeCannon),
            (29, Move::IronDefense),
            (31, Move::RockBlast),
            (35, Move::Endure),
            (38, Move::AquaRing),
            (41, Move::PowerGem),
            (45, Move::MirrorCoat),
            (47, Move::EarthPower),
            (52, Move::Flail),
        ]
    };
    pub const Remoraid: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::Hustle, Ability::Sniper),
        hidden_ability: Some(Ability::Moody),
        height: 6,
        weight: 120,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(65, 0),
            Stat::new(35, 0),
            Stat::new(65, 1),
            Stat::new(35, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (6, Move::LockOn),
            (10, Move::Psybeam),
            (14, Move::AuroraBeam),
            (18, Move::BubbleBeam),
            (22, Move::FocusEnergy),
            (26, Move::WaterPulse),
            (30, Move::SignalBeam),
            (34, Move::IceBeam),
            (38, Move::BulletSeed),
            (42, Move::HydroPump),
            (46, Move::HyperBeam),
            (50, Move::Soak),
        ]
    };
    pub const Octillery: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SuctionCups, Ability::Sniper),
        hidden_ability: Some(Ability::Moody),
        height: 9,
        weight: 285,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(105, 1),
            Stat::new(75, 0),
            Stat::new(105, 1),
            Stat::new(75, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (1, Move::Psybeam),
            (1, Move::AuroraBeam),
            (1, Move::Constrict),
            (1, Move::RockBlast),
            (1, Move::GunkShot),
            (18, Move::BubbleBeam),
            (22, Move::FocusEnergy),
            (25, Move::Octazooka),
            (28, Move::WringOut),
            (34, Move::SignalBeam),
            (40, Move::IceBeam),
            (46, Move::BulletSeed),
            (52, Move::HydroPump),
            (58, Move::HyperBeam),
            (64, Move::Soak),
        ]
    };
    pub const Delibird: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Flying),
        ability: PokemonAbility::Two(Ability::VitalSpirit, Ability::Hustle),
        hidden_ability: Some(Ability::Insomnia),
        height: 9,
        weight: 160,
        base_exp_yield: 116,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(65, 0),
            Stat::new(45, 0),
            Stat::new(75, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Present),
        ]
    };
    pub const Mantine: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::WaterAbsorb),
        hidden_ability: Some(Ability::WaterVeil),
        height: 21,
        weight: 2200,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(40, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(140, 2),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Supersonic),
            (1, Move::Psybeam),
            (1, Move::BubbleBeam),
            (1, Move::Bubble),
            (1, Move::SignalBeam),
            (1, Move::BulletSeed),
            (11, Move::ConfuseRay),
            (14, Move::WingAttack),
            (16, Move::Headbutt),
            (19, Move::WaterPulse),
            (23, Move::WideGuard),
            (27, Move::TakeDown),
            (32, Move::Agility),
            (36, Move::AirSlash),
            (39, Move::AquaRing),
            (46, Move::Bounce),
            (49, Move::HydroPump),
        ]
    };
    pub const Skarmory: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::Sturdy),
        hidden_ability: Some(Ability::WeakArmor),
        height: 17,
        weight: 505,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(80, 0),
            Stat::new(140, 2),
            Stat::new(40, 0),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Peck),
            (6, Move::SandAttack),
            (9, Move::Swift),
            (12, Move::Agility),
            (17, Move::FuryAttack),
            (20, Move::Feint),
            (23, Move::AirCutter),
            (28, Move::Spikes),
            (31, Move::MetalSound),
            (34, Move::SteelWing),
            (39, Move::Autotomize),
            (42, Move::AirSlash),
            (45, Move::Slash),
            (50, Move::NightSlash),
        ]
    };
    pub const Houndour: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Fire),
        ability: PokemonAbility::Two(Ability::EarlyBird, Ability::FlashFire),
        hidden_ability: Some(Ability::Unnerve),
        height: 6,
        weight: 108,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(60, 0),
            Stat::new(30, 0),
            Stat::new(80, 1),
            Stat::new(50, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Ember),
            (4, Move::Howl),
            (8, Move::Smog),
            (13, Move::Roar),
            (16, Move::Bite),
            (20, Move::OdorSleuth),
            (25, Move::BeatUp),
            (28, Move::FireFang),
            (32, Move::FeintAttack),
            (37, Move::Embargo),
            (40, Move::FoulPlay),
            (44, Move::Flamethrower),
            (49, Move::Crunch),
            (52, Move::NastyPlot),
            (56, Move::Inferno),
        ]
    };
    pub const Houndoom: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Fire),
        ability: PokemonAbility::Two(Ability::EarlyBird, Ability::FlashFire),
        hidden_ability: Some(Ability::Unnerve),
        height: 14,
        weight: 350,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(90, 0),
            Stat::new(50, 0),
            Stat::new(110, 2),
            Stat::new(80, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Ember),
            (1, Move::Smog),
            (1, Move::Howl),
            (1, Move::ThunderFang),
            (13, Move::Roar),
            (16, Move::Bite),
            (20, Move::OdorSleuth),
            (26, Move::BeatUp),
            (30, Move::FireFang),
            (35, Move::FeintAttack),
            (41, Move::Embargo),
            (45, Move::FoulPlay),
            (50, Move::Flamethrower),
            (56, Move::Crunch),
            (60, Move::NastyPlot),
            (65, Move::Inferno),
        ]
    };
    pub const Kingdra: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Dragon),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::Sniper),
        hidden_ability: Some(Ability::Damp),
        height: 18,
        weight: 1520,
        base_exp_yield: 243,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(95, 1),
            Stat::new(95, 0),
            Stat::new(95, 1),
            Stat::new(95, 1),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::WaterGun),
            (1, Move::Smokescreen),
            (1, Move::Bubble),
            (1, Move::Yawn),
            (14, Move::FocusEnergy),
            (18, Move::BubbleBeam),
            (23, Move::Agility),
            (26, Move::Twister),
            (30, Move::Brine),
            (40, Move::HydroPump),
            (48, Move::DragonDance),
            (57, Move::DragonPulse),
        ]
    };
    pub const Phanpy: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::Pickup),
        hidden_ability: Some(Ability::SandVeil),
        height: 5,
        weight: 335,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(90, 1),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::DefenseCurl),
            (1, Move::OdorSleuth),
            (6, Move::Flail),
            (10, Move::TakeDown),
            (15, Move::Rollout),
            (19, Move::NaturalGift),
            (24, Move::Slam),
            (28, Move::Endure),
            (33, Move::Charm),
            (37, Move::LastResort),
            (42, Move::DoubleEdge),
        ]
    };
    pub const Donphan: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::Sturdy),
        hidden_ability: Some(Ability::SandVeil),
        height: 11,
        weight: 1200,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(120, 1),
            Stat::new(120, 1),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::HornAttack),
            (1, Move::Growl),
            (1, Move::DefenseCurl),
            (1, Move::ThunderFang),
            (1, Move::FireFang),
            (1, Move::Bulldoze),
            (6, Move::RapidSpin),
            (10, Move::KnockOff),
            (15, Move::Rollout),
            (19, Move::Magnitude),
            (24, Move::Slam),
            (25, Move::FuryAttack),
            (31, Move::Assurance),
            (39, Move::ScaryFace),
            (46, Move::Earthquake),
            (54, Move::GigaImpact),
        ]
    };
    pub const Porygon2: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Trace, Ability::Download),
        hidden_ability: Some(Ability::Analytic),
        height: 6,
        weight: 325,
        base_exp_yield: 180,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(80, 0),
            Stat::new(90, 0),
            Stat::new(105, 2),
            Stat::new(95, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::DefenseCurl),
            (1, Move::Conversion),
            (1, Move::Conversion2),
            (7, Move::Psybeam),
            (12, Move::Agility),
            (18, Move::Recover),
            (23, Move::MagnetRise),
            (29, Move::SignalBeam),
            (34, Move::Recycle),
            (40, Move::Discharge),
            (45, Move::LockOn),
            (51, Move::TriAttack),
            (56, Move::MagicCoat),
            (62, Move::ZapCannon),
            (67, Move::HyperBeam),
        ]
    };
    pub const Stantler: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::Frisk),
        hidden_ability: Some(Ability::SapSipper),
        height: 14,
        weight: 712,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(73, 0),
            Stat::new(95, 1),
            Stat::new(62, 0),
            Stat::new(85, 0),
            Stat::new(65, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::Leer),
            (7, Move::Astonish),
            (10, Move::Hypnosis),
            (13, Move::Stomp),
            (16, Move::SandAttack),
            (21, Move::TakeDown),
            (23, Move::ConfuseRay),
            (27, Move::CalmMind),
            (33, Move::RolePlay),
            (38, Move::ZenHeadbutt),
            (43, Move::JumpKick),
            (49, Move::Imprison),
            (53, Move::Captivate),
            (55, Move::MeFirst),
        ]
    };
    pub const Smeargle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::OwnTempo, Ability::Technician),
        hidden_ability: Some(Ability::Moody),
        height: 12,
        weight: 580,
        base_exp_yield: 88,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(20, 0),
            Stat::new(35, 0),
            Stat::new(20, 0),
            Stat::new(45, 0),
            Stat::new(75, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Sketch),
        ]
    };
    pub const Tyrogue: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::Steadfast),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 7,
        weight: 210,
        base_exp_yield: 42,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(35, 1),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Foresight),
            (1, Move::FakeOut),
            (1, Move::HelpingHand),
        ]
    };
    pub const Hitmontop: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::Technician),
        hidden_ability: Some(Ability::Steadfast),
        height: 14,
        weight: 480,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(95, 0),
            Stat::new(95, 0),
            Stat::new(35, 0),
            Stat::new(110, 2),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::RollingKick),
            (1, Move::Revenge),
            (6, Move::FocusEnergy),
            (10, Move::Pursuit),
            (15, Move::QuickAttack),
            (19, Move::TripleKick),
            (24, Move::RapidSpin),
            (28, Move::Counter),
            (33, Move::Feint),
            (37, Move::Agility),
            (42, Move::GyroBall),
            (46, Move::WideGuard),
            (46, Move::QuickGuard),
            (51, Move::Detect),
            (55, Move::CloseCombat),
            (60, Move::Endeavor),
        ]
    };
    pub const Smoochum: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::Forewarn),
        hidden_ability: Some(Ability::Hydration),
        height: 4,
        weight: 60,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(30, 0),
            Stat::new(15, 0),
            Stat::new(85, 1),
            Stat::new(65, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (5, Move::Lick),
            (8, Move::SweetKiss),
            (11, Move::PowderSnow),
            (15, Move::Confusion),
            (18, Move::Sing),
            (21, Move::HeartStamp),
            (25, Move::MeanLook),
            (28, Move::FakeTears),
            (31, Move::LuckyChant),
            (35, Move::Avalanche),
            (38, Move::Psychic),
            (41, Move::Copycat),
            (45, Move::PerishSong),
            (48, Move::Blizzard),
        ]
    };
    pub const Elekid: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 6,
        weight: 235,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(63, 0),
            Stat::new(37, 0),
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (5, Move::ThunderShock),
            (8, Move::LowKick),
            (12, Move::Swift),
            (15, Move::ShockWave),
            (19, Move::ThunderWave),
            (22, Move::ElectroBall),
            (26, Move::LightScreen),
            (29, Move::ThunderPunch),
            (33, Move::Discharge),
            (36, Move::Screech),
            (40, Move::Thunderbolt),
            (43, Move::Thunder),
        ]
    };
    pub const Magby: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::FlameBody),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 7,
        weight: 214,
        base_exp_yield: 73,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(75, 0),
            Stat::new(37, 0),
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(83, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Smog),
            (5, Move::Ember),
            (8, Move::Smokescreen),
            (12, Move::FeintAttack),
            (15, Move::FireSpin),
            (19, Move::ClearSmog),
            (22, Move::FlameBurst),
            (26, Move::ConfuseRay),
            (29, Move::FirePunch),
            (33, Move::LavaPlume),
            (36, Move::SunnyDay),
            (40, Move::Flamethrower),
            (43, Move::FireBlast),
        ]
    };
    pub const Miltank: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::Scrappy),
        hidden_ability: Some(Ability::SapSipper),
        height: 12,
        weight: 755,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(80, 0),
            Stat::new(105, 2),
            Stat::new(40, 0),
            Stat::new(70, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::Growl),
            (5, Move::DefenseCurl),
            (8, Move::Stomp),
            (11, Move::MilkDrink),
            (15, Move::Bide),
            (19, Move::Rollout),
            (24, Move::BodySlam),
            (29, Move::ZenHeadbutt),
            (35, Move::Captivate),
            (41, Move::GyroBall),
            (48, Move::HealBell),
            (55, Move::WakeUpSlap),
        ]
    };
    pub const Blissey: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::NaturalCure, Ability::SereneGrace),
        hidden_ability: Some(Ability::Healer),
        height: 15,
        weight: 468,
        base_exp_yield: 608,
        stats: Stats(
            Stat::new(255, 3),
            Stat::new(10, 0),
            Stat::new(10, 0),
            Stat::new(75, 0),
            Stat::new(135, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Growl),
            (1, Move::DefenseCurl),
            (5, Move::TailWhip),
            (9, Move::Refresh),
            (12, Move::DoubleSlap),
            (16, Move::SoftBoiled),
            (20, Move::Bestow),
            (23, Move::Minimize),
            (27, Move::TakeDown),
            (31, Move::Sing),
            (34, Move::Fling),
            (38, Move::HealPulse),
            (42, Move::EggBomb),
            (46, Move::LightScreen),
            (50, Move::HealingWish),
            (54, Move::DoubleEdge),
        ]
    };
    pub const Raikou: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::InnerFocus),
        height: 19,
        weight: 1780,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(85, 0),
            Stat::new(75, 0),
            Stat::new(115, 1),
            Stat::new(100, 0),
            Stat::new(115, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (8, Move::ThunderShock),
            (15, Move::Roar),
            (22, Move::QuickAttack),
            (29, Move::Spark),
            (36, Move::Reflect),
            (43, Move::Crunch),
            (50, Move::ThunderFang),
            (57, Move::Discharge),
            (64, Move::Extrasensory),
            (71, Move::RainDance),
            (78, Move::CalmMind),
            (85, Move::Thunder),
        ]
    };
    pub const Entei: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::InnerFocus),
        height: 21,
        weight: 1980,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(115, 1),
            Stat::new(115, 2),
            Stat::new(85, 0),
            Stat::new(90, 0),
            Stat::new(75, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (8, Move::Ember),
            (15, Move::Roar),
            (22, Move::FireSpin),
            (29, Move::Stomp),
            (36, Move::Flamethrower),
            (43, Move::Swagger),
            (50, Move::FireFang),
            (57, Move::LavaPlume),
            (64, Move::Extrasensory),
            (71, Move::FireBlast),
            (78, Move::CalmMind),
            (85, Move::Eruption),
        ]
    };
    pub const Suicune: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::InnerFocus),
        height: 20,
        weight: 1870,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(75, 0),
            Stat::new(115, 1),
            Stat::new(90, 0),
            Stat::new(115, 2),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (8, Move::BubbleBeam),
            (15, Move::RainDance),
            (22, Move::Gust),
            (29, Move::AuroraBeam),
            (36, Move::Mist),
            (43, Move::MirrorCoat),
            (50, Move::IceFang),
            (57, Move::Tailwind),
            (64, Move::Extrasensory),
            (71, Move::HydroPump),
            (78, Move::CalmMind),
            (85, Move::Blizzard),
        ]
    };
    pub const Larvitar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Ground),
        ability: PokemonAbility::One(Ability::Guts),
        hidden_ability: Some(Ability::SandVeil),
        height: 6,
        weight: 720,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(64, 1),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(41, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (5, Move::Sandstorm),
            (10, Move::Screech),
            (14, Move::ChipAway),
            (19, Move::RockSlide),
            (23, Move::ScaryFace),
            (28, Move::Thrash),
            (32, Move::DarkPulse),
            (37, Move::Payback),
            (41, Move::Crunch),
            (46, Move::Earthquake),
            (50, Move::StoneEdge),
            (55, Move::HyperBeam),
        ]
    };
    pub const Pupitar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Ground),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: None,
        height: 12,
        weight: 1520,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(84, 2),
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(51, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Screech),
            (1, Move::Sandstorm),
            (14, Move::ChipAway),
            (19, Move::RockSlide),
            (23, Move::ScaryFace),
            (28, Move::Thrash),
            (34, Move::DarkPulse),
            (41, Move::Payback),
            (47, Move::Crunch),
            (54, Move::Earthquake),
            (60, Move::StoneEdge),
            (67, Move::HyperBeam),
        ]
    };
    pub const Tyranitar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Dark),
        ability: PokemonAbility::One(Ability::SandStream),
        hidden_ability: Some(Ability::Unnerve),
        height: 20,
        weight: 2020,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(134, 3),
            Stat::new(110, 0),
            Stat::new(95, 0),
            Stat::new(100, 0),
            Stat::new(61, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Screech),
            (1, Move::Sandstorm),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (14, Move::ChipAway),
            (19, Move::RockSlide),
            (23, Move::ScaryFace),
            (28, Move::Thrash),
            (34, Move::DarkPulse),
            (41, Move::Payback),
            (47, Move::Crunch),
            (54, Move::Earthquake),
            (63, Move::StoneEdge),
            (73, Move::HyperBeam),
            (82, Move::GigaImpact),
        ]
    };
    pub const Lugia: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Flying),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Multiscale),
        height: 52,
        weight: 2160,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(106, 0),
            Stat::new(90, 0),
            Stat::new(130, 0),
            Stat::new(90, 0),
            Stat::new(154, 3),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Whirlwind),
            (1, Move::WeatherBall),
            (9, Move::Gust),
            (15, Move::DragonRush),
            (23, Move::Extrasensory),
            (29, Move::RainDance),
            (37, Move::HydroPump),
            (43, Move::Aeroblast),
            (50, Move::Punishment),
            (57, Move::AncientPower),
            (65, Move::Safeguard),
            (71, Move::Recover),
            (79, Move::FutureSight),
            (85, Move::NaturalGift),
            (93, Move::CalmMind),
            (99, Move::SkyAttack),
        ]
    };
    pub const HoOh: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Flying),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Regenerator),
        height: 38,
        weight: 1990,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(106, 0),
            Stat::new(130, 0),
            Stat::new(90, 0),
            Stat::new(110, 0),
            Stat::new(154, 3),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Whirlwind),
            (1, Move::WeatherBall),
            (9, Move::Gust),
            (15, Move::BraveBird),
            (23, Move::Extrasensory),
            (29, Move::SunnyDay),
            (37, Move::FireBlast),
            (43, Move::SacredFire),
            (50, Move::Punishment),
            (57, Move::AncientPower),
            (65, Move::Safeguard),
            (71, Move::Recover),
            (79, Move::FutureSight),
            (85, Move::NaturalGift),
            (93, Move::CalmMind),
            (99, Move::SkyAttack),
        ]
    };
    pub const Celebi: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Grass),
        ability: PokemonAbility::One(Ability::NaturalCure),
        hidden_ability: None,
        height: 6,
        weight: 50,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::LeechSeed),
            (1, Move::Confusion),
            (1, Move::Recover),
            (1, Move::HealBell),
            (10, Move::Safeguard),
            (19, Move::MagicalLeaf),
            (28, Move::AncientPower),
            (37, Move::BatonPass),
            (46, Move::NaturalGift),
            (55, Move::HealBlock),
            (64, Move::FutureSight),
            (73, Move::HealingWish),
            (82, Move::LeafStorm),
            (91, Move::PerishSong),
        ]
    };
    pub const Treecko: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Unburden),
        height: 5,
        weight: 50,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(70, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Leer),
            (6, Move::Absorb),
            (11, Move::QuickAttack),
            (16, Move::Pursuit),
            (21, Move::Screech),
            (26, Move::MegaDrain),
            (31, Move::Agility),
            (36, Move::Slam),
            (41, Move::Detect),
            (46, Move::GigaDrain),
            (51, Move::EnergyBall),
        ]
    };
    pub const Grovyle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Unburden),
        height: 9,
        weight: 216,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(65, 0),
            Stat::new(45, 0),
            Stat::new(85, 0),
            Stat::new(65, 0),
            Stat::new(95, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Leer),
            (1, Move::Absorb),
            (1, Move::QuickAttack),
            (16, Move::FuryCutter),
            (17, Move::Pursuit),
            (23, Move::Screech),
            (29, Move::LeafBlade),
            (35, Move::Agility),
            (41, Move::Slam),
            (47, Move::Detect),
            (53, Move::FalseSwipe),
            (59, Move::LeafStorm),
        ]
    };
    pub const Sceptile: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Unburden),
        height: 17,
        weight: 522,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(85, 0),
            Stat::new(65, 0),
            Stat::new(105, 0),
            Stat::new(85, 0),
            Stat::new(120, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Leer),
            (1, Move::Absorb),
            (1, Move::QuickAttack),
            (1, Move::NightSlash),
            (16, Move::XScissor),
            (17, Move::Pursuit),
            (23, Move::Screech),
            (29, Move::LeafBlade),
            (35, Move::Agility),
            (43, Move::Slam),
            (51, Move::Detect),
            (59, Move::FalseSwipe),
            (67, Move::LeafStorm),
        ]
    };
    pub const Torchic: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 4,
        weight: 25,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(70, 1),
            Stat::new(50, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (7, Move::FocusEnergy),
            (10, Move::Ember),
            (16, Move::Peck),
            (19, Move::SandAttack),
            (25, Move::FireSpin),
            (28, Move::QuickAttack),
            (34, Move::Slash),
            (37, Move::MirrorMove),
            (43, Move::Flamethrower),
        ]
    };
    pub const Combusken: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Fighting),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 9,
        weight: 195,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(85, 1),
            Stat::new(60, 0),
            Stat::new(85, 1),
            Stat::new(60, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (1, Move::Ember),
            (1, Move::FocusEnergy),
            (16, Move::DoubleKick),
            (17, Move::Peck),
            (21, Move::SandAttack),
            (28, Move::BulkUp),
            (32, Move::QuickAttack),
            (39, Move::Slash),
            (43, Move::MirrorMove),
            (50, Move::SkyUppercut),
            (54, Move::FlareBlitz),
        ]
    };
    pub const Blaziken: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Fighting),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 19,
        weight: 520,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(120, 3),
            Stat::new(70, 0),
            Stat::new(110, 0),
            Stat::new(70, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::Scratch),
            (1, Move::Growl),
            (1, Move::Ember),
            (1, Move::FocusEnergy),
            (1, Move::HighJumpKick),
            (16, Move::DoubleKick),
            (17, Move::Peck),
            (21, Move::SandAttack),
            (28, Move::BulkUp),
            (32, Move::QuickAttack),
            (36, Move::BlazeKick),
            (42, Move::Slash),
            (49, Move::BraveBird),
            (59, Move::SkyUppercut),
            (66, Move::FlareBlitz),
        ]
    };
    pub const Mudkip: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::Damp),
        height: 4,
        weight: 76,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(70, 1),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (6, Move::MudSlap),
            (10, Move::WaterGun),
            (15, Move::Bide),
            (19, Move::Foresight),
            (24, Move::MudSport),
            (28, Move::TakeDown),
            (33, Move::Whirlpool),
            (37, Move::Protect),
            (42, Move::HydroPump),
            (46, Move::Endeavor),
        ]
    };
    pub const Marshtomp: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::Damp),
        height: 7,
        weight: 280,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(85, 2),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::MudSlap),
            (15, Move::Bide),
            (16, Move::MudShot),
            (20, Move::Foresight),
            (25, Move::MudBomb),
            (31, Move::TakeDown),
            (37, Move::MuddyWater),
            (42, Move::Protect),
            (46, Move::Earthquake),
            (53, Move::Endeavor),
        ]
    };
    pub const Swampert: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::Damp),
        height: 15,
        weight: 819,
        base_exp_yield: 241,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(110, 3),
            Stat::new(90, 0),
            Stat::new(85, 0),
            Stat::new(90, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::MudSlap),
            (15, Move::Bide),
            (16, Move::MudShot),
            (20, Move::Foresight),
            (25, Move::MudBomb),
            (31, Move::TakeDown),
            (39, Move::MuddyWater),
            (46, Move::Protect),
            (52, Move::Earthquake),
            (61, Move::Endeavor),
            (69, Move::HammerArm),
        ]
    };
    pub const Poochyena: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::QuickFeet),
        hidden_ability: Some(Ability::Rattled),
        height: 5,
        weight: 136,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(55, 1),
            Stat::new(35, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::Howl),
            (9, Move::SandAttack),
            (13, Move::Bite),
            (17, Move::OdorSleuth),
            (21, Move::Roar),
            (25, Move::Swagger),
            (29, Move::Assurance),
            (33, Move::ScaryFace),
            (37, Move::Taunt),
            (41, Move::Embargo),
            (45, Move::TakeDown),
            (49, Move::SuckerPunch),
            (53, Move::Crunch),
        ]
    };
    pub const Mightyena: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::QuickFeet),
        hidden_ability: Some(Ability::Moxie),
        height: 10,
        weight: 370,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(90, 2),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (1, Move::Bite),
            (1, Move::Howl),
            (17, Move::OdorSleuth),
            (22, Move::Roar),
            (27, Move::Swagger),
            (32, Move::Assurance),
            (37, Move::ScaryFace),
            (42, Move::Taunt),
            (47, Move::Embargo),
            (52, Move::TakeDown),
            (57, Move::Thief),
            (62, Move::SuckerPunch),
        ]
    };
    pub const Zigzagoon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Pickup, Ability::Gluttony),
        hidden_ability: Some(Ability::QuickFeet),
        height: 4,
        weight: 175,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(38, 0),
            Stat::new(30, 0),
            Stat::new(41, 0),
            Stat::new(30, 0),
            Stat::new(41, 0),
            Stat::new(60, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (5, Move::TailWhip),
            (9, Move::Headbutt),
            (13, Move::SandAttack),
            (17, Move::OdorSleuth),
            (21, Move::MudSport),
            (25, Move::PinMissile),
            (29, Move::Covet),
            (33, Move::Bestow),
            (37, Move::Flail),
            (41, Move::Rest),
            (45, Move::BellyDrum),
            (49, Move::Fling),
        ]
    };
    pub const Linoone: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Pickup, Ability::Gluttony),
        hidden_ability: Some(Ability::QuickFeet),
        height: 5,
        weight: 325,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(78, 0),
            Stat::new(70, 0),
            Stat::new(61, 0),
            Stat::new(50, 0),
            Stat::new(61, 0),
            Stat::new(100, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Growl),
            (1, Move::Switcheroo),
            (13, Move::SandAttack),
            (17, Move::OdorSleuth),
            (23, Move::MudSport),
            (29, Move::FurySwipes),
            (35, Move::Covet),
            (41, Move::Bestow),
            (47, Move::Slash),
            (53, Move::Rest),
            (59, Move::BellyDrum),
            (65, Move::Fling),
        ]
    };
    pub const Wurmple: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShieldDust),
        hidden_ability: Some(Ability::RunAway),
        height: 3,
        weight: 36,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(45, 1),
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(20, 0),
            Stat::new(30, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::StringShot),
            (5, Move::PoisonSting),
            (15, Move::BugBite),
        ]
    };
    pub const Silcoon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: None,
        height: 6,
        weight: 100,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(35, 0),
            Stat::new(55, 2),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
        ]
    };
    pub const Beautifly: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::Swarm),
        hidden_ability: Some(Ability::Rivalry),
        height: 10,
        weight: 284,
        base_exp_yield: 178,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(50, 0),
            Stat::new(100, 3),
            Stat::new(50, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (13, Move::Gust),
            (17, Move::StunSpore),
            (20, Move::MorningSun),
            (24, Move::MegaDrain),
            (27, Move::Whirlwind),
            (31, Move::Attract),
            (34, Move::SilverWind),
            (38, Move::GigaDrain),
            (41, Move::BugBuzz),
            (45, Move::QuiverDance),
        ]
    };
    pub const Cascoon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: None,
        height: 7,
        weight: 115,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(35, 0),
            Stat::new(55, 2),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
        ]
    };
    pub const Dustox: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::One(Ability::ShieldDust),
        hidden_ability: Some(Ability::CompoundEyes),
        height: 12,
        weight: 316,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(70, 0),
            Stat::new(50, 0),
            Stat::new(90, 3),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (13, Move::Gust),
            (17, Move::Protect),
            (20, Move::Moonlight),
            (24, Move::Psybeam),
            (27, Move::Whirlwind),
            (31, Move::LightScreen),
            (34, Move::SilverWind),
            (38, Move::Toxic),
            (41, Move::BugBuzz),
            (45, Move::QuiverDance),
        ]
    };
    pub const Lotad: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Grass),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::RainDish),
        hidden_ability: Some(Ability::OwnTempo),
        height: 5,
        weight: 26,
        base_exp_yield: 44,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(50, 1),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Astonish),
            (3, Move::Growl),
            (5, Move::Absorb),
            (7, Move::NaturePower),
            (11, Move::Mist),
            (15, Move::NaturalGift),
            (19, Move::MegaDrain),
            (25, Move::BubbleBeam),
            (31, Move::ZenHeadbutt),
            (37, Move::RainDance),
            (45, Move::EnergyBall),
        ]
    };
    pub const Lombre: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Grass),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::RainDish),
        hidden_ability: Some(Ability::OwnTempo),
        height: 12,
        weight: 325,
        base_exp_yield: 119,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(60, 0),
            Stat::new(70, 2),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Astonish),
            (3, Move::Growl),
            (5, Move::Absorb),
            (7, Move::NaturePower),
            (11, Move::FakeOut),
            (15, Move::FurySwipes),
            (19, Move::WaterSport),
            (25, Move::BubbleBeam),
            (31, Move::ZenHeadbutt),
            (37, Move::Uproar),
            (45, Move::HydroPump),
        ]
    };
    pub const Ludicolo: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Grass),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::RainDish),
        hidden_ability: Some(Ability::OwnTempo),
        height: 15,
        weight: 550,
        base_exp_yield: 216,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(90, 0),
            Stat::new(100, 3),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::MegaDrain),
            (1, Move::NaturePower),
            (1, Move::Astonish),
        ]
    };
    pub const Seedot: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::EarlyBird),
        hidden_ability: Some(Ability::Pickpocket),
        height: 5,
        weight: 40,
        base_exp_yield: 44,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(50, 1),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bide),
            (3, Move::Harden),
            (7, Move::Growth),
            (13, Move::NaturePower),
            (21, Move::Synthesis),
            (31, Move::SunnyDay),
            (43, Move::Explosion),
        ]
    };
    pub const Nuzleaf: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Dark),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::EarlyBird),
        hidden_ability: Some(Ability::Pickpocket),
        height: 10,
        weight: 280,
        base_exp_yield: 119,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(70, 2),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::RazorLeaf),
            (3, Move::Harden),
            (7, Move::Growth),
            (13, Move::NaturePower),
            (19, Move::FakeOut),
            (25, Move::Torment),
            (31, Move::FeintAttack),
            (37, Move::RazorWind),
            (43, Move::Swagger),
            (49, Move::Extrasensory),
        ]
    };
    pub const Shiftry: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Dark),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::EarlyBird),
        hidden_ability: Some(Ability::Pickpocket),
        height: 13,
        weight: 596,
        base_exp_yield: 216,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(100, 3),
            Stat::new(60, 0),
            Stat::new(90, 0),
            Stat::new(60, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Whirlwind),
            (1, Move::RazorLeaf),
            (1, Move::FeintAttack),
            (1, Move::NastyPlot),
            (19, Move::LeafTornado),
            (49, Move::LeafStorm),
        ]
    };
    pub const Taillow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::Guts),
        hidden_ability: Some(Ability::Scrappy),
        height: 3,
        weight: 23,
        base_exp_yield: 54,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Peck),
            (4, Move::FocusEnergy),
            (8, Move::QuickAttack),
            (13, Move::WingAttack),
            (19, Move::DoubleTeam),
            (26, Move::Endeavor),
            (34, Move::AerialAce),
            (43, Move::Agility),
            (53, Move::AirSlash),
        ]
    };
    pub const Swellow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::Guts),
        hidden_ability: Some(Ability::Scrappy),
        height: 7,
        weight: 198,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(85, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(50, 0),
            Stat::new(125, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Peck),
            (1, Move::QuickAttack),
            (1, Move::FocusEnergy),
            (1, Move::Pluck),
            (13, Move::WingAttack),
            (19, Move::DoubleTeam),
            (28, Move::Endeavor),
            (38, Move::AerialAce),
            (49, Move::Agility),
            (61, Move::AirSlash),
        ]
    };
    pub const Wingull: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::Hydration),
        hidden_ability: Some(Ability::RainDish),
        height: 6,
        weight: 95,
        base_exp_yield: 54,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(55, 0),
            Stat::new(30, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::WaterGun),
            (6, Move::Supersonic),
            (9, Move::WingAttack),
            (14, Move::Mist),
            (17, Move::WaterPulse),
            (22, Move::QuickAttack),
            (26, Move::Roost),
            (30, Move::Pursuit),
            (33, Move::AirCutter),
            (38, Move::Agility),
            (42, Move::AerialAce),
            (46, Move::AirSlash),
            (49, Move::Hurricane),
        ]
    };
    pub const Pelipper: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::Drizzle),
        hidden_ability: Some(Ability::RainDish),
        height: 12,
        weight: 280,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(100, 2),
            Stat::new(95, 0),
            Stat::new(70, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::WaterSport),
            (1, Move::Soak),
            (6, Move::Supersonic),
            (14, Move::Mist),
            (17, Move::WaterPulse),
            (22, Move::Payback),
            (25, Move::Protect),
            (28, Move::Roost),
            (34, Move::Brine),
            (39, Move::Stockpile),
            (39, Move::SpitUp),
            (39, Move::Swallow),
            (46, Move::Fling),
            (52, Move::Tailwind),
            (58, Move::HydroPump),
            (63, Move::Hurricane),
        ]
    };
    pub const Ralts: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::Trace),
        hidden_ability: Some(Ability::Telepathy),
        height: 4,
        weight: 66,
        base_exp_yield: 40,
        stats: Stats(
            Stat::new(28, 0),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(45, 1),
            Stat::new(35, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (6, Move::Confusion),
            (10, Move::DoubleTeam),
            (12, Move::Teleport),
            (17, Move::LuckyChant),
            (21, Move::MagicalLeaf),
            (23, Move::HealPulse),
            (28, Move::CalmMind),
            (32, Move::Psychic),
            (34, Move::Imprison),
            (39, Move::FutureSight),
            (43, Move::Charm),
            (45, Move::Hypnosis),
            (50, Move::DreamEater),
            (54, Move::StoredPower),
        ]
    };
    pub const Kirlia: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::Trace),
        hidden_ability: Some(Ability::Telepathy),
        height: 8,
        weight: 202,
        base_exp_yield: 97,
        stats: Stats(
            Stat::new(38, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(65, 2),
            Stat::new(55, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Confusion),
            (1, Move::Teleport),
            (1, Move::DoubleTeam),
            (17, Move::LuckyChant),
            (22, Move::MagicalLeaf),
            (25, Move::HealPulse),
            (31, Move::CalmMind),
            (36, Move::Psychic),
            (39, Move::Imprison),
            (45, Move::FutureSight),
            (50, Move::Charm),
            (53, Move::Hypnosis),
            (59, Move::DreamEater),
            (64, Move::StoredPower),
        ]
    };
    pub const Gardevoir: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Synchronize, Ability::Trace),
        hidden_ability: Some(Ability::Telepathy),
        height: 16,
        weight: 484,
        base_exp_yield: 233,
        stats: Stats(
            Stat::new(68, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(125, 3),
            Stat::new(115, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Confusion),
            (1, Move::Teleport),
            (1, Move::DoubleTeam),
            (1, Move::HealingWish),
            (17, Move::Wish),
            (22, Move::MagicalLeaf),
            (25, Move::HealPulse),
            (33, Move::CalmMind),
            (40, Move::Psychic),
            (45, Move::Imprison),
            (53, Move::FutureSight),
            (60, Move::Captivate),
            (65, Move::Hypnosis),
            (73, Move::DreamEater),
            (80, Move::StoredPower),
        ]
    };
    pub const Surskit: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::RainDish),
        height: 5,
        weight: 17,
        base_exp_yield: 54,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(32, 0),
            Stat::new(50, 0),
            Stat::new(52, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (7, Move::QuickAttack),
            (13, Move::SweetScent),
            (19, Move::WaterSport),
            (25, Move::BubbleBeam),
            (31, Move::Agility),
            (37, Move::Mist),
            (37, Move::Haze),
            (43, Move::BatonPass),
        ]
    };
    pub const Masquerain: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::Intimidate),
        hidden_ability: Some(Ability::Unnerve),
        height: 8,
        weight: 36,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(62, 0),
            Stat::new(100, 1),
            Stat::new(82, 1),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::QuickAttack),
            (1, Move::Bubble),
            (1, Move::SweetScent),
            (1, Move::WaterSport),
            (1, Move::OminousWind),
            (22, Move::Gust),
            (26, Move::ScaryFace),
            (33, Move::StunSpore),
            (40, Move::SilverWind),
            (47, Move::AirSlash),
            (54, Move::Whirlwind),
            (61, Move::BugBuzz),
            (68, Move::QuiverDance),
        ]
    };
    pub const Shroomish: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::EffectSpore, Ability::PoisonHeal),
        hidden_ability: Some(Ability::QuickFeet),
        height: 4,
        weight: 45,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(60, 1),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (5, Move::Tackle),
            (9, Move::StunSpore),
            (13, Move::LeechSeed),
            (17, Move::MegaDrain),
            (21, Move::Headbutt),
            (25, Move::PoisonPowder),
            (29, Move::WorrySeed),
            (33, Move::Growth),
            (37, Move::GigaDrain),
            (41, Move::SeedBomb),
            (45, Move::Spore),
        ]
    };
    pub const Breloom: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Fighting),
        ability: PokemonAbility::Two(Ability::EffectSpore, Ability::PoisonHeal),
        hidden_ability: Some(Ability::Technician),
        height: 12,
        weight: 392,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(130, 2),
            Stat::new(80, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Absorb),
            (1, Move::LeechSeed),
            (1, Move::StunSpore),
            (17, Move::MegaDrain),
            (21, Move::Headbutt),
            (23, Move::MachPunch),
            (25, Move::Counter),
            (29, Move::ForcePalm),
            (33, Move::SkyUppercut),
            (37, Move::MindReader),
            (41, Move::SeedBomb),
            (45, Move::DynamicPunch),
        ]
    };
    pub const Slakoth: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Truant),
        hidden_ability: None,
        height: 8,
        weight: 240,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(60, 1),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Yawn),
            (7, Move::Encore),
            (13, Move::SlackOff),
            (19, Move::FeintAttack),
            (25, Move::Amnesia),
            (31, Move::Covet),
            (37, Move::ChipAway),
            (43, Move::Counter),
            (49, Move::Flail),
        ]
    };
    pub const Vigoroth: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::VitalSpirit),
        hidden_ability: None,
        height: 14,
        weight: 465,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(90, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::FocusEnergy),
            (1, Move::Encore),
            (1, Move::Uproar),
            (19, Move::FurySwipes),
            (25, Move::Endure),
            (31, Move::Slash),
            (37, Move::Counter),
            (43, Move::ChipAway),
            (49, Move::FocusPunch),
            (55, Move::Reversal),
        ]
    };
    pub const Slaking: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Truant),
        hidden_ability: None,
        height: 20,
        weight: 1305,
        base_exp_yield: 252,
        stats: Stats(
            Stat::new(150, 3),
            Stat::new(160, 0),
            Stat::new(100, 0),
            Stat::new(95, 0),
            Stat::new(65, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Encore),
            (1, Move::Yawn),
            (1, Move::SlackOff),
            (19, Move::FeintAttack),
            (25, Move::Amnesia),
            (31, Move::Covet),
            (36, Move::Swagger),
            (37, Move::ChipAway),
            (43, Move::Counter),
            (49, Move::Flail),
            (55, Move::Fling),
            (61, Move::Punishment),
            (67, Move::HammerArm),
        ]
    };
    pub const Nincada: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Ground),
        ability: PokemonAbility::One(Ability::CompoundEyes),
        hidden_ability: Some(Ability::RunAway),
        height: 5,
        weight: 55,
        base_exp_yield: 53,
        stats: Stats(
            Stat::new(31, 0),
            Stat::new(45, 0),
            Stat::new(90, 1),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Harden),
            (5, Move::LeechLife),
            (9, Move::SandAttack),
            (14, Move::FurySwipes),
            (19, Move::MindReader),
            (25, Move::FalseSwipe),
            (31, Move::MudSlap),
            (38, Move::MetalClaw),
            (45, Move::Dig),
        ]
    };
    pub const Ninjask: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::SpeedBoost),
        hidden_ability: Some(Ability::Infiltrator),
        height: 8,
        weight: 120,
        base_exp_yield: 160,
        stats: Stats(
            Stat::new(61, 0),
            Stat::new(90, 0),
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(160, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::SandAttack),
            (1, Move::Harden),
            (1, Move::LeechLife),
            (1, Move::BugBite),
            (14, Move::FurySwipes),
            (19, Move::MindReader),
            (20, Move::Screech),
            (20, Move::DoubleTeam),
            (20, Move::FuryCutter),
            (25, Move::SwordsDance),
            (31, Move::Slash),
            (38, Move::Agility),
            (45, Move::BatonPass),
            (52, Move::XScissor),
        ]
    };
    pub const Shedinja: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Ghost),
        ability: PokemonAbility::One(Ability::WonderGuard),
        hidden_ability: None,
        height: 8,
        weight: 12,
        base_exp_yield: 83,
        stats: Stats(
            Stat::new(1, 2),
            Stat::new(90, 0),
            Stat::new(45, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Harden),
            (5, Move::LeechLife),
            (9, Move::SandAttack),
            (14, Move::FurySwipes),
            (19, Move::MindReader),
            (25, Move::Spite),
            (31, Move::ConfuseRay),
            (38, Move::ShadowSneak),
            (45, Move::Grudge),
            (52, Move::HealBlock),
            (59, Move::ShadowBall),
        ]
    };
    pub const Whismur: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Soundproof),
        hidden_ability: Some(Ability::Rattled),
        height: 6,
        weight: 163,
        base_exp_yield: 48,
        stats: Stats(
            Stat::new(64, 1),
            Stat::new(51, 0),
            Stat::new(23, 0),
            Stat::new(51, 0),
            Stat::new(23, 0),
            Stat::new(28, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (5, Move::Uproar),
            (11, Move::Astonish),
            (15, Move::Howl),
            (21, Move::Supersonic),
            (25, Move::Stomp),
            (31, Move::Screech),
            (35, Move::Roar),
            (41, Move::Synchronoise),
            (45, Move::Rest),
            (45, Move::SleepTalk),
            (51, Move::HyperVoice),
        ]
    };
    pub const Loudred: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Soundproof),
        hidden_ability: Some(Ability::Scrappy),
        height: 10,
        weight: 405,
        base_exp_yield: 126,
        stats: Stats(
            Stat::new(84, 2),
            Stat::new(71, 0),
            Stat::new(43, 0),
            Stat::new(71, 0),
            Stat::new(43, 0),
            Stat::new(48, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Uproar),
            (1, Move::Astonish),
            (1, Move::Howl),
            (20, Move::Bite),
            (23, Move::Supersonic),
            (29, Move::Stomp),
            (37, Move::Screech),
            (43, Move::Roar),
            (51, Move::Synchronoise),
            (57, Move::Rest),
            (57, Move::SleepTalk),
            (65, Move::HyperVoice),
        ]
    };
    pub const Exploud: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Soundproof),
        hidden_ability: Some(Ability::Scrappy),
        height: 15,
        weight: 840,
        base_exp_yield: 221,
        stats: Stats(
            Stat::new(104, 3),
            Stat::new(91, 0),
            Stat::new(63, 0),
            Stat::new(91, 0),
            Stat::new(73, 0),
            Stat::new(68, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Uproar),
            (1, Move::Astonish),
            (1, Move::Howl),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (20, Move::Bite),
            (23, Move::Supersonic),
            (29, Move::Stomp),
            (37, Move::Screech),
            (40, Move::Crunch),
            (45, Move::Roar),
            (55, Move::Rest),
            (55, Move::Synchronoise),
            (63, Move::SleepTalk),
            (71, Move::HyperVoice),
            (79, Move::HyperBeam),
        ]
    };
    pub const Makuhita: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::Guts),
        hidden_ability: Some(Ability::SheerForce),
        height: 10,
        weight: 864,
        base_exp_yield: 47,
        stats: Stats(
            Stat::new(72, 1),
            Stat::new(60, 0),
            Stat::new(30, 0),
            Stat::new(20, 0),
            Stat::new(30, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::FocusEnergy),
            (4, Move::SandAttack),
            (7, Move::ArmThrust),
            (10, Move::VitalThrow),
            (13, Move::FakeOut),
            (16, Move::Whirlwind),
            (19, Move::KnockOff),
            (22, Move::SmellingSalts),
            (25, Move::BellyDrum),
            (28, Move::ForcePalm),
            (31, Move::SeismicToss),
            (34, Move::WakeUpSlap),
            (37, Move::Endure),
            (40, Move::CloseCombat),
            (43, Move::Reversal),
            (46, Move::HeavySlam),
        ]
    };
    pub const Hariyama: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::Guts),
        hidden_ability: Some(Ability::SheerForce),
        height: 23,
        weight: 2538,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(144, 2),
            Stat::new(120, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (1, Move::FocusEnergy),
            (1, Move::ArmThrust),
            (1, Move::Brine),
            (10, Move::VitalThrow),
            (13, Move::FakeOut),
            (16, Move::Whirlwind),
            (19, Move::KnockOff),
            (22, Move::SmellingSalts),
            (27, Move::BellyDrum),
            (32, Move::ForcePalm),
            (37, Move::SeismicToss),
            (42, Move::WakeUpSlap),
            (47, Move::Endure),
            (52, Move::CloseCombat),
            (57, Move::Reversal),
            (62, Move::HeavySlam),
        ]
    };
    pub const Azurill: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Fairy),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::HugePower),
        hidden_ability: Some(Ability::SapSipper),
        height: 2,
        weight: 20,
        base_exp_yield: 38,
        stats: Stats(
            Stat::new(50, 1),
            Stat::new(20, 0),
            Stat::new(40, 0),
            Stat::new(20, 0),
            Stat::new(40, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (1, Move::Splash),
            (2, Move::TailWhip),
            (5, Move::WaterSport),
            (7, Move::WaterGun),
            (10, Move::Charm),
            (13, Move::BubbleBeam),
            (16, Move::HelpingHand),
            (20, Move::Slam),
            (23, Move::Bounce),
        ]
    };
    pub const Nosepass: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::MagnetPull),
        hidden_ability: Some(Ability::SandForce),
        height: 10,
        weight: 970,
        base_exp_yield: 75,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(45, 0),
            Stat::new(135, 1),
            Stat::new(45, 0),
            Stat::new(90, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Harden),
            (8, Move::Block),
            (11, Move::RockThrow),
            (15, Move::ThunderWave),
            (18, Move::RockBlast),
            (22, Move::Rest),
            (25, Move::Spark),
            (29, Move::RockSlide),
            (32, Move::PowerGem),
            (36, Move::Sandstorm),
            (39, Move::Discharge),
            (43, Move::EarthPower),
            (46, Move::StoneEdge),
            (50, Move::ZapCannon),
            (50, Move::LockOn),
        ]
    };
    pub const Skitty: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::Normalize),
        hidden_ability: Some(Ability::WonderSkin),
        height: 6,
        weight: 110,
        base_exp_yield: 52,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(50, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Growl),
            (1, Move::FakeOut),
            (4, Move::Foresight),
            (8, Move::Attract),
            (11, Move::Sing),
            (15, Move::DoubleSlap),
            (18, Move::Copycat),
            (22, Move::Assist),
            (25, Move::Charm),
            (29, Move::FeintAttack),
            (32, Move::WakeUpSlap),
            (36, Move::Covet),
            (39, Move::HealBell),
            (42, Move::DoubleEdge),
            (46, Move::Captivate),
        ]
    };
    pub const Delcatty: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::Normalize),
        hidden_ability: Some(Ability::WonderSkin),
        height: 11,
        weight: 326,
        base_exp_yield: 140,
        stats: Stats(
            Stat::new(70, 1),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(90, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DoubleSlap),
            (1, Move::Sing),
            (1, Move::Attract),
            (1, Move::FakeOut),
        ]
    };
    pub const Sableye: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Ghost),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::Stall),
        hidden_ability: Some(Ability::Prankster),
        height: 5,
        weight: 110,
        base_exp_yield: 133,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(75, 1),
            Stat::new(75, 1),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (4, Move::Foresight),
            (8, Move::NightShade),
            (11, Move::Astonish),
            (15, Move::FurySwipes),
            (18, Move::FakeOut),
            (22, Move::Detect),
            (25, Move::ShadowSneak),
            (29, Move::KnockOff),
            (32, Move::FeintAttack),
            (36, Move::Punishment),
            (39, Move::ShadowClaw),
            (43, Move::PowerGem),
            (46, Move::ConfuseRay),
            (50, Move::FoulPlay),
            (53, Move::ZenHeadbutt),
            (57, Move::ShadowBall),
            (60, Move::MeanLook),
        ]
    };
    pub const Mawile: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Fairy),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::Intimidate),
        hidden_ability: Some(Ability::SheerForce),
        height: 6,
        weight: 115,
        base_exp_yield: 133,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(85, 1),
            Stat::new(85, 1),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Astonish),
            (6, Move::FakeTears),
            (11, Move::Bite),
            (16, Move::SweetScent),
            (21, Move::ViseGrip),
            (26, Move::FeintAttack),
            (31, Move::BatonPass),
            (36, Move::Crunch),
            (41, Move::IronDefense),
            (46, Move::SuckerPunch),
            (51, Move::Stockpile),
            (51, Move::SpitUp),
            (51, Move::Swallow),
            (56, Move::IronHead),
        ]
    };
    pub const Aron: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::RockHead),
        hidden_ability: Some(Ability::HeavyMetal),
        height: 4,
        weight: 600,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(70, 0),
            Stat::new(100, 1),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Harden),
            (4, Move::MudSlap),
            (8, Move::Headbutt),
            (11, Move::MetalClaw),
            (15, Move::IronDefense),
            (18, Move::Roar),
            (22, Move::TakeDown),
            (25, Move::IronHead),
            (29, Move::Protect),
            (32, Move::MetalSound),
            (36, Move::IronTail),
            (39, Move::Autotomize),
            (43, Move::HeavySlam),
            (46, Move::DoubleEdge),
            (50, Move::MetalBurst),
        ]
    };
    pub const Lairon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::RockHead),
        hidden_ability: Some(Ability::HeavyMetal),
        height: 9,
        weight: 1200,
        base_exp_yield: 151,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(90, 0),
            Stat::new(140, 2),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Tackle),
            (1, Move::Harden),
            (1, Move::MudSlap),
            (11, Move::MetalClaw),
            (15, Move::IronDefense),
            (18, Move::Roar),
            (22, Move::TakeDown),
            (25, Move::IronHead),
            (29, Move::Protect),
            (34, Move::MetalSound),
            (40, Move::IronTail),
            (45, Move::Autotomize),
            (51, Move::HeavySlam),
            (56, Move::DoubleEdge),
            (62, Move::MetalBurst),
        ]
    };
    pub const Aggron: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::RockHead),
        hidden_ability: Some(Ability::HeavyMetal),
        height: 21,
        weight: 3600,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(110, 0),
            Stat::new(180, 3),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Tackle),
            (1, Move::Harden),
            (1, Move::MudSlap),
            (11, Move::MetalClaw),
            (15, Move::IronDefense),
            (18, Move::Roar),
            (22, Move::TakeDown),
            (25, Move::IronHead),
            (29, Move::Protect),
            (34, Move::MetalSound),
            (40, Move::IronTail),
            (48, Move::Autotomize),
            (57, Move::HeavySlam),
            (65, Move::DoubleEdge),
            (74, Move::MetalBurst),
        ]
    };
    pub const Meditite: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fighting, Type::Psychic),
        ability: PokemonAbility::One(Ability::PurePower),
        hidden_ability: Some(Ability::Telepathy),
        height: 6,
        weight: 112,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(60, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bide),
            (4, Move::Meditate),
            (8, Move::Confusion),
            (11, Move::Detect),
            (15, Move::HiddenPower),
            (18, Move::MindReader),
            (22, Move::Feint),
            (25, Move::CalmMind),
            (29, Move::ForcePalm),
            (32, Move::HighJumpKick),
            (36, Move::PsychUp),
            (39, Move::Acupressure),
            (43, Move::PowerTrick),
            (46, Move::Reversal),
            (50, Move::Recover),
        ]
    };
    pub const Medicham: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fighting, Type::Psychic),
        ability: PokemonAbility::One(Ability::PurePower),
        hidden_ability: Some(Ability::Telepathy),
        height: 13,
        weight: 315,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(80, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::IcePunch),
            (1, Move::ThunderPunch),
            (1, Move::Confusion),
            (1, Move::Meditate),
            (1, Move::Bide),
            (1, Move::Detect),
            (15, Move::HiddenPower),
            (18, Move::MindReader),
            (22, Move::Feint),
            (25, Move::CalmMind),
            (29, Move::ForcePalm),
            (32, Move::HighJumpKick),
            (36, Move::PsychUp),
            (42, Move::Acupressure),
            (49, Move::PowerTrick),
            (55, Move::Reversal),
            (62, Move::Recover),
        ]
    };
    pub const Electrike: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Static, Ability::LightningRod),
        hidden_ability: Some(Ability::Minus),
        height: 6,
        weight: 152,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(65, 0),
            Stat::new(40, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::ThunderWave),
            (9, Move::Leer),
            (12, Move::Howl),
            (17, Move::QuickAttack),
            (20, Move::Spark),
            (25, Move::OdorSleuth),
            (28, Move::Bite),
            (33, Move::ThunderFang),
            (36, Move::Roar),
            (41, Move::Discharge),
            (44, Move::Charge),
            (49, Move::WildCharge),
            (52, Move::Thunder),
        ]
    };
    pub const Manectric: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Static, Ability::LightningRod),
        hidden_ability: Some(Ability::Minus),
        height: 15,
        weight: 402,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(105, 0),
            Stat::new(60, 0),
            Stat::new(105, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::ThunderWave),
            (1, Move::Howl),
            (1, Move::FireFang),
            (17, Move::QuickAttack),
            (20, Move::Spark),
            (25, Move::OdorSleuth),
            (30, Move::Bite),
            (37, Move::ThunderFang),
            (42, Move::Roar),
            (49, Move::Discharge),
            (54, Move::Charge),
            (61, Move::WildCharge),
            (66, Move::Thunder),
        ]
    };
    pub const Plusle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Plus),
        hidden_ability: Some(Ability::LightningRod),
        height: 4,
        weight: 42,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(85, 0),
            Stat::new(75, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (3, Move::ThunderWave),
            (7, Move::QuickAttack),
            (10, Move::HelpingHand),
            (15, Move::Spark),
            (17, Move::Encore),
            (21, Move::FakeTears),
            (24, Move::Copycat),
            (29, Move::ElectroBall),
            (31, Move::Swift),
            (38, Move::Charge),
            (42, Move::Thunder),
            (44, Move::BatonPass),
            (48, Move::Agility),
            (51, Move::LastResort),
            (56, Move::NastyPlot),
            (63, Move::Entrainment),
        ]
    };
    pub const Minun: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Minus),
        hidden_ability: Some(Ability::VoltAbsorb),
        height: 4,
        weight: 42,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (3, Move::ThunderWave),
            (7, Move::QuickAttack),
            (10, Move::HelpingHand),
            (15, Move::Spark),
            (17, Move::Encore),
            (21, Move::Charm),
            (24, Move::Copycat),
            (29, Move::ElectroBall),
            (31, Move::Swift),
            (35, Move::FakeTears),
            (38, Move::Charge),
            (42, Move::Thunder),
            (44, Move::BatonPass),
            (48, Move::Agility),
            (51, Move::TrumpCard),
            (56, Move::NastyPlot),
            (63, Move::Entrainment),
        ]
    };
    pub const Volbeat: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::Two(Ability::Illuminate, Ability::Swarm),
        hidden_ability: Some(Ability::Prankster),
        height: 7,
        weight: 177,
        base_exp_yield: 151,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(73, 0),
            Stat::new(75, 0),
            Stat::new(47, 0),
            Stat::new(85, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Flash),
            (5, Move::DoubleTeam),
            (9, Move::ConfuseRay),
            (13, Move::Moonlight),
            (17, Move::QuickAttack),
            (21, Move::TailGlow),
            (25, Move::SignalBeam),
            (29, Move::Protect),
            (33, Move::HelpingHand),
            (37, Move::ZenHeadbutt),
            (41, Move::BugBuzz),
            (45, Move::DoubleEdge),
        ]
    };
    pub const Illumise: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::TintedLens),
        hidden_ability: Some(Ability::Prankster),
        height: 6,
        weight: 177,
        base_exp_yield: 151,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(47, 0),
            Stat::new(75, 0),
            Stat::new(73, 0),
            Stat::new(85, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::SweetScent),
            (9, Move::Charm),
            (13, Move::Moonlight),
            (17, Move::QuickAttack),
            (21, Move::Wish),
            (25, Move::Encore),
            (29, Move::Flatter),
            (33, Move::HelpingHand),
            (37, Move::ZenHeadbutt),
            (41, Move::BugBuzz),
            (45, Move::Covet),
        ]
    };
    pub const Roselia: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::Two(Ability::NaturalCure, Ability::PoisonPoint),
        hidden_ability: Some(Ability::LeafGuard),
        height: 3,
        weight: 20,
        base_exp_yield: 140,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(60, 0),
            Stat::new(45, 0),
            Stat::new(100, 2),
            Stat::new(80, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (4, Move::Growth),
            (7, Move::PoisonSting),
            (10, Move::StunSpore),
            (13, Move::MegaDrain),
            (16, Move::LeechSeed),
            (19, Move::MagicalLeaf),
            (22, Move::GrassWhistle),
            (25, Move::GigaDrain),
            (28, Move::ToxicSpikes),
            (31, Move::SweetScent),
            (34, Move::Ingrain),
            (37, Move::PetalDance),
            (40, Move::Toxic),
            (43, Move::Aromatherapy),
            (46, Move::Synthesis),
        ]
    };
    pub const Gulpin: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::LiquidOoze, Ability::StickyHold),
        hidden_ability: Some(Ability::Gluttony),
        height: 4,
        weight: 103,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(70, 1),
            Stat::new(43, 0),
            Stat::new(53, 0),
            Stat::new(43, 0),
            Stat::new(53, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (6, Move::Yawn),
            (9, Move::PoisonGas),
            (14, Move::Sludge),
            (17, Move::Amnesia),
            (23, Move::Encore),
            (28, Move::Toxic),
            (34, Move::AcidSpray),
            (39, Move::Stockpile),
            (39, Move::SpitUp),
            (39, Move::Swallow),
            (44, Move::SludgeBomb),
            (49, Move::GastroAcid),
            (54, Move::WringOut),
            (59, Move::GunkShot),
        ]
    };
    pub const Swalot: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::LiquidOoze, Ability::StickyHold),
        hidden_ability: Some(Ability::Gluttony),
        height: 17,
        weight: 800,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(100, 2),
            Stat::new(73, 0),
            Stat::new(83, 0),
            Stat::new(73, 0),
            Stat::new(83, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Sludge),
            (1, Move::PoisonGas),
            (1, Move::Yawn),
            (17, Move::Amnesia),
            (23, Move::Encore),
            (26, Move::BodySlam),
            (30, Move::Toxic),
            (38, Move::AcidSpray),
            (45, Move::Stockpile),
            (45, Move::SpitUp),
            (45, Move::Swallow),
            (52, Move::SludgeBomb),
            (59, Move::GastroAcid),
            (66, Move::WringOut),
            (73, Move::GunkShot),
        ]
    };
    pub const Carvanha: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Dark),
        ability: PokemonAbility::One(Ability::RoughSkin),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 8,
        weight: 208,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(90, 1),
            Stat::new(20, 0),
            Stat::new(65, 0),
            Stat::new(20, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (6, Move::Rage),
            (8, Move::FocusEnergy),
            (11, Move::ScaryFace),
            (16, Move::IceFang),
            (18, Move::Screech),
            (21, Move::Swagger),
            (26, Move::Assurance),
            (28, Move::Crunch),
            (31, Move::AquaJet),
            (36, Move::Agility),
            (38, Move::TakeDown),
        ]
    };
    pub const Sharpedo: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Dark),
        ability: PokemonAbility::One(Ability::RoughSkin),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 18,
        weight: 888,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(120, 2),
            Stat::new(40, 0),
            Stat::new(95, 0),
            Stat::new(40, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Rage),
            (1, Move::FocusEnergy),
            (1, Move::Feint),
            (11, Move::ScaryFace),
            (16, Move::IceFang),
            (18, Move::Screech),
            (21, Move::Swagger),
            (26, Move::Assurance),
            (28, Move::Crunch),
            (30, Move::Slash),
            (34, Move::AquaJet),
            (40, Move::Taunt),
            (45, Move::Agility),
            (50, Move::SkullBash),
            (56, Move::NightSlash),
        ]
    };
    pub const Wailmer: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::WaterVeil, Ability::Oblivious),
        hidden_ability: Some(Ability::Pressure),
        height: 20,
        weight: 1300,
        base_exp_yield: 80,
        stats: Stats(
            Stat::new(130, 1),
            Stat::new(70, 0),
            Stat::new(35, 0),
            Stat::new(70, 0),
            Stat::new(35, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Splash),
            (4, Move::Growl),
            (7, Move::WaterGun),
            (11, Move::Rollout),
            (14, Move::Whirlpool),
            (17, Move::Astonish),
            (21, Move::WaterPulse),
            (24, Move::Mist),
            (27, Move::Rest),
            (31, Move::Brine),
            (34, Move::WaterSpout),
            (37, Move::Amnesia),
            (41, Move::Dive),
            (44, Move::Bounce),
            (47, Move::HydroPump),
            (50, Move::HeavySlam),
        ]
    };
    pub const Wailord: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::WaterVeil, Ability::Oblivious),
        hidden_ability: Some(Ability::Pressure),
        height: 145,
        weight: 3980,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(170, 2),
            Stat::new(90, 0),
            Stat::new(45, 0),
            Stat::new(90, 0),
            Stat::new(45, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::Splash),
            (1, Move::Rollout),
            (14, Move::Whirlpool),
            (17, Move::Astonish),
            (21, Move::WaterPulse),
            (24, Move::Mist),
            (27, Move::Rest),
            (31, Move::Brine),
            (34, Move::WaterSpout),
            (37, Move::Amnesia),
            (46, Move::Dive),
            (54, Move::Bounce),
            (62, Move::HydroPump),
            (70, Move::HeavySlam),
        ]
    };
    pub const Numel: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Ground),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::Simple),
        hidden_ability: Some(Ability::OwnTempo),
        height: 7,
        weight: 240,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(65, 1),
            Stat::new(45, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (5, Move::Ember),
            (8, Move::Magnitude),
            (12, Move::FocusEnergy),
            (15, Move::FlameBurst),
            (19, Move::Amnesia),
            (22, Move::LavaPlume),
            (26, Move::EarthPower),
            (29, Move::Curse),
            (31, Move::TakeDown),
            (36, Move::Yawn),
            (40, Move::Earthquake),
            (43, Move::Flamethrower),
            (47, Move::DoubleEdge),
        ]
    };
    pub const Camerupt: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Ground),
        ability: PokemonAbility::Two(Ability::MagmaArmor, Ability::SolidRock),
        hidden_ability: Some(Ability::AngerPoint),
        height: 19,
        weight: 2200,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(100, 1),
            Stat::new(70, 0),
            Stat::new(105, 1),
            Stat::new(75, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Ember),
            (1, Move::Magnitude),
            (12, Move::FocusEnergy),
            (15, Move::FlameBurst),
            (19, Move::Amnesia),
            (22, Move::LavaPlume),
            (26, Move::EarthPower),
            (29, Move::Curse),
            (31, Move::TakeDown),
            (33, Move::RockSlide),
            (39, Move::Yawn),
            (46, Move::Earthquake),
            (52, Move::Eruption),
            (59, Move::Fissure),
        ]
    };
    pub const Torkoal: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::WhiteSmoke, Ability::Drought),
        hidden_ability: Some(Ability::ShellArmor),
        height: 5,
        weight: 804,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(85, 0),
            Stat::new(140, 2),
            Stat::new(85, 0),
            Stat::new(70, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (4, Move::Smog),
            (7, Move::Withdraw),
            (12, Move::Curse),
            (17, Move::FireSpin),
            (20, Move::Smokescreen),
            (23, Move::RapidSpin),
            (28, Move::Flamethrower),
            (33, Move::BodySlam),
            (36, Move::Protect),
            (39, Move::LavaPlume),
            (44, Move::IronDefense),
            (49, Move::Amnesia),
            (52, Move::Flail),
            (55, Move::HeatWave),
            (60, Move::Inferno),
            (65, Move::ShellSmash),
        ]
    };
    pub const Spoink: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::OwnTempo),
        hidden_ability: Some(Ability::Gluttony),
        height: 7,
        weight: 306,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(25, 0),
            Stat::new(35, 0),
            Stat::new(70, 0),
            Stat::new(80, 1),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Splash),
            (7, Move::Psywave),
            (10, Move::OdorSleuth),
            (14, Move::Psybeam),
            (15, Move::PsychUp),
            (18, Move::ConfuseRay),
            (21, Move::MagicCoat),
            (26, Move::ZenHeadbutt),
            (29, Move::Rest),
            (29, Move::Snore),
            (33, Move::PowerGem),
            (38, Move::Psyshock),
            (40, Move::Payback),
            (44, Move::Psychic),
            (50, Move::Bounce),
        ]
    };
    pub const Grumpig: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::OwnTempo),
        hidden_ability: Some(Ability::Gluttony),
        height: 9,
        weight: 715,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(45, 0),
            Stat::new(65, 0),
            Stat::new(90, 0),
            Stat::new(110, 2),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Psybeam),
            (1, Move::Psywave),
            (1, Move::Splash),
            (1, Move::OdorSleuth),
            (15, Move::PsychUp),
            (18, Move::ConfuseRay),
            (21, Move::MagicCoat),
            (26, Move::ZenHeadbutt),
            (29, Move::Rest),
            (29, Move::Snore),
            (35, Move::PowerGem),
            (42, Move::Psyshock),
            (46, Move::Payback),
            (52, Move::Psychic),
            (60, Move::Bounce),
        ]
    };
    pub const Spinda: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::OwnTempo, Ability::TangledFeet),
        hidden_ability: Some(Ability::Contrary),
        height: 11,
        weight: 50,
        base_exp_yield: 126,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(60, 1),
            Stat::new(60, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::Uproar),
            (10, Move::Copycat),
            (14, Move::FeintAttack),
            (19, Move::Psybeam),
            (23, Move::Hypnosis),
            (28, Move::DizzyPunch),
            (32, Move::SuckerPunch),
            (37, Move::TeeterDance),
            (41, Move::PsychUp),
            (46, Move::DoubleEdge),
            (50, Move::Flail),
            (55, Move::Thrash),
        ]
    };
    pub const Trapinch: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::ArenaTrap),
        hidden_ability: Some(Ability::SheerForce),
        height: 7,
        weight: 150,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(100, 1),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(10, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bite),
            (4, Move::SandAttack),
            (7, Move::FeintAttack),
            (10, Move::SandTomb),
            (13, Move::MudSlap),
            (17, Move::Bide),
            (21, Move::Bulldoze),
            (25, Move::RockSlide),
            (29, Move::Dig),
            (34, Move::Crunch),
            (39, Move::EarthPower),
            (44, Move::Sandstorm),
            (49, Move::HyperBeam),
            (55, Move::Earthquake),
            (61, Move::Feint),
            (67, Move::Superpower),
            (73, Move::Fissure),
        ]
    };
    pub const Vibrava: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Dragon),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 11,
        weight: 153,
        base_exp_yield: 119,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(70, 1),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(70, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::SonicBoom),
            (1, Move::FeintAttack),
            (1, Move::SandTomb),
            (13, Move::MudSlap),
            (17, Move::Bide),
            (21, Move::Bulldoze),
            (25, Move::RockSlide),
            (29, Move::Supersonic),
            (34, Move::Screech),
            (35, Move::DragonBreath),
            (39, Move::EarthPower),
            (44, Move::Sandstorm),
            (49, Move::HyperBeam),
        ]
    };
    pub const Flygon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Dragon),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 20,
        weight: 820,
        base_exp_yield: 234,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(100, 1),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(100, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::SonicBoom),
            (1, Move::FeintAttack),
            (1, Move::SandTomb),
            (13, Move::MudSlap),
            (17, Move::Bide),
            (21, Move::Bulldoze),
            (25, Move::RockSlide),
            (29, Move::Supersonic),
            (34, Move::Screech),
            (35, Move::DragonBreath),
            (39, Move::EarthPower),
            (44, Move::Sandstorm),
            (45, Move::DragonTail),
            (49, Move::HyperBeam),
            (55, Move::DragonClaw),
        ]
    };
    pub const Cacnea: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 4,
        weight: 513,
        base_exp_yield: 67,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(85, 0),
            Stat::new(40, 0),
            Stat::new(85, 1),
            Stat::new(40, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Plant, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Leer),
            (5, Move::Absorb),
            (9, Move::Growth),
            (13, Move::LeechSeed),
            (17, Move::SandAttack),
            (21, Move::PinMissile),
            (25, Move::Ingrain),
            (29, Move::FeintAttack),
            (33, Move::Spikes),
            (37, Move::SuckerPunch),
            (41, Move::Payback),
            (45, Move::NeedleArm),
            (49, Move::CottonSpore),
            (53, Move::Sandstorm),
            (57, Move::DestinyBond),
        ]
    };
    pub const Cacturne: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Dark),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 13,
        weight: 774,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(115, 1),
            Stat::new(60, 0),
            Stat::new(115, 1),
            Stat::new(60, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Plant, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Leer),
            (1, Move::Absorb),
            (1, Move::Growth),
            (1, Move::Revenge),
            (13, Move::LeechSeed),
            (17, Move::SandAttack),
            (21, Move::PinMissile),
            (25, Move::Ingrain),
            (29, Move::FeintAttack),
            (35, Move::Spikes),
            (41, Move::SuckerPunch),
            (47, Move::Payback),
            (53, Move::NeedleArm),
            (59, Move::CottonSpore),
            (65, Move::Sandstorm),
            (71, Move::DestinyBond),
        ]
    };
    pub const Swablu: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::NaturalCure),
        hidden_ability: Some(Ability::CloudNine),
        height: 4,
        weight: 12,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(75, 1),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Peck),
            (4, Move::Astonish),
            (8, Move::Sing),
            (10, Move::FuryAttack),
            (13, Move::Safeguard),
            (15, Move::Mist),
            (18, Move::Round),
            (21, Move::NaturalGift),
            (25, Move::TakeDown),
            (29, Move::Refresh),
            (34, Move::MirrorMove),
            (39, Move::CottonGuard),
            (42, Move::DragonPulse),
            (48, Move::PerishSong),
        ]
    };
    pub const Altaria: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Flying),
        ability: PokemonAbility::One(Ability::NaturalCure),
        hidden_ability: Some(Ability::CloudNine),
        height: 11,
        weight: 206,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(70, 0),
            Stat::new(90, 0),
            Stat::new(70, 0),
            Stat::new(105, 2),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Sing),
            (1, Move::Peck),
            (1, Move::Astonish),
            (1, Move::Pluck),
            (10, Move::FuryAttack),
            (13, Move::Safeguard),
            (15, Move::Mist),
            (18, Move::Round),
            (21, Move::NaturalGift),
            (25, Move::TakeDown),
            (29, Move::Refresh),
            (34, Move::DragonDance),
            (35, Move::DragonBreath),
            (42, Move::CottonGuard),
            (48, Move::DragonPulse),
            (57, Move::PerishSong),
            (64, Move::SkyAttack),
        ]
    };
    pub const Zangoose: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Immunity),
        hidden_ability: Some(Ability::ToxicBoost),
        height: 13,
        weight: 403,
        base_exp_yield: 160,
        stats: Stats(
            Stat::new(73, 0),
            Stat::new(115, 2),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (5, Move::QuickAttack),
            (8, Move::FuryCutter),
            (12, Move::Pursuit),
            (15, Move::Slash),
            (19, Move::Embargo),
            (22, Move::CrushClaw),
            (26, Move::Revenge),
            (29, Move::FalseSwipe),
            (33, Move::Detect),
            (36, Move::XScissor),
            (40, Move::Taunt),
            (43, Move::SwordsDance),
            (47, Move::CloseCombat),
        ]
    };
    pub const Seviper: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: Some(Ability::Infiltrator),
        height: 27,
        weight: 525,
        base_exp_yield: 160,
        stats: Stats(
            Stat::new(73, 0),
            Stat::new(100, 1),
            Stat::new(60, 0),
            Stat::new(100, 1),
            Stat::new(60, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::Lick),
            (5, Move::Bite),
            (9, Move::Swagger),
            (12, Move::PoisonTail),
            (16, Move::Screech),
            (20, Move::Venoshock),
            (23, Move::Glare),
            (27, Move::PoisonFang),
            (31, Move::NightSlash),
            (34, Move::GastroAcid),
            (38, Move::Haze),
            (42, Move::PoisonJab),
            (45, Move::Crunch),
            (49, Move::Coil),
            (53, Move::WringOut),
        ]
    };
    pub const Lunatone: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 10,
        weight: 1680,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(95, 2),
            Stat::new(85, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Confusion),
            (1, Move::Harden),
            (5, Move::RockThrow),
            (9, Move::Hypnosis),
            (13, Move::RockPolish),
            (17, Move::Psywave),
            (21, Move::Embargo),
            (25, Move::RockSlide),
            (29, Move::CosmicPower),
            (33, Move::Psychic),
            (37, Move::HealBlock),
            (41, Move::StoneEdge),
            (45, Move::FutureSight),
            (49, Move::Explosion),
            (53, Move::MagicRoom),
        ]
    };
    pub const Solrock: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 12,
        weight: 1540,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(95, 2),
            Stat::new(85, 0),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Confusion),
            (1, Move::Harden),
            (5, Move::RockThrow),
            (9, Move::FireSpin),
            (13, Move::RockPolish),
            (17, Move::Psywave),
            (21, Move::Embargo),
            (25, Move::RockSlide),
            (29, Move::CosmicPower),
            (33, Move::Psychic),
            (37, Move::HealBlock),
            (41, Move::StoneEdge),
            (45, Move::SolarBeam),
            (49, Move::Explosion),
            (53, Move::WonderRoom),
        ]
    };
    pub const Barboach: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::Anticipation),
        hidden_ability: Some(Ability::Hydration),
        height: 4,
        weight: 19,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(50, 1),
            Stat::new(48, 0),
            Stat::new(43, 0),
            Stat::new(46, 0),
            Stat::new(41, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MudSlap),
            (6, Move::MudSport),
            (6, Move::WaterSport),
            (10, Move::WaterGun),
            (14, Move::MudBomb),
            (18, Move::Amnesia),
            (22, Move::WaterPulse),
            (26, Move::Magnitude),
            (31, Move::Rest),
            (31, Move::Snore),
            (35, Move::AquaTail),
            (39, Move::Earthquake),
            (43, Move::FutureSight),
            (47, Move::Fissure),
        ]
    };
    pub const Whiscash: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::Anticipation),
        hidden_ability: Some(Ability::Hydration),
        height: 9,
        weight: 236,
        base_exp_yield: 164,
        stats: Stats(
            Stat::new(110, 2),
            Stat::new(78, 0),
            Stat::new(73, 0),
            Stat::new(76, 0),
            Stat::new(71, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MudSlap),
            (1, Move::MudSport),
            (1, Move::Tickle),
            (1, Move::WaterSport),
            (1, Move::ZenHeadbutt),
            (10, Move::WaterGun),
            (14, Move::MudBomb),
            (18, Move::Amnesia),
            (22, Move::WaterPulse),
            (26, Move::Magnitude),
            (33, Move::Rest),
            (33, Move::Snore),
            (39, Move::AquaTail),
            (45, Move::Earthquake),
            (51, Move::FutureSight),
            (57, Move::Fissure),
        ]
    };
    pub const Corphish: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::ShellArmor),
        hidden_ability: Some(Ability::Adaptability),
        height: 6,
        weight: 115,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(43, 0),
            Stat::new(80, 1),
            Stat::new(65, 0),
            Stat::new(50, 0),
            Stat::new(35, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (7, Move::Harden),
            (10, Move::ViseGrip),
            (13, Move::Leer),
            (20, Move::BubbleBeam),
            (23, Move::Protect),
            (26, Move::KnockOff),
            (32, Move::Taunt),
            (35, Move::NightSlash),
            (38, Move::Crabhammer),
            (44, Move::SwordsDance),
            (47, Move::Crunch),
            (53, Move::Guillotine),
        ]
    };
    pub const Crawdaunt: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Dark),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::ShellArmor),
        hidden_ability: Some(Ability::Adaptability),
        height: 11,
        weight: 328,
        base_exp_yield: 164,
        stats: Stats(
            Stat::new(63, 0),
            Stat::new(120, 2),
            Stat::new(85, 0),
            Stat::new(90, 0),
            Stat::new(55, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (1, Move::Leer),
            (1, Move::Harden),
            (1, Move::Bubble),
            (20, Move::BubbleBeam),
            (23, Move::Protect),
            (26, Move::KnockOff),
            (30, Move::Swift),
            (34, Move::Taunt),
            (39, Move::NightSlash),
            (44, Move::Crabhammer),
            (52, Move::SwordsDance),
            (57, Move::Crunch),
            (65, Move::Guillotine),
        ]
    };
    pub const Baltoy: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 5,
        weight: 215,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(70, 1),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Harden),
            (4, Move::RapidSpin),
            (7, Move::MudSlap),
            (10, Move::RockTomb),
            (13, Move::Psybeam),
            (17, Move::PowerTrick),
            (21, Move::AncientPower),
            (25, Move::SelfDestruct),
            (28, Move::Extrasensory),
            (31, Move::CosmicPower),
            (34, Move::GuardSplit),
            (34, Move::PowerSplit),
            (37, Move::EarthPower),
            (41, Move::Sandstorm),
            (45, Move::HealBlock),
            (49, Move::Explosion),
        ]
    };
    pub const Claydol: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 15,
        weight: 1080,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(105, 0),
            Stat::new(70, 0),
            Stat::new(120, 2),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Teleport),
            (1, Move::Harden),
            (1, Move::RapidSpin),
            (7, Move::MudSlap),
            (10, Move::RockTomb),
            (13, Move::Psybeam),
            (17, Move::PowerTrick),
            (21, Move::AncientPower),
            (25, Move::SelfDestruct),
            (28, Move::Extrasensory),
            (31, Move::CosmicPower),
            (34, Move::GuardSplit),
            (34, Move::PowerSplit),
            (36, Move::HyperBeam),
            (40, Move::EarthPower),
            (47, Move::Sandstorm),
            (54, Move::HealBlock),
            (61, Move::Explosion),
        ]
    };
    pub const Lileep: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Grass),
        ability: PokemonAbility::One(Ability::SuctionCups),
        hidden_ability: Some(Ability::StormDrain),
        height: 10,
        weight: 238,
        base_exp_yield: 71,
        stats: Stats(
            Stat::new(66, 0),
            Stat::new(41, 0),
            Stat::new(77, 0),
            Stat::new(61, 0),
            Stat::new(87, 1),
            Stat::new(23, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Constrict),
            (1, Move::Astonish),
            (8, Move::Acid),
            (15, Move::Ingrain),
            (22, Move::ConfuseRay),
            (29, Move::Amnesia),
            (36, Move::GastroAcid),
            (43, Move::AncientPower),
            (50, Move::EnergyBall),
            (57, Move::Stockpile),
            (57, Move::SpitUp),
            (57, Move::Swallow),
            (64, Move::WringOut),
        ]
    };
    pub const Cradily: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Grass),
        ability: PokemonAbility::One(Ability::SuctionCups),
        hidden_ability: Some(Ability::StormDrain),
        height: 15,
        weight: 604,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(86, 0),
            Stat::new(81, 0),
            Stat::new(97, 0),
            Stat::new(81, 0),
            Stat::new(107, 2),
            Stat::new(43, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Acid),
            (1, Move::Constrict),
            (1, Move::Ingrain),
            (1, Move::Astonish),
            (22, Move::ConfuseRay),
            (29, Move::Amnesia),
            (36, Move::AncientPower),
            (46, Move::GastroAcid),
            (56, Move::EnergyBall),
            (66, Move::Stockpile),
            (66, Move::SpitUp),
            (66, Move::Swallow),
            (76, Move::WringOut),
        ]
    };
    pub const Anorith: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Bug),
        ability: PokemonAbility::One(Ability::BattleArmor),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 7,
        weight: 125,
        base_exp_yield: 71,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(95, 1),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Harden),
            (7, Move::MudSport),
            (13, Move::WaterGun),
            (19, Move::MetalClaw),
            (25, Move::Protect),
            (31, Move::AncientPower),
            (37, Move::FuryCutter),
            (43, Move::Slash),
            (49, Move::RockBlast),
            (55, Move::CrushClaw),
            (61, Move::XScissor),
        ]
    };
    pub const Armaldo: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Bug),
        ability: PokemonAbility::One(Ability::BattleArmor),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 15,
        weight: 682,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(125, 2),
            Stat::new(100, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::WaterGun),
            (1, Move::Harden),
            (1, Move::MudSport),
            (19, Move::MetalClaw),
            (25, Move::Protect),
            (31, Move::AncientPower),
            (37, Move::FuryCutter),
            (46, Move::Slash),
            (55, Move::RockBlast),
            (67, Move::CrushClaw),
            (73, Move::XScissor),
        ]
    };
    pub const Feebas: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::Oblivious),
        hidden_ability: Some(Ability::Adaptability),
        height: 6,
        weight: 74,
        base_exp_yield: 40,
        stats: Stats(
            Stat::new(20, 0),
            Stat::new(15, 0),
            Stat::new(20, 0),
            Stat::new(10, 0),
            Stat::new(55, 0),
            Stat::new(80, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Splash),
            (15, Move::Tackle),
            (30, Move::Flail),
        ]
    };
    pub const Milotic: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::MarvelScale),
        hidden_ability: Some(Ability::CuteCharm),
        height: 62,
        weight: 1620,
        base_exp_yield: 189,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(60, 0),
            Stat::new(79, 0),
            Stat::new(100, 0),
            Stat::new(125, 2),
            Stat::new(81, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::WaterGun),
            (5, Move::WaterSport),
            (9, Move::Refresh),
            (13, Move::WaterPulse),
            (17, Move::Twister),
            (21, Move::Recover),
            (25, Move::Captivate),
            (29, Move::AquaTail),
            (33, Move::RainDance),
            (37, Move::HydroPump),
            (41, Move::Attract),
            (45, Move::Safeguard),
            (49, Move::AquaRing),
        ]
    };
    pub const Castform: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Forecast),
        hidden_ability: None,
        height: 3,
        weight: 8,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(70, 1),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (10, Move::Ember),
            (10, Move::WaterGun),
            (10, Move::PowderSnow),
            (15, Move::Headbutt),
            (20, Move::RainDance),
            (20, Move::SunnyDay),
            (20, Move::Hail),
            (30, Move::WeatherBall),
            (40, Move::HydroPump),
            (40, Move::Blizzard),
            (40, Move::FireBlast),
        ]
    };
    pub const Kecleon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::ColorChange),
        hidden_ability: None,
        height: 10,
        weight: 220,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(90, 0),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(120, 1),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::TailWhip),
            (1, Move::Lick),
            (1, Move::Thief),
            (1, Move::Astonish),
            (4, Move::Bind),
            (7, Move::FeintAttack),
            (10, Move::FurySwipes),
            (14, Move::Feint),
            (18, Move::Psybeam),
            (22, Move::ShadowSneak),
            (27, Move::Slash),
            (32, Move::Screech),
            (37, Move::Substitute),
            (43, Move::SuckerPunch),
            (49, Move::ShadowClaw),
            (55, Move::AncientPower),
            (58, Move::Synchronoise),
        ]
    };
    pub const Shuppet: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::Frisk),
        hidden_ability: Some(Ability::CursedBody),
        height: 6,
        weight: 23,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(44, 0),
            Stat::new(75, 1),
            Stat::new(35, 0),
            Stat::new(63, 0),
            Stat::new(33, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::KnockOff),
            (4, Move::Screech),
            (7, Move::NightShade),
            (10, Move::Spite),
            (13, Move::WillOWisp),
            (16, Move::ShadowSneak),
            (19, Move::Curse),
            (22, Move::FeintAttack),
            (26, Move::Hex),
            (30, Move::ShadowBall),
            (34, Move::SuckerPunch),
            (38, Move::Embargo),
            (42, Move::Snatch),
            (46, Move::Grudge),
            (50, Move::Trick),
        ]
    };
    pub const Banette: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::Frisk),
        hidden_ability: Some(Ability::CursedBody),
        height: 11,
        weight: 125,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(64, 0),
            Stat::new(115, 2),
            Stat::new(65, 0),
            Stat::new(83, 0),
            Stat::new(63, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::NightShade),
            (1, Move::Screech),
            (1, Move::Curse),
            (1, Move::KnockOff),
            (10, Move::Spite),
            (13, Move::WillOWisp),
            (16, Move::ShadowSneak),
            (22, Move::FeintAttack),
            (26, Move::Hex),
            (30, Move::ShadowBall),
            (34, Move::SuckerPunch),
            (40, Move::Embargo),
            (46, Move::Snatch),
            (52, Move::Grudge),
            (58, Move::Trick),
        ]
    };
    pub const Duskull: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: Some(Ability::Frisk),
        height: 8,
        weight: 150,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(20, 0),
            Stat::new(40, 0),
            Stat::new(90, 0),
            Stat::new(30, 0),
            Stat::new(90, 1),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::NightShade),
            (6, Move::Disable),
            (9, Move::Foresight),
            (14, Move::Astonish),
            (17, Move::ConfuseRay),
            (22, Move::ShadowSneak),
            (25, Move::Pursuit),
            (30, Move::Curse),
            (33, Move::WillOWisp),
            (38, Move::Hex),
            (41, Move::MeanLook),
            (46, Move::Payback),
            (49, Move::FutureSight),
        ]
    };
    pub const Dusclops: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Frisk),
        height: 16,
        weight: 306,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(70, 0),
            Stat::new(130, 1),
            Stat::new(60, 0),
            Stat::new(130, 1),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::IcePunch),
            (1, Move::ThunderPunch),
            (1, Move::Bind),
            (1, Move::Leer),
            (1, Move::Disable),
            (1, Move::NightShade),
            (1, Move::Gravity),
            (9, Move::Foresight),
            (14, Move::Astonish),
            (17, Move::ConfuseRay),
            (22, Move::ShadowSneak),
            (25, Move::Pursuit),
            (30, Move::Curse),
            (33, Move::WillOWisp),
            (37, Move::ShadowPunch),
            (42, Move::Hex),
            (49, Move::MeanLook),
            (58, Move::Payback),
            (61, Move::FutureSight),
        ]
    };
    pub const Tropius: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Flying),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::SolarPower),
        hidden_ability: Some(Ability::Harvest),
        height: 20,
        weight: 1000,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(99, 2),
            Stat::new(68, 0),
            Stat::new(83, 0),
            Stat::new(72, 0),
            Stat::new(87, 0),
            Stat::new(51, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Leer),
            (7, Move::Growth),
            (11, Move::RazorLeaf),
            (17, Move::Stomp),
            (21, Move::SweetScent),
            (27, Move::Whirlwind),
            (31, Move::MagicalLeaf),
            (37, Move::BodySlam),
            (41, Move::Synthesis),
            (47, Move::LeafTornado),
            (51, Move::AirSlash),
            (57, Move::Bestow),
            (61, Move::SolarBeam),
            (67, Move::NaturalGift),
            (71, Move::LeafStorm),
        ]
    };
    pub const Chimecho: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 6,
        weight: 10,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(50, 0),
            Stat::new(80, 0),
            Stat::new(95, 1),
            Stat::new(90, 1),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (6, Move::Growl),
            (9, Move::Astonish),
            (14, Move::Confusion),
            (17, Move::Uproar),
            (22, Move::TakeDown),
            (25, Move::Yawn),
            (30, Move::Psywave),
            (33, Move::DoubleEdge),
            (38, Move::HealBell),
            (41, Move::Safeguard),
            (46, Move::Extrasensory),
            (49, Move::HealPulse),
            (54, Move::Synchronoise),
            (57, Move::HealingWish),
        ]
    };
    pub const Absol: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::Two(Ability::Pressure, Ability::SuperLuck),
        hidden_ability: Some(Ability::Justified),
        height: 12,
        weight: 470,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(130, 2),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Feint),
            (4, Move::Leer),
            (9, Move::QuickAttack),
            (12, Move::Pursuit),
            (17, Move::Taunt),
            (20, Move::Bite),
            (25, Move::DoubleTeam),
            (28, Move::Slash),
            (33, Move::SwordsDance),
            (36, Move::FutureSight),
            (41, Move::NightSlash),
            (44, Move::Detect),
            (49, Move::PsychoCut),
            (52, Move::SuckerPunch),
            (57, Move::RazorWind),
            (60, Move::MeFirst),
            (65, Move::PerishSong),
        ]
    };
    pub const Wynaut: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::ShadowTag),
        hidden_ability: Some(Ability::Telepathy),
        height: 6,
        weight: 140,
        base_exp_yield: 52,
        stats: Stats(
            Stat::new(95, 1),
            Stat::new(23, 0),
            Stat::new(48, 0),
            Stat::new(23, 0),
            Stat::new(48, 0),
            Stat::new(23, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Splash),
            (1, Move::Charm),
            (1, Move::Encore),
            (15, Move::Counter),
            (15, Move::DestinyBond),
            (15, Move::Safeguard),
            (15, Move::MirrorCoat),
        ]
    };
    pub const Snorunt: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::Two(Ability::InnerFocus, Ability::IceBody),
        hidden_ability: Some(Ability::Moody),
        height: 7,
        weight: 168,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(50, 1),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::PowderSnow),
            (4, Move::DoubleTeam),
            (10, Move::Bite),
            (13, Move::IcyWind),
            (19, Move::Headbutt),
            (22, Move::Protect),
            (28, Move::IceFang),
            (31, Move::Crunch),
            (37, Move::IceShard),
            (40, Move::Hail),
            (46, Move::Blizzard),
        ]
    };
    pub const Glalie: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::Two(Ability::InnerFocus, Ability::IceBody),
        hidden_ability: Some(Ability::Moody),
        height: 15,
        weight: 2565,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(80, 2),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::DoubleTeam),
            (1, Move::PowderSnow),
            (13, Move::IcyWind),
            (19, Move::Headbutt),
            (22, Move::Protect),
            (28, Move::IceFang),
            (31, Move::Crunch),
            (37, Move::IceBeam),
            (40, Move::Hail),
            (51, Move::Blizzard),
            (59, Move::SheerCold),
        ]
    };
    pub const Spheal: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Water),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::IceBody),
        hidden_ability: Some(Ability::Oblivious),
        height: 8,
        weight: 395,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(70, 1),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::DefenseCurl),
            (1, Move::PowderSnow),
            (7, Move::Encore),
            (13, Move::IceBall),
            (19, Move::BodySlam),
            (25, Move::AuroraBeam),
            (31, Move::Hail),
            (37, Move::Rest),
            (37, Move::Snore),
            (43, Move::Blizzard),
            (49, Move::SheerCold),
        ]
    };
    pub const Sealeo: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Water),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::IceBody),
        hidden_ability: Some(Ability::Oblivious),
        height: 11,
        weight: 876,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(90, 2),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(75, 0),
            Stat::new(70, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::PowderSnow),
            (1, Move::Encore),
            (13, Move::IceBall),
            (19, Move::BodySlam),
            (25, Move::AuroraBeam),
            (31, Move::Hail),
            (32, Move::Swagger),
            (39, Move::Rest),
            (39, Move::Snore),
            (47, Move::Blizzard),
            (55, Move::SheerCold),
        ]
    };
    pub const Walrein: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Water),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::IceBody),
        hidden_ability: Some(Ability::Oblivious),
        height: 14,
        weight: 1506,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(110, 3),
            Stat::new(80, 0),
            Stat::new(90, 0),
            Stat::new(95, 0),
            Stat::new(90, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::WaterGun),
            (1, Move::PowderSnow),
            (1, Move::Encore),
            (1, Move::Crunch),
            (13, Move::IceBall),
            (19, Move::BodySlam),
            (25, Move::AuroraBeam),
            (31, Move::Hail),
            (32, Move::Swagger),
            (39, Move::Rest),
            (39, Move::Snore),
            (44, Move::IceFang),
            (52, Move::Blizzard),
            (65, Move::SheerCold),
        ]
    };
    pub const Clamperl: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::ShellArmor),
        hidden_ability: Some(Ability::Rattled),
        height: 4,
        weight: 525,
        base_exp_yield: 69,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(64, 0),
            Stat::new(85, 1),
            Stat::new(74, 0),
            Stat::new(55, 0),
            Stat::new(32, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (1, Move::Clamp),
            (1, Move::Whirlpool),
            (1, Move::IronDefense),
            (51, Move::ShellSmash),
        ]
    };
    pub const Huntail: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::WaterVeil),
        height: 17,
        weight: 270,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(104, 1),
            Stat::new(105, 1),
            Stat::new(94, 0),
            Stat::new(75, 0),
            Stat::new(52, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Whirlpool),
            (6, Move::Bite),
            (10, Move::Screech),
            (15, Move::WaterPulse),
            (19, Move::ScaryFace),
            (24, Move::IceFang),
            (28, Move::Brine),
            (33, Move::BatonPass),
            (37, Move::Dive),
            (42, Move::Crunch),
            (46, Move::AquaTail),
            (51, Move::HydroPump),
        ]
    };
    pub const Gorebyss: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::Hydration),
        height: 18,
        weight: 226,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(84, 0),
            Stat::new(105, 0),
            Stat::new(114, 2),
            Stat::new(75, 0),
            Stat::new(52, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Whirlpool),
            (6, Move::Confusion),
            (10, Move::Agility),
            (15, Move::WaterPulse),
            (19, Move::Amnesia),
            (24, Move::AquaRing),
            (28, Move::Captivate),
            (33, Move::BatonPass),
            (37, Move::Dive),
            (42, Move::Psychic),
            (46, Move::AquaTail),
            (51, Move::HydroPump),
        ]
    };
    pub const Relicanth: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Rock),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::RockHead),
        hidden_ability: Some(Ability::Sturdy),
        height: 10,
        weight: 234,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(100, 1),
            Stat::new(90, 0),
            Stat::new(130, 1),
            Stat::new(45, 0),
            Stat::new(65, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Harden),
            (8, Move::WaterGun),
            (15, Move::RockTomb),
            (22, Move::Yawn),
            (29, Move::TakeDown),
            (36, Move::MudSport),
            (43, Move::AncientPower),
            (50, Move::DoubleEdge),
            (57, Move::Dive),
            (64, Move::Rest),
            (71, Move::HydroPump),
            (78, Move::HeadSmash),
        ]
    };
    pub const Luvdisc: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::Hydration),
        height: 6,
        weight: 87,
        base_exp_yield: 116,
        stats: Stats(
            Stat::new(43, 0),
            Stat::new(30, 0),
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(65, 0),
            Stat::new(97, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Charm),
            (7, Move::WaterGun),
            (9, Move::Agility),
            (14, Move::TakeDown),
            (17, Move::LuckyChant),
            (22, Move::WaterPulse),
            (27, Move::Attract),
            (31, Move::Flail),
            (37, Move::SweetKiss),
            (40, Move::HydroPump),
            (46, Move::AquaRing),
            (51, Move::Captivate),
            (55, Move::Safeguard),
        ]
    };
    pub const Bagon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::One(Ability::RockHead),
        hidden_ability: Some(Ability::SheerForce),
        height: 6,
        weight: 421,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(75, 1),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Rage),
            (5, Move::Bite),
            (10, Move::Leer),
            (16, Move::Headbutt),
            (20, Move::FocusEnergy),
            (25, Move::Ember),
            (31, Move::DragonBreath),
            (35, Move::ZenHeadbutt),
            (40, Move::ScaryFace),
            (46, Move::Crunch),
            (50, Move::DragonClaw),
            (55, Move::DoubleEdge),
        ]
    };
    pub const Shelgon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::One(Ability::RockHead),
        hidden_ability: Some(Ability::Overcoat),
        height: 11,
        weight: 1105,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(95, 0),
            Stat::new(100, 2),
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Rage),
            (20, Move::FocusEnergy),
            (25, Move::Ember),
            (30, Move::Protect),
            (32, Move::DragonBreath),
            (37, Move::ZenHeadbutt),
            (43, Move::ScaryFace),
            (50, Move::Crunch),
            (55, Move::DragonClaw),
            (61, Move::DoubleEdge),
        ]
    };
    pub const Salamence: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Flying),
        ability: PokemonAbility::One(Ability::Intimidate),
        hidden_ability: Some(Ability::Moxie),
        height: 15,
        weight: 1026,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(135, 3),
            Stat::new(80, 0),
            Stat::new(110, 0),
            Stat::new(80, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Rage),
            (1, Move::ThunderFang),
            (1, Move::FireFang),
            (20, Move::FocusEnergy),
            (25, Move::Ember),
            (30, Move::Protect),
            (32, Move::DragonBreath),
            (37, Move::ZenHeadbutt),
            (43, Move::ScaryFace),
            (50, Move::Fly),
            (53, Move::Crunch),
            (61, Move::DragonClaw),
            (70, Move::DoubleEdge),
            (80, Move::DragonTail),
        ]
    };
    pub const Beldum: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Psychic),
        ability: PokemonAbility::One(Ability::ClearBody),
        hidden_ability: Some(Ability::LightMetal),
        height: 6,
        weight: 952,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(80, 1),
            Stat::new(35, 0),
            Stat::new(60, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TakeDown),
        ]
    };
    pub const Metang: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Psychic),
        ability: PokemonAbility::One(Ability::ClearBody),
        hidden_ability: Some(Ability::LightMetal),
        height: 12,
        weight: 2025,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(100, 2),
            Stat::new(55, 0),
            Stat::new(80, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TakeDown),
            (1, Move::Confusion),
            (1, Move::MetalClaw),
            (1, Move::MagnetRise),
            (23, Move::Pursuit),
            (26, Move::MiracleEye),
            (29, Move::ZenHeadbutt),
            (32, Move::BulletPunch),
            (35, Move::ScaryFace),
            (38, Move::Agility),
            (41, Move::Psychic),
            (44, Move::MeteorMash),
            (47, Move::IronDefense),
            (50, Move::HyperBeam),
        ]
    };
    pub const Metagross: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Psychic),
        ability: PokemonAbility::One(Ability::ClearBody),
        hidden_ability: Some(Ability::LightMetal),
        height: 16,
        weight: 5500,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(135, 0),
            Stat::new(130, 3),
            Stat::new(95, 0),
            Stat::new(90, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TakeDown),
            (1, Move::Confusion),
            (1, Move::MetalClaw),
            (1, Move::MagnetRise),
            (23, Move::Pursuit),
            (26, Move::MiracleEye),
            (29, Move::ZenHeadbutt),
            (32, Move::BulletPunch),
            (35, Move::ScaryFace),
            (38, Move::Agility),
            (41, Move::Psychic),
            (44, Move::MeteorMash),
            (45, Move::HammerArm),
            (53, Move::IronDefense),
            (62, Move::HyperBeam),
        ]
    };
    pub const Regirock: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::One(Ability::ClearBody),
        hidden_ability: Some(Ability::Sturdy),
        height: 17,
        weight: 2300,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(100, 0),
            Stat::new(200, 3),
            Stat::new(50, 0),
            Stat::new(100, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Stomp),
            (1, Move::Explosion),
            (9, Move::RockThrow),
            (17, Move::Curse),
            (25, Move::Superpower),
            (33, Move::AncientPower),
            (41, Move::IronDefense),
            (49, Move::ChargeBeam),
            (57, Move::LockOn),
            (65, Move::ZapCannon),
            (73, Move::StoneEdge),
            (81, Move::HammerArm),
            (89, Move::HyperBeam),
        ]
    };
    pub const Regice: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::One(Ability::ClearBody),
        hidden_ability: Some(Ability::IceBody),
        height: 18,
        weight: 1750,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(50, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(200, 3),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Stomp),
            (1, Move::Explosion),
            (9, Move::IcyWind),
            (17, Move::Curse),
            (25, Move::Superpower),
            (33, Move::AncientPower),
            (41, Move::Amnesia),
            (49, Move::ChargeBeam),
            (57, Move::LockOn),
            (65, Move::ZapCannon),
            (73, Move::IceBeam),
            (81, Move::HammerArm),
            (89, Move::HyperBeam),
        ]
    };
    pub const Registeel: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Steel),
        ability: PokemonAbility::One(Ability::ClearBody),
        hidden_ability: Some(Ability::LightMetal),
        height: 19,
        weight: 2050,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(75, 0),
            Stat::new(150, 2),
            Stat::new(75, 0),
            Stat::new(150, 1),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Stomp),
            (1, Move::Explosion),
            (9, Move::MetalClaw),
            (17, Move::Curse),
            (25, Move::Superpower),
            (33, Move::AncientPower),
            (41, Move::Amnesia),
            (41, Move::IronDefense),
            (49, Move::ChargeBeam),
            (57, Move::LockOn),
            (65, Move::ZapCannon),
            (73, Move::FlashCannon),
            (73, Move::IronHead),
            (81, Move::HammerArm),
            (89, Move::HyperBeam),
        ]
    };
    pub const Latias: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 14,
        weight: 400,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(90, 0),
            Stat::new(110, 0),
            Stat::new(130, 3),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Psywave),
            (5, Move::Wish),
            (10, Move::HelpingHand),
            (15, Move::Safeguard),
            (20, Move::DragonBreath),
            (25, Move::WaterSport),
            (30, Move::Refresh),
            (35, Move::MistBall),
            (40, Move::ZenHeadbutt),
            (45, Move::Recover),
            (50, Move::PsychoShift),
            (55, Move::Charm),
            (60, Move::Psychic),
            (65, Move::HealPulse),
            (70, Move::ReflectType),
            (75, Move::GuardSplit),
            (80, Move::DragonPulse),
            (85, Move::HealingWish),
        ]
    };
    pub const Latios: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 20,
        weight: 600,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(90, 0),
            Stat::new(80, 0),
            Stat::new(130, 3),
            Stat::new(110, 0),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Psywave),
            (5, Move::HealBlock),
            (10, Move::HelpingHand),
            (15, Move::Safeguard),
            (20, Move::DragonBreath),
            (25, Move::Protect),
            (30, Move::Refresh),
            (35, Move::LusterPurge),
            (40, Move::ZenHeadbutt),
            (45, Move::Recover),
            (50, Move::PsychoShift),
            (55, Move::DragonDance),
            (60, Move::Psychic),
            (65, Move::HealPulse),
            (70, Move::Telekinesis),
            (75, Move::PowerSplit),
            (80, Move::DragonPulse),
            (85, Move::Memento),
        ]
    };
    pub const Kyogre: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Drizzle),
        hidden_ability: None,
        height: 45,
        weight: 3520,
        base_exp_yield: 302,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(90, 0),
            Stat::new(150, 3),
            Stat::new(140, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterPulse),
            (5, Move::ScaryFace),
            (15, Move::BodySlam),
            (20, Move::MuddyWater),
            (30, Move::AquaRing),
            (35, Move::IceBeam),
            (45, Move::AncientPower),
            (50, Move::WaterSpout),
            (60, Move::CalmMind),
            (65, Move::AquaTail),
            (75, Move::SheerCold),
            (80, Move::DoubleEdge),
            (90, Move::HydroPump),
        ]
    };
    pub const Groudon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::Drought),
        hidden_ability: None,
        height: 35,
        weight: 9500,
        base_exp_yield: 302,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(150, 3),
            Stat::new(140, 0),
            Stat::new(100, 0),
            Stat::new(90, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MudShot),
            (5, Move::ScaryFace),
            (15, Move::LavaPlume),
            (20, Move::HammerArm),
            (30, Move::Rest),
            (35, Move::Earthquake),
            (45, Move::AncientPower),
            (50, Move::Eruption),
            (60, Move::BulkUp),
            (65, Move::EarthPower),
            (75, Move::Fissure),
            (80, Move::SolarBeam),
            (90, Move::FireBlast),
        ]
    };
    pub const Rayquaza: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Flying),
        ability: PokemonAbility::One(Ability::AirLock),
        hidden_ability: None,
        height: 70,
        weight: 2065,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(105, 0),
            Stat::new(150, 2),
            Stat::new(90, 0),
            Stat::new(150, 1),
            Stat::new(90, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Twister),
            (5, Move::ScaryFace),
            (15, Move::Crunch),
            (20, Move::HyperVoice),
            (30, Move::Rest),
            (35, Move::AirSlash),
            (45, Move::AncientPower),
            (50, Move::Outrage),
            (60, Move::DragonDance),
            (65, Move::Fly),
            (75, Move::ExtremeSpeed),
            (80, Move::HyperBeam),
            (90, Move::DragonPulse),
        ]
    };
    pub const Jirachi: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Psychic),
        ability: PokemonAbility::One(Ability::SereneGrace),
        hidden_ability: None,
        height: 3,
        weight: 11,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Wish),
            (5, Move::Rest),
            (10, Move::Swift),
            (15, Move::HelpingHand),
            (20, Move::Psychic),
            (25, Move::Refresh),
            (35, Move::ZenHeadbutt),
            (40, Move::DoubleEdge),
            (45, Move::Gravity),
            (50, Move::HealingWish),
            (55, Move::FutureSight),
            (60, Move::CosmicPower),
            (65, Move::LastResort),
            (70, Move::DoomDesire),
        ]
    };
    pub const DeoxysNormal: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: None,
        height: 17,
        weight: 608,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(150, 1),
            Stat::new(50, 0),
            Stat::new(150, 1),
            Stat::new(50, 0),
            Stat::new(150, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (1, Move::Leer),
            (9, Move::NightShade),
            (17, Move::Teleport),
            (25, Move::KnockOff),
            (33, Move::Pursuit),
            (41, Move::Psychic),
            (49, Move::Snatch),
            (57, Move::PsychoShift),
            (65, Move::ZenHeadbutt),
            (73, Move::CosmicPower),
            (81, Move::Recover),
            (89, Move::PsychoBoost),
            (97, Move::HyperBeam),
        ]
    };
    pub const Turtwig: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::ShellArmor),
        height: 4,
        weight: 102,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(68, 1),
            Stat::new(64, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(31, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::Withdraw),
            (9, Move::Absorb),
            (13, Move::RazorLeaf),
            (17, Move::Curse),
            (21, Move::Bite),
            (25, Move::MegaDrain),
            (29, Move::LeechSeed),
            (33, Move::Synthesis),
            (37, Move::Crunch),
            (41, Move::GigaDrain),
            (45, Move::LeafStorm),
        ]
    };
    pub const Grotle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::ShellArmor),
        height: 11,
        weight: 970,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(89, 1),
            Stat::new(85, 1),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(36, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Withdraw),
            (9, Move::Absorb),
            (13, Move::RazorLeaf),
            (17, Move::Curse),
            (22, Move::Bite),
            (27, Move::MegaDrain),
            (32, Move::LeechSeed),
            (37, Move::Synthesis),
            (42, Move::Crunch),
            (47, Move::GigaDrain),
            (52, Move::LeafStorm),
        ]
    };
    pub const Torterra: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Ground),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::ShellArmor),
        height: 22,
        weight: 3100,
        base_exp_yield: 236,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(109, 2),
            Stat::new(105, 1),
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(56, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Absorb),
            (1, Move::RazorLeaf),
            (1, Move::Withdraw),
            (1, Move::WoodHammer),
            (17, Move::Curse),
            (22, Move::Bite),
            (27, Move::MegaDrain),
            (32, Move::Earthquake),
            (33, Move::LeechSeed),
            (39, Move::Synthesis),
            (45, Move::Crunch),
            (51, Move::GigaDrain),
            (57, Move::LeafStorm),
        ]
    };
    pub const Chimchar: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::IronFist),
        height: 5,
        weight: 62,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(44, 0),
            Stat::new(58, 0),
            Stat::new(44, 0),
            Stat::new(58, 0),
            Stat::new(44, 0),
            Stat::new(61, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (7, Move::Ember),
            (9, Move::Taunt),
            (15, Move::FurySwipes),
            (17, Move::FlameWheel),
            (23, Move::NastyPlot),
            (25, Move::Torment),
            (31, Move::Facade),
            (33, Move::FireSpin),
            (39, Move::Acrobatics),
            (41, Move::SlackOff),
            (47, Move::Flamethrower),
        ]
    };
    pub const Monferno: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Fighting),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::IronFist),
        height: 9,
        weight: 220,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(64, 0),
            Stat::new(78, 0),
            Stat::new(52, 0),
            Stat::new(78, 1),
            Stat::new(52, 0),
            Stat::new(81, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Ember),
            (9, Move::Taunt),
            (14, Move::MachPunch),
            (16, Move::FurySwipes),
            (19, Move::FlameWheel),
            (26, Move::Feint),
            (29, Move::Torment),
            (36, Move::CloseCombat),
            (39, Move::FireSpin),
            (46, Move::Acrobatics),
            (49, Move::SlackOff),
            (56, Move::FlareBlitz),
        ]
    };
    pub const Infernape: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Fighting),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::IronFist),
        height: 12,
        weight: 550,
        base_exp_yield: 240,
        stats: Stats(
            Stat::new(76, 0),
            Stat::new(104, 1),
            Stat::new(71, 0),
            Stat::new(104, 1),
            Stat::new(71, 0),
            Stat::new(108, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Ember),
            (1, Move::Taunt),
            (14, Move::MachPunch),
            (16, Move::FurySwipes),
            (19, Move::FlameWheel),
            (26, Move::Feint),
            (29, Move::Punishment),
            (36, Move::CloseCombat),
            (42, Move::FireSpin),
            (52, Move::Acrobatics),
            (58, Move::CalmMind),
            (68, Move::FlareBlitz),
        ]
    };
    pub const Piplup: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::Defiant),
        height: 4,
        weight: 52,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(53, 0),
            Stat::new(51, 0),
            Stat::new(53, 0),
            Stat::new(61, 1),
            Stat::new(56, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (4, Move::Growl),
            (8, Move::Bubble),
            (11, Move::WaterSport),
            (15, Move::Peck),
            (18, Move::BubbleBeam),
            (22, Move::Bide),
            (25, Move::FuryAttack),
            (29, Move::Brine),
            (32, Move::Whirlpool),
            (36, Move::Mist),
            (39, Move::DrillPeck),
            (43, Move::HydroPump),
        ]
    };
    pub const Prinplup: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::Defiant),
        height: 8,
        weight: 230,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(64, 0),
            Stat::new(66, 0),
            Stat::new(68, 0),
            Stat::new(81, 2),
            Stat::new(76, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (8, Move::Bubble),
            (11, Move::WaterSport),
            (15, Move::Peck),
            (16, Move::MetalClaw),
            (19, Move::BubbleBeam),
            (24, Move::Bide),
            (28, Move::FuryAttack),
            (33, Move::Brine),
            (37, Move::Whirlpool),
            (42, Move::Mist),
            (46, Move::DrillPeck),
            (51, Move::HydroPump),
        ]
    };
    pub const Empoleon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Steel),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::Defiant),
        height: 17,
        weight: 845,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(84, 0),
            Stat::new(86, 0),
            Stat::new(88, 0),
            Stat::new(111, 3),
            Stat::new(101, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Bubble),
            (11, Move::SwordsDance),
            (15, Move::Peck),
            (16, Move::MetalClaw),
            (19, Move::BubbleBeam),
            (24, Move::Swagger),
            (28, Move::FuryAttack),
            (33, Move::Brine),
            (36, Move::AquaJet),
            (39, Move::Whirlpool),
            (46, Move::Mist),
            (52, Move::DrillPeck),
            (59, Move::HydroPump),
        ]
    };
    pub const Starly: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::KeenEye),
        hidden_ability: Some(Ability::Reckless),
        height: 3,
        weight: 20,
        base_exp_yield: 49,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(60, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (5, Move::QuickAttack),
            (9, Move::WingAttack),
            (13, Move::DoubleTeam),
            (17, Move::Endeavor),
            (21, Move::Whirlwind),
            (25, Move::AerialAce),
            (29, Move::TakeDown),
            (33, Move::Agility),
            (37, Move::BraveBird),
            (41, Move::FinalGambit),
        ]
    };
    pub const Staravia: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::Intimidate),
        hidden_ability: Some(Ability::Reckless),
        height: 6,
        weight: 155,
        base_exp_yield: 119,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(75, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(80, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::QuickAttack),
            (9, Move::WingAttack),
            (13, Move::DoubleTeam),
            (18, Move::Endeavor),
            (23, Move::Whirlwind),
            (28, Move::AerialAce),
            (33, Move::TakeDown),
            (38, Move::Agility),
            (43, Move::BraveBird),
            (48, Move::FinalGambit),
        ]
    };
    pub const Staraptor: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::One(Ability::Intimidate),
        hidden_ability: Some(Ability::Reckless),
        height: 12,
        weight: 249,
        base_exp_yield: 218,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(120, 3),
            Stat::new(70, 0),
            Stat::new(50, 0),
            Stat::new(60, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::QuickAttack),
            (13, Move::DoubleTeam),
            (18, Move::Endeavor),
            (23, Move::Whirlwind),
            (28, Move::AerialAce),
            (33, Move::TakeDown),
            (34, Move::CloseCombat),
            (41, Move::Agility),
            (49, Move::BraveBird),
            (57, Move::FinalGambit),
        ]
    };
    pub const Bidoof: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Simple, Ability::Unaware),
        hidden_ability: Some(Ability::Moody),
        height: 5,
        weight: 200,
        base_exp_yield: 50,
        stats: Stats(
            Stat::new(59, 1),
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(40, 0),
            Stat::new(31, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::Growl),
            (9, Move::DefenseCurl),
            (13, Move::Rollout),
            (17, Move::Headbutt),
            (21, Move::HyperFang),
            (25, Move::Yawn),
            (29, Move::Amnesia),
            (33, Move::TakeDown),
            (37, Move::SuperFang),
            (41, Move::Superpower),
            (45, Move::Curse),
        ]
    };
    pub const Bibarel: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Water),
        ability: PokemonAbility::Two(Ability::Simple, Ability::Unaware),
        hidden_ability: Some(Ability::Moody),
        height: 10,
        weight: 315,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(79, 0),
            Stat::new(85, 2),
            Stat::new(60, 0),
            Stat::new(55, 0),
            Stat::new(60, 0),
            Stat::new(71, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growl),
            (9, Move::DefenseCurl),
            (13, Move::Rollout),
            (15, Move::WaterGun),
            (18, Move::Headbutt),
            (23, Move::HyperFang),
            (28, Move::Yawn),
            (33, Move::Amnesia),
            (38, Move::TakeDown),
            (43, Move::SuperFang),
            (48, Move::Superpower),
            (53, Move::Curse),
        ]
    };
    pub const Kricketot: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: Some(Ability::RunAway),
        height: 3,
        weight: 22,
        base_exp_yield: 39,
        stats: Stats(
            Stat::new(37, 0),
            Stat::new(25, 0),
            Stat::new(41, 1),
            Stat::new(25, 0),
            Stat::new(41, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Bide),
            (6, Move::StruggleBug),
            (16, Move::BugBite),
        ]
    };
    pub const Kricketune: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::Swarm),
        hidden_ability: Some(Ability::Technician),
        height: 10,
        weight: 255,
        base_exp_yield: 134,
        stats: Stats(
            Stat::new(77, 0),
            Stat::new(85, 2),
            Stat::new(51, 0),
            Stat::new(55, 0),
            Stat::new(51, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Bide),
            (10, Move::FuryCutter),
            (14, Move::LeechLife),
            (18, Move::Sing),
            (22, Move::FocusEnergy),
            (26, Move::Slash),
            (30, Move::XScissor),
            (34, Move::Screech),
            (38, Move::Taunt),
            (42, Move::NightSlash),
            (46, Move::BugBuzz),
            (50, Move::PerishSong),
        ]
    };
    pub const Shinx: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Rivalry, Ability::Intimidate),
        hidden_ability: Some(Ability::Guts),
        height: 5,
        weight: 95,
        base_exp_yield: 53,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(65, 1),
            Stat::new(34, 0),
            Stat::new(40, 0),
            Stat::new(34, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::Leer),
            (9, Move::Charge),
            (13, Move::Spark),
            (17, Move::Bite),
            (21, Move::Roar),
            (25, Move::Swagger),
            (29, Move::ThunderFang),
            (33, Move::Crunch),
            (37, Move::ScaryFace),
            (41, Move::Discharge),
            (45, Move::WildCharge),
        ]
    };
    pub const Luxio: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Rivalry, Ability::Intimidate),
        hidden_ability: Some(Ability::Guts),
        height: 9,
        weight: 305,
        base_exp_yield: 127,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(85, 2),
            Stat::new(49, 0),
            Stat::new(60, 0),
            Stat::new(49, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (9, Move::Charge),
            (13, Move::Spark),
            (18, Move::Bite),
            (23, Move::Roar),
            (28, Move::Swagger),
            (33, Move::ThunderFang),
            (38, Move::Crunch),
            (43, Move::ScaryFace),
            (48, Move::Discharge),
            (53, Move::WildCharge),
        ]
    };
    pub const Luxray: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::Rivalry, Ability::Intimidate),
        hidden_ability: Some(Ability::Guts),
        height: 14,
        weight: 420,
        base_exp_yield: 235,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(120, 3),
            Stat::new(79, 0),
            Stat::new(95, 0),
            Stat::new(79, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Charge),
            (13, Move::Spark),
            (18, Move::Bite),
            (23, Move::Roar),
            (28, Move::Swagger),
            (35, Move::ThunderFang),
            (42, Move::Crunch),
            (49, Move::ScaryFace),
            (56, Move::Discharge),
            (63, Move::WildCharge),
        ]
    };
    pub const Budew: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::Two(Ability::NaturalCure, Ability::PoisonPoint),
        hidden_ability: Some(Ability::LeafGuard),
        height: 2,
        weight: 12,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(35, 0),
            Stat::new(50, 1),
            Stat::new(70, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (4, Move::Growth),
            (7, Move::WaterSport),
            (10, Move::StunSpore),
            (13, Move::MegaDrain),
            (16, Move::WorrySeed),
        ]
    };
    pub const Roserade: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::Two(Ability::NaturalCure, Ability::PoisonPoint),
        hidden_ability: Some(Ability::Technician),
        height: 9,
        weight: 145,
        base_exp_yield: 232,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(125, 3),
            Stat::new(105, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::MegaDrain),
            (1, Move::SweetScent),
            (1, Move::WeatherBall),
            (1, Move::MagicalLeaf),
        ]
    };
    pub const Cranidos: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::One(Ability::MoldBreaker),
        hidden_ability: Some(Ability::SheerForce),
        height: 9,
        weight: 315,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(67, 0),
            Stat::new(125, 1),
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(58, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Leer),
            (6, Move::FocusEnergy),
            (10, Move::Pursuit),
            (15, Move::TakeDown),
            (19, Move::ScaryFace),
            (24, Move::Assurance),
            (28, Move::ChipAway),
            (33, Move::AncientPower),
            (37, Move::ZenHeadbutt),
            (42, Move::Screech),
            (46, Move::HeadSmash),
        ]
    };
    pub const Rampardos: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::One(Ability::MoldBreaker),
        hidden_ability: Some(Ability::SheerForce),
        height: 16,
        weight: 1025,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(97, 0),
            Stat::new(165, 2),
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(50, 0),
            Stat::new(58, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Leer),
            (6, Move::FocusEnergy),
            (10, Move::Pursuit),
            (15, Move::TakeDown),
            (19, Move::ScaryFace),
            (24, Move::Assurance),
            (28, Move::ChipAway),
            (30, Move::Endeavor),
            (36, Move::AncientPower),
            (43, Move::ZenHeadbutt),
            (51, Move::Screech),
            (58, Move::HeadSmash),
        ]
    };
    pub const Shieldon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Steel),
        ability: PokemonAbility::One(Ability::Sturdy),
        hidden_ability: Some(Ability::Soundproof),
        height: 5,
        weight: 570,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(42, 0),
            Stat::new(118, 1),
            Stat::new(42, 0),
            Stat::new(88, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Protect),
            (6, Move::Taunt),
            (10, Move::MetalSound),
            (15, Move::TakeDown),
            (19, Move::IronDefense),
            (24, Move::Swagger),
            (28, Move::AncientPower),
            (33, Move::Endure),
            (37, Move::MetalBurst),
            (42, Move::IronHead),
            (46, Move::HeavySlam),
        ]
    };
    pub const Bastiodon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Steel),
        ability: PokemonAbility::One(Ability::Sturdy),
        hidden_ability: Some(Ability::Soundproof),
        height: 13,
        weight: 1495,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(52, 0),
            Stat::new(168, 2),
            Stat::new(47, 0),
            Stat::new(138, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Protect),
            (1, Move::Taunt),
            (1, Move::MetalSound),
            (15, Move::TakeDown),
            (19, Move::IronDefense),
            (24, Move::Swagger),
            (28, Move::AncientPower),
            (30, Move::Block),
            (36, Move::Endure),
            (43, Move::MetalBurst),
            (51, Move::IronHead),
            (58, Move::HeavySlam),
        ]
    };
    pub const Burmy: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::One(Ability::ShedSkin),
        hidden_ability: Some(Ability::Overcoat),
        height: 2,
        weight: 34,
        base_exp_yield: 45,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(29, 0),
            Stat::new(45, 0),
            Stat::new(29, 0),
            Stat::new(45, 1),
            Stat::new(36, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Protect),
            (10, Move::Tackle),
            (15, Move::BugBite),
            (20, Move::HiddenPower),
        ]
    };
    pub const WormadamPlant: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Grass),
        ability: PokemonAbility::One(Ability::Anticipation),
        hidden_ability: Some(Ability::Overcoat),
        height: 5,
        weight: 65,
        base_exp_yield: 148,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(59, 0),
            Stat::new(85, 0),
            Stat::new(79, 0),
            Stat::new(105, 2),
            Stat::new(36, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (10, Move::Protect),
            (15, Move::BugBite),
            (20, Move::HiddenPower),
            (23, Move::Confusion),
            (26, Move::RazorLeaf),
            (29, Move::Growth),
            (32, Move::Psybeam),
            (35, Move::Captivate),
            (38, Move::Flail),
            (41, Move::Attract),
            (44, Move::Psychic),
            (47, Move::LeafStorm),
        ]
    };
    pub const Mothim: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::Swarm),
        hidden_ability: Some(Ability::TintedLens),
        height: 9,
        weight: 233,
        base_exp_yield: 148,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(94, 1),
            Stat::new(50, 0),
            Stat::new(94, 1),
            Stat::new(50, 0),
            Stat::new(66, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (10, Move::Protect),
            (15, Move::BugBite),
            (20, Move::HiddenPower),
            (23, Move::Confusion),
            (26, Move::Gust),
            (29, Move::PoisonPowder),
            (32, Move::Psybeam),
            (35, Move::Camouflage),
            (38, Move::SilverWind),
            (41, Move::AirSlash),
            (44, Move::Psychic),
            (47, Move::BugBuzz),
            (50, Move::QuiverDance),
        ]
    };
    pub const Combee: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::HoneyGather),
        hidden_ability: Some(Ability::Hustle),
        height: 3,
        weight: 55,
        base_exp_yield: 49,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(30, 0),
            Stat::new(42, 0),
            Stat::new(30, 0),
            Stat::new(42, 0),
            Stat::new(70, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::SweetScent),
            (13, Move::BugBite),
            (29, Move::BugBuzz),
        ]
    };
    pub const Vespiquen: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Unnerve),
        height: 12,
        weight: 385,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(102, 1),
            Stat::new(80, 0),
            Stat::new(102, 1),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::PoisonSting),
            (1, Move::ConfuseRay),
            (1, Move::SweetScent),
            (5, Move::FuryCutter),
            (9, Move::Pursuit),
            (13, Move::FurySwipes),
            (17, Move::DefendOrder),
            (21, Move::Slash),
            (25, Move::PowerGem),
            (29, Move::HealOrder),
            (33, Move::Toxic),
            (37, Move::AirSlash),
            (41, Move::Captivate),
            (45, Move::AttackOrder),
            (49, Move::Swagger),
            (53, Move::DestinyBond),
        ]
    };
    pub const Pachirisu: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::Pickup),
        hidden_ability: Some(Ability::VoltAbsorb),
        height: 4,
        weight: 39,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(45, 0),
            Stat::new(70, 0),
            Stat::new(45, 0),
            Stat::new(90, 0),
            Stat::new(95, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Bide),
            (5, Move::QuickAttack),
            (9, Move::Charm),
            (13, Move::Spark),
            (17, Move::Endure),
            (21, Move::Swift),
            (25, Move::ElectroBall),
            (29, Move::SweetKiss),
            (33, Move::ThunderWave),
            (37, Move::SuperFang),
            (41, Move::Discharge),
            (45, Move::LastResort),
            (49, Move::HyperFang),
        ]
    };
    pub const Buizel: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::WaterVeil),
        height: 7,
        weight: 295,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(35, 0),
            Stat::new(60, 0),
            Stat::new(30, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SonicBoom),
            (4, Move::Growl),
            (7, Move::WaterSport),
            (11, Move::QuickAttack),
            (15, Move::WaterGun),
            (18, Move::Pursuit),
            (21, Move::Swift),
            (24, Move::AquaJet),
            (27, Move::DoubleHit),
            (31, Move::Whirlpool),
            (35, Move::RazorWind),
            (38, Move::AquaTail),
            (41, Move::Agility),
            (45, Move::HydroPump),
        ]
    };
    pub const Floatzel: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::SwiftSwim),
        hidden_ability: Some(Ability::WaterVeil),
        height: 11,
        weight: 335,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(105, 0),
            Stat::new(55, 0),
            Stat::new(85, 0),
            Stat::new(50, 0),
            Stat::new(115, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::SonicBoom),
            (1, Move::QuickAttack),
            (1, Move::Crunch),
            (1, Move::WaterSport),
            (1, Move::IceFang),
            (15, Move::WaterGun),
            (18, Move::Pursuit),
            (21, Move::Swift),
            (24, Move::AquaJet),
            (29, Move::DoubleHit),
            (35, Move::Whirlpool),
            (41, Move::RazorWind),
            (46, Move::AquaTail),
            (51, Move::Agility),
            (57, Move::HydroPump),
        ]
    };
    pub const Cherubi: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Chlorophyll),
        hidden_ability: None,
        height: 4,
        weight: 33,
        base_exp_yield: 55,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(45, 0),
            Stat::new(62, 1),
            Stat::new(53, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::MorningSun),
            (7, Move::Growth),
            (10, Move::LeechSeed),
            (13, Move::HelpingHand),
            (19, Move::MagicalLeaf),
            (22, Move::SunnyDay),
            (28, Move::WorrySeed),
            (31, Move::TakeDown),
            (37, Move::SolarBeam),
            (40, Move::LuckyChant),
        ]
    };
    pub const Cherrim: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::FlowerGift),
        hidden_ability: None,
        height: 5,
        weight: 93,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(87, 2),
            Stat::new(78, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Growth),
            (1, Move::MorningSun),
            (10, Move::LeechSeed),
            (13, Move::HelpingHand),
            (19, Move::MagicalLeaf),
            (22, Move::SunnyDay),
            (25, Move::PetalDance),
            (30, Move::WorrySeed),
            (35, Move::TakeDown),
            (43, Move::SolarBeam),
            (48, Move::LuckyChant),
        ]
    };
    pub const Shellos: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::StickyHold, Ability::StormDrain),
        hidden_ability: Some(Ability::SandForce),
        height: 3,
        weight: 63,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(76, 1),
            Stat::new(48, 0),
            Stat::new(48, 0),
            Stat::new(57, 0),
            Stat::new(62, 0),
            Stat::new(34, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MudSlap),
            (2, Move::MudSport),
            (4, Move::Harden),
            (7, Move::WaterPulse),
            (11, Move::MudBomb),
            (16, Move::HiddenPower),
            (22, Move::RainDance),
            (29, Move::BodySlam),
            (37, Move::MuddyWater),
            (46, Move::Recover),
        ]
    };
    pub const Gastrodon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::StickyHold, Ability::StormDrain),
        hidden_ability: Some(Ability::SandForce),
        height: 9,
        weight: 299,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(111, 2),
            Stat::new(83, 0),
            Stat::new(68, 0),
            Stat::new(92, 0),
            Stat::new(82, 0),
            Stat::new(39, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
            (1, Move::MudSlap),
            (1, Move::MudSport),
            (1, Move::WaterPulse),
            (11, Move::MudBomb),
            (16, Move::HiddenPower),
            (22, Move::RainDance),
            (29, Move::BodySlam),
            (41, Move::MuddyWater),
            (54, Move::Recover),
        ]
    };
    pub const Ambipom: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Technician, Ability::Pickup),
        hidden_ability: Some(Ability::SkillLink),
        height: 12,
        weight: 203,
        base_exp_yield: 169,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(100, 0),
            Stat::new(66, 0),
            Stat::new(60, 0),
            Stat::new(66, 0),
            Stat::new(115, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::SandAttack),
            (1, Move::TailWhip),
            (1, Move::Astonish),
            (11, Move::BatonPass),
            (15, Move::Tickle),
            (18, Move::FurySwipes),
            (22, Move::Swift),
            (25, Move::Screech),
            (29, Move::Agility),
            (32, Move::DoubleHit),
            (36, Move::Fling),
            (39, Move::NastyPlot),
            (43, Move::LastResort),
        ]
    };
    pub const Drifloon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Flying),
        ability: PokemonAbility::Two(Ability::Aftermath, Ability::Unburden),
        hidden_ability: Some(Ability::FlareBoost),
        height: 4,
        weight: 12,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(90, 1),
            Stat::new(50, 0),
            Stat::new(34, 0),
            Stat::new(60, 0),
            Stat::new(44, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Minimize),
            (1, Move::Constrict),
            (4, Move::Astonish),
            (8, Move::Gust),
            (13, Move::FocusEnergy),
            (16, Move::Payback),
            (20, Move::OminousWind),
            (25, Move::Stockpile),
            (27, Move::Hex),
            (32, Move::SpitUp),
            (32, Move::Swallow),
            (36, Move::ShadowBall),
            (40, Move::Amnesia),
            (44, Move::BatonPass),
            (50, Move::Explosion),
        ]
    };
    pub const Drifblim: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Flying),
        ability: PokemonAbility::Two(Ability::Aftermath, Ability::Unburden),
        hidden_ability: Some(Ability::FlareBoost),
        height: 12,
        weight: 150,
        base_exp_yield: 174,
        stats: Stats(
            Stat::new(150, 2),
            Stat::new(80, 0),
            Stat::new(44, 0),
            Stat::new(90, 0),
            Stat::new(54, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Minimize),
            (1, Move::Constrict),
            (1, Move::Astonish),
            (13, Move::FocusEnergy),
            (16, Move::Payback),
            (20, Move::OminousWind),
            (25, Move::Stockpile),
            (27, Move::Hex),
            (34, Move::SpitUp),
            (34, Move::Swallow),
            (40, Move::ShadowBall),
            (46, Move::Amnesia),
            (52, Move::BatonPass),
            (60, Move::Explosion),
        ]
    };
    pub const Buneary: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::Klutz),
        hidden_ability: Some(Ability::Limber),
        height: 4,
        weight: 55,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(66, 0),
            Stat::new(44, 0),
            Stat::new(44, 0),
            Stat::new(56, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::DefenseCurl),
            (1, Move::Splash),
            (1, Move::Foresight),
            (6, Move::Endure),
            (13, Move::Frustration),
            (16, Move::QuickAttack),
            (23, Move::JumpKick),
            (26, Move::BatonPass),
            (33, Move::Agility),
            (36, Move::DizzyPunch),
            (43, Move::AfterYou),
            (46, Move::Charm),
            (53, Move::Entrainment),
            (56, Move::Bounce),
            (63, Move::HealingWish),
        ]
    };
    pub const Lopunny: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::Klutz),
        hidden_ability: Some(Ability::Limber),
        height: 12,
        weight: 333,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(76, 0),
            Stat::new(84, 0),
            Stat::new(54, 0),
            Stat::new(96, 0),
            Stat::new(105, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::DefenseCurl),
            (1, Move::Splash),
            (1, Move::Foresight),
            (1, Move::MirrorCoat),
            (1, Move::MagicCoat),
            (6, Move::Endure),
            (13, Move::Return),
            (16, Move::QuickAttack),
            (23, Move::JumpKick),
            (26, Move::BatonPass),
            (33, Move::Agility),
            (36, Move::DizzyPunch),
            (43, Move::AfterYou),
            (46, Move::Charm),
            (53, Move::Entrainment),
            (56, Move::Bounce),
            (63, Move::HealingWish),
        ]
    };
    pub const Mismagius: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 9,
        weight: 44,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(105, 1),
            Stat::new(105, 1),
            Stat::new(105, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Psywave),
            (1, Move::Spite),
            (1, Move::Astonish),
            (1, Move::MagicalLeaf),
            (1, Move::LuckyChant),
        ]
    };
    pub const Honchkrow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Flying),
        ability: PokemonAbility::Two(Ability::Insomnia, Ability::SuperLuck),
        hidden_ability: Some(Ability::Moxie),
        height: 9,
        weight: 273,
        base_exp_yield: 177,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(125, 2),
            Stat::new(52, 0),
            Stat::new(105, 0),
            Stat::new(52, 0),
            Stat::new(71, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Haze),
            (1, Move::Pursuit),
            (1, Move::Astonish),
            (25, Move::Swagger),
            (35, Move::NastyPlot),
            (45, Move::FoulPlay),
            (55, Move::NightSlash),
            (65, Move::Quash),
            (75, Move::DarkPulse),
        ]
    };
    pub const Glameow: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Limber, Ability::OwnTempo),
        hidden_ability: Some(Ability::KeenEye),
        height: 5,
        weight: 39,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(49, 0),
            Stat::new(55, 0),
            Stat::new(42, 0),
            Stat::new(42, 0),
            Stat::new(37, 0),
            Stat::new(85, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FakeOut),
            (5, Move::Scratch),
            (8, Move::Growl),
            (13, Move::Hypnosis),
            (17, Move::FeintAttack),
            (20, Move::FurySwipes),
            (25, Move::Charm),
            (29, Move::Assist),
            (32, Move::Captivate),
            (37, Move::Slash),
            (41, Move::SuckerPunch),
            (44, Move::Attract),
            (48, Move::HoneClaws),
        ]
    };
    pub const Purugly: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::ThickFat, Ability::OwnTempo),
        hidden_ability: Some(Ability::Defiant),
        height: 10,
        weight: 438,
        base_exp_yield: 158,
        stats: Stats(
            Stat::new(71, 0),
            Stat::new(82, 0),
            Stat::new(64, 0),
            Stat::new(64, 0),
            Stat::new(59, 0),
            Stat::new(112, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Growl),
            (1, Move::FakeOut),
            (13, Move::Hypnosis),
            (17, Move::FeintAttack),
            (20, Move::FurySwipes),
            (25, Move::Charm),
            (29, Move::Assist),
            (32, Move::Captivate),
            (37, Move::Slash),
            (38, Move::Swagger),
            (45, Move::BodySlam),
            (52, Move::Attract),
            (60, Move::HoneClaws),
        ]
    };
    pub const Chingling: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 2,
        weight: 6,
        base_exp_yield: 57,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(30, 0),
            Stat::new(50, 0),
            Stat::new(65, 1),
            Stat::new(50, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Wrap),
            (6, Move::Growl),
            (9, Move::Astonish),
            (14, Move::Confusion),
            (17, Move::Uproar),
            (22, Move::LastResort),
            (25, Move::Entrainment),
        ]
    };
    pub const Stunky: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Dark),
        ability: PokemonAbility::Two(Ability::Stench, Ability::Aftermath),
        hidden_ability: Some(Ability::KeenEye),
        height: 4,
        weight: 192,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(63, 0),
            Stat::new(63, 0),
            Stat::new(47, 0),
            Stat::new(41, 0),
            Stat::new(41, 0),
            Stat::new(74, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::FocusEnergy),
            (4, Move::PoisonGas),
            (7, Move::Screech),
            (10, Move::FurySwipes),
            (14, Move::Smokescreen),
            (18, Move::Feint),
            (22, Move::Slash),
            (27, Move::Toxic),
            (32, Move::AcidSpray),
            (37, Move::NightSlash),
            (43, Move::Memento),
            (49, Move::Explosion),
        ]
    };
    pub const Skuntank: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Dark),
        ability: PokemonAbility::Two(Ability::Stench, Ability::Aftermath),
        hidden_ability: Some(Ability::KeenEye),
        height: 10,
        weight: 380,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(103, 2),
            Stat::new(93, 0),
            Stat::new(67, 0),
            Stat::new(71, 0),
            Stat::new(61, 0),
            Stat::new(84, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::FocusEnergy),
            (1, Move::PoisonGas),
            (7, Move::Screech),
            (10, Move::FurySwipes),
            (14, Move::Smokescreen),
            (18, Move::Feint),
            (22, Move::Slash),
            (27, Move::Toxic),
            (32, Move::AcidSpray),
            (34, Move::Flamethrower),
            (41, Move::NightSlash),
            (51, Move::Memento),
            (61, Move::Explosion),
        ]
    };
    pub const Bronzor: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Levitate, Ability::Heatproof),
        hidden_ability: Some(Ability::HeavyMetal),
        height: 5,
        weight: 605,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(57, 0),
            Stat::new(24, 0),
            Stat::new(86, 1),
            Stat::new(24, 0),
            Stat::new(86, 0),
            Stat::new(23, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Confusion),
            (5, Move::Hypnosis),
            (9, Move::Imprison),
            (11, Move::ConfuseRay),
            (15, Move::Psywave),
            (19, Move::IronDefense),
            (21, Move::FeintAttack),
            (25, Move::Safeguard),
            (29, Move::FutureSight),
            (31, Move::MetalSound),
            (35, Move::GyroBall),
            (39, Move::Extrasensory),
            (41, Move::Payback),
            (45, Move::HealBlock),
            (49, Move::HeavySlam),
        ]
    };
    pub const Bronzong: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Psychic),
        ability: PokemonAbility::Two(Ability::Levitate, Ability::Heatproof),
        hidden_ability: Some(Ability::HeavyMetal),
        height: 13,
        weight: 1870,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(67, 0),
            Stat::new(89, 0),
            Stat::new(116, 1),
            Stat::new(79, 0),
            Stat::new(116, 1),
            Stat::new(33, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Confusion),
            (1, Move::Hypnosis),
            (1, Move::RainDance),
            (1, Move::SunnyDay),
            (1, Move::Imprison),
            (11, Move::ConfuseRay),
            (15, Move::Psywave),
            (19, Move::IronDefense),
            (21, Move::FeintAttack),
            (25, Move::Safeguard),
            (29, Move::FutureSight),
            (31, Move::MetalSound),
            (33, Move::Block),
            (36, Move::GyroBall),
            (42, Move::Extrasensory),
            (46, Move::Payback),
            (52, Move::HealBlock),
            (58, Move::HeavySlam),
        ]
    };
    pub const Bonsly: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::RockHead),
        hidden_ability: Some(Ability::Rattled),
        height: 5,
        weight: 150,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(80, 0),
            Stat::new(95, 1),
            Stat::new(10, 0),
            Stat::new(45, 0),
            Stat::new(10, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FakeTears),
            (1, Move::Copycat),
            (5, Move::Flail),
            (8, Move::LowKick),
            (12, Move::RockThrow),
            (15, Move::Slam),
            (19, Move::FeintAttack),
            (22, Move::RockTomb),
            (26, Move::Block),
            (29, Move::RockSlide),
            (33, Move::Mimic),
            (36, Move::SuckerPunch),
            (40, Move::DoubleEdge),
        ]
    };
    pub const MimeJr: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Soundproof, Ability::Filter),
        hidden_ability: Some(Ability::Technician),
        height: 6,
        weight: 130,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(20, 0),
            Stat::new(25, 0),
            Stat::new(45, 0),
            Stat::new(70, 0),
            Stat::new(90, 1),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Barrier),
            (1, Move::Tickle),
            (4, Move::Copycat),
            (8, Move::Meditate),
            (11, Move::DoubleSlap),
            (15, Move::Mimic),
            (18, Move::Encore),
            (22, Move::LightScreen),
            (22, Move::Reflect),
            (25, Move::Psybeam),
            (29, Move::Substitute),
            (32, Move::Recycle),
            (36, Move::Trick),
            (39, Move::Psychic),
            (43, Move::RolePlay),
            (46, Move::BatonPass),
            (50, Move::Safeguard),
        ]
    };
    pub const Happiny: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::NaturalCure, Ability::SereneGrace),
        hidden_ability: Some(Ability::FriendGuard),
        height: 6,
        weight: 244,
        base_exp_yield: 110,
        stats: Stats(
            Stat::new(100, 1),
            Stat::new(5, 0),
            Stat::new(5, 0),
            Stat::new(15, 0),
            Stat::new(65, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Charm),
            (5, Move::Copycat),
            (9, Move::Refresh),
            (12, Move::SweetKiss),
        ]
    };
    pub const Chatot: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::TangledFeet),
        hidden_ability: Some(Ability::BigPecks),
        height: 5,
        weight: 19,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(76, 0),
            Stat::new(65, 1),
            Stat::new(45, 0),
            Stat::new(92, 0),
            Stat::new(42, 0),
            Stat::new(91, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (5, Move::Growl),
            (9, Move::MirrorMove),
            (13, Move::Sing),
            (17, Move::FuryAttack),
            (21, Move::Chatter),
            (25, Move::Taunt),
            (29, Move::Round),
            (33, Move::Mimic),
            (37, Move::EchoedVoice),
            (41, Move::Roost),
            (45, Move::Uproar),
            (49, Move::Synchronoise),
            (53, Move::FeatherDance),
            (57, Move::HyperVoice),
        ]
    };
    pub const Spiritomb: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Dark),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Infiltrator),
        height: 10,
        weight: 1080,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(92, 0),
            Stat::new(108, 1),
            Stat::new(92, 0),
            Stat::new(108, 1),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ConfuseRay),
            (1, Move::Curse),
            (1, Move::Spite),
            (1, Move::Pursuit),
            (1, Move::ShadowSneak),
            (7, Move::FeintAttack),
            (13, Move::Hypnosis),
            (19, Move::DreamEater),
            (25, Move::OminousWind),
            (31, Move::SuckerPunch),
            (37, Move::NastyPlot),
            (43, Move::Memento),
            (49, Move::DarkPulse),
        ]
    };
    pub const Gible: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Ground),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::RoughSkin),
        height: 7,
        weight: 205,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(58, 0),
            Stat::new(70, 1),
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(42, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::SandAttack),
            (7, Move::DragonRage),
            (13, Move::Sandstorm),
            (15, Move::TakeDown),
            (19, Move::SandTomb),
            (25, Move::Slash),
            (27, Move::DragonClaw),
            (31, Move::Dig),
            (37, Move::DragonRush),
        ]
    };
    pub const Gabite: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Ground),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::RoughSkin),
        height: 14,
        weight: 560,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(68, 0),
            Stat::new(90, 2),
            Stat::new(65, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(82, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (7, Move::DragonRage),
            (13, Move::Sandstorm),
            (15, Move::TakeDown),
            (19, Move::SandTomb),
            (24, Move::DualChop),
            (28, Move::Slash),
            (33, Move::DragonClaw),
            (40, Move::Dig),
            (49, Move::DragonRush),
        ]
    };
    pub const Garchomp: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Ground),
        ability: PokemonAbility::One(Ability::SandVeil),
        hidden_ability: Some(Ability::RoughSkin),
        height: 19,
        weight: 950,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(108, 0),
            Stat::new(130, 3),
            Stat::new(95, 0),
            Stat::new(80, 0),
            Stat::new(85, 0),
            Stat::new(102, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (1, Move::DragonRage),
            (1, Move::Sandstorm),
            (1, Move::FireFang),
            (15, Move::TakeDown),
            (19, Move::SandTomb),
            (24, Move::DualChop),
            (28, Move::Slash),
            (33, Move::DragonClaw),
            (40, Move::Dig),
            (48, Move::Crunch),
            (55, Move::DragonRush),
        ]
    };
    pub const Munchlax: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Pickup, Ability::ThickFat),
        hidden_ability: Some(Ability::Gluttony),
        height: 6,
        weight: 1050,
        base_exp_yield: 78,
        stats: Stats(
            Stat::new(135, 1),
            Stat::new(85, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(85, 0),
            Stat::new(5, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Metronome),
            (1, Move::OdorSleuth),
            (4, Move::DefenseCurl),
            (9, Move::Amnesia),
            (12, Move::Lick),
            (17, Move::Recycle),
            (20, Move::Screech),
            (25, Move::ChipAway),
            (28, Move::Stockpile),
            (33, Move::Swallow),
            (36, Move::BodySlam),
            (41, Move::Fling),
            (44, Move::Rollout),
            (49, Move::NaturalGift),
            (52, Move::Snatch),
            (57, Move::LastResort),
        ]
    };
    pub const Riolu: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Steadfast, Ability::InnerFocus),
        hidden_ability: Some(Ability::Prankster),
        height: 7,
        weight: 202,
        base_exp_yield: 57,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(70, 1),
            Stat::new(40, 0),
            Stat::new(35, 0),
            Stat::new(40, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::QuickAttack),
            (1, Move::Foresight),
            (1, Move::Endure),
            (6, Move::Counter),
            (11, Move::Feint),
            (15, Move::ForcePalm),
            (19, Move::Copycat),
            (24, Move::Screech),
            (29, Move::Reversal),
            (47, Move::NastyPlot),
            (55, Move::FinalGambit),
        ]
    };
    pub const Lucario: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fighting, Type::Steel),
        ability: PokemonAbility::Two(Ability::Steadfast, Ability::InnerFocus),
        hidden_ability: Some(Ability::Justified),
        height: 12,
        weight: 540,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(110, 1),
            Stat::new(70, 0),
            Stat::new(115, 1),
            Stat::new(70, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::QuickAttack),
            (1, Move::Foresight),
            (1, Move::Detect),
            (1, Move::MetalClaw),
            (1, Move::DarkPulse),
            (6, Move::Counter),
            (11, Move::Feint),
            (15, Move::ForcePalm),
            (19, Move::MeFirst),
            (24, Move::MetalSound),
            (29, Move::BoneRush),
            (33, Move::QuickGuard),
            (37, Move::SwordsDance),
            (42, Move::HealPulse),
            (47, Move::CalmMind),
            (51, Move::AuraSphere),
            (55, Move::CloseCombat),
            (60, Move::DragonPulse),
            (65, Move::ExtremeSpeed),
        ]
    };
    pub const Hippopotas: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::SandStream),
        hidden_ability: Some(Ability::SandForce),
        height: 8,
        weight: 495,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(68, 0),
            Stat::new(72, 0),
            Stat::new(78, 1),
            Stat::new(38, 0),
            Stat::new(42, 0),
            Stat::new(32, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (7, Move::Bite),
            (13, Move::Yawn),
            (19, Move::TakeDown),
            (19, Move::Dig),
            (25, Move::SandTomb),
            (31, Move::Crunch),
            (37, Move::Earthquake),
            (44, Move::DoubleEdge),
            (50, Move::Fissure),
        ]
    };
    pub const Hippowdon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::One(Ability::SandStream),
        hidden_ability: Some(Ability::SandForce),
        height: 20,
        weight: 3000,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(108, 0),
            Stat::new(112, 0),
            Stat::new(118, 2),
            Stat::new(68, 0),
            Stat::new(72, 0),
            Stat::new(47, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (1, Move::Bite),
            (1, Move::Yawn),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (19, Move::TakeDown),
            (19, Move::Dig),
            (25, Move::SandTomb),
            (31, Move::Crunch),
            (40, Move::Earthquake),
            (50, Move::DoubleEdge),
            (60, Move::Fissure),
        ]
    };
    pub const Skorupi: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Bug),
        ability: PokemonAbility::Two(Ability::BattleArmor, Ability::Sniper),
        hidden_ability: Some(Ability::KeenEye),
        height: 8,
        weight: 120,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(90, 1),
            Stat::new(30, 0),
            Stat::new(55, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Leer),
            (1, Move::Bite),
            (5, Move::KnockOff),
            (9, Move::PinMissile),
            (13, Move::Acupressure),
            (16, Move::Pursuit),
            (20, Move::BugBite),
            (23, Move::PoisonFang),
            (27, Move::Venoshock),
            (30, Move::HoneClaws),
            (34, Move::ToxicSpikes),
            (38, Move::NightSlash),
            (41, Move::ScaryFace),
            (45, Move::Crunch),
            (49, Move::CrossPoison),
        ]
    };
    pub const Drapion: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Dark),
        ability: PokemonAbility::Two(Ability::BattleArmor, Ability::Sniper),
        hidden_ability: Some(Ability::KeenEye),
        height: 13,
        weight: 615,
        base_exp_yield: 175,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(90, 0),
            Stat::new(110, 2),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::KnockOff),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (9, Move::PinMissile),
            (13, Move::Acupressure),
            (16, Move::Pursuit),
            (20, Move::BugBite),
            (23, Move::PoisonFang),
            (27, Move::Venoshock),
            (30, Move::HoneClaws),
            (34, Move::ToxicSpikes),
            (38, Move::NightSlash),
            (43, Move::ScaryFace),
            (49, Move::Crunch),
            (57, Move::CrossPoison),
        ]
    };
    pub const Croagunk: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Fighting),
        ability: PokemonAbility::Two(Ability::Anticipation, Ability::DrySkin),
        hidden_ability: Some(Ability::PoisonTouch),
        height: 7,
        weight: 230,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(48, 0),
            Stat::new(61, 1),
            Stat::new(40, 0),
            Stat::new(61, 0),
            Stat::new(40, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Astonish),
            (3, Move::MudSlap),
            (8, Move::PoisonSting),
            (10, Move::Taunt),
            (15, Move::Pursuit),
            (17, Move::FeintAttack),
            (22, Move::Revenge),
            (24, Move::Swagger),
            (29, Move::MudBomb),
            (31, Move::SuckerPunch),
            (36, Move::Venoshock),
            (38, Move::NastyPlot),
            (43, Move::PoisonJab),
            (45, Move::SludgeBomb),
            (50, Move::Flatter),
        ]
    };
    pub const Toxicroak: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Poison, Type::Fighting),
        ability: PokemonAbility::Two(Ability::Anticipation, Ability::DrySkin),
        hidden_ability: Some(Ability::PoisonTouch),
        height: 13,
        weight: 444,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(83, 0),
            Stat::new(106, 2),
            Stat::new(65, 0),
            Stat::new(86, 0),
            Stat::new(65, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::MudSlap),
            (1, Move::Astonish),
            (10, Move::Taunt),
            (15, Move::Pursuit),
            (17, Move::FeintAttack),
            (22, Move::Revenge),
            (24, Move::Swagger),
            (29, Move::MudBomb),
            (31, Move::SuckerPunch),
            (36, Move::Venoshock),
            (41, Move::NastyPlot),
            (49, Move::PoisonJab),
            (54, Move::SludgeBomb),
            (62, Move::Flatter),
        ]
    };
    pub const Carnivine: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 14,
        weight: 270,
        base_exp_yield: 159,
        stats: Stats(
            Stat::new(74, 0),
            Stat::new(100, 2),
            Stat::new(72, 0),
            Stat::new(90, 0),
            Stat::new(72, 0),
            Stat::new(46, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bind),
            (1, Move::Growth),
            (7, Move::Bite),
            (11, Move::VineWhip),
            (17, Move::SweetScent),
            (21, Move::Ingrain),
            (27, Move::FeintAttack),
            (31, Move::LeafTornado),
            (37, Move::Stockpile),
            (37, Move::SpitUp),
            (37, Move::Swallow),
            (41, Move::Crunch),
            (47, Move::WringOut),
            (51, Move::PowerWhip),
        ]
    };
    pub const Finneon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::StormDrain),
        hidden_ability: Some(Ability::WaterVeil),
        height: 4,
        weight: 70,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(49, 0),
            Stat::new(49, 0),
            Stat::new(56, 0),
            Stat::new(49, 0),
            Stat::new(61, 0),
            Stat::new(66, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (6, Move::WaterGun),
            (10, Move::Attract),
            (13, Move::RainDance),
            (17, Move::Gust),
            (22, Move::WaterPulse),
            (26, Move::Captivate),
            (29, Move::Safeguard),
            (33, Move::AquaRing),
            (38, Move::Whirlpool),
            (42, Move::UTurn),
            (45, Move::Bounce),
            (49, Move::SilverWind),
            (54, Move::Soak),
        ]
    };
    pub const Lumineon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::StormDrain),
        hidden_ability: Some(Ability::WaterVeil),
        height: 12,
        weight: 240,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(69, 0),
            Stat::new(69, 0),
            Stat::new(76, 0),
            Stat::new(69, 0),
            Stat::new(86, 0),
            Stat::new(91, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::WaterGun),
            (1, Move::Attract),
            (13, Move::RainDance),
            (17, Move::Gust),
            (22, Move::WaterPulse),
            (26, Move::Captivate),
            (29, Move::Safeguard),
            (35, Move::AquaRing),
            (42, Move::Whirlpool),
            (48, Move::UTurn),
            (53, Move::Bounce),
            (59, Move::SilverWind),
            (66, Move::Soak),
        ]
    };
    pub const Mantyke: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::WaterAbsorb),
        hidden_ability: Some(Ability::WaterVeil),
        height: 10,
        weight: 650,
        base_exp_yield: 69,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(20, 0),
            Stat::new(50, 0),
            Stat::new(60, 0),
            Stat::new(120, 1),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Bubble),
            (3, Move::Supersonic),
            (7, Move::BubbleBeam),
            (11, Move::ConfuseRay),
            (14, Move::WingAttack),
            (16, Move::Headbutt),
            (19, Move::WaterPulse),
            (23, Move::WideGuard),
            (27, Move::TakeDown),
            (32, Move::Agility),
            (36, Move::AirSlash),
            (39, Move::AquaRing),
            (46, Move::Bounce),
            (49, Move::HydroPump),
        ]
    };
    pub const Snover: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Ice),
        ability: PokemonAbility::One(Ability::SnowWarning),
        hidden_ability: Some(Ability::Soundproof),
        height: 10,
        weight: 505,
        base_exp_yield: 67,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(62, 1),
            Stat::new(50, 0),
            Stat::new(62, 0),
            Stat::new(60, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::PowderSnow),
            (5, Move::RazorLeaf),
            (9, Move::IcyWind),
            (13, Move::GrassWhistle),
            (17, Move::Swagger),
            (21, Move::Mist),
            (26, Move::IceShard),
            (31, Move::Ingrain),
            (36, Move::WoodHammer),
            (41, Move::Blizzard),
            (46, Move::SheerCold),
        ]
    };
    pub const Abomasnow: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Ice),
        ability: PokemonAbility::One(Ability::SnowWarning),
        hidden_ability: Some(Ability::Soundproof),
        height: 22,
        weight: 1355,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(92, 1),
            Stat::new(75, 0),
            Stat::new(92, 1),
            Stat::new(85, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::IcePunch),
            (1, Move::Leer),
            (1, Move::RazorLeaf),
            (1, Move::PowderSnow),
            (1, Move::IcyWind),
            (13, Move::GrassWhistle),
            (17, Move::Swagger),
            (21, Move::Mist),
            (26, Move::IceShard),
            (31, Move::Ingrain),
            (36, Move::WoodHammer),
            (47, Move::Blizzard),
            (58, Move::SheerCold),
        ]
    };
    pub const Weavile: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Ice),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Pickpocket),
        height: 11,
        weight: 340,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(120, 1),
            Stat::new(65, 0),
            Stat::new(45, 0),
            Stat::new(85, 0),
            Stat::new(125, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (1, Move::Taunt),
            (1, Move::Revenge),
            (1, Move::Assurance),
            (1, Move::Embargo),
            (10, Move::FeintAttack),
            (14, Move::IcyWind),
            (16, Move::FurySwipes),
            (20, Move::NastyPlot),
            (22, Move::MetalClaw),
            (25, Move::HoneClaws),
            (28, Move::Fling),
            (32, Move::Screech),
            (35, Move::NightSlash),
            (40, Move::Snatch),
            (44, Move::Punishment),
            (47, Move::DarkPulse),
        ]
    };
    pub const Magnezone: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Steel),
        ability: PokemonAbility::Two(Ability::MagnetPull, Ability::Sturdy),
        hidden_ability: Some(Ability::Analytic),
        height: 12,
        weight: 1800,
        base_exp_yield: 241,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(70, 0),
            Stat::new(115, 0),
            Stat::new(130, 3),
            Stat::new(90, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Supersonic),
            (1, Move::SonicBoom),
            (1, Move::ThunderShock),
            (1, Move::Barrier),
            (1, Move::MirrorCoat),
            (15, Move::ThunderWave),
            (18, Move::MagnetBomb),
            (21, Move::Spark),
            (25, Move::MirrorShot),
            (29, Move::MetalSound),
            (34, Move::ElectroBall),
            (39, Move::FlashCannon),
            (45, Move::Screech),
            (51, Move::Discharge),
            (56, Move::LockOn),
            (62, Move::MagnetRise),
            (67, Move::GyroBall),
            (73, Move::ZapCannon),
        ]
    };
    pub const Lickilicky: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::OwnTempo, Ability::Oblivious),
        hidden_ability: Some(Ability::CloudNine),
        height: 17,
        weight: 1400,
        base_exp_yield: 180,
        stats: Stats(
            Stat::new(110, 3),
            Stat::new(85, 0),
            Stat::new(95, 0),
            Stat::new(80, 0),
            Stat::new(95, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Monster),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Lick),
            (5, Move::Supersonic),
            (9, Move::DefenseCurl),
            (13, Move::KnockOff),
            (17, Move::Wrap),
            (21, Move::Stomp),
            (25, Move::Disable),
            (29, Move::Slam),
            (33, Move::Rollout),
            (37, Move::ChipAway),
            (41, Move::MeFirst),
            (45, Move::Refresh),
            (49, Move::Screech),
            (53, Move::PowerWhip),
            (57, Move::WringOut),
            (61, Move::GyroBall),
        ]
    };
    pub const Rhyperior: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Rock),
        ability: PokemonAbility::Two(Ability::LightningRod, Ability::SolidRock),
        hidden_ability: Some(Ability::Reckless),
        height: 24,
        weight: 2828,
        base_exp_yield: 241,
        stats: Stats(
            Stat::new(115, 0),
            Stat::new(140, 3),
            Stat::new(130, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Stomp),
            (1, Move::HornAttack),
            (1, Move::FuryAttack),
            (1, Move::TailWhip),
            (1, Move::PoisonJab),
            (19, Move::ScaryFace),
            (23, Move::RockBlast),
            (30, Move::ChipAway),
            (41, Move::TakeDown),
            (42, Move::HammerArm),
            (47, Move::DrillRun),
            (56, Move::StoneEdge),
            (62, Move::Earthquake),
            (71, Move::HornDrill),
            (77, Move::Megahorn),
            (86, Move::RockWrecker),
        ]
    };
    pub const Tangrowth: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::LeafGuard),
        hidden_ability: Some(Ability::Regenerator),
        height: 20,
        weight: 1286,
        base_exp_yield: 187,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(125, 2),
            Stat::new(110, 0),
            Stat::new(50, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Constrict),
            (1, Move::Ingrain),
            (4, Move::SleepPowder),
            (7, Move::VineWhip),
            (10, Move::Absorb),
            (14, Move::PoisonPowder),
            (17, Move::Bind),
            (20, Move::Growth),
            (23, Move::MegaDrain),
            (27, Move::KnockOff),
            (30, Move::StunSpore),
            (33, Move::NaturalGift),
            (36, Move::GigaDrain),
            (40, Move::AncientPower),
            (43, Move::Slam),
            (46, Move::Tickle),
            (49, Move::WringOut),
            (53, Move::PowerWhip),
            (56, Move::Block),
        ]
    };
    pub const Electivire: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::MotorDrive),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 18,
        weight: 1386,
        base_exp_yield: 243,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(123, 3),
            Stat::new(67, 0),
            Stat::new(95, 0),
            Stat::new(85, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::Leer),
            (1, Move::LowKick),
            (1, Move::ThunderShock),
            (1, Move::QuickAttack),
            (12, Move::Swift),
            (15, Move::ShockWave),
            (19, Move::ThunderWave),
            (22, Move::ElectroBall),
            (26, Move::LightScreen),
            (29, Move::ThunderPunch),
            (36, Move::Discharge),
            (42, Move::Screech),
            (49, Move::Thunderbolt),
            (55, Move::Thunder),
            (62, Move::GigaImpact),
        ]
    };
    pub const Magmortar: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::FlameBody),
        hidden_ability: Some(Ability::VitalSpirit),
        height: 16,
        weight: 680,
        base_exp_yield: 243,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(95, 0),
            Stat::new(67, 0),
            Stat::new(125, 3),
            Stat::new(95, 0),
            Stat::new(83, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ThunderPunch),
            (1, Move::Leer),
            (1, Move::Ember),
            (1, Move::Smokescreen),
            (1, Move::Smog),
            (12, Move::FeintAttack),
            (15, Move::FireSpin),
            (19, Move::ClearSmog),
            (22, Move::FlameBurst),
            (26, Move::ConfuseRay),
            (29, Move::FirePunch),
            (36, Move::LavaPlume),
            (42, Move::SunnyDay),
            (49, Move::Flamethrower),
            (55, Move::FireBlast),
            (62, Move::HyperBeam),
        ]
    };
    pub const Togekiss: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fairy, Type::Flying),
        ability: PokemonAbility::Two(Ability::Hustle, Ability::SereneGrace),
        hidden_ability: Some(Ability::SuperLuck),
        height: 15,
        weight: 380,
        base_exp_yield: 245,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(50, 0),
            Stat::new(95, 0),
            Stat::new(120, 2),
            Stat::new(115, 1),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SkyAttack),
            (1, Move::ExtremeSpeed),
            (1, Move::AuraSphere),
            (1, Move::AirSlash),
        ]
    };
    pub const Yanmega: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Flying),
        ability: PokemonAbility::Two(Ability::SpeedBoost, Ability::TintedLens),
        hidden_ability: Some(Ability::Frisk),
        height: 19,
        weight: 515,
        base_exp_yield: 180,
        stats: Stats(
            Stat::new(86, 0),
            Stat::new(76, 2),
            Stat::new(86, 0),
            Stat::new(116, 0),
            Stat::new(56, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::QuickAttack),
            (1, Move::DoubleTeam),
            (1, Move::Foresight),
            (1, Move::NightSlash),
            (1, Move::BugBite),
            (14, Move::SonicBoom),
            (17, Move::Detect),
            (22, Move::Supersonic),
            (27, Move::Uproar),
            (30, Move::Pursuit),
            (33, Move::AncientPower),
            (38, Move::Feint),
            (43, Move::Slash),
            (46, Move::Screech),
            (49, Move::UTurn),
            (54, Move::AirSlash),
            (57, Move::BugBuzz),
        ]
    };
    pub const Leafeon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::LeafGuard),
        hidden_ability: Some(Ability::Chlorophyll),
        height: 10,
        weight: 255,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(110, 0),
            Stat::new(130, 2),
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::RazorLeaf),
            (13, Move::QuickAttack),
            (17, Move::GrassWhistle),
            (21, Move::MagicalLeaf),
            (25, Move::GigaDrain),
            (29, Move::SwordsDance),
            (33, Move::Synthesis),
            (37, Move::SunnyDay),
            (41, Move::LastResort),
            (45, Move::LeafBlade),
        ]
    };
    pub const Glaceon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::One(Ability::SnowCloak),
        hidden_ability: Some(Ability::IceBody),
        height: 8,
        weight: 259,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(110, 0),
            Stat::new(130, 2),
            Stat::new(95, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::HelpingHand),
            (5, Move::SandAttack),
            (9, Move::IcyWind),
            (13, Move::QuickAttack),
            (17, Move::Bite),
            (21, Move::IceFang),
            (25, Move::IceShard),
            (29, Move::Barrier),
            (33, Move::MirrorCoat),
            (37, Move::Hail),
            (41, Move::LastResort),
            (45, Move::Blizzard),
        ]
    };
    pub const Gliscor: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Flying),
        ability: PokemonAbility::Two(Ability::HyperCutter, Ability::SandVeil),
        hidden_ability: Some(Ability::PoisonHeal),
        height: 20,
        weight: 425,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(95, 0),
            Stat::new(125, 2),
            Stat::new(45, 0),
            Stat::new(75, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Harden),
            (1, Move::KnockOff),
            (1, Move::PoisonJab),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (13, Move::QuickAttack),
            (16, Move::FuryCutter),
            (19, Move::FeintAttack),
            (22, Move::Acrobatics),
            (27, Move::NightSlash),
            (30, Move::UTurn),
            (35, Move::Screech),
            (40, Move::XScissor),
            (45, Move::SkyUppercut),
            (50, Move::SwordsDance),
            (55, Move::Guillotine),
        ]
    };
    pub const Mamoswine: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Ground),
        ability: PokemonAbility::Two(Ability::Oblivious, Ability::SnowCloak),
        hidden_ability: Some(Ability::ThickFat),
        height: 25,
        weight: 2910,
        base_exp_yield: 239,
        stats: Stats(
            Stat::new(110, 0),
            Stat::new(130, 3),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (1, Move::PowderSnow),
            (1, Move::AncientPower),
            (1, Move::MudSport),
            (1, Move::OdorSleuth),
            (11, Move::MudSlap),
            (14, Move::Endure),
            (18, Move::MudBomb),
            (21, Move::Hail),
            (24, Move::IceFang),
            (28, Move::TakeDown),
            (33, Move::DoubleHit),
            (37, Move::Mist),
            (41, Move::Thrash),
            (46, Move::Earthquake),
            (52, Move::Blizzard),
            (58, Move::ScaryFace),
        ]
    };
    pub const PorygonZ: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Adaptability, Ability::Download),
        hidden_ability: Some(Ability::Analytic),
        height: 9,
        weight: 340,
        base_exp_yield: 241,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(135, 3),
            Stat::new(75, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Conversion),
            (1, Move::Conversion2),
            (1, Move::NastyPlot),
            (1, Move::TrickRoom),
            (7, Move::Psybeam),
            (12, Move::Agility),
            (18, Move::Recover),
            (23, Move::MagnetRise),
            (29, Move::SignalBeam),
            (34, Move::Embargo),
            (40, Move::Discharge),
            (45, Move::LockOn),
            (51, Move::TriAttack),
            (56, Move::MagicCoat),
            (62, Move::ZapCannon),
            (67, Move::HyperBeam),
        ]
    };
    pub const Gallade: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fighting),
        ability: PokemonAbility::One(Ability::Steadfast),
        hidden_ability: Some(Ability::Justified),
        height: 16,
        weight: 520,
        base_exp_yield: 233,
        stats: Stats(
            Stat::new(68, 0),
            Stat::new(125, 3),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(115, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Confusion),
            (1, Move::Teleport),
            (1, Move::DoubleTeam),
            (1, Move::LeafBlade),
            (1, Move::NightSlash),
            (17, Move::FuryCutter),
            (22, Move::Slash),
            (25, Move::HealPulse),
            (31, Move::SwordsDance),
            (36, Move::PsychoCut),
            (39, Move::HelpingHand),
            (45, Move::Feint),
            (50, Move::FalseSwipe),
            (53, Move::Protect),
            (59, Move::CloseCombat),
            (64, Move::StoredPower),
        ]
    };
    pub const Probopass: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Steel),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::MagnetPull),
        hidden_ability: Some(Ability::SandForce),
        height: 14,
        weight: 3400,
        base_exp_yield: 184,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(55, 0),
            Stat::new(145, 1),
            Stat::new(75, 0),
            Stat::new(150, 2),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::IronDefense),
            (1, Move::Block),
            (1, Move::Gravity),
            (1, Move::MagnetRise),
            (1, Move::MagnetBomb),
            (15, Move::ThunderWave),
            (18, Move::RockBlast),
            (22, Move::Rest),
            (25, Move::Spark),
            (29, Move::RockSlide),
            (32, Move::PowerGem),
            (36, Move::Sandstorm),
            (39, Move::Discharge),
            (43, Move::EarthPower),
            (46, Move::StoneEdge),
            (50, Move::ZapCannon),
            (50, Move::LockOn),
        ]
    };
    pub const Dusknoir: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Frisk),
        height: 22,
        weight: 1066,
        base_exp_yield: 236,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(100, 0),
            Stat::new(135, 1),
            Stat::new(65, 0),
            Stat::new(135, 2),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::IcePunch),
            (1, Move::ThunderPunch),
            (1, Move::Bind),
            (1, Move::Leer),
            (1, Move::Disable),
            (1, Move::NightShade),
            (1, Move::Gravity),
            (9, Move::Foresight),
            (14, Move::Astonish),
            (17, Move::ConfuseRay),
            (22, Move::ShadowSneak),
            (25, Move::Pursuit),
            (30, Move::Curse),
            (33, Move::WillOWisp),
            (37, Move::ShadowPunch),
            (42, Move::Hex),
            (49, Move::MeanLook),
            (58, Move::Payback),
            (61, Move::FutureSight),
        ]
    };
    pub const Froslass: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ice, Type::Ghost),
        ability: PokemonAbility::One(Ability::SnowCloak),
        hidden_ability: Some(Ability::CursedBody),
        height: 13,
        weight: 266,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(110, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::DoubleTeam),
            (1, Move::PowderSnow),
            (1, Move::Astonish),
            (13, Move::IcyWind),
            (19, Move::ConfuseRay),
            (22, Move::OminousWind),
            (28, Move::WakeUpSlap),
            (31, Move::Captivate),
            (37, Move::IceShard),
            (40, Move::Hail),
            (51, Move::Blizzard),
            (59, Move::DestinyBond),
        ]
    };
    pub const Rotom: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Ghost),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 3,
        weight: 3,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(77, 0),
            Stat::new(95, 1),
            Stat::new(77, 0),
            Stat::new(91, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ThunderShock),
            (1, Move::ThunderWave),
            (1, Move::ConfuseRay),
            (1, Move::Trick),
            (1, Move::Astonish),
            (8, Move::Uproar),
            (15, Move::DoubleTeam),
            (22, Move::ShockWave),
            (29, Move::OminousWind),
            (36, Move::Substitute),
            (43, Move::ElectroBall),
            (50, Move::Hex),
            (57, Move::Charge),
            (64, Move::Discharge),
        ]
    };
    pub const Uxie: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 3,
        weight: 3,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(130, 2),
            Stat::new(75, 0),
            Stat::new(130, 1),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Rest),
            (6, Move::Imprison),
            (16, Move::Endure),
            (21, Move::Swift),
            (31, Move::Yawn),
            (36, Move::FutureSight),
            (46, Move::Amnesia),
            (51, Move::Extrasensory),
            (61, Move::Flail),
            (66, Move::NaturalGift),
            (76, Move::Memento),
        ]
    };
    pub const Mesprit: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 3,
        weight: 3,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(105, 1),
            Stat::new(105, 0),
            Stat::new(105, 1),
            Stat::new(105, 1),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Rest),
            (6, Move::Imprison),
            (16, Move::Protect),
            (21, Move::Swift),
            (31, Move::LuckyChant),
            (36, Move::FutureSight),
            (46, Move::Charm),
            (51, Move::Extrasensory),
            (61, Move::Copycat),
            (66, Move::NaturalGift),
            (76, Move::HealingWish),
        ]
    };
    pub const Azelf: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 3,
        weight: 3,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(125, 2),
            Stat::new(70, 0),
            Stat::new(125, 1),
            Stat::new(70, 0),
            Stat::new(115, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::Rest),
            (6, Move::Imprison),
            (16, Move::Detect),
            (21, Move::Swift),
            (31, Move::Uproar),
            (36, Move::FutureSight),
            (46, Move::NastyPlot),
            (51, Move::Extrasensory),
            (61, Move::LastResort),
            (66, Move::NaturalGift),
            (76, Move::Explosion),
        ]
    };
    pub const Dialga: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Dragon),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Telepathy),
        height: 54,
        weight: 6830,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(120, 0),
            Stat::new(120, 0),
            Stat::new(150, 3),
            Stat::new(100, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ScaryFace),
            (1, Move::DragonBreath),
            (6, Move::MetalClaw),
            (10, Move::AncientPower),
            (15, Move::Slash),
            (19, Move::PowerGem),
            (24, Move::MetalBurst),
            (28, Move::DragonClaw),
            (33, Move::EarthPower),
            (37, Move::AuraSphere),
            (42, Move::IronTail),
            (46, Move::RoarOfTime),
            (50, Move::FlashCannon),
        ]
    };
    pub const Palkia: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Dragon),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Telepathy),
        height: 42,
        weight: 3360,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(120, 0),
            Stat::new(100, 0),
            Stat::new(150, 3),
            Stat::new(120, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ScaryFace),
            (1, Move::DragonBreath),
            (6, Move::WaterPulse),
            (10, Move::AncientPower),
            (15, Move::Slash),
            (19, Move::PowerGem),
            (24, Move::AquaTail),
            (28, Move::DragonClaw),
            (33, Move::EarthPower),
            (37, Move::AuraSphere),
            (46, Move::SpacialRend),
            (50, Move::HydroPump),
        ]
    };
    pub const Heatran: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Steel),
        ability: PokemonAbility::One(Ability::FlashFire),
        hidden_ability: Some(Ability::FlameBody),
        height: 17,
        weight: 4300,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(91, 0),
            Stat::new(90, 0),
            Stat::new(106, 0),
            Stat::new(130, 3),
            Stat::new(106, 0),
            Stat::new(77, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::AncientPower),
            (9, Move::Leer),
            (17, Move::FireFang),
            (25, Move::MetalSound),
            (33, Move::Crunch),
            (41, Move::ScaryFace),
            (49, Move::LavaPlume),
            (57, Move::FireSpin),
            (65, Move::IronHead),
            (73, Move::EarthPower),
            (81, Move::HeatWave),
            (88, Move::StoneEdge),
            (96, Move::MagmaStorm),
        ]
    };
    pub const Regigigas: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::SlowStart),
        hidden_ability: None,
        height: 37,
        weight: 4200,
        base_exp_yield: 302,
        stats: Stats(
            Stat::new(110, 0),
            Stat::new(160, 3),
            Stat::new(110, 0),
            Stat::new(80, 0),
            Stat::new(110, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FirePunch),
            (1, Move::IcePunch),
            (1, Move::ThunderPunch),
            (1, Move::ConfuseRay),
            (1, Move::DizzyPunch),
            (1, Move::Foresight),
            (1, Move::KnockOff),
            (25, Move::Revenge),
            (40, Move::WideGuard),
            (50, Move::ZenHeadbutt),
            (65, Move::Payback),
            (75, Move::CrushGrip),
            (90, Move::HeavySlam),
            (100, Move::GigaImpact),
        ]
    };
    pub const GiratinaAltered: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Dragon),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: Some(Ability::Telepathy),
        height: 45,
        weight: 7500,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(150, 3),
            Stat::new(100, 0),
            Stat::new(120, 0),
            Stat::new(100, 0),
            Stat::new(120, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ScaryFace),
            (1, Move::DragonBreath),
            (6, Move::OminousWind),
            (10, Move::AncientPower),
            (15, Move::Slash),
            (19, Move::ShadowSneak),
            (24, Move::DestinyBond),
            (28, Move::DragonClaw),
            (33, Move::EarthPower),
            (37, Move::AuraSphere),
            (42, Move::ShadowClaw),
            (46, Move::ShadowForce),
            (50, Move::Hex),
        ]
    };
    pub const Cresselia: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 15,
        weight: 856,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(120, 0),
            Stat::new(70, 0),
            Stat::new(120, 0),
            Stat::new(75, 0),
            Stat::new(130, 3),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::DoubleTeam),
            (11, Move::Safeguard),
            (20, Move::Mist),
            (29, Move::AuroraBeam),
            (38, Move::FutureSight),
            (47, Move::Slash),
            (57, Move::Moonlight),
            (66, Move::PsychoCut),
            (75, Move::PsychoShift),
            (84, Move::LunarDance),
            (93, Move::Psychic),
        ]
    };
    pub const Phione: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Hydration),
        hidden_ability: None,
        height: 4,
        weight: 31,
        base_exp_yield: 216,
        stats: Stats(
            Stat::new(80, 1),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (1, Move::WaterSport),
            (9, Move::Charm),
            (16, Move::Supersonic),
            (24, Move::BubbleBeam),
            (31, Move::AcidArmor),
            (39, Move::Whirlpool),
            (46, Move::WaterPulse),
            (54, Move::AquaRing),
            (61, Move::Dive),
            (69, Move::RainDance),
        ]
    };
    pub const Manaphy: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Hydration),
        hidden_ability: None,
        height: 3,
        weight: 14,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (1, Move::TailGlow),
            (1, Move::WaterSport),
            (9, Move::Charm),
            (16, Move::Supersonic),
            (24, Move::BubbleBeam),
            (31, Move::AcidArmor),
            (39, Move::Whirlpool),
            (46, Move::WaterPulse),
            (54, Move::AquaRing),
            (61, Move::Dive),
            (69, Move::RainDance),
            (76, Move::HeartSwap),
        ]
    };
    pub const Darkrai: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::One(Ability::BadDreams),
        hidden_ability: None,
        height: 15,
        weight: 505,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(90, 0),
            Stat::new(90, 0),
            Stat::new(135, 2),
            Stat::new(90, 0),
            Stat::new(125, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Disable),
            (1, Move::OminousWind),
            (11, Move::QuickAttack),
            (20, Move::Hypnosis),
            (29, Move::FeintAttack),
            (38, Move::Nightmare),
            (47, Move::DoubleTeam),
            (57, Move::Haze),
            (66, Move::DarkVoid),
            (75, Move::NastyPlot),
            (84, Move::DreamEater),
            (93, Move::DarkPulse),
        ]
    };
    pub const ShayminLand: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::NaturalCure),
        hidden_ability: None,
        height: 2,
        weight: 21,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growth),
            (10, Move::MagicalLeaf),
            (19, Move::LeechSeed),
            (28, Move::Synthesis),
            (37, Move::SweetScent),
            (46, Move::NaturalGift),
            (55, Move::WorrySeed),
            (64, Move::Aromatherapy),
            (73, Move::EnergyBall),
            (82, Move::SweetKiss),
            (91, Move::HealingWish),
            (100, Move::SeedFlare),
        ]
    };
    pub const Arceus: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::One(Ability::Multitype),
        hidden_ability: None,
        height: 32,
        weight: 3200,
        base_exp_yield: 324,
        stats: Stats(
            Stat::new(120, 3),
            Stat::new(120, 0),
            Stat::new(120, 0),
            Stat::new(120, 0),
            Stat::new(120, 0),
            Stat::new(120, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SeismicToss),
            (1, Move::CosmicPower),
            (1, Move::NaturalGift),
            (1, Move::Punishment),
            (10, Move::Gravity),
            (20, Move::EarthPower),
            (30, Move::HyperVoice),
            (40, Move::ExtremeSpeed),
            (50, Move::Refresh),
            (60, Move::FutureSight),
            (70, Move::Recover),
            (80, Move::HyperBeam),
            (90, Move::PerishSong),
            (100, Move::Judgment),
        ]
    };
    pub const Victini: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Fire),
        ability: PokemonAbility::One(Ability::VictoryStar),
        hidden_ability: None,
        height: 4,
        weight: 40,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 3),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (1, Move::QuickAttack),
            (1, Move::FocusEnergy),
            (1, Move::Incinerate),
            (1, Move::SearingShot),
            (9, Move::Endure),
            (17, Move::Headbutt),
            (25, Move::FlameCharge),
            (33, Move::Reversal),
            (41, Move::FlameBurst),
            (49, Move::ZenHeadbutt),
            (57, Move::Inferno),
            (65, Move::DoubleEdge),
            (73, Move::FlareBlitz),
            (81, Move::FinalGambit),
            (89, Move::StoredPower),
            (97, Move::Overheat),
        ]
    };
    pub const Snivy: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Contrary),
        height: 6,
        weight: 81,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(63, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Leer),
            (7, Move::VineWhip),
            (10, Move::Wrap),
            (13, Move::Growth),
            (16, Move::LeafTornado),
            (19, Move::LeechSeed),
            (22, Move::MegaDrain),
            (25, Move::Slam),
            (28, Move::LeafBlade),
            (31, Move::Coil),
            (34, Move::GigaDrain),
            (37, Move::WringOut),
            (40, Move::GastroAcid),
            (43, Move::LeafStorm),
        ]
    };
    pub const Servine: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Contrary),
        height: 8,
        weight: 160,
        base_exp_yield: 145,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(83, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::VineWhip),
            (1, Move::Tackle),
            (1, Move::Wrap),
            (1, Move::Leer),
            (13, Move::Growth),
            (16, Move::LeafTornado),
            (20, Move::LeechSeed),
            (24, Move::MegaDrain),
            (28, Move::Slam),
            (32, Move::LeafBlade),
            (36, Move::Coil),
            (40, Move::GigaDrain),
            (44, Move::WringOut),
            (48, Move::GastroAcid),
            (52, Move::LeafStorm),
        ]
    };
    pub const Serperior: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Overgrow),
        hidden_ability: Some(Ability::Contrary),
        height: 33,
        weight: 630,
        base_exp_yield: 238,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(95, 0),
            Stat::new(75, 0),
            Stat::new(95, 0),
            Stat::new(113, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::VineWhip),
            (1, Move::Tackle),
            (1, Move::Wrap),
            (1, Move::Leer),
            (13, Move::Growth),
            (16, Move::LeafTornado),
            (20, Move::LeechSeed),
            (24, Move::MegaDrain),
            (28, Move::Slam),
            (32, Move::LeafBlade),
            (38, Move::Coil),
            (44, Move::GigaDrain),
            (50, Move::WringOut),
            (56, Move::GastroAcid),
            (62, Move::LeafStorm),
        ]
    };
    pub const Tepig: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::ThickFat),
        height: 5,
        weight: 99,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(65, 1),
            Stat::new(63, 0),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::TailWhip),
            (7, Move::Ember),
            (9, Move::OdorSleuth),
            (13, Move::DefenseCurl),
            (15, Move::FlameCharge),
            (19, Move::Smog),
            (21, Move::Rollout),
            (25, Move::TakeDown),
            (27, Move::HeatCrash),
            (31, Move::Assurance),
            (33, Move::Flamethrower),
            (37, Move::HeadSmash),
            (39, Move::Roar),
            (43, Move::FlareBlitz),
        ]
    };
    pub const Pignite: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Fighting),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::ThickFat),
        height: 10,
        weight: 555,
        base_exp_yield: 146,
        stats: Stats(
            Stat::new(90, 0),
            Stat::new(93, 2),
            Stat::new(55, 0),
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Ember),
            (1, Move::OdorSleuth),
            (13, Move::DefenseCurl),
            (15, Move::FlameCharge),
            (17, Move::ArmThrust),
            (20, Move::Smog),
            (23, Move::Rollout),
            (28, Move::TakeDown),
            (31, Move::HeatCrash),
            (36, Move::Assurance),
            (39, Move::Flamethrower),
            (44, Move::HeadSmash),
            (47, Move::Roar),
            (52, Move::FlareBlitz),
        ]
    };
    pub const Emboar: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Fire, Type::Fighting),
        ability: PokemonAbility::One(Ability::Blaze),
        hidden_ability: Some(Ability::Reckless),
        height: 16,
        weight: 1500,
        base_exp_yield: 238,
        stats: Stats(
            Stat::new(110, 0),
            Stat::new(123, 3),
            Stat::new(65, 0),
            Stat::new(100, 0),
            Stat::new(65, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::Ember),
            (1, Move::OdorSleuth),
            (1, Move::HammerArm),
            (13, Move::DefenseCurl),
            (15, Move::FlameCharge),
            (17, Move::ArmThrust),
            (20, Move::Smog),
            (23, Move::Rollout),
            (28, Move::TakeDown),
            (31, Move::HeatCrash),
            (38, Move::Assurance),
            (43, Move::Flamethrower),
            (50, Move::HeadSmash),
            (55, Move::Roar),
            (62, Move::FlareBlitz),
        ]
    };
    pub const Oshawott: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::ShellArmor),
        height: 5,
        weight: 59,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(63, 1),
            Stat::new(45, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (5, Move::TailWhip),
            (7, Move::WaterGun),
            (11, Move::WaterSport),
            (13, Move::FocusEnergy),
            (17, Move::RazorShell),
            (19, Move::FuryCutter),
            (23, Move::WaterPulse),
            (25, Move::Revenge),
            (29, Move::AquaJet),
            (31, Move::Encore),
            (35, Move::AquaTail),
            (37, Move::Retaliate),
            (41, Move::SwordsDance),
            (43, Move::HydroPump),
        ]
    };
    pub const Dewott: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::ShellArmor),
        height: 8,
        weight: 245,
        base_exp_yield: 145,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(83, 2),
            Stat::new(60, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::WaterGun),
            (1, Move::WaterSport),
            (13, Move::FocusEnergy),
            (17, Move::RazorShell),
            (20, Move::FuryCutter),
            (25, Move::WaterPulse),
            (28, Move::Revenge),
            (33, Move::AquaJet),
            (36, Move::Encore),
            (41, Move::AquaTail),
            (44, Move::Retaliate),
            (49, Move::SwordsDance),
            (52, Move::HydroPump),
        ]
    };
    pub const Samurott: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Torrent),
        hidden_ability: Some(Ability::ShellArmor),
        height: 15,
        weight: 946,
        base_exp_yield: 238,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(100, 0),
            Stat::new(85, 0),
            Stat::new(108, 3),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::TailWhip),
            (1, Move::WaterGun),
            (1, Move::Megahorn),
            (1, Move::WaterSport),
            (13, Move::FocusEnergy),
            (17, Move::RazorShell),
            (20, Move::FuryCutter),
            (25, Move::WaterPulse),
            (28, Move::Revenge),
            (33, Move::AquaJet),
            (36, Move::Slash),
            (38, Move::Encore),
            (45, Move::AquaTail),
            (50, Move::Retaliate),
            (57, Move::SwordsDance),
            (62, Move::HydroPump),
        ]
    };
    pub const Patrat: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::RunAway, Ability::KeenEye),
        hidden_ability: Some(Ability::Analytic),
        height: 5,
        weight: 116,
        base_exp_yield: 51,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(55, 1),
            Stat::new(39, 0),
            Stat::new(35, 0),
            Stat::new(39, 0),
            Stat::new(42, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::Leer),
            (6, Move::Bite),
            (8, Move::Bide),
            (11, Move::Detect),
            (13, Move::SandAttack),
            (16, Move::Crunch),
            (18, Move::Hypnosis),
            (21, Move::SuperFang),
            (23, Move::AfterYou),
            (26, Move::WorkUp),
            (28, Move::HyperFang),
            (31, Move::MeanLook),
            (33, Move::BatonPass),
            (36, Move::Slam),
        ]
    };
    pub const Watchog: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Illuminate, Ability::KeenEye),
        hidden_ability: Some(Ability::Analytic),
        height: 11,
        weight: 270,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(85, 2),
            Stat::new(69, 0),
            Stat::new(60, 0),
            Stat::new(69, 0),
            Stat::new(77, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::LowKick),
            (8, Move::Bide),
            (11, Move::Detect),
            (13, Move::SandAttack),
            (16, Move::Crunch),
            (18, Move::Hypnosis),
            (20, Move::ConfuseRay),
            (22, Move::SuperFang),
            (25, Move::AfterYou),
            (29, Move::PsychUp),
            (32, Move::HyperFang),
            (36, Move::MeanLook),
            (39, Move::BatonPass),
            (43, Move::Slam),
        ]
    };
    pub const Lillipup: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::VitalSpirit, Ability::Pickup),
        hidden_ability: Some(Ability::RunAway),
        height: 4,
        weight: 41,
        base_exp_yield: 55,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(60, 1),
            Stat::new(45, 0),
            Stat::new(25, 0),
            Stat::new(45, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (5, Move::OdorSleuth),
            (8, Move::Bite),
            (12, Move::HelpingHand),
            (15, Move::TakeDown),
            (19, Move::WorkUp),
            (22, Move::Crunch),
            (26, Move::Roar),
            (29, Move::Retaliate),
            (33, Move::Reversal),
            (36, Move::LastResort),
            (40, Move::GigaImpact),
        ]
    };
    pub const Herdier: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::SandRush),
        hidden_ability: Some(Ability::Scrappy),
        height: 9,
        weight: 147,
        base_exp_yield: 130,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(80, 2),
            Stat::new(65, 0),
            Stat::new(35, 0),
            Stat::new(65, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::OdorSleuth),
            (12, Move::HelpingHand),
            (15, Move::TakeDown),
            (20, Move::WorkUp),
            (24, Move::Crunch),
            (29, Move::Roar),
            (33, Move::Retaliate),
            (38, Move::Reversal),
            (42, Move::LastResort),
            (47, Move::GigaImpact),
        ]
    };
    pub const Stoutland: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::SandRush),
        hidden_ability: Some(Ability::Scrappy),
        height: 12,
        weight: 610,
        base_exp_yield: 225,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(110, 3),
            Stat::new(90, 0),
            Stat::new(45, 0),
            Stat::new(90, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::OdorSleuth),
            (1, Move::ThunderFang),
            (1, Move::IceFang),
            (1, Move::FireFang),
            (12, Move::HelpingHand),
            (15, Move::TakeDown),
            (20, Move::WorkUp),
            (24, Move::Crunch),
            (29, Move::Roar),
            (36, Move::Retaliate),
            (42, Move::Reversal),
            (51, Move::LastResort),
            (59, Move::GigaImpact),
        ]
    };
    pub const Purrloin: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::Two(Ability::Limber, Ability::Unburden),
        hidden_ability: Some(Ability::Prankster),
        height: 4,
        weight: 101,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(41, 0),
            Stat::new(50, 0),
            Stat::new(37, 0),
            Stat::new(50, 0),
            Stat::new(37, 0),
            Stat::new(66, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (3, Move::Growl),
            (6, Move::Assist),
            (10, Move::SandAttack),
            (12, Move::FurySwipes),
            (15, Move::Pursuit),
            (19, Move::Torment),
            (21, Move::FakeOut),
            (24, Move::HoneClaws),
            (28, Move::Assurance),
            (30, Move::Slash),
            (33, Move::Captivate),
            (37, Move::NightSlash),
            (39, Move::Snatch),
            (42, Move::NastyPlot),
            (46, Move::SuckerPunch),
        ]
    };
    pub const Liepard: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::Two(Ability::Limber, Ability::Unburden),
        hidden_ability: Some(Ability::Prankster),
        height: 11,
        weight: 375,
        base_exp_yield: 156,
        stats: Stats(
            Stat::new(64, 0),
            Stat::new(88, 0),
            Stat::new(50, 0),
            Stat::new(88, 0),
            Stat::new(50, 0),
            Stat::new(106, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::SandAttack),
            (1, Move::Growl),
            (1, Move::Assist),
            (12, Move::FurySwipes),
            (15, Move::Pursuit),
            (19, Move::Torment),
            (22, Move::FakeOut),
            (26, Move::HoneClaws),
            (31, Move::Assurance),
            (34, Move::Slash),
            (38, Move::Taunt),
            (43, Move::NightSlash),
            (47, Move::Snatch),
            (50, Move::NastyPlot),
            (55, Move::SuckerPunch),
        ]
    };
    pub const Pansage: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Gluttony),
        hidden_ability: Some(Ability::Overgrow),
        height: 6,
        weight: 105,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(53, 0),
            Stat::new(48, 0),
            Stat::new(53, 0),
            Stat::new(48, 0),
            Stat::new(64, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (4, Move::Leer),
            (7, Move::Lick),
            (10, Move::VineWhip),
            (13, Move::FurySwipes),
            (16, Move::LeechSeed),
            (19, Move::Bite),
            (22, Move::SeedBomb),
            (25, Move::Torment),
            (28, Move::Fling),
            (31, Move::Acrobatics),
            (34, Move::GrassKnot),
            (37, Move::Recycle),
            (40, Move::NaturalGift),
            (43, Move::Crunch),
        ]
    };
    pub const Simisage: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::One(Ability::Gluttony),
        hidden_ability: Some(Ability::Overgrow),
        height: 11,
        weight: 305,
        base_exp_yield: 174,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(98, 0),
            Stat::new(63, 0),
            Stat::new(98, 0),
            Stat::new(63, 0),
            Stat::new(101, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Lick),
            (1, Move::FurySwipes),
            (1, Move::SeedBomb),
        ]
    };
    pub const Pansear: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Gluttony),
        hidden_ability: Some(Ability::Blaze),
        height: 6,
        weight: 110,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(53, 0),
            Stat::new(48, 0),
            Stat::new(53, 0),
            Stat::new(48, 0),
            Stat::new(64, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (4, Move::Leer),
            (7, Move::Lick),
            (10, Move::Incinerate),
            (13, Move::FurySwipes),
            (16, Move::Yawn),
            (19, Move::Bite),
            (22, Move::FlameBurst),
            (25, Move::Amnesia),
            (28, Move::Fling),
            (31, Move::Acrobatics),
            (34, Move::FireBlast),
            (37, Move::Recycle),
            (40, Move::NaturalGift),
            (43, Move::Crunch),
        ]
    };
    pub const Simisear: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Gluttony),
        hidden_ability: Some(Ability::Blaze),
        height: 10,
        weight: 280,
        base_exp_yield: 174,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(98, 0),
            Stat::new(63, 0),
            Stat::new(98, 0),
            Stat::new(63, 0),
            Stat::new(101, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Lick),
            (1, Move::FurySwipes),
            (1, Move::FlameBurst),
        ]
    };
    pub const Panpour: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Gluttony),
        hidden_ability: Some(Ability::Torrent),
        height: 6,
        weight: 135,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(53, 0),
            Stat::new(48, 0),
            Stat::new(53, 0),
            Stat::new(48, 0),
            Stat::new(64, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (4, Move::Leer),
            (7, Move::Lick),
            (10, Move::WaterGun),
            (13, Move::FurySwipes),
            (16, Move::WaterSport),
            (19, Move::Bite),
            (22, Move::Scald),
            (25, Move::Taunt),
            (28, Move::Fling),
            (31, Move::Acrobatics),
            (34, Move::Brine),
            (37, Move::Recycle),
            (40, Move::NaturalGift),
            (43, Move::Crunch),
        ]
    };
    pub const Simipour: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::One(Ability::Gluttony),
        hidden_ability: Some(Ability::Torrent),
        height: 10,
        weight: 290,
        base_exp_yield: 174,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(98, 0),
            Stat::new(63, 0),
            Stat::new(98, 0),
            Stat::new(63, 0),
            Stat::new(101, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Lick),
            (1, Move::FurySwipes),
            (1, Move::Scald),
        ]
    };
    pub const Munna: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Forewarn, Ability::Synchronize),
        hidden_ability: Some(Ability::Telepathy),
        height: 6,
        weight: 233,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(76, 1),
            Stat::new(25, 0),
            Stat::new(45, 0),
            Stat::new(67, 0),
            Stat::new(55, 0),
            Stat::new(24, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DefenseCurl),
            (1, Move::Psywave),
            (5, Move::LuckyChant),
            (7, Move::Yawn),
            (11, Move::Psybeam),
            (13, Move::Imprison),
            (17, Move::Moonlight),
            (19, Move::Hypnosis),
            (23, Move::ZenHeadbutt),
            (25, Move::Synchronoise),
            (29, Move::Nightmare),
            (31, Move::FutureSight),
            (35, Move::CalmMind),
            (37, Move::Psychic),
            (41, Move::DreamEater),
            (43, Move::Telekinesis),
            (47, Move::StoredPower),
        ]
    };
    pub const Musharna: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Forewarn, Ability::Synchronize),
        hidden_ability: Some(Ability::Telepathy),
        height: 11,
        weight: 605,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(116, 2),
            Stat::new(55, 0),
            Stat::new(85, 0),
            Stat::new(107, 0),
            Stat::new(95, 0),
            Stat::new(29, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Psybeam),
            (1, Move::Hypnosis),
            (1, Move::DefenseCurl),
            (1, Move::LuckyChant),
        ]
    };
    pub const Pidove: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::BigPecks, Ability::SuperLuck),
        hidden_ability: Some(Ability::Rivalry),
        height: 3,
        weight: 21,
        base_exp_yield: 53,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(55, 1),
            Stat::new(50, 0),
            Stat::new(36, 0),
            Stat::new(30, 0),
            Stat::new(43, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (4, Move::Growl),
            (8, Move::Leer),
            (11, Move::QuickAttack),
            (15, Move::AirCutter),
            (18, Move::Roost),
            (22, Move::Detect),
            (25, Move::Taunt),
            (29, Move::AirSlash),
            (32, Move::RazorWind),
            (36, Move::FeatherDance),
            (39, Move::Swagger),
            (43, Move::Facade),
            (46, Move::Tailwind),
            (50, Move::SkyAttack),
        ]
    };
    pub const Tranquill: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::BigPecks, Ability::SuperLuck),
        hidden_ability: Some(Ability::Rivalry),
        height: 6,
        weight: 150,
        base_exp_yield: 125,
        stats: Stats(
            Stat::new(62, 0),
            Stat::new(77, 2),
            Stat::new(62, 0),
            Stat::new(50, 0),
            Stat::new(42, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Leer),
            (1, Move::Growl),
            (1, Move::QuickAttack),
            (15, Move::AirCutter),
            (18, Move::Roost),
            (23, Move::Detect),
            (27, Move::Taunt),
            (32, Move::AirSlash),
            (36, Move::RazorWind),
            (41, Move::FeatherDance),
            (45, Move::Swagger),
            (50, Move::Facade),
            (54, Move::Tailwind),
            (59, Move::SkyAttack),
        ]
    };
    pub const Unfezant: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::BigPecks, Ability::SuperLuck),
        hidden_ability: Some(Ability::Rivalry),
        height: 12,
        weight: 290,
        base_exp_yield: 220,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(115, 3),
            Stat::new(80, 0),
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(93, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Leer),
            (1, Move::Growl),
            (1, Move::QuickAttack),
            (15, Move::AirCutter),
            (18, Move::Roost),
            (23, Move::Detect),
            (27, Move::Taunt),
            (33, Move::AirSlash),
            (38, Move::RazorWind),
            (44, Move::FeatherDance),
            (49, Move::Swagger),
            (55, Move::Facade),
            (60, Move::Tailwind),
            (66, Move::SkyAttack),
        ]
    };
    pub const Blitzle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::LightningRod, Ability::MotorDrive),
        hidden_ability: Some(Ability::SapSipper),
        height: 8,
        weight: 298,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(60, 0),
            Stat::new(32, 0),
            Stat::new(50, 0),
            Stat::new(32, 0),
            Stat::new(76, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::QuickAttack),
            (4, Move::TailWhip),
            (8, Move::Charge),
            (11, Move::ShockWave),
            (15, Move::ThunderWave),
            (18, Move::FlameCharge),
            (22, Move::Pursuit),
            (25, Move::Spark),
            (29, Move::Stomp),
            (32, Move::Discharge),
            (36, Move::Agility),
            (39, Move::WildCharge),
            (43, Move::Thrash),
        ]
    };
    pub const Zebstrika: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::Two(Ability::LightningRod, Ability::MotorDrive),
        hidden_ability: Some(Ability::SapSipper),
        height: 16,
        weight: 795,
        base_exp_yield: 174,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(100, 0),
            Stat::new(63, 0),
            Stat::new(80, 0),
            Stat::new(63, 0),
            Stat::new(116, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::TailWhip),
            (1, Move::ThunderWave),
            (1, Move::QuickAttack),
            (1, Move::Charge),
            (11, Move::ShockWave),
            (18, Move::FlameCharge),
            (22, Move::Pursuit),
            (25, Move::Spark),
            (31, Move::Stomp),
            (36, Move::Discharge),
            (42, Move::Agility),
            (47, Move::WildCharge),
            (53, Move::Thrash),
        ]
    };
    pub const Roggenrola: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::WeakArmor),
        hidden_ability: Some(Ability::SandForce),
        height: 4,
        weight: 180,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(75, 0),
            Stat::new(85, 1),
            Stat::new(25, 0),
            Stat::new(25, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (4, Move::Harden),
            (7, Move::SandAttack),
            (10, Move::Headbutt),
            (14, Move::RockBlast),
            (17, Move::MudSlap),
            (20, Move::IronDefense),
            (23, Move::SmackDown),
            (27, Move::RockSlide),
            (30, Move::StealthRock),
            (33, Move::Sandstorm),
            (36, Move::StoneEdge),
            (40, Move::Explosion),
        ]
    };
    pub const Boldore: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::WeakArmor),
        hidden_ability: Some(Ability::SandForce),
        height: 9,
        weight: 1020,
        base_exp_yield: 137,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(105, 1),
            Stat::new(105, 1),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Headbutt),
            (1, Move::Tackle),
            (1, Move::Harden),
            (14, Move::RockBlast),
            (17, Move::MudSlap),
            (20, Move::IronDefense),
            (23, Move::SmackDown),
            (25, Move::PowerGem),
            (30, Move::RockSlide),
            (36, Move::StealthRock),
            (42, Move::Sandstorm),
            (48, Move::StoneEdge),
            (55, Move::Explosion),
        ]
    };
    pub const Gigalith: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::SandStream),
        hidden_ability: Some(Ability::SandForce),
        height: 17,
        weight: 2600,
        base_exp_yield: 232,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(135, 3),
            Stat::new(130, 0),
            Stat::new(60, 0),
            Stat::new(80, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Headbutt),
            (1, Move::Tackle),
            (1, Move::Harden),
            (14, Move::RockBlast),
            (17, Move::MudSlap),
            (20, Move::IronDefense),
            (23, Move::SmackDown),
            (25, Move::PowerGem),
            (30, Move::RockSlide),
            (36, Move::StealthRock),
            (42, Move::Sandstorm),
            (48, Move::StoneEdge),
            (55, Move::Explosion),
        ]
    };
    pub const Woobat: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Flying),
        ability: PokemonAbility::Two(Ability::Unaware, Ability::Klutz),
        hidden_ability: Some(Ability::Simple),
        height: 4,
        weight: 21,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(45, 0),
            Stat::new(43, 0),
            Stat::new(55, 0),
            Stat::new(43, 0),
            Stat::new(72, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (4, Move::OdorSleuth),
            (8, Move::Gust),
            (12, Move::Assurance),
            (15, Move::HeartStamp),
            (19, Move::Imprison),
            (21, Move::AirCutter),
            (25, Move::Attract),
            (29, Move::Amnesia),
            (29, Move::CalmMind),
            (32, Move::AirSlash),
            (36, Move::FutureSight),
            (41, Move::Psychic),
            (47, Move::Endeavor),
        ]
    };
    pub const Swoobat: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Flying),
        ability: PokemonAbility::Two(Ability::Unaware, Ability::Klutz),
        hidden_ability: Some(Ability::Simple),
        height: 9,
        weight: 105,
        base_exp_yield: 149,
        stats: Stats(
            Stat::new(67, 0),
            Stat::new(57, 0),
            Stat::new(55, 0),
            Stat::new(77, 0),
            Stat::new(55, 0),
            Stat::new(114, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Confusion),
            (1, Move::OdorSleuth),
            (1, Move::Assurance),
            (15, Move::HeartStamp),
            (19, Move::Imprison),
            (21, Move::AirCutter),
            (25, Move::Attract),
            (29, Move::Amnesia),
            (29, Move::CalmMind),
            (32, Move::AirSlash),
            (36, Move::FutureSight),
            (41, Move::Psychic),
            (47, Move::Endeavor),
        ]
    };
    pub const Drilbur: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ground),
        ability: PokemonAbility::Two(Ability::SandRush, Ability::SandForce),
        hidden_ability: Some(Ability::MoldBreaker),
        height: 3,
        weight: 85,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(85, 1),
            Stat::new(40, 0),
            Stat::new(30, 0),
            Stat::new(45, 0),
            Stat::new(68, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::MudSport),
            (5, Move::RapidSpin),
            (8, Move::MudSlap),
            (12, Move::FurySwipes),
            (15, Move::MetalClaw),
            (19, Move::Dig),
            (22, Move::HoneClaws),
            (26, Move::Slash),
            (29, Move::RockSlide),
            (33, Move::Earthquake),
            (36, Move::SwordsDance),
            (40, Move::Sandstorm),
            (43, Move::DrillRun),
            (47, Move::Fissure),
        ]
    };
    pub const Excadrill: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Steel),
        ability: PokemonAbility::Two(Ability::SandRush, Ability::SandForce),
        hidden_ability: Some(Ability::MoldBreaker),
        height: 7,
        weight: 404,
        base_exp_yield: 178,
        stats: Stats(
            Stat::new(110, 0),
            Stat::new(135, 2),
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(65, 0),
            Stat::new(88, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::MudSlap),
            (1, Move::RapidSpin),
            (1, Move::MudSport),
            (12, Move::FurySwipes),
            (15, Move::MetalClaw),
            (19, Move::Dig),
            (22, Move::HoneClaws),
            (26, Move::Slash),
            (29, Move::RockSlide),
            (31, Move::HornDrill),
            (36, Move::Earthquake),
            (42, Move::SwordsDance),
            (49, Move::Sandstorm),
            (55, Move::DrillRun),
            (62, Move::Fissure),
        ]
    };
    pub const Audino: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Healer, Ability::Regenerator),
        hidden_ability: Some(Ability::Klutz),
        height: 11,
        weight: 310,
        base_exp_yield: 390,
        stats: Stats(
            Stat::new(103, 2),
            Stat::new(60, 0),
            Stat::new(86, 0),
            Stat::new(60, 0),
            Stat::new(86, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Fairy),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Growl),
            (1, Move::HelpingHand),
            (5, Move::Refresh),
            (10, Move::DoubleSlap),
            (15, Move::Attract),
            (20, Move::SecretPower),
            (25, Move::Entrainment),
            (30, Move::TakeDown),
            (35, Move::HealPulse),
            (40, Move::AfterYou),
            (45, Move::SimpleBeam),
            (50, Move::DoubleEdge),
            (55, Move::LastResort),
        ]
    };
    pub const Timburr: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::SheerForce),
        hidden_ability: Some(Ability::IronFist),
        height: 6,
        weight: 125,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(80, 1),
            Stat::new(55, 0),
            Stat::new(25, 0),
            Stat::new(35, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Leer),
            (4, Move::FocusEnergy),
            (8, Move::Bide),
            (12, Move::LowKick),
            (16, Move::RockThrow),
            (20, Move::WakeUpSlap),
            (24, Move::ChipAway),
            (28, Move::BulkUp),
            (31, Move::RockSlide),
            (34, Move::DynamicPunch),
            (37, Move::ScaryFace),
            (40, Move::HammerArm),
            (43, Move::StoneEdge),
            (46, Move::FocusPunch),
            (49, Move::Superpower),
        ]
    };
    pub const Gurdurr: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::SheerForce),
        hidden_ability: Some(Ability::IronFist),
        height: 12,
        weight: 400,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(105, 2),
            Stat::new(85, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Leer),
            (1, Move::FocusEnergy),
            (1, Move::Bide),
            (12, Move::LowKick),
            (16, Move::RockThrow),
            (20, Move::WakeUpSlap),
            (24, Move::ChipAway),
            (29, Move::BulkUp),
            (33, Move::RockSlide),
            (37, Move::DynamicPunch),
            (41, Move::ScaryFace),
            (45, Move::HammerArm),
            (49, Move::StoneEdge),
            (53, Move::FocusPunch),
            (57, Move::Superpower),
        ]
    };
    pub const Conkeldurr: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::SheerForce),
        hidden_ability: Some(Ability::IronFist),
        height: 14,
        weight: 870,
        base_exp_yield: 227,
        stats: Stats(
            Stat::new(105, 0),
            Stat::new(140, 3),
            Stat::new(95, 0),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Leer),
            (1, Move::FocusEnergy),
            (1, Move::Bide),
            (12, Move::LowKick),
            (16, Move::RockThrow),
            (20, Move::WakeUpSlap),
            (24, Move::ChipAway),
            (29, Move::BulkUp),
            (33, Move::RockSlide),
            (37, Move::DynamicPunch),
            (41, Move::ScaryFace),
            (45, Move::HammerArm),
            (49, Move::StoneEdge),
            (53, Move::FocusPunch),
            (57, Move::Superpower),
        ]
    };
    pub const Tympole: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::Hydration),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 5,
        weight: 45,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(64, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Bubble),
            (5, Move::Supersonic),
            (9, Move::Round),
            (12, Move::BubbleBeam),
            (16, Move::MudShot),
            (20, Move::AquaRing),
            (23, Move::Uproar),
            (27, Move::MuddyWater),
            (31, Move::RainDance),
            (34, Move::Flail),
            (38, Move::EchoedVoice),
            (42, Move::HydroPump),
            (45, Move::HyperVoice),
        ]
    };
    pub const Palpitoad: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::Hydration),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 8,
        weight: 170,
        base_exp_yield: 134,
        stats: Stats(
            Stat::new(75, 2),
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(55, 0),
            Stat::new(69, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Supersonic),
            (1, Move::Bubble),
            (1, Move::Round),
            (12, Move::BubbleBeam),
            (16, Move::MudShot),
            (20, Move::AquaRing),
            (23, Move::Uproar),
            (28, Move::MuddyWater),
            (33, Move::RainDance),
            (37, Move::Flail),
            (42, Move::EchoedVoice),
            (47, Move::HydroPump),
            (51, Move::HyperVoice),
        ]
    };
    pub const Seismitoad: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ground),
        ability: PokemonAbility::Two(Ability::SwiftSwim, Ability::PoisonTouch),
        hidden_ability: Some(Ability::WaterAbsorb),
        height: 15,
        weight: 620,
        base_exp_yield: 229,
        stats: Stats(
            Stat::new(105, 3),
            Stat::new(95, 0),
            Stat::new(75, 0),
            Stat::new(85, 0),
            Stat::new(75, 0),
            Stat::new(74, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Water1),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Supersonic),
            (1, Move::Bubble),
            (1, Move::Round),
            (12, Move::BubbleBeam),
            (16, Move::MudShot),
            (20, Move::AquaRing),
            (23, Move::Uproar),
            (28, Move::MuddyWater),
            (33, Move::RainDance),
            (36, Move::Acid),
            (39, Move::Flail),
            (44, Move::DrainPunch),
            (49, Move::EchoedVoice),
            (53, Move::HydroPump),
            (59, Move::HyperVoice),
        ]
    };
    pub const Throh: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Guts, Ability::InnerFocus),
        hidden_ability: Some(Ability::MoldBreaker),
        height: 13,
        weight: 555,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(120, 2),
            Stat::new(100, 0),
            Stat::new(85, 0),
            Stat::new(30, 0),
            Stat::new(85, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bind),
            (1, Move::Leer),
            (5, Move::Bide),
            (9, Move::FocusEnergy),
            (13, Move::SeismicToss),
            (17, Move::VitalThrow),
            (21, Move::Revenge),
            (25, Move::StormThrow),
            (29, Move::BodySlam),
            (33, Move::BulkUp),
            (37, Move::CircleThrow),
            (41, Move::Endure),
            (45, Move::WideGuard),
            (49, Move::Superpower),
            (53, Move::Reversal),
        ]
    };
    pub const Sawk: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::InnerFocus),
        hidden_ability: Some(Ability::MoldBreaker),
        height: 14,
        weight: 510,
        base_exp_yield: 163,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(125, 2),
            Stat::new(75, 0),
            Stat::new(30, 0),
            Stat::new(75, 0),
            Stat::new(85, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::RockSmash),
            (5, Move::Bide),
            (9, Move::FocusEnergy),
            (13, Move::DoubleKick),
            (17, Move::LowSweep),
            (21, Move::Counter),
            (25, Move::KarateChop),
            (29, Move::BrickBreak),
            (33, Move::BulkUp),
            (37, Move::Retaliate),
            (41, Move::Endure),
            (45, Move::QuickGuard),
            (49, Move::CloseCombat),
            (53, Move::Reversal),
        ]
    };
    pub const Sewaddle: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Grass),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Chlorophyll),
        hidden_ability: Some(Ability::Overcoat),
        height: 3,
        weight: 25,
        base_exp_yield: 62,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(53, 0),
            Stat::new(70, 1),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(42, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::StringShot),
            (8, Move::BugBite),
            (15, Move::RazorLeaf),
            (22, Move::StruggleBug),
            (29, Move::Endure),
            (36, Move::BugBuzz),
            (43, Move::Flail),
        ]
    };
    pub const Swadloon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Grass),
        ability: PokemonAbility::Two(Ability::LeafGuard, Ability::Chlorophyll),
        hidden_ability: Some(Ability::Overcoat),
        height: 5,
        weight: 73,
        base_exp_yield: 133,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(63, 0),
            Stat::new(90, 2),
            Stat::new(50, 0),
            Stat::new(80, 0),
            Stat::new(42, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::RazorLeaf),
            (1, Move::StringShot),
            (1, Move::GrassWhistle),
            (1, Move::BugBite),
            (20, Move::Protect),
        ]
    };
    pub const Leavanny: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Grass),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Chlorophyll),
        hidden_ability: Some(Ability::Overcoat),
        height: 12,
        weight: 205,
        base_exp_yield: 225,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(103, 3),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(92, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::RazorLeaf),
            (1, Move::StringShot),
            (1, Move::FalseSwipe),
            (1, Move::BugBite),
            (22, Move::StruggleBug),
            (29, Move::Slash),
            (32, Move::HelpingHand),
            (36, Move::LeafBlade),
            (39, Move::XScissor),
            (43, Move::Entrainment),
            (46, Move::SwordsDance),
            (50, Move::LeafStorm),
        ]
    };
    pub const Venipede: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Swarm),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 4,
        weight: 53,
        base_exp_yield: 52,
        stats: Stats(
            Stat::new(30, 0),
            Stat::new(45, 0),
            Stat::new(59, 1),
            Stat::new(30, 0),
            Stat::new(39, 0),
            Stat::new(57, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DefenseCurl),
            (1, Move::Rollout),
            (5, Move::PoisonSting),
            (8, Move::Screech),
            (12, Move::Pursuit),
            (15, Move::Protect),
            (19, Move::PoisonTail),
            (22, Move::BugBite),
            (26, Move::Venoshock),
            (29, Move::Agility),
            (33, Move::Steamroller),
            (36, Move::Toxic),
            (40, Move::RockClimb),
            (43, Move::DoubleEdge),
        ]
    };
    pub const Whirlipede: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Swarm),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 12,
        weight: 585,
        base_exp_yield: 126,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(99, 2),
            Stat::new(40, 0),
            Stat::new(79, 0),
            Stat::new(47, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Screech),
            (1, Move::DefenseCurl),
            (1, Move::Rollout),
            (12, Move::Pursuit),
            (15, Move::Protect),
            (19, Move::PoisonTail),
            (22, Move::IronDefense),
            (23, Move::BugBite),
            (28, Move::Venoshock),
            (32, Move::Agility),
            (37, Move::Steamroller),
            (41, Move::Toxic),
            (46, Move::RockClimb),
            (50, Move::DoubleEdge),
        ]
    };
    pub const Scolipede: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Poison),
        ability: PokemonAbility::Two(Ability::PoisonPoint, Ability::Swarm),
        hidden_ability: Some(Ability::SpeedBoost),
        height: 25,
        weight: 2005,
        base_exp_yield: 218,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(100, 0),
            Stat::new(89, 0),
            Stat::new(55, 0),
            Stat::new(69, 0),
            Stat::new(112, 3)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PoisonSting),
            (1, Move::Screech),
            (1, Move::DefenseCurl),
            (1, Move::Rollout),
            (1, Move::Megahorn),
            (12, Move::Pursuit),
            (15, Move::Protect),
            (19, Move::PoisonTail),
            (23, Move::BugBite),
            (28, Move::Venoshock),
            (30, Move::BatonPass),
            (33, Move::Agility),
            (39, Move::Steamroller),
            (44, Move::Toxic),
            (50, Move::RockClimb),
            (55, Move::DoubleEdge),
        ]
    };
    pub const Cottonee: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Prankster, Ability::Infiltrator),
        hidden_ability: Some(Ability::Chlorophyll),
        height: 3,
        weight: 6,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(27, 0),
            Stat::new(60, 0),
            Stat::new(37, 0),
            Stat::new(50, 0),
            Stat::new(66, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (4, Move::Growth),
            (8, Move::LeechSeed),
            (10, Move::StunSpore),
            (13, Move::MegaDrain),
            (17, Move::CottonSpore),
            (19, Move::RazorLeaf),
            (22, Move::PoisonPowder),
            (26, Move::GigaDrain),
            (28, Move::Charm),
            (31, Move::HelpingHand),
            (35, Move::EnergyBall),
            (37, Move::CottonGuard),
            (40, Move::SunnyDay),
            (44, Move::Endeavor),
            (46, Move::SolarBeam),
        ]
    };
    pub const Whimsicott: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Fairy),
        ability: PokemonAbility::Two(Ability::Prankster, Ability::Infiltrator),
        hidden_ability: Some(Ability::Chlorophyll),
        height: 7,
        weight: 66,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(67, 0),
            Stat::new(85, 0),
            Stat::new(77, 0),
            Stat::new(75, 0),
            Stat::new(116, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Fairy, EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MegaDrain),
            (1, Move::LeechSeed),
            (1, Move::Growth),
            (1, Move::CottonSpore),
            (10, Move::Gust),
            (28, Move::Tailwind),
            (46, Move::Hurricane),
        ]
    };
    pub const Petilil: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::OwnTempo),
        hidden_ability: Some(Ability::LeafGuard),
        height: 5,
        weight: 66,
        base_exp_yield: 56,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(35, 0),
            Stat::new(50, 0),
            Stat::new(70, 1),
            Stat::new(50, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (4, Move::Growth),
            (8, Move::LeechSeed),
            (10, Move::SleepPowder),
            (13, Move::MegaDrain),
            (17, Move::Synthesis),
            (19, Move::MagicalLeaf),
            (22, Move::StunSpore),
            (26, Move::GigaDrain),
            (28, Move::Aromatherapy),
            (31, Move::HelpingHand),
            (35, Move::EnergyBall),
            (37, Move::Entrainment),
            (40, Move::SunnyDay),
            (44, Move::AfterYou),
            (46, Move::LeafStorm),
        ]
    };
    pub const Lilligant: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::OwnTempo),
        hidden_ability: Some(Ability::LeafGuard),
        height: 11,
        weight: 163,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(110, 2),
            Stat::new(75, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MegaDrain),
            (1, Move::LeechSeed),
            (1, Move::Growth),
            (1, Move::Synthesis),
            (10, Move::TeeterDance),
            (28, Move::QuiverDance),
            (46, Move::PetalDance),
        ]
    };
    pub const BasculinRedStriped: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::Reckless, Ability::Adaptability),
        hidden_ability: Some(Ability::MoldBreaker),
        height: 10,
        weight: 180,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(92, 0),
            Stat::new(65, 0),
            Stat::new(80, 0),
            Stat::new(55, 0),
            Stat::new(98, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::WaterGun),
            (4, Move::Uproar),
            (7, Move::Headbutt),
            (10, Move::Bite),
            (13, Move::AquaJet),
            (16, Move::ChipAway),
            (20, Move::TakeDown),
            (24, Move::Crunch),
            (28, Move::AquaTail),
            (32, Move::Soak),
            (36, Move::DoubleEdge),
            (41, Move::ScaryFace),
            (46, Move::Flail),
            (51, Move::FinalGambit),
            (56, Move::Thrash),
        ]
    };
    pub const Sandile: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Dark),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::Moxie),
        hidden_ability: Some(Ability::AngerPoint),
        height: 7,
        weight: 152,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(72, 1),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Rage),
            (4, Move::Bite),
            (7, Move::SandAttack),
            (10, Move::Torment),
            (13, Move::SandTomb),
            (16, Move::Assurance),
            (19, Move::MudSlap),
            (22, Move::Embargo),
            (25, Move::Swagger),
            (28, Move::Crunch),
            (31, Move::Dig),
            (34, Move::ScaryFace),
            (37, Move::FoulPlay),
            (40, Move::Sandstorm),
            (43, Move::Earthquake),
            (46, Move::Thrash),
        ]
    };
    pub const Krokorok: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Dark),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::Moxie),
        hidden_ability: Some(Ability::AngerPoint),
        height: 10,
        weight: 334,
        base_exp_yield: 123,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(82, 2),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(45, 0),
            Stat::new(74, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Rage),
            (10, Move::Torment),
            (13, Move::SandTomb),
            (16, Move::Assurance),
            (19, Move::MudSlap),
            (22, Move::Embargo),
            (25, Move::Swagger),
            (28, Move::Crunch),
            (32, Move::Dig),
            (36, Move::ScaryFace),
            (40, Move::FoulPlay),
            (44, Move::Sandstorm),
            (48, Move::Earthquake),
            (52, Move::Thrash),
        ]
    };
    pub const Krookodile: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Dark),
        ability: PokemonAbility::Two(Ability::Intimidate, Ability::Moxie),
        hidden_ability: Some(Ability::AngerPoint),
        height: 15,
        weight: 963,
        base_exp_yield: 234,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(117, 3),
            Stat::new(80, 0),
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(92, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Leer),
            (1, Move::Bite),
            (1, Move::Rage),
            (10, Move::Torment),
            (13, Move::SandTomb),
            (16, Move::Assurance),
            (19, Move::MudSlap),
            (22, Move::Embargo),
            (25, Move::Swagger),
            (28, Move::Crunch),
            (32, Move::Dig),
            (36, Move::ScaryFace),
            (42, Move::FoulPlay),
            (48, Move::Sandstorm),
            (54, Move::Earthquake),
            (60, Move::Outrage),
        ]
    };
    pub const Darumaka: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::Hustle),
        hidden_ability: Some(Ability::InnerFocus),
        height: 6,
        weight: 375,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(90, 1),
            Stat::new(45, 0),
            Stat::new(15, 0),
            Stat::new(45, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (3, Move::Rollout),
            (6, Move::Incinerate),
            (9, Move::Rage),
            (11, Move::FireFang),
            (14, Move::Headbutt),
            (17, Move::Uproar),
            (19, Move::Facade),
            (22, Move::FirePunch),
            (25, Move::WorkUp),
            (27, Move::Thrash),
            (30, Move::BellyDrum),
            (33, Move::FlareBlitz),
            (35, Move::Taunt),
            (39, Move::Superpower),
            (42, Move::Overheat),
        ]
    };
    pub const DarmanitanStandard: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::One(Ability::SheerForce),
        hidden_ability: Some(Ability::ZenMode),
        height: 13,
        weight: 929,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(105, 0),
            Stat::new(140, 2),
            Stat::new(55, 0),
            Stat::new(30, 0),
            Stat::new(55, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Rage),
            (1, Move::Rollout),
            (1, Move::Incinerate),
            (11, Move::FireFang),
            (14, Move::Headbutt),
            (17, Move::Swagger),
            (19, Move::Facade),
            (22, Move::FirePunch),
            (25, Move::WorkUp),
            (27, Move::Thrash),
            (30, Move::BellyDrum),
            (33, Move::FlareBlitz),
            (35, Move::HammerArm),
            (39, Move::Taunt),
            (47, Move::Superpower),
            (54, Move::Overheat),
        ]
    };
    pub const Maractus: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Grass),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::Chlorophyll),
        hidden_ability: Some(Ability::StormDrain),
        height: 10,
        weight: 280,
        base_exp_yield: 161,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(86, 0),
            Stat::new(67, 0),
            Stat::new(106, 2),
            Stat::new(67, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (1, Move::Absorb),
            (3, Move::SweetScent),
            (6, Move::Growth),
            (10, Move::PinMissile),
            (13, Move::MegaDrain),
            (15, Move::Synthesis),
            (18, Move::CottonSpore),
            (22, Move::NeedleArm),
            (26, Move::GigaDrain),
            (29, Move::Acupressure),
            (33, Move::Ingrain),
            (38, Move::PetalDance),
            (42, Move::SuckerPunch),
            (45, Move::SunnyDay),
            (50, Move::SolarBeam),
            (55, Move::CottonGuard),
            (57, Move::AfterYou),
        ]
    };
    pub const Dwebble: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::ShellArmor),
        hidden_ability: Some(Ability::WeakArmor),
        height: 3,
        weight: 145,
        base_exp_yield: 65,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(65, 0),
            Stat::new(85, 1),
            Stat::new(35, 0),
            Stat::new(35, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::FuryCutter),
            (5, Move::RockBlast),
            (7, Move::Withdraw),
            (11, Move::SandAttack),
            (13, Move::FeintAttack),
            (17, Move::SmackDown),
            (19, Move::RockPolish),
            (23, Move::BugBite),
            (24, Move::StealthRock),
            (29, Move::RockSlide),
            (31, Move::Slash),
            (35, Move::XScissor),
            (37, Move::ShellSmash),
            (41, Move::Flail),
            (43, Move::RockWrecker),
        ]
    };
    pub const Crustle: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Rock),
        ability: PokemonAbility::Two(Ability::Sturdy, Ability::ShellArmor),
        hidden_ability: Some(Ability::WeakArmor),
        height: 14,
        weight: 2000,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(105, 0),
            Stat::new(125, 2),
            Stat::new(65, 0),
            Stat::new(75, 0),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Bug, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Withdraw),
            (1, Move::RockBlast),
            (1, Move::ShellSmash),
            (13, Move::FeintAttack),
            (17, Move::SmackDown),
            (19, Move::RockPolish),
            (23, Move::BugBite),
            (24, Move::StealthRock),
            (29, Move::RockSlide),
            (31, Move::Slash),
            (38, Move::XScissor),
            (50, Move::Flail),
            (55, Move::RockWrecker),
        ]
    };
    pub const Scraggy: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Fighting),
        ability: PokemonAbility::Two(Ability::ShedSkin, Ability::Moxie),
        hidden_ability: Some(Ability::Intimidate),
        height: 6,
        weight: 118,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(75, 1),
            Stat::new(70, 0),
            Stat::new(35, 0),
            Stat::new(70, 0),
            Stat::new(48, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::LowKick),
            (5, Move::SandAttack),
            (9, Move::FeintAttack),
            (12, Move::Headbutt),
            (16, Move::Swagger),
            (20, Move::BrickBreak),
            (23, Move::Payback),
            (27, Move::ChipAway),
            (31, Move::HighJumpKick),
            (34, Move::ScaryFace),
            (38, Move::Crunch),
            (42, Move::Facade),
            (45, Move::RockClimb),
            (49, Move::FocusPunch),
            (53, Move::HeadSmash),
        ]
    };
    pub const Scrafty: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Fighting),
        ability: PokemonAbility::Two(Ability::ShedSkin, Ability::Moxie),
        hidden_ability: Some(Ability::Intimidate),
        height: 11,
        weight: 300,
        base_exp_yield: 171,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(90, 0),
            Stat::new(115, 1),
            Stat::new(45, 0),
            Stat::new(115, 1),
            Stat::new(58, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Leer),
            (1, Move::LowKick),
            (1, Move::FeintAttack),
            (12, Move::Headbutt),
            (16, Move::Swagger),
            (20, Move::BrickBreak),
            (23, Move::Payback),
            (27, Move::ChipAway),
            (31, Move::HighJumpKick),
            (34, Move::ScaryFace),
            (38, Move::Crunch),
            (45, Move::Facade),
            (51, Move::RockClimb),
            (58, Move::FocusPunch),
            (65, Move::HeadSmash),
        ]
    };
    pub const Sigilyph: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Psychic, Type::Flying),
        ability: PokemonAbility::Two(Ability::WonderSkin, Ability::MagicGuard),
        hidden_ability: Some(Ability::TintedLens),
        height: 14,
        weight: 140,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(72, 0),
            Stat::new(58, 0),
            Stat::new(80, 0),
            Stat::new(103, 2),
            Stat::new(80, 0),
            Stat::new(97, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::MiracleEye),
            (4, Move::Hypnosis),
            (8, Move::Psywave),
            (11, Move::Tailwind),
            (14, Move::Whirlwind),
            (18, Move::Psybeam),
            (21, Move::AirCutter),
            (24, Move::LightScreen),
            (28, Move::Reflect),
            (31, Move::Synchronoise),
            (34, Move::MirrorMove),
            (38, Move::Gravity),
            (41, Move::AirSlash),
            (44, Move::Psychic),
            (48, Move::CosmicPower),
            (51, Move::SkyAttack),
        ]
    };
    pub const Yamask: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Mummy),
        hidden_ability: None,
        height: 5,
        weight: 15,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(38, 0),
            Stat::new(30, 0),
            Stat::new(85, 1),
            Stat::new(55, 0),
            Stat::new(65, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Mineral, EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Protect),
            (1, Move::Astonish),
            (5, Move::Disable),
            (9, Move::Haze),
            (13, Move::NightShade),
            (17, Move::Hex),
            (21, Move::WillOWisp),
            (25, Move::OminousWind),
            (29, Move::Curse),
            (33, Move::GuardSplit),
            (33, Move::PowerSplit),
            (37, Move::ShadowBall),
            (41, Move::Grudge),
            (45, Move::MeanLook),
            (49, Move::DestinyBond),
        ]
    };
    pub const Cofagrigus: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ghost),
        ability: PokemonAbility::One(Ability::Mummy),
        hidden_ability: None,
        height: 17,
        weight: 765,
        base_exp_yield: 169,
        stats: Stats(
            Stat::new(58, 0),
            Stat::new(50, 0),
            Stat::new(145, 2),
            Stat::new(95, 0),
            Stat::new(105, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Mineral, EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Disable),
            (1, Move::Haze),
            (1, Move::Protect),
            (1, Move::Astonish),
            (13, Move::NightShade),
            (17, Move::Hex),
            (21, Move::WillOWisp),
            (25, Move::OminousWind),
            (29, Move::Curse),
            (33, Move::GuardSplit),
            (33, Move::PowerSplit),
            (34, Move::ScaryFace),
            (39, Move::ShadowBall),
            (45, Move::Grudge),
            (51, Move::MeanLook),
            (57, Move::DestinyBond),
        ]
    };
    pub const Tirtouga: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Rock),
        ability: PokemonAbility::Two(Ability::SolidRock, Ability::Sturdy),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 7,
        weight: 165,
        base_exp_yield: 71,
        stats: Stats(
            Stat::new(54, 0),
            Stat::new(78, 0),
            Stat::new(103, 1),
            Stat::new(53, 0),
            Stat::new(45, 0),
            Stat::new(22, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (1, Move::Withdraw),
            (1, Move::Bide),
            (5, Move::Rollout),
            (8, Move::Bite),
            (11, Move::Protect),
            (15, Move::AquaJet),
            (18, Move::AncientPower),
            (21, Move::Crunch),
            (25, Move::WideGuard),
            (28, Move::Brine),
            (31, Move::SmackDown),
            (35, Move::Curse),
            (38, Move::ShellSmash),
            (41, Move::AquaTail),
            (45, Move::RockSlide),
            (48, Move::RainDance),
            (51, Move::HydroPump),
        ]
    };
    pub const Carracosta: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Rock),
        ability: PokemonAbility::Two(Ability::SolidRock, Ability::Sturdy),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 12,
        weight: 810,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(74, 0),
            Stat::new(108, 0),
            Stat::new(133, 2),
            Stat::new(83, 0),
            Stat::new(65, 0),
            Stat::new(32, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (1, Move::Withdraw),
            (1, Move::Bide),
            (1, Move::Rollout),
            (8, Move::Bite),
            (11, Move::Protect),
            (15, Move::AquaJet),
            (18, Move::AncientPower),
            (21, Move::Crunch),
            (25, Move::WideGuard),
            (28, Move::Brine),
            (31, Move::SmackDown),
            (35, Move::Curse),
            (40, Move::ShellSmash),
            (45, Move::AquaTail),
            (51, Move::RockSlide),
            (56, Move::RainDance),
            (61, Move::HydroPump),
        ]
    };
    pub const Archen: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Flying),
        ability: PokemonAbility::One(Ability::Defeatist),
        hidden_ability: None,
        height: 5,
        weight: 95,
        base_exp_yield: 71,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(112, 1),
            Stat::new(45, 0),
            Stat::new(74, 0),
            Stat::new(45, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (5, Move::RockThrow),
            (8, Move::DoubleTeam),
            (11, Move::ScaryFace),
            (15, Move::Pluck),
            (18, Move::AncientPower),
            (21, Move::Agility),
            (25, Move::QuickGuard),
            (28, Move::Acrobatics),
            (31, Move::DragonBreath),
            (35, Move::Crunch),
            (38, Move::Endeavor),
            (41, Move::UTurn),
            (45, Move::RockSlide),
            (48, Move::DragonClaw),
            (51, Move::Thrash),
        ]
    };
    pub const Archeops: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Flying),
        ability: PokemonAbility::One(Ability::Defeatist),
        hidden_ability: None,
        height: 14,
        weight: 320,
        base_exp_yield: 177,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(140, 2),
            Stat::new(65, 0),
            Stat::new(112, 0),
            Stat::new(65, 0),
            Stat::new(110, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Flying, EggGroup::Water3),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::Leer),
            (1, Move::RockThrow),
            (1, Move::QuickAttack),
            (8, Move::DoubleTeam),
            (11, Move::ScaryFace),
            (15, Move::Pluck),
            (18, Move::AncientPower),
            (21, Move::Agility),
            (25, Move::QuickGuard),
            (28, Move::Acrobatics),
            (31, Move::DragonBreath),
            (35, Move::Crunch),
            (40, Move::Endeavor),
            (45, Move::UTurn),
            (51, Move::RockSlide),
            (56, Move::DragonClaw),
            (61, Move::Thrash),
        ]
    };
    pub const Trubbish: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::Stench, Ability::StickyHold),
        hidden_ability: Some(Ability::Aftermath),
        height: 6,
        weight: 310,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(62, 0),
            Stat::new(40, 0),
            Stat::new(62, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::PoisonGas),
            (3, Move::Recycle),
            (7, Move::ToxicSpikes),
            (12, Move::AcidSpray),
            (14, Move::DoubleSlap),
            (18, Move::Sludge),
            (23, Move::Stockpile),
            (23, Move::Swallow),
            (25, Move::TakeDown),
            (29, Move::SludgeBomb),
            (34, Move::ClearSmog),
            (36, Move::Toxic),
            (40, Move::Amnesia),
            (45, Move::GunkShot),
            (47, Move::Explosion),
        ]
    };
    pub const Garbodor: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Poison),
        ability: PokemonAbility::Two(Ability::Stench, Ability::WeakArmor),
        hidden_ability: Some(Ability::Aftermath),
        height: 19,
        weight: 1073,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(95, 2),
            Stat::new(82, 0),
            Stat::new(60, 0),
            Stat::new(82, 0),
            Stat::new(75, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::PoisonGas),
            (1, Move::Recycle),
            (1, Move::ToxicSpikes),
            (12, Move::AcidSpray),
            (14, Move::DoubleSlap),
            (18, Move::Sludge),
            (23, Move::Stockpile),
            (23, Move::Swallow),
            (25, Move::BodySlam),
            (29, Move::SludgeBomb),
            (34, Move::ClearSmog),
            (39, Move::Toxic),
            (46, Move::Amnesia),
            (54, Move::GunkShot),
            (59, Move::Explosion),
        ]
    };
    pub const Zorua: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::One(Ability::Illusion),
        hidden_ability: None,
        height: 7,
        weight: 125,
        base_exp_yield: 66,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(65, 0),
            Stat::new(40, 0),
            Stat::new(80, 1),
            Stat::new(40, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (5, Move::Pursuit),
            (9, Move::FakeTears),
            (13, Move::FurySwipes),
            (17, Move::FeintAttack),
            (21, Move::ScaryFace),
            (25, Move::Taunt),
            (29, Move::FoulPlay),
            (33, Move::Torment),
            (37, Move::Agility),
            (41, Move::Embargo),
            (45, Move::Punishment),
            (49, Move::NastyPlot),
            (53, Move::Imprison),
            (57, Move::NightDaze),
        ]
    };
    pub const Zoroark: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dark),
        ability: PokemonAbility::One(Ability::Illusion),
        hidden_ability: None,
        height: 16,
        weight: 811,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(105, 0),
            Stat::new(60, 0),
            Stat::new(120, 2),
            Stat::new(60, 0),
            Stat::new(105, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::Pursuit),
            (1, Move::UTurn),
            (1, Move::HoneClaws),
            (13, Move::FurySwipes),
            (17, Move::FeintAttack),
            (21, Move::ScaryFace),
            (25, Move::Taunt),
            (29, Move::FoulPlay),
            (30, Move::NightSlash),
            (34, Move::Torment),
            (39, Move::Agility),
            (44, Move::Embargo),
            (49, Move::Punishment),
            (54, Move::NastyPlot),
            (59, Move::Imprison),
            (64, Move::NightDaze),
        ]
    };
    pub const Minccino: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::Technician),
        hidden_ability: Some(Ability::SkillLink),
        height: 4,
        weight: 58,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(75, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (3, Move::Growl),
            (7, Move::HelpingHand),
            (9, Move::Tickle),
            (13, Move::DoubleSlap),
            (15, Move::Encore),
            (19, Move::Swift),
            (21, Move::Sing),
            (25, Move::TailSlap),
            (27, Move::Charm),
            (31, Move::WakeUpSlap),
            (33, Move::EchoedVoice),
            (37, Move::Slam),
            (39, Move::Captivate),
            (43, Move::HyperVoice),
            (45, Move::LastResort),
            (49, Move::AfterYou),
        ]
    };
    pub const Cinccino: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::CuteCharm, Ability::Technician),
        hidden_ability: Some(Ability::SkillLink),
        height: 5,
        weight: 75,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(95, 0),
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(60, 0),
            Stat::new(115, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Sing),
            (1, Move::HelpingHand),
            (1, Move::Tickle),
            (1, Move::BulletSeed),
            (1, Move::RockBlast),
            (1, Move::TailSlap),
        ]
    };
    pub const Gothita: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Frisk),
        hidden_ability: Some(Ability::ShadowTag),
        height: 4,
        weight: 58,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(30, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(65, 1),
            Stat::new(45, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (3, Move::Confusion),
            (7, Move::Tickle),
            (10, Move::FakeTears),
            (14, Move::DoubleSlap),
            (16, Move::Psybeam),
            (19, Move::Embargo),
            (24, Move::FeintAttack),
            (25, Move::Psyshock),
            (28, Move::Flatter),
            (31, Move::FutureSight),
            (33, Move::HealBlock),
            (37, Move::Psychic),
            (40, Move::Telekinesis),
            (46, Move::Charm),
            (48, Move::MagicRoom),
        ]
    };
    pub const Gothorita: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Frisk),
        hidden_ability: Some(Ability::ShadowTag),
        height: 7,
        weight: 180,
        base_exp_yield: 137,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(45, 0),
            Stat::new(70, 0),
            Stat::new(75, 0),
            Stat::new(85, 2),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Confusion),
            (1, Move::FakeTears),
            (1, Move::Tickle),
            (14, Move::DoubleSlap),
            (16, Move::Psybeam),
            (19, Move::Embargo),
            (24, Move::FeintAttack),
            (25, Move::Psyshock),
            (28, Move::Flatter),
            (31, Move::FutureSight),
            (34, Move::HealBlock),
            (39, Move::Psychic),
            (43, Move::Telekinesis),
            (50, Move::Charm),
            (53, Move::MagicRoom),
        ]
    };
    pub const Gothitelle: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::One(Ability::Frisk),
        hidden_ability: Some(Ability::ShadowTag),
        height: 15,
        weight: 440,
        base_exp_yield: 221,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(95, 0),
            Stat::new(95, 0),
            Stat::new(110, 3),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Confusion),
            (1, Move::FakeTears),
            (1, Move::Tickle),
            (14, Move::DoubleSlap),
            (16, Move::Psybeam),
            (19, Move::Embargo),
            (24, Move::FeintAttack),
            (25, Move::Psyshock),
            (28, Move::Flatter),
            (31, Move::FutureSight),
            (34, Move::HealBlock),
            (39, Move::Psychic),
            (45, Move::Telekinesis),
            (54, Move::Charm),
            (59, Move::MagicRoom),
        ]
    };
    pub const Solosis: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Overcoat, Ability::MagicGuard),
        hidden_ability: Some(Ability::Regenerator),
        height: 3,
        weight: 10,
        base_exp_yield: 58,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(105, 1),
            Stat::new(50, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Psywave),
            (3, Move::Reflect),
            (7, Move::Rollout),
            (10, Move::Snatch),
            (14, Move::HiddenPower),
            (16, Move::LightScreen),
            (19, Move::Charm),
            (24, Move::Recover),
            (25, Move::Psyshock),
            (28, Move::Endeavor),
            (31, Move::FutureSight),
            (33, Move::PainSplit),
            (37, Move::Psychic),
            (40, Move::SkillSwap),
            (46, Move::HealBlock),
            (48, Move::WonderRoom),
        ]
    };
    pub const Duosion: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Overcoat, Ability::MagicGuard),
        hidden_ability: Some(Ability::Regenerator),
        height: 6,
        weight: 80,
        base_exp_yield: 130,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(125, 2),
            Stat::new(60, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Reflect),
            (1, Move::Psywave),
            (1, Move::Rollout),
            (1, Move::Snatch),
            (14, Move::HiddenPower),
            (16, Move::LightScreen),
            (19, Move::Charm),
            (24, Move::Recover),
            (25, Move::Psyshock),
            (28, Move::Endeavor),
            (31, Move::FutureSight),
            (34, Move::PainSplit),
            (39, Move::Psychic),
            (43, Move::SkillSwap),
            (50, Move::HealBlock),
            (53, Move::WonderRoom),
        ]
    };
    pub const Reuniclus: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Overcoat, Ability::MagicGuard),
        hidden_ability: Some(Ability::Regenerator),
        height: 10,
        weight: 201,
        base_exp_yield: 221,
        stats: Stats(
            Stat::new(110, 0),
            Stat::new(65, 0),
            Stat::new(75, 0),
            Stat::new(125, 3),
            Stat::new(85, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Reflect),
            (1, Move::Psywave),
            (1, Move::Rollout),
            (1, Move::Snatch),
            (14, Move::HiddenPower),
            (16, Move::LightScreen),
            (19, Move::Charm),
            (24, Move::Recover),
            (25, Move::Psyshock),
            (28, Move::Endeavor),
            (31, Move::FutureSight),
            (34, Move::PainSplit),
            (39, Move::Psychic),
            (41, Move::DizzyPunch),
            (45, Move::SkillSwap),
            (54, Move::HealBlock),
            (59, Move::WonderRoom),
        ]
    };
    pub const Ducklett: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::BigPecks),
        hidden_ability: Some(Ability::Hydration),
        height: 5,
        weight: 55,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(62, 1),
            Stat::new(44, 0),
            Stat::new(50, 0),
            Stat::new(44, 0),
            Stat::new(50, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WaterGun),
            (3, Move::WaterSport),
            (6, Move::Defog),
            (9, Move::WingAttack),
            (13, Move::WaterPulse),
            (15, Move::AerialAce),
            (19, Move::BubbleBeam),
            (21, Move::FeatherDance),
            (24, Move::AquaRing),
            (27, Move::AirSlash),
            (30, Move::Roost),
            (34, Move::RainDance),
            (37, Move::Tailwind),
            (41, Move::BraveBird),
            (46, Move::Hurricane),
        ]
    };
    pub const Swanna: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::BigPecks),
        hidden_ability: Some(Ability::Hydration),
        height: 13,
        weight: 242,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(87, 0),
            Stat::new(63, 0),
            Stat::new(87, 0),
            Stat::new(63, 0),
            Stat::new(98, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::WaterGun),
            (1, Move::WaterSport),
            (1, Move::Defog),
            (13, Move::WaterPulse),
            (15, Move::AerialAce),
            (19, Move::BubbleBeam),
            (21, Move::FeatherDance),
            (24, Move::AquaRing),
            (27, Move::AirSlash),
            (30, Move::Roost),
            (34, Move::RainDance),
            (40, Move::Tailwind),
            (47, Move::BraveBird),
            (55, Move::Hurricane),
        ]
    };
    pub const Vanillite: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::Two(Ability::IceBody, Ability::SnowCloak),
        hidden_ability: Some(Ability::WeakArmor),
        height: 4,
        weight: 57,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(36, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(65, 1),
            Stat::new(60, 0),
            Stat::new(44, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::IcicleSpear),
            (4, Move::Harden),
            (7, Move::Astonish),
            (10, Move::Uproar),
            (13, Move::IcyWind),
            (16, Move::Mist),
            (19, Move::Avalanche),
            (22, Move::Taunt),
            (26, Move::MirrorShot),
            (31, Move::AcidArmor),
            (35, Move::IceBeam),
            (40, Move::Hail),
            (44, Move::MirrorCoat),
            (49, Move::Blizzard),
            (53, Move::SheerCold),
        ]
    };
    pub const Vanillish: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::Two(Ability::IceBody, Ability::SnowCloak),
        hidden_ability: Some(Ability::WeakArmor),
        height: 11,
        weight: 410,
        base_exp_yield: 138,
        stats: Stats(
            Stat::new(51, 0),
            Stat::new(65, 0),
            Stat::new(65, 0),
            Stat::new(80, 2),
            Stat::new(75, 0),
            Stat::new(59, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
            (1, Move::Uproar),
            (1, Move::Astonish),
            (1, Move::IcicleSpear),
            (13, Move::IcyWind),
            (16, Move::Mist),
            (19, Move::Avalanche),
            (22, Move::Taunt),
            (26, Move::MirrorShot),
            (31, Move::AcidArmor),
            (36, Move::IceBeam),
            (42, Move::Hail),
            (47, Move::MirrorCoat),
            (53, Move::Blizzard),
            (58, Move::SheerCold),
        ]
    };
    pub const Vanilluxe: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::Two(Ability::IceBody, Ability::SnowWarning),
        hidden_ability: Some(Ability::WeakArmor),
        height: 13,
        weight: 575,
        base_exp_yield: 241,
        stats: Stats(
            Stat::new(71, 0),
            Stat::new(95, 0),
            Stat::new(85, 0),
            Stat::new(110, 3),
            Stat::new(95, 0),
            Stat::new(79, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Harden),
            (1, Move::Uproar),
            (1, Move::Astonish),
            (1, Move::WeatherBall),
            (1, Move::IcicleSpear),
            (13, Move::IcyWind),
            (16, Move::Mist),
            (19, Move::Avalanche),
            (22, Move::Taunt),
            (26, Move::MirrorShot),
            (31, Move::AcidArmor),
            (36, Move::IceBeam),
            (42, Move::Hail),
            (50, Move::MirrorCoat),
            (59, Move::Blizzard),
            (67, Move::SheerCold),
        ]
    };
    pub const Deerling: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::SapSipper),
        hidden_ability: Some(Ability::SereneGrace),
        height: 6,
        weight: 195,
        base_exp_yield: 67,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(60, 0),
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(75, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Camouflage),
            (4, Move::Growl),
            (7, Move::SandAttack),
            (10, Move::DoubleKick),
            (13, Move::LeechSeed),
            (16, Move::FeintAttack),
            (20, Move::TakeDown),
            (24, Move::JumpKick),
            (28, Move::Aromatherapy),
            (32, Move::EnergyBall),
            (36, Move::Charm),
            (41, Move::NaturePower),
            (46, Move::DoubleEdge),
            (51, Move::SolarBeam),
        ]
    };
    pub const Sawsbuck: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Grass),
        ability: PokemonAbility::Two(Ability::Chlorophyll, Ability::SapSipper),
        hidden_ability: Some(Ability::SereneGrace),
        height: 19,
        weight: 925,
        base_exp_yield: 166,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(100, 2),
            Stat::new(70, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::SandAttack),
            (1, Move::Tackle),
            (1, Move::Growl),
            (1, Move::Megahorn),
            (1, Move::Camouflage),
            (10, Move::DoubleKick),
            (13, Move::LeechSeed),
            (16, Move::FeintAttack),
            (20, Move::TakeDown),
            (24, Move::JumpKick),
            (28, Move::Aromatherapy),
            (32, Move::EnergyBall),
            (36, Move::Charm),
            (37, Move::HornLeech),
            (44, Move::NaturePower),
            (52, Move::DoubleEdge),
            (60, Move::SolarBeam),
        ]
    };
    pub const Emolga: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Flying),
        ability: PokemonAbility::One(Ability::Static),
        hidden_ability: Some(Ability::MotorDrive),
        height: 4,
        weight: 50,
        base_exp_yield: 150,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(75, 0),
            Stat::new(60, 0),
            Stat::new(103, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ThunderShock),
            (4, Move::QuickAttack),
            (7, Move::TailWhip),
            (10, Move::Charge),
            (13, Move::Spark),
            (16, Move::Pursuit),
            (19, Move::DoubleTeam),
            (22, Move::ShockWave),
            (26, Move::ElectroBall),
            (30, Move::Acrobatics),
            (34, Move::LightScreen),
            (38, Move::Encore),
            (42, Move::VoltSwitch),
            (46, Move::Agility),
            (50, Move::Discharge),
        ]
    };
    pub const Karrablast: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::ShedSkin),
        hidden_ability: Some(Ability::NoGuard),
        height: 5,
        weight: 59,
        base_exp_yield: 63,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(75, 1),
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Peck),
            (4, Move::Leer),
            (8, Move::Endure),
            (13, Move::FuryCutter),
            (16, Move::FuryAttack),
            (20, Move::Headbutt),
            (25, Move::FalseSwipe),
            (28, Move::BugBuzz),
            (32, Move::Slash),
            (37, Move::TakeDown),
            (40, Move::ScaryFace),
            (44, Move::XScissor),
            (49, Move::Flail),
            (52, Move::SwordsDance),
            (56, Move::DoubleEdge),
        ]
    };
    pub const Escavalier: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Steel),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::ShellArmor),
        hidden_ability: Some(Ability::Overcoat),
        height: 10,
        weight: 330,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(135, 2),
            Stat::new(105, 0),
            Stat::new(60, 0),
            Stat::new(105, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Twineedle),
            (1, Move::Leer),
            (1, Move::Peck),
            (1, Move::QuickGuard),
            (16, Move::FuryAttack),
            (20, Move::Headbutt),
            (25, Move::FalseSwipe),
            (28, Move::BugBuzz),
            (32, Move::Slash),
            (37, Move::IronHead),
            (40, Move::IronDefense),
            (44, Move::XScissor),
            (49, Move::Reversal),
            (52, Move::SwordsDance),
            (56, Move::GigaImpact),
        ]
    };
    pub const Foongus: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::EffectSpore),
        hidden_ability: Some(Ability::Regenerator),
        height: 2,
        weight: 10,
        base_exp_yield: 59,
        stats: Stats(
            Stat::new(69, 1),
            Stat::new(55, 0),
            Stat::new(45, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(15, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (6, Move::Growth),
            (8, Move::Astonish),
            (12, Move::Bide),
            (15, Move::MegaDrain),
            (18, Move::Ingrain),
            (20, Move::FeintAttack),
            (24, Move::SweetScent),
            (28, Move::GigaDrain),
            (32, Move::Toxic),
            (35, Move::Synthesis),
            (39, Move::ClearSmog),
            (43, Move::SolarBeam),
            (45, Move::RagePowder),
            (50, Move::Spore),
        ]
    };
    pub const Amoonguss: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Poison),
        ability: PokemonAbility::One(Ability::EffectSpore),
        hidden_ability: Some(Ability::Regenerator),
        height: 6,
        weight: 105,
        base_exp_yield: 162,
        stats: Stats(
            Stat::new(114, 2),
            Stat::new(85, 0),
            Stat::new(70, 0),
            Stat::new(85, 0),
            Stat::new(80, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Plant),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (1, Move::Growth),
            (1, Move::Bide),
            (1, Move::Astonish),
            (15, Move::MegaDrain),
            (18, Move::Ingrain),
            (20, Move::FeintAttack),
            (24, Move::SweetScent),
            (28, Move::GigaDrain),
            (32, Move::Toxic),
            (35, Move::Synthesis),
            (43, Move::ClearSmog),
            (49, Move::SolarBeam),
            (54, Move::RagePowder),
            (62, Move::Spore),
        ]
    };
    pub const Frillish: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ghost),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::CursedBody),
        hidden_ability: Some(Ability::Damp),
        height: 12,
        weight: 330,
        base_exp_yield: 67,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(65, 0),
            Stat::new(85, 1),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bubble),
            (1, Move::WaterSport),
            (5, Move::Absorb),
            (9, Move::NightShade),
            (13, Move::BubbleBeam),
            (17, Move::Recover),
            (22, Move::WaterPulse),
            (27, Move::OminousWind),
            (32, Move::Brine),
            (37, Move::RainDance),
            (43, Move::Hex),
            (49, Move::HydroPump),
            (55, Move::WringOut),
            (61, Move::WaterSpout),
        ]
    };
    pub const Jellicent: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Ghost),
        ability: PokemonAbility::Two(Ability::WaterAbsorb, Ability::CursedBody),
        hidden_ability: Some(Ability::Damp),
        height: 22,
        weight: 1350,
        base_exp_yield: 168,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(85, 0),
            Stat::new(105, 2),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Absorb),
            (1, Move::NightShade),
            (1, Move::Bubble),
            (1, Move::WaterSport),
            (13, Move::BubbleBeam),
            (17, Move::Recover),
            (22, Move::WaterPulse),
            (27, Move::OminousWind),
            (32, Move::Brine),
            (37, Move::RainDance),
            (45, Move::Hex),
            (53, Move::HydroPump),
            (61, Move::WringOut),
            (69, Move::WaterSpout),
        ]
    };
    pub const Alomomola: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Water),
        ability: PokemonAbility::Two(Ability::Healer, Ability::Hydration),
        hidden_ability: Some(Ability::Regenerator),
        height: 12,
        weight: 316,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(165, 2),
            Stat::new(75, 0),
            Stat::new(80, 0),
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Water2),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::WaterSport),
            (5, Move::AquaRing),
            (9, Move::AquaJet),
            (13, Move::DoubleSlap),
            (17, Move::HealPulse),
            (21, Move::Protect),
            (25, Move::WaterPulse),
            (29, Move::WakeUpSlap),
            (33, Move::Soak),
            (37, Move::Wish),
            (41, Move::Brine),
            (45, Move::Safeguard),
            (49, Move::HelpingHand),
            (53, Move::WideGuard),
            (57, Move::HealingWish),
            (61, Move::HydroPump),
        ]
    };
    pub const Joltik: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Electric),
        ability: PokemonAbility::Two(Ability::CompoundEyes, Ability::Unnerve),
        hidden_ability: Some(Ability::Swarm),
        height: 1,
        weight: 6,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(47, 0),
            Stat::new(50, 0),
            Stat::new(57, 0),
            Stat::new(50, 0),
            Stat::new(65, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::StringShot),
            (1, Move::LeechLife),
            (1, Move::SpiderWeb),
            (4, Move::ThunderWave),
            (7, Move::Screech),
            (12, Move::FuryCutter),
            (15, Move::Electroweb),
            (18, Move::BugBite),
            (23, Move::GastroAcid),
            (26, Move::Slash),
            (29, Move::ElectroBall),
            (34, Move::SignalBeam),
            (37, Move::Agility),
            (40, Move::SuckerPunch),
            (45, Move::Discharge),
            (48, Move::BugBuzz),
        ]
    };
    pub const Galvantula: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Electric),
        ability: PokemonAbility::Two(Ability::CompoundEyes, Ability::Unnerve),
        hidden_ability: Some(Ability::Swarm),
        height: 8,
        weight: 143,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(77, 0),
            Stat::new(60, 0),
            Stat::new(97, 0),
            Stat::new(60, 0),
            Stat::new(108, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::StringShot),
            (1, Move::ThunderWave),
            (1, Move::LeechLife),
            (1, Move::SpiderWeb),
            (7, Move::Screech),
            (12, Move::FuryCutter),
            (15, Move::Electroweb),
            (18, Move::BugBite),
            (23, Move::GastroAcid),
            (26, Move::Slash),
            (29, Move::ElectroBall),
            (34, Move::SignalBeam),
            (40, Move::Agility),
            (46, Move::SuckerPunch),
            (54, Move::Discharge),
            (60, Move::BugBuzz),
        ]
    };
    pub const Ferroseed: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Steel),
        ability: PokemonAbility::One(Ability::IronBarbs),
        hidden_ability: None,
        height: 6,
        weight: 188,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(44, 0),
            Stat::new(50, 0),
            Stat::new(91, 1),
            Stat::new(24, 0),
            Stat::new(86, 0),
            Stat::new(10, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Plant, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Harden),
            (6, Move::Rollout),
            (9, Move::Curse),
            (14, Move::MetalClaw),
            (18, Move::PinMissile),
            (21, Move::GyroBall),
            (26, Move::IronDefense),
            (30, Move::MirrorShot),
            (35, Move::Ingrain),
            (38, Move::SelfDestruct),
            (43, Move::IronHead),
            (47, Move::Payback),
            (52, Move::FlashCannon),
            (55, Move::Explosion),
        ]
    };
    pub const Ferrothorn: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Steel),
        ability: PokemonAbility::One(Ability::IronBarbs),
        hidden_ability: Some(Ability::Anticipation),
        height: 10,
        weight: 1100,
        base_exp_yield: 171,
        stats: Stats(
            Stat::new(74, 0),
            Stat::new(94, 0),
            Stat::new(131, 2),
            Stat::new(54, 0),
            Stat::new(116, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Plant, EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::Harden),
            (1, Move::Curse),
            (1, Move::Rollout),
            (1, Move::RockClimb),
            (14, Move::MetalClaw),
            (18, Move::PinMissile),
            (21, Move::GyroBall),
            (26, Move::IronDefense),
            (30, Move::MirrorShot),
            (35, Move::Ingrain),
            (38, Move::SelfDestruct),
            (40, Move::PowerWhip),
            (46, Move::IronHead),
            (53, Move::Payback),
            (61, Move::FlashCannon),
            (67, Move::Explosion),
        ]
    };
    pub const Klink: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Steel),
        ability: PokemonAbility::Two(Ability::Plus, Ability::Minus),
        hidden_ability: Some(Ability::ClearBody),
        height: 3,
        weight: 210,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(40, 0),
            Stat::new(55, 0),
            Stat::new(70, 1),
            Stat::new(45, 0),
            Stat::new(60, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (6, Move::Charge),
            (11, Move::ThunderShock),
            (16, Move::GearGrind),
            (21, Move::Bind),
            (26, Move::ChargeBeam),
            (31, Move::Autotomize),
            (36, Move::MirrorShot),
            (39, Move::Screech),
            (42, Move::Discharge),
            (45, Move::MetalSound),
            (48, Move::ShiftGear),
            (51, Move::LockOn),
            (54, Move::ZapCannon),
            (57, Move::HyperBeam),
        ]
    };
    pub const Klang: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Steel),
        ability: PokemonAbility::Two(Ability::Plus, Ability::Minus),
        hidden_ability: Some(Ability::ClearBody),
        height: 6,
        weight: 510,
        base_exp_yield: 154,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(80, 0),
            Stat::new(95, 2),
            Stat::new(70, 0),
            Stat::new(85, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (1, Move::ThunderShock),
            (1, Move::Charge),
            (1, Move::GearGrind),
            (21, Move::Bind),
            (26, Move::ChargeBeam),
            (31, Move::Autotomize),
            (36, Move::MirrorShot),
            (40, Move::Screech),
            (44, Move::Discharge),
            (48, Move::MetalSound),
            (52, Move::ShiftGear),
            (56, Move::LockOn),
            (60, Move::ZapCannon),
            (64, Move::HyperBeam),
        ]
    };
    pub const Klinklang: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Steel),
        ability: PokemonAbility::Two(Ability::Plus, Ability::Minus),
        hidden_ability: Some(Ability::ClearBody),
        height: 6,
        weight: 810,
        base_exp_yield: 234,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(100, 0),
            Stat::new(115, 3),
            Stat::new(70, 0),
            Stat::new(85, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (1, Move::ThunderShock),
            (1, Move::Charge),
            (1, Move::GearGrind),
            (21, Move::Bind),
            (25, Move::ChargeBeam),
            (31, Move::Autotomize),
            (36, Move::MirrorShot),
            (40, Move::Screech),
            (44, Move::Discharge),
            (48, Move::MetalSound),
            (54, Move::ShiftGear),
            (60, Move::LockOn),
            (66, Move::ZapCannon),
            (72, Move::HyperBeam),
        ]
    };
    pub const Tynamo: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 2,
        weight: 3,
        base_exp_yield: 55,
        stats: Stats(
            Stat::new(35, 0),
            Stat::new(55, 0),
            Stat::new(40, 0),
            Stat::new(45, 0),
            Stat::new(40, 0),
            Stat::new(60, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::ThunderWave),
            (1, Move::Spark),
            (1, Move::ChargeBeam),
        ]
    };
    pub const Eelektrik: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 12,
        weight: 220,
        base_exp_yield: 142,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(85, 2),
            Stat::new(70, 0),
            Stat::new(75, 0),
            Stat::new(70, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::ThunderWave),
            (1, Move::Spark),
            (1, Move::ChargeBeam),
            (9, Move::Bind),
            (19, Move::Acid),
            (29, Move::Discharge),
            (39, Move::Crunch),
            (44, Move::Thunderbolt),
            (49, Move::AcidSpray),
            (54, Move::Coil),
            (59, Move::WildCharge),
            (64, Move::GastroAcid),
            (69, Move::ZapCannon),
            (74, Move::Thrash),
        ]
    };
    pub const Eelektross: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Electric),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 21,
        weight: 805,
        base_exp_yield: 232,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(115, 3),
            Stat::new(80, 0),
            Stat::new(105, 0),
            Stat::new(80, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Headbutt),
            (1, Move::Acid),
            (1, Move::Crunch),
            (1, Move::CrushClaw),
            (1, Move::Discharge),
        ]
    };
    pub const Elgyem: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Telepathy, Ability::Synchronize),
        hidden_ability: Some(Ability::Analytic),
        height: 5,
        weight: 90,
        base_exp_yield: 67,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(55, 0),
            Stat::new(85, 1),
            Stat::new(55, 0),
            Stat::new(30, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Confusion),
            (4, Move::Growl),
            (8, Move::HealBlock),
            (11, Move::MiracleEye),
            (15, Move::Psybeam),
            (18, Move::Headbutt),
            (22, Move::HiddenPower),
            (25, Move::Imprison),
            (29, Move::SimpleBeam),
            (32, Move::ZenHeadbutt),
            (36, Move::PsychUp),
            (39, Move::Psychic),
            (43, Move::CalmMind),
            (46, Move::Recover),
            (50, Move::GuardSplit),
            (50, Move::PowerSplit),
            (53, Move::Synchronoise),
            (56, Move::WonderRoom),
        ]
    };
    pub const Beheeyem: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Psychic),
        ability: PokemonAbility::Two(Ability::Telepathy, Ability::Synchronize),
        hidden_ability: Some(Ability::Analytic),
        height: 10,
        weight: 345,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(75, 0),
            Stat::new(125, 2),
            Stat::new(95, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Confusion),
            (1, Move::MiracleEye),
            (1, Move::HealBlock),
            (15, Move::Psybeam),
            (18, Move::Headbutt),
            (22, Move::HiddenPower),
            (25, Move::Imprison),
            (29, Move::SimpleBeam),
            (32, Move::ZenHeadbutt),
            (36, Move::PsychUp),
            (39, Move::Psychic),
            (45, Move::CalmMind),
            (50, Move::Recover),
            (56, Move::GuardSplit),
            (58, Move::PowerSplit),
            (63, Move::Synchronoise),
            (68, Move::WonderRoom),
        ]
    };
    pub const Litwick: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Fire),
        ability: PokemonAbility::Two(Ability::FlashFire, Ability::FlameBody),
        hidden_ability: Some(Ability::Infiltrator),
        height: 3,
        weight: 31,
        base_exp_yield: 55,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(30, 0),
            Stat::new(55, 0),
            Stat::new(65, 1),
            Stat::new(55, 0),
            Stat::new(20, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (1, Move::Astonish),
            (3, Move::Minimize),
            (5, Move::Smog),
            (7, Move::FireSpin),
            (10, Move::ConfuseRay),
            (13, Move::NightShade),
            (16, Move::WillOWisp),
            (20, Move::FlameBurst),
            (24, Move::Imprison),
            (28, Move::Hex),
            (33, Move::Memento),
            (38, Move::Inferno),
            (43, Move::Curse),
            (49, Move::ShadowBall),
            (55, Move::PainSplit),
            (61, Move::Overheat),
        ]
    };
    pub const Lampent: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Fire),
        ability: PokemonAbility::Two(Ability::FlashFire, Ability::FlameBody),
        hidden_ability: Some(Ability::Infiltrator),
        height: 6,
        weight: 130,
        base_exp_yield: 130,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(95, 2),
            Stat::new(60, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (1, Move::Minimize),
            (1, Move::Smog),
            (1, Move::Astonish),
            (7, Move::FireSpin),
            (10, Move::ConfuseRay),
            (13, Move::NightShade),
            (16, Move::WillOWisp),
            (20, Move::FlameBurst),
            (24, Move::Imprison),
            (28, Move::Hex),
            (33, Move::Memento),
            (38, Move::Inferno),
            (45, Move::Curse),
            (53, Move::ShadowBall),
            (61, Move::PainSplit),
            (69, Move::Overheat),
        ]
    };
    pub const Chandelure: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ghost, Type::Fire),
        ability: PokemonAbility::Two(Ability::FlashFire, Ability::FlameBody),
        hidden_ability: Some(Ability::Infiltrator),
        height: 10,
        weight: 343,
        base_exp_yield: 234,
        stats: Stats(
            Stat::new(60, 0),
            Stat::new(55, 0),
            Stat::new(90, 0),
            Stat::new(145, 3),
            Stat::new(90, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ConfuseRay),
            (1, Move::Smog),
            (1, Move::FlameBurst),
            (1, Move::Hex),
        ]
    };
    pub const Axew: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::Two(Ability::Rivalry, Ability::MoldBreaker),
        hidden_ability: Some(Ability::Unnerve),
        height: 6,
        weight: 180,
        base_exp_yield: 64,
        stats: Stats(
            Stat::new(46, 0),
            Stat::new(87, 1),
            Stat::new(60, 0),
            Stat::new(30, 0),
            Stat::new(40, 0),
            Stat::new(57, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (4, Move::Leer),
            (7, Move::Assurance),
            (10, Move::DragonRage),
            (13, Move::DualChop),
            (16, Move::ScaryFace),
            (20, Move::Slash),
            (24, Move::FalseSwipe),
            (28, Move::DragonClaw),
            (32, Move::DragonDance),
            (36, Move::Taunt),
            (41, Move::DragonPulse),
            (46, Move::SwordsDance),
            (51, Move::Guillotine),
            (56, Move::Outrage),
            (61, Move::GigaImpact),
        ]
    };
    pub const Fraxure: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::Two(Ability::Rivalry, Ability::MoldBreaker),
        hidden_ability: Some(Ability::Unnerve),
        height: 10,
        weight: 360,
        base_exp_yield: 144,
        stats: Stats(
            Stat::new(66, 0),
            Stat::new(117, 2),
            Stat::new(70, 0),
            Stat::new(40, 0),
            Stat::new(50, 0),
            Stat::new(67, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::DragonRage),
            (1, Move::Assurance),
            (13, Move::DualChop),
            (16, Move::ScaryFace),
            (20, Move::Slash),
            (24, Move::FalseSwipe),
            (28, Move::DragonClaw),
            (32, Move::DragonDance),
            (36, Move::Taunt),
            (42, Move::DragonPulse),
            (48, Move::SwordsDance),
            (54, Move::Guillotine),
            (60, Move::Outrage),
            (66, Move::GigaImpact),
        ]
    };
    pub const Haxorus: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::Two(Ability::Rivalry, Ability::MoldBreaker),
        hidden_ability: Some(Ability::Unnerve),
        height: 18,
        weight: 1055,
        base_exp_yield: 243,
        stats: Stats(
            Stat::new(76, 0),
            Stat::new(147, 3),
            Stat::new(90, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(97, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::DragonRage),
            (1, Move::Assurance),
            (13, Move::DualChop),
            (16, Move::ScaryFace),
            (20, Move::Slash),
            (24, Move::FalseSwipe),
            (28, Move::DragonClaw),
            (32, Move::DragonDance),
            (36, Move::Taunt),
            (42, Move::DragonPulse),
            (50, Move::SwordsDance),
            (58, Move::Guillotine),
            (66, Move::Outrage),
            (74, Move::GigaImpact),
        ]
    };
    pub const Cubchoo: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::One(Ability::SnowCloak),
        hidden_ability: Some(Ability::Rattled),
        height: 5,
        weight: 85,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(70, 1),
            Stat::new(40, 0),
            Stat::new(60, 0),
            Stat::new(40, 0),
            Stat::new(40, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::PowderSnow),
            (5, Move::Growl),
            (9, Move::Bide),
            (13, Move::IcyWind),
            (17, Move::FurySwipes),
            (21, Move::Brine),
            (25, Move::Endure),
            (29, Move::Charm),
            (33, Move::Slash),
            (36, Move::Flail),
            (41, Move::Rest),
            (45, Move::Blizzard),
            (49, Move::Hail),
            (53, Move::Thrash),
            (57, Move::SheerCold),
        ]
    };
    pub const Beartic: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::One(Ability::SnowCloak),
        hidden_ability: Some(Ability::SwiftSwim),
        height: 26,
        weight: 2600,
        base_exp_yield: 177,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(130, 2),
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(80, 0),
            Stat::new(50, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Growl),
            (1, Move::Bide),
            (1, Move::PowderSnow),
            (1, Move::IcyWind),
            (1, Move::Superpower),
            (1, Move::AquaJet),
            (17, Move::FurySwipes),
            (21, Move::Brine),
            (25, Move::Endure),
            (29, Move::Swagger),
            (33, Move::Slash),
            (36, Move::Flail),
            (37, Move::IcicleCrash),
            (41, Move::Rest),
            (45, Move::Blizzard),
            (53, Move::Hail),
            (59, Move::Thrash),
            (66, Move::SheerCold),
        ]
    };
    pub const Cryogonal: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Ice),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 11,
        weight: 1480,
        base_exp_yield: 180,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(50, 0),
            Stat::new(50, 0),
            Stat::new(95, 0),
            Stat::new(135, 2),
            Stat::new(105, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bind),
            (5, Move::IceShard),
            (9, Move::Sharpen),
            (13, Move::RapidSpin),
            (17, Move::IcyWind),
            (21, Move::Mist),
            (21, Move::Haze),
            (25, Move::AuroraBeam),
            (29, Move::AcidArmor),
            (33, Move::IceBeam),
            (37, Move::LightScreen),
            (37, Move::Reflect),
            (41, Move::Slash),
            (45, Move::ConfuseRay),
            (49, Move::Recover),
            (53, Move::SolarBeam),
            (57, Move::NightSlash),
            (61, Move::SheerCold),
        ]
    };
    pub const Shelmet: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::Two(Ability::Hydration, Ability::ShellArmor),
        hidden_ability: Some(Ability::Overcoat),
        height: 4,
        weight: 77,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(50, 0),
            Stat::new(40, 0),
            Stat::new(85, 1),
            Stat::new(40, 0),
            Stat::new(65, 0),
            Stat::new(25, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::LeechLife),
            (4, Move::Acid),
            (8, Move::Bide),
            (13, Move::Curse),
            (16, Move::StruggleBug),
            (20, Move::MegaDrain),
            (25, Move::Yawn),
            (28, Move::Protect),
            (32, Move::AcidArmor),
            (37, Move::GigaDrain),
            (40, Move::BodySlam),
            (44, Move::BugBuzz),
            (49, Move::Recover),
            (52, Move::GuardSwap),
            (56, Move::FinalGambit),
        ]
    };
    pub const Accelgor: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Bug),
        ability: PokemonAbility::Two(Ability::Hydration, Ability::StickyHold),
        hidden_ability: Some(Ability::Unburden),
        height: 8,
        weight: 253,
        base_exp_yield: 173,
        stats: Stats(
            Stat::new(80, 0),
            Stat::new(70, 0),
            Stat::new(40, 0),
            Stat::new(100, 0),
            Stat::new(60, 0),
            Stat::new(145, 2)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::QuickAttack),
            (1, Move::DoubleTeam),
            (1, Move::LeechLife),
            (1, Move::AcidSpray),
            (16, Move::StruggleBug),
            (20, Move::MegaDrain),
            (25, Move::Swift),
            (28, Move::MeFirst),
            (32, Move::Agility),
            (37, Move::GigaDrain),
            (40, Move::UTurn),
            (44, Move::BugBuzz),
            (49, Move::Recover),
            (52, Move::PowerSwap),
            (56, Move::FinalGambit),
        ]
    };
    pub const Stunfisk: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Electric),
        ability: PokemonAbility::Two(Ability::Static, Ability::Limber),
        hidden_ability: Some(Ability::SandVeil),
        height: 7,
        weight: 110,
        base_exp_yield: 165,
        stats: Stats(
            Stat::new(109, 2),
            Stat::new(66, 0),
            Stat::new(84, 0),
            Stat::new(81, 0),
            Stat::new(99, 0),
            Stat::new(32, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Water1, EggGroup::Indeterminate),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::MudSlap),
            (1, Move::MudSport),
            (5, Move::Bide),
            (9, Move::ThunderShock),
            (13, Move::MudShot),
            (17, Move::Camouflage),
            (21, Move::MudBomb),
            (25, Move::Discharge),
            (30, Move::Endure),
            (35, Move::Bounce),
            (40, Move::MuddyWater),
            (45, Move::Thunderbolt),
            (50, Move::Revenge),
            (55, Move::Flail),
            (61, Move::Fissure),
        ]
    };
    pub const Mienfoo: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::InnerFocus, Ability::Regenerator),
        hidden_ability: Some(Ability::Reckless),
        height: 9,
        weight: 200,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(85, 1),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (5, Move::Meditate),
            (9, Move::Detect),
            (13, Move::FakeOut),
            (17, Move::DoubleSlap),
            (21, Move::Swift),
            (25, Move::CalmMind),
            (29, Move::ForcePalm),
            (33, Move::DrainPunch),
            (37, Move::JumpKick),
            (41, Move::UTurn),
            (45, Move::QuickGuard),
            (49, Move::Bounce),
            (53, Move::HighJumpKick),
            (57, Move::Reversal),
            (61, Move::AuraSphere),
        ]
    };
    pub const Mienshao: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fighting),
        ability: PokemonAbility::Two(Ability::InnerFocus, Ability::Regenerator),
        hidden_ability: Some(Ability::Reckless),
        height: 14,
        weight: 355,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(125, 2),
            Stat::new(60, 0),
            Stat::new(95, 0),
            Stat::new(60, 0),
            Stat::new(105, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Ground, EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::Meditate),
            (1, Move::Detect),
            (1, Move::FakeOut),
            (17, Move::DoubleSlap),
            (21, Move::Swift),
            (25, Move::CalmMind),
            (29, Move::ForcePalm),
            (33, Move::DrainPunch),
            (37, Move::JumpKick),
            (41, Move::UTurn),
            (45, Move::WideGuard),
            (49, Move::Bounce),
            (56, Move::HighJumpKick),
            (63, Move::Reversal),
            (70, Move::AuraSphere),
        ]
    };
    pub const Druddigon: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Dragon),
        ability: PokemonAbility::Two(Ability::RoughSkin, Ability::SheerForce),
        hidden_ability: Some(Ability::MoldBreaker),
        height: 16,
        weight: 1390,
        base_exp_yield: 170,
        stats: Stats(
            Stat::new(77, 0),
            Stat::new(120, 2),
            Stat::new(90, 0),
            Stat::new(60, 0),
            Stat::new(90, 0),
            Stat::new(48, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::Two(EggGroup::Monster, EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (5, Move::HoneClaws),
            (9, Move::Bite),
            (13, Move::ScaryFace),
            (18, Move::DragonRage),
            (21, Move::Slash),
            (25, Move::Crunch),
            (27, Move::DragonClaw),
            (31, Move::ChipAway),
            (35, Move::Revenge),
            (40, Move::NightSlash),
            (45, Move::DragonTail),
            (49, Move::RockClimb),
            (55, Move::Superpower),
            (62, Move::Outrage),
        ]
    };
    pub const Golett: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Ghost),
        ability: PokemonAbility::Two(Ability::IronFist, Ability::Klutz),
        hidden_ability: Some(Ability::NoGuard),
        height: 10,
        weight: 920,
        base_exp_yield: 61,
        stats: Stats(
            Stat::new(59, 0),
            Stat::new(74, 1),
            Stat::new(50, 0),
            Stat::new(35, 0),
            Stat::new(50, 0),
            Stat::new(35, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::DefenseCurl),
            (1, Move::Astonish),
            (5, Move::MudSlap),
            (9, Move::Rollout),
            (13, Move::ShadowPunch),
            (17, Move::IronDefense),
            (21, Move::MegaPunch),
            (25, Move::Magnitude),
            (30, Move::DynamicPunch),
            (35, Move::NightShade),
            (40, Move::Curse),
            (45, Move::Earthquake),
            (50, Move::HammerArm),
            (55, Move::FocusPunch),
        ]
    };
    pub const Golurk: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Ghost),
        ability: PokemonAbility::Two(Ability::IronFist, Ability::Klutz),
        hidden_ability: Some(Ability::NoGuard),
        height: 28,
        weight: 3300,
        base_exp_yield: 169,
        stats: Stats(
            Stat::new(89, 0),
            Stat::new(124, 2),
            Stat::new(80, 0),
            Stat::new(55, 0),
            Stat::new(80, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Mineral),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Pound),
            (1, Move::DefenseCurl),
            (1, Move::MudSlap),
            (1, Move::Astonish),
            (9, Move::Rollout),
            (13, Move::ShadowPunch),
            (17, Move::IronDefense),
            (21, Move::MegaPunch),
            (25, Move::Magnitude),
            (30, Move::DynamicPunch),
            (35, Move::NightShade),
            (40, Move::Curse),
            (43, Move::HeavySlam),
            (50, Move::Earthquake),
            (60, Move::HammerArm),
            (70, Move::FocusPunch),
        ]
    };
    pub const Pawniard: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Steel),
        ability: PokemonAbility::Two(Ability::Defiant, Ability::InnerFocus),
        hidden_ability: Some(Ability::Pressure),
        height: 5,
        weight: 102,
        base_exp_yield: 68,
        stats: Stats(
            Stat::new(45, 0),
            Stat::new(85, 1),
            Stat::new(70, 0),
            Stat::new(40, 0),
            Stat::new(40, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (6, Move::Leer),
            (9, Move::FuryCutter),
            (14, Move::Torment),
            (17, Move::FeintAttack),
            (22, Move::ScaryFace),
            (25, Move::MetalClaw),
            (30, Move::Slash),
            (33, Move::Assurance),
            (38, Move::MetalSound),
            (41, Move::Embargo),
            (46, Move::IronDefense),
            (49, Move::NightSlash),
            (54, Move::IronHead),
            (57, Move::SwordsDance),
            (62, Move::Guillotine),
        ]
    };
    pub const Bisharp: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Steel),
        ability: PokemonAbility::Two(Ability::Defiant, Ability::InnerFocus),
        hidden_ability: Some(Ability::Pressure),
        height: 16,
        weight: 700,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(65, 0),
            Stat::new(125, 2),
            Stat::new(100, 0),
            Stat::new(60, 0),
            Stat::new(70, 0),
            Stat::new(70, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Humanshape),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Scratch),
            (1, Move::Leer),
            (1, Move::FuryCutter),
            (1, Move::Torment),
            (1, Move::MetalBurst),
            (17, Move::FeintAttack),
            (22, Move::ScaryFace),
            (25, Move::MetalClaw),
            (30, Move::Slash),
            (33, Move::Assurance),
            (38, Move::MetalSound),
            (41, Move::Embargo),
            (46, Move::IronDefense),
            (49, Move::NightSlash),
            (57, Move::IronHead),
            (63, Move::SwordsDance),
            (71, Move::Guillotine),
        ]
    };
    pub const Bouffalant: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Normal),
        ability: PokemonAbility::Two(Ability::Reckless, Ability::SapSipper),
        hidden_ability: Some(Ability::Soundproof),
        height: 16,
        weight: 946,
        base_exp_yield: 172,
        stats: Stats(
            Stat::new(95, 0),
            Stat::new(110, 2),
            Stat::new(95, 0),
            Stat::new(40, 0),
            Stat::new(95, 0),
            Stat::new(55, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Pursuit),
            (6, Move::Rage),
            (11, Move::FuryAttack),
            (16, Move::HornAttack),
            (21, Move::ScaryFace),
            (26, Move::Revenge),
            (31, Move::HeadCharge),
            (36, Move::FocusEnergy),
            (41, Move::Megahorn),
            (46, Move::Reversal),
            (51, Move::Thrash),
            (56, Move::SwordsDance),
            (61, Move::GigaImpact),
        ]
    };
    pub const Rufflet: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::SheerForce),
        hidden_ability: Some(Ability::Hustle),
        height: 5,
        weight: 105,
        base_exp_yield: 70,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(83, 1),
            Stat::new(50, 0),
            Stat::new(37, 0),
            Stat::new(50, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::Peck),
            (5, Move::FuryAttack),
            (10, Move::WingAttack),
            (14, Move::HoneClaws),
            (19, Move::ScaryFace),
            (23, Move::AerialAce),
            (28, Move::Slash),
            (32, Move::Defog),
            (37, Move::Tailwind),
            (41, Move::AirSlash),
            (46, Move::CrushClaw),
            (50, Move::SkyDrop),
            (55, Move::Whirlwind),
            (59, Move::BraveBird),
            (64, Move::Thrash),
        ]
    };
    pub const Braviary: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Flying),
        ability: PokemonAbility::Two(Ability::KeenEye, Ability::SheerForce),
        hidden_ability: Some(Ability::Defiant),
        height: 15,
        weight: 410,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(123, 2),
            Stat::new(75, 0),
            Stat::new(57, 0),
            Stat::new(75, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::WingAttack),
            (1, Move::FuryAttack),
            (1, Move::Leer),
            (1, Move::Peck),
            (14, Move::HoneClaws),
            (19, Move::ScaryFace),
            (23, Move::AerialAce),
            (28, Move::Slash),
            (32, Move::Defog),
            (37, Move::Tailwind),
            (41, Move::AirSlash),
            (46, Move::CrushClaw),
            (50, Move::SkyDrop),
            (51, Move::Superpower),
            (57, Move::Whirlwind),
            (63, Move::BraveBird),
            (70, Move::Thrash),
        ]
    };
    pub const Vullaby: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Flying),
        ability: PokemonAbility::Two(Ability::BigPecks, Ability::Overcoat),
        hidden_ability: Some(Ability::WeakArmor),
        height: 5,
        weight: 90,
        base_exp_yield: 74,
        stats: Stats(
            Stat::new(70, 0),
            Stat::new(55, 0),
            Stat::new(75, 1),
            Stat::new(45, 0),
            Stat::new(65, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Leer),
            (5, Move::FuryAttack),
            (10, Move::Pluck),
            (14, Move::NastyPlot),
            (19, Move::Flatter),
            (23, Move::FeintAttack),
            (28, Move::Punishment),
            (32, Move::Defog),
            (37, Move::Tailwind),
            (41, Move::AirSlash),
            (46, Move::DarkPulse),
            (50, Move::Embargo),
            (55, Move::Whirlwind),
            (59, Move::BraveBird),
            (64, Move::MirrorMove),
        ]
    };
    pub const Mandibuzz: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Flying),
        ability: PokemonAbility::Two(Ability::BigPecks, Ability::Overcoat),
        hidden_ability: Some(Ability::WeakArmor),
        height: 12,
        weight: 395,
        base_exp_yield: 179,
        stats: Stats(
            Stat::new(110, 0),
            Stat::new(65, 0),
            Stat::new(105, 0),
            Stat::new(55, 2),
            Stat::new(95, 0),
            Stat::new(80, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Flying),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::FuryAttack),
            (1, Move::Leer),
            (1, Move::Pluck),
            (14, Move::NastyPlot),
            (19, Move::Flatter),
            (23, Move::FeintAttack),
            (28, Move::Punishment),
            (32, Move::Defog),
            (37, Move::Tailwind),
            (41, Move::AirSlash),
            (46, Move::DarkPulse),
            (50, Move::Embargo),
            (51, Move::BoneRush),
            (57, Move::Whirlwind),
            (63, Move::BraveBird),
            (70, Move::MirrorMove),
        ]
    };
    pub const Heatmor: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Fire),
        ability: PokemonAbility::Two(Ability::Gluttony, Ability::FlashFire),
        hidden_ability: Some(Ability::WhiteSmoke),
        height: 14,
        weight: 580,
        base_exp_yield: 169,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(97, 0),
            Stat::new(66, 0),
            Stat::new(105, 2),
            Stat::new(66, 0),
            Stat::new(65, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Ground),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Lick),
            (1, Move::Incinerate),
            (6, Move::OdorSleuth),
            (11, Move::Bind),
            (16, Move::FireSpin),
            (21, Move::FurySwipes),
            (26, Move::Snatch),
            (31, Move::FlameBurst),
            (36, Move::BugBite),
            (41, Move::Slash),
            (46, Move::Amnesia),
            (51, Move::Flamethrower),
            (56, Move::Stockpile),
            (56, Move::SpitUp),
            (56, Move::Swallow),
            (61, Move::Inferno),
        ]
    };
    pub const Durant: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Steel),
        ability: PokemonAbility::Two(Ability::Swarm, Ability::Hustle),
        hidden_ability: Some(Ability::Truant),
        height: 3,
        weight: 330,
        base_exp_yield: 169,
        stats: Stats(
            Stat::new(58, 0),
            Stat::new(109, 0),
            Stat::new(112, 2),
            Stat::new(48, 0),
            Stat::new(48, 0),
            Stat::new(109, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ViseGrip),
            (1, Move::SandAttack),
            (6, Move::FuryCutter),
            (11, Move::Bite),
            (16, Move::Agility),
            (21, Move::MetalClaw),
            (26, Move::BugBite),
            (31, Move::Crunch),
            (36, Move::IronHead),
            (41, Move::Dig),
            (46, Move::Entrainment),
            (51, Move::XScissor),
            (56, Move::IronDefense),
            (61, Move::Guillotine),
            (66, Move::MetalSound),
        ]
    };
    pub const Deino: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Dragon),
        ability: PokemonAbility::One(Ability::Hustle),
        hidden_ability: None,
        height: 8,
        weight: 173,
        base_exp_yield: 60,
        stats: Stats(
            Stat::new(52, 0),
            Stat::new(65, 1),
            Stat::new(50, 0),
            Stat::new(45, 0),
            Stat::new(50, 0),
            Stat::new(38, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Tackle),
            (1, Move::DragonRage),
            (4, Move::FocusEnergy),
            (9, Move::Bite),
            (12, Move::Headbutt),
            (17, Move::DragonBreath),
            (20, Move::Roar),
            (25, Move::Crunch),
            (28, Move::Slam),
            (32, Move::DragonPulse),
            (38, Move::WorkUp),
            (42, Move::DragonRush),
            (48, Move::BodySlam),
            (52, Move::ScaryFace),
            (58, Move::HyperVoice),
            (62, Move::Outrage),
        ]
    };
    pub const Zweilous: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Dragon),
        ability: PokemonAbility::One(Ability::Hustle),
        hidden_ability: None,
        height: 14,
        weight: 500,
        base_exp_yield: 147,
        stats: Stats(
            Stat::new(72, 0),
            Stat::new(85, 2),
            Stat::new(70, 0),
            Stat::new(65, 0),
            Stat::new(70, 0),
            Stat::new(58, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bite),
            (1, Move::DragonRage),
            (1, Move::FocusEnergy),
            (1, Move::DoubleHit),
            (12, Move::Headbutt),
            (17, Move::DragonBreath),
            (20, Move::Roar),
            (25, Move::Crunch),
            (28, Move::Slam),
            (32, Move::DragonPulse),
            (38, Move::WorkUp),
            (42, Move::DragonRush),
            (48, Move::BodySlam),
            (55, Move::ScaryFace),
            (64, Move::HyperVoice),
            (71, Move::Outrage),
        ]
    };
    pub const Hydreigon: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dark, Type::Dragon),
        ability: PokemonAbility::One(Ability::Levitate),
        hidden_ability: None,
        height: 18,
        weight: 1600,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(92, 0),
            Stat::new(105, 0),
            Stat::new(90, 0),
            Stat::new(125, 3),
            Stat::new(90, 0),
            Stat::new(98, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Dragon),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Bite),
            (1, Move::DragonRage),
            (1, Move::FocusEnergy),
            (1, Move::TriAttack),
            (12, Move::Headbutt),
            (17, Move::DragonBreath),
            (20, Move::Roar),
            (25, Move::Crunch),
            (28, Move::Slam),
            (32, Move::DragonPulse),
            (38, Move::WorkUp),
            (42, Move::DragonRush),
            (48, Move::BodySlam),
            (55, Move::ScaryFace),
            (68, Move::HyperVoice),
            (79, Move::Outrage),
        ]
    };
    pub const Larvesta: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Fire),
        ability: PokemonAbility::One(Ability::FlameBody),
        hidden_ability: Some(Ability::Swarm),
        height: 11,
        weight: 288,
        base_exp_yield: 72,
        stats: Stats(
            Stat::new(55, 0),
            Stat::new(85, 1),
            Stat::new(55, 0),
            Stat::new(50, 0),
            Stat::new(55, 0),
            Stat::new(60, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Ember),
            (1, Move::StringShot),
            (10, Move::LeechLife),
            (20, Move::TakeDown),
            (30, Move::FlameCharge),
            (40, Move::BugBite),
            (50, Move::DoubleEdge),
            (60, Move::FlameWheel),
            (70, Move::BugBuzz),
            (80, Move::Amnesia),
            (90, Move::Thrash),
            (100, Move::FlareBlitz),
        ]
    };
    pub const Volcarona: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Fire),
        ability: PokemonAbility::One(Ability::FlameBody),
        hidden_ability: Some(Ability::Swarm),
        height: 16,
        weight: 460,
        base_exp_yield: 248,
        stats: Stats(
            Stat::new(85, 0),
            Stat::new(60, 0),
            Stat::new(65, 0),
            Stat::new(135, 3),
            Stat::new(105, 0),
            Stat::new(100, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::One(EggGroup::Bug),
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Ember),
            (1, Move::StringShot),
            (1, Move::LeechLife),
            (30, Move::FireSpin),
            (40, Move::Whirlwind),
            (50, Move::SilverWind),
            (59, Move::QuiverDance),
            (60, Move::HeatWave),
            (70, Move::BugBuzz),
            (80, Move::RagePowder),
            (90, Move::Hurricane),
            (100, Move::FieryDance),
        ]
    };
    pub const Cobalion: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Steel, Type::Fighting),
        ability: PokemonAbility::One(Ability::Justified),
        hidden_ability: None,
        height: 21,
        weight: 2500,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(91, 0),
            Stat::new(90, 0),
            Stat::new(129, 3),
            Stat::new(90, 0),
            Stat::new(72, 0),
            Stat::new(108, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (7, Move::DoubleKick),
            (13, Move::MetalClaw),
            (19, Move::TakeDown),
            (25, Move::HelpingHand),
            (31, Move::Retaliate),
            (37, Move::IronHead),
            (42, Move::SacredSword),
            (49, Move::SwordsDance),
            (55, Move::QuickGuard),
            (61, Move::WorkUp),
            (67, Move::MetalBurst),
            (73, Move::CloseCombat),
        ]
    };
    pub const Terrakion: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Rock, Type::Fighting),
        ability: PokemonAbility::One(Ability::Justified),
        hidden_ability: None,
        height: 19,
        weight: 2600,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(91, 0),
            Stat::new(129, 3),
            Stat::new(90, 0),
            Stat::new(72, 0),
            Stat::new(90, 0),
            Stat::new(108, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (7, Move::DoubleKick),
            (13, Move::SmackDown),
            (19, Move::TakeDown),
            (25, Move::HelpingHand),
            (31, Move::Retaliate),
            (37, Move::RockSlide),
            (42, Move::SacredSword),
            (49, Move::SwordsDance),
            (55, Move::QuickGuard),
            (61, Move::WorkUp),
            (67, Move::StoneEdge),
            (73, Move::CloseCombat),
        ]
    };
    pub const Virizion: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Grass, Type::Fighting),
        ability: PokemonAbility::One(Ability::Justified),
        hidden_ability: None,
        height: 20,
        weight: 2000,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(91, 0),
            Stat::new(90, 0),
            Stat::new(72, 0),
            Stat::new(90, 0),
            Stat::new(129, 3),
            Stat::new(108, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::QuickAttack),
            (7, Move::DoubleKick),
            (13, Move::MagicalLeaf),
            (19, Move::TakeDown),
            (25, Move::HelpingHand),
            (31, Move::Retaliate),
            (37, Move::GigaDrain),
            (42, Move::SacredSword),
            (49, Move::SwordsDance),
            (55, Move::QuickGuard),
            (61, Move::WorkUp),
            (67, Move::LeafBlade),
            (73, Move::CloseCombat),
        ]
    };
    pub const TornadusIncarnate: PokemonData = PokemonData {
        _type: PokemonType::Single(Type::Flying),
        ability: PokemonAbility::One(Ability::Prankster),
        hidden_ability: Some(Ability::Defiant),
        height: 15,
        weight: 630,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(79, 0),
            Stat::new(115, 3),
            Stat::new(70, 0),
            Stat::new(125, 0),
            Stat::new(80, 0),
            Stat::new(111, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Gust),
            (1, Move::Uproar),
            (1, Move::Astonish),
            (7, Move::Swagger),
            (13, Move::Bite),
            (19, Move::Revenge),
            (25, Move::AirCutter),
            (31, Move::Extrasensory),
            (37, Move::Agility),
            (43, Move::AirSlash),
            (49, Move::Crunch),
            (55, Move::Tailwind),
            (61, Move::RainDance),
            (67, Move::Hurricane),
            (73, Move::DarkPulse),
            (79, Move::HammerArm),
            (85, Move::Thrash),
        ]
    };
    pub const ThundurusIncarnate: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Electric, Type::Flying),
        ability: PokemonAbility::One(Ability::Prankster),
        hidden_ability: Some(Ability::Defiant),
        height: 15,
        weight: 610,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(79, 0),
            Stat::new(115, 3),
            Stat::new(70, 0),
            Stat::new(125, 0),
            Stat::new(80, 0),
            Stat::new(111, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::ThunderShock),
            (1, Move::Uproar),
            (1, Move::Astonish),
            (7, Move::Swagger),
            (13, Move::Bite),
            (19, Move::Revenge),
            (25, Move::ShockWave),
            (31, Move::HealBlock),
            (37, Move::Agility),
            (43, Move::Discharge),
            (49, Move::Crunch),
            (55, Move::Charge),
            (61, Move::NastyPlot),
            (67, Move::Thunder),
            (73, Move::DarkPulse),
            (79, Move::HammerArm),
            (85, Move::Thrash),
        ]
    };
    pub const Reshiram: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Fire),
        ability: PokemonAbility::One(Ability::Turboblaze),
        hidden_ability: None,
        height: 32,
        weight: 3300,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(120, 0),
            Stat::new(100, 0),
            Stat::new(150, 3),
            Stat::new(120, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DragonRage),
            (1, Move::FireFang),
            (8, Move::Imprison),
            (15, Move::AncientPower),
            (22, Move::Flamethrower),
            (29, Move::DragonBreath),
            (36, Move::Slash),
            (43, Move::Extrasensory),
            (50, Move::FusionFlare),
            (54, Move::DragonPulse),
            (71, Move::Crunch),
            (78, Move::FireBlast),
            (85, Move::Outrage),
            (92, Move::HyperVoice),
            (100, Move::BlueFlare),
        ]
    };
    pub const Zekrom: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Electric),
        ability: PokemonAbility::One(Ability::Teravolt),
        hidden_ability: None,
        height: 29,
        weight: 3450,
        base_exp_yield: 306,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(150, 3),
            Stat::new(120, 0),
            Stat::new(120, 0),
            Stat::new(100, 0),
            Stat::new(90, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DragonRage),
            (1, Move::ThunderFang),
            (8, Move::Imprison),
            (15, Move::AncientPower),
            (22, Move::Thunderbolt),
            (29, Move::DragonBreath),
            (36, Move::Slash),
            (43, Move::ZenHeadbutt),
            (50, Move::FusionBolt),
            (54, Move::DragonClaw),
            (71, Move::Crunch),
            (78, Move::Thunder),
            (85, Move::Outrage),
            (92, Move::HyperVoice),
            (100, Move::BoltStrike),
        ]
    };
    pub const LandorusIncarnate: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Ground, Type::Flying),
        ability: PokemonAbility::One(Ability::SandForce),
        hidden_ability: Some(Ability::SheerForce),
        height: 15,
        weight: 680,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(89, 0),
            Stat::new(125, 0),
            Stat::new(90, 0),
            Stat::new(115, 3),
            Stat::new(80, 0),
            Stat::new(101, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::RockTomb),
            (1, Move::Block),
            (1, Move::MudShot),
            (7, Move::Imprison),
            (13, Move::Punishment),
            (19, Move::Bulldoze),
            (25, Move::RockThrow),
            (31, Move::Extrasensory),
            (37, Move::SwordsDance),
            (43, Move::EarthPower),
            (49, Move::RockSlide),
            (55, Move::Earthquake),
            (61, Move::Sandstorm),
            (67, Move::Fissure),
            (73, Move::StoneEdge),
            (79, Move::HammerArm),
            (85, Move::Outrage),
        ]
    };
    pub const Kyurem: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Dragon, Type::Ice),
        ability: PokemonAbility::One(Ability::Pressure),
        hidden_ability: None,
        height: 30,
        weight: 3250,
        base_exp_yield: 297,
        stats: Stats(
            Stat::new(125, 1),
            Stat::new(130, 1),
            Stat::new(90, 0),
            Stat::new(130, 1),
            Stat::new(90, 0),
            Stat::new(95, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::DragonRage),
            (1, Move::IcyWind),
            (8, Move::Imprison),
            (15, Move::AncientPower),
            (22, Move::IceBeam),
            (29, Move::DragonBreath),
            (36, Move::Slash),
            (43, Move::ScaryFace),
            (50, Move::Glaciate),
            (57, Move::DragonPulse),
            (71, Move::Endeavor),
            (78, Move::Blizzard),
            (85, Move::Outrage),
            (92, Move::HyperVoice),
        ]
    };
    pub const KeldeoOrdinary: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Water, Type::Fighting),
        ability: PokemonAbility::One(Ability::Justified),
        hidden_ability: None,
        height: 14,
        weight: 485,
        base_exp_yield: 261,
        stats: Stats(
            Stat::new(91, 0),
            Stat::new(72, 0),
            Stat::new(90, 0),
            Stat::new(129, 3),
            Stat::new(90, 0),
            Stat::new(108, 0)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Leer),
            (1, Move::AquaJet),
            (7, Move::DoubleKick),
            (13, Move::BubbleBeam),
            (19, Move::TakeDown),
            (25, Move::HelpingHand),
            (31, Move::Retaliate),
            (37, Move::AquaTail),
            (43, Move::SacredSword),
            (49, Move::SwordsDance),
            (55, Move::QuickGuard),
            (61, Move::WorkUp),
            (67, Move::HydroPump),
            (73, Move::CloseCombat),
        ]
    };
    pub const MeloettaAria: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Normal, Type::Psychic),
        ability: PokemonAbility::One(Ability::SereneGrace),
        hidden_ability: None,
        height: 6,
        weight: 65,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(100, 0),
            Stat::new(77, 0),
            Stat::new(77, 0),
            Stat::new(128, 1),
            Stat::new(128, 1),
            Stat::new(90, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::Round),
            (6, Move::QuickAttack),
            (11, Move::Confusion),
            (16, Move::Sing),
            (21, Move::TeeterDance),
            (26, Move::Acrobatics),
            (31, Move::Psybeam),
            (36, Move::EchoedVoice),
            (43, Move::UTurn),
            (50, Move::WakeUpSlap),
            (57, Move::Psychic),
            (64, Move::HyperVoice),
            (71, Move::RolePlay),
            (78, Move::CloseCombat),
            (85, Move::PerishSong),
        ]
    };
    pub const Genesect: PokemonData = PokemonData {
        _type: PokemonType::Double(Type::Bug, Type::Steel),
        ability: PokemonAbility::One(Ability::Download),
        hidden_ability: None,
        height: 15,
        weight: 825,
        base_exp_yield: 270,
        stats: Stats(
            Stat::new(71, 0),
            Stat::new(120, 1),
            Stat::new(95, 0),
            Stat::new(120, 1),
            Stat::new(95, 0),
            Stat::new(99, 1)
        ),
        gender_ratio: GenderRatio::Proportion(1, 1), //Hard-coded
        catch_rate: 45, //Hard-coded
        egg_group: PokemonEggGroup::None,
        egg_cycles: 20, //Hard-coded
        base_friendship: 70, //Hard-coded
        level_rate: LevelRate::MediumSlow, //Hard-coded
        level_up_moves: &[
            (1, Move::QuickAttack),
            (1, Move::Screech),
            (1, Move::MetalClaw),
            (1, Move::MagnetRise),
            (1, Move::TechnoBlast),
            (7, Move::FuryCutter),
            (11, Move::LockOn),
            (18, Move::FlameCharge),
            (22, Move::MagnetBomb),
            (29, Move::Slash),
            (33, Move::MetalSound),
            (40, Move::SignalBeam),
            (44, Move::TriAttack),
            (51, Move::XScissor),
            (55, Move::BugBuzz),
            (62, Move::SimpleBeam),
            (66, Move::ZapCannon),
            (73, Move::HyperBeam),
            (77, Move::SelfDestruct),
        ]
    };
}
//endregion