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
    base_friendship: u8
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
            Species::NidoranF => PokemonData::Nidoranf,
            Species::Nidorina => PokemonData::Nidorina,
            Species::Nidoqueen => PokemonData::Nidoqueen,
            Species::NidoranM => PokemonData::Nidoranm,
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
            Species::MrMime => PokemonData::Mrmime,
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
            Species::HoOh => PokemonData::Hooh,
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
            Species::Deoxys(_) => PokemonData::Deoxysnormal,
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
            Species::Wormadam(_) => PokemonData::Wormadamplant,
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
            Species::MimeJr => PokemonData::Mimejr,
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
            Species::PorygonZ => PokemonData::Porygonz,
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
            Species::Giratina(_) => PokemonData::Giratinaaltered,
            Species::Cresselia => PokemonData::Cresselia,
            Species::Phione => PokemonData::Phione,
            Species::Manaphy => PokemonData::Manaphy,
            Species::Darkrai => PokemonData::Darkrai,
            Species::Shaymin(_) => PokemonData::Shayminland,
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
            Species::Basculin(_) => PokemonData::Basculinredstriped,
            Species::Sandile => PokemonData::Sandile,
            Species::Krokorok => PokemonData::Krokorok,
            Species::Krookodile => PokemonData::Krookodile,
            Species::Darumaka => PokemonData::Darumaka,
            Species::Darmanitan(_) => PokemonData::Darmanitanstandard,
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
            Species::Tornadus(_) => PokemonData::Tornadusincarnate,
            Species::Thundurus(_) => PokemonData::Thundurusincarnate,
            Species::Reshiram => PokemonData::Reshiram,
            Species::Zekrom => PokemonData::Zekrom,
            Species::Landorus(_) => PokemonData::Landorusincarnate,
            Species::Kyurem(_) => PokemonData::Kyurem,
            Species::Keldeo(_) => PokemonData::Keldeoordinary,
            Species::Meloetta(_) => PokemonData::Meloettaaria,
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Nidoranf: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Nidoranm: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Mrmime: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Hooh: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Deoxysnormal: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Wormadamplant: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Mimejr: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Porygonz: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Giratinaaltered: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Shayminland: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Basculinredstriped: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Darmanitanstandard: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Tornadusincarnate: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Thundurusincarnate: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Landorusincarnate: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Keldeoordinary: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
    pub const Meloettaaria: PokemonData = PokemonData {
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
        level_rate: LevelRate::MediumSlow //Hard-coded
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
        level_rate: LevelRate::MediumSlow //Hard-coded
    };
}
//endregion