#![allow(non_upper_case_globals)]
use serde::{Serialize, Deserialize};

use crate::attack::Move;
use crate::types::Type;

/// All Pokeballs in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Pokeball {
    PokeBall,
    GreatBall,
    UltraBall,
    MasterBall,
    SafariBall,
    FastBall,
    LevelBall,
    LureBall,
    HeavyBall,
    LoveBall,
    FriendBall,
    MoonBall,
    SportBall,
    NetBall,
    NestBall,
    RepeatBall,
    TimerBall,
    LuxuryBall,
    PremierBall,
    DiveBall,
    DuskBall,
    HealBall,
    QuickBall,
    CherishBall,
    ParkBall,
    DreamBall
}
impl From<Pokeball> for Item {
    fn from(p: Pokeball) -> Self { Item::Pokeball(p) }
}

/// All Evolution Stones in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EvolutionStone {
    FireStone,
    WaterStone,
    ThunderStone,
    LeafStone,
    MoonStone,
    SunStone,
    ShinyStone,
    DuskStone,
    DawnStone,
    IceStone
}
impl From<EvolutionStone> for Item {
    fn from(p: EvolutionStone) -> Self { Item::Stone(p) }
}

/// All incenses in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Incense {
    FullIncense,
    LaxIncense,
    LuckIncense,
    OddIncense,
    PureIncense,
    RockIncense,
    RoseIncense,
    SeaIncense,
    WaveIncense
}
impl From<Incense> for Item {
    fn from(p: Incense) -> Self { Item::Incense(p) }
}

/// All Evolution-inducing held items in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EvolutionHeldItem {
    DeepSeaTooth,
    DeepSeaScale,
    DragonScale,
    DubiousDisk,
    Electirizer,
    KingsRock,
    Magmarizer,
    MetalCoat,
    OvalStone,
    PrismScale,
    Protector,
    RazorClaw,
    RazorFang,
    ReaperCloth,
    UpGrade
}
impl From<EvolutionHeldItem> for Item {
    fn from(p: EvolutionHeldItem) -> Self { Item::EvolutionHeldItem(p) }
}

/// All berries in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Berry {
    CheriBerry,
    ChestoBerry,
    PechaBerry,
    RawstBerry,
    AspearBerry,
    LeppaBerry,
    OranBerry,
    PersimBerry,
    LumBerry,
    SitrusBerry,
    FigyBerry,
    WikiBerry,
    MagoBerry,
    AguavBerry,
    IapapaBerry,
    RazzBerry,
    BlukBerry,
    NanabBerry,
    WepearBerry,
    PinapBerry,
    PomegBerry,
    KelpsyBerry,
    QualotBerry,
    HondewBerry,
    GrepaBerry,
    TamatoBerry,
    CornnBerry,
    MagostBerry,
    RabutaBerry,
    NomelBerry,
    SpelonBerry,
    PamtreBerry,
    WatmelBerry,
    DurinBerry,
    BelueBerry,
    OccaBerry,
    PasshoBerry,
    WacanBerry,
    RindoBerry,
    YacheBerry,
    ChopleBerry,
    KebiaBerry,
    ShucaBerry,
    CobaBerry,
    PayapaBerry,
    TangaBerry,
    ChartiBerry,
    KasibBerry,
    HabanBerry,
    ColburBerry,
    BabiriBerry,
    ChilanBerry,
    LiechiBerry,
    GanlonBerry,
    SalacBerry,
    PetayaBerry,
    ApicotBerry,
    LansatBerry,
    StarfBerry,
    EnigmaBerry,
    MicleBerry,
    CustapBerry,
    JabocaBerry,
    RowapBerry,
    RoseliBerry,
    KeeBerry,
    MarangaBerry
}
impl From<Berry> for Item {
    fn from(p: Berry) -> Self { Item::Berry(p) }
}
impl Berry {
    pub fn get_natural_gift_type(&self) -> Type {
        match self {
            Berry::FigyBerry | Berry::CornnBerry | Berry::TangaBerry | Berry::EnigmaBerry => Type::Bug,
            Berry::IapapaBerry | Berry::SpelonBerry | Berry::ColburBerry | Berry::RowapBerry | Berry::MarangaBerry => Type::Dark,
            Berry::AguavBerry | Berry::NomelBerry | Berry::HabanBerry | Berry::JabocaBerry => Type::Dragon,
            Berry::PechaBerry | Berry::WepearBerry | Berry::BelueBerry | Berry::WacanBerry => Type::Electric,
            Berry::RoseliBerry | Berry::KeeBerry => Type::Fairy,
            Berry::LeppaBerry | Berry::KelpsyBerry | Berry::ChopleBerry | Berry::SalacBerry => Type::Fighting,
            Berry::CheriBerry | Berry::BlukBerry | Berry::WatmelBerry | Berry::OccaBerry => Type::Fire,
            Berry::LumBerry | Berry::GrepaBerry | Berry::CobaBerry | Berry::LansatBerry => Type::Flying,
            Berry::MagoBerry | Berry::RabutaBerry | Berry::KasibBerry | Berry::CustapBerry => Type::Ghost,
            Berry::RawstBerry | Berry::PinapBerry | Berry::RindoBerry | Berry::LiechiBerry => Type::Grass,
            Berry::PersimBerry | Berry::HondewBerry | Berry::ShucaBerry | Berry::ApicotBerry => Type::Ground,
            Berry::AspearBerry | Berry::PomegBerry | Berry::YacheBerry | Berry::GanlonBerry => Type::Ice,
            Berry::ChilanBerry => Type::Normal,
            Berry::OranBerry | Berry::QualotBerry | Berry::KebiaBerry | Berry::PetayaBerry => Type::Poison,
            Berry::SitrusBerry | Berry::TamatoBerry | Berry::PayapaBerry | Berry::StarfBerry => Type::Psychic,
            Berry::WikiBerry | Berry::MagostBerry | Berry::ChartiBerry | Berry::MicleBerry => Type::Rock,
            Berry::RazzBerry | Berry::PamtreBerry | Berry::BabiriBerry => Type::Steel,
            Berry::ChestoBerry | Berry::NanabBerry | Berry::DurinBerry | Berry::PasshoBerry => Type::Water
        }
    }

    pub fn get_resistance_berry_type(&self) -> Option<Type> {
        match self {
            Berry::OccaBerry => Some(Type::Fire),
            Berry::PasshoBerry => Some(Type::Water),
            Berry::WacanBerry => Some(Type::Electric),
            Berry::RindoBerry => Some(Type::Grass),
            Berry::YacheBerry => Some(Type::Ice),
            Berry::ChopleBerry => Some(Type::Fighting),
            Berry::KebiaBerry => Some(Type::Poison),
            Berry::ShucaBerry => Some(Type::Ground),
            Berry::CobaBerry => Some(Type::Flying),
            Berry::PayapaBerry => Some(Type::Psychic),
            Berry::TangaBerry => Some(Type::Bug),
            Berry::ChartiBerry => Some(Type::Rock),
            Berry::KasibBerry => Some(Type::Ghost),
            Berry::HabanBerry => Some(Type::Dragon),
            Berry::ColburBerry => Some(Type::Dark),
            Berry::BabiriBerry => Some(Type::Steel),
            Berry::ChilanBerry => Some(Type::Normal),
            Berry::RoseliBerry => Some(Type::Fairy),
            _ => None
        }
    }
}

//region TMs and HMs
/// All TMs in the game, and their mappings
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TM {
    TM01, TM02, TM03, TM04, TM05, TM06, TM07, TM08, TM09, TM10, TM11, TM12, TM13, TM14, TM15, TM16, TM17, TM18, TM19, TM20, TM21, TM22, TM23, TM24, TM25, TM26, TM27, TM28, TM29, TM30, TM31, TM32, TM33, TM34, TM35, TM36, TM37, TM38, TM39, TM40, TM41, TM42, TM43, TM44, TM45, TM46, TM47, TM48, TM49, TM50, TM51, TM52, TM53, TM54, TM55, TM56, TM57, TM58, TM59, TM60, TM61, TM62, TM63, TM64, TM65, TM66, TM67, TM68, TM69, TM70, TM71, TM72, TM73, TM74, TM75, TM76, TM77, TM78, TM79, TM80, TM81, TM82, TM83, TM84, TM85, TM86, TM87, TM88, TM89, TM90, TM91, TM92, TM93, TM94, TM95, TM96, TM97
}
impl TM {
    pub fn get_move(&self) -> &Move {
        match self {
            TM::TM01 => &Move::HoneClaws,
            TM::TM02 => &Move::DragonClaw,
            TM::TM03 => &Move::Psyshock,
            TM::TM04 => &Move::CalmMind,
            TM::TM05 => &Move::Roar,
            TM::TM06 => &Move::Toxic,
            TM::TM07 => &Move::Hail,
            TM::TM08 => &Move::BulkUp,
            TM::TM09 => &Move::Venoshock,
            TM::TM10 => &Move::HiddenPower,
            TM::TM11 => &Move::SunnyDay,
            TM::TM12 => &Move::Taunt,
            TM::TM13 => &Move::IceBeam,
            TM::TM14 => &Move::Blizzard,
            TM::TM15 => &Move::HyperBeam,
            TM::TM16 => &Move::LightScreen,
            TM::TM17 => &Move::Protect,
            TM::TM18 => &Move::RainDance,
            TM::TM19 => &Move::Roost,
            TM::TM20 => &Move::Safeguard,
            TM::TM21 => &Move::Frustration,
            TM::TM22 => &Move::SolarBeam,
            TM::TM23 => &Move::SmackDown,
            TM::TM24 => &Move::Thunderbolt,
            TM::TM25 => &Move::Thunder,
            TM::TM26 => &Move::Earthquake,
            TM::TM27 => &Move::Return,
            TM::TM28 => &Move::Dig,
            TM::TM29 => &Move::Psychic,
            TM::TM30 => &Move::ShadowBall,
            TM::TM31 => &Move::BrickBreak,
            TM::TM32 => &Move::DoubleTeam,
            TM::TM33 => &Move::Reflect,
            TM::TM34 => &Move::SludgeWave,
            TM::TM35 => &Move::Flamethrower,
            TM::TM36 => &Move::SludgeBomb,
            TM::TM37 => &Move::Sandstorm,
            TM::TM38 => &Move::FireBlast,
            TM::TM39 => &Move::RockTomb,
            TM::TM40 => &Move::AerialAce,
            TM::TM41 => &Move::Torment,
            TM::TM42 => &Move::Facade,
            TM::TM43 => &Move::FlameCharge,
            TM::TM44 => &Move::Rest,
            TM::TM45 => &Move::Attract,
            TM::TM46 => &Move::Thief,
            TM::TM47 => &Move::LowSweep,
            TM::TM48 => &Move::Round,
            TM::TM49 => &Move::EchoedVoice,
            TM::TM50 => &Move::Overheat,
            TM::TM51 => &Move::SteelWing,
            TM::TM52 => &Move::FocusBlast,
            TM::TM53 => &Move::EnergyBall,
            TM::TM54 => &Move::FalseSwipe,
            TM::TM55 => &Move::Scald,
            TM::TM56 => &Move::Fling,
            TM::TM57 => &Move::ChargeBeam,
            TM::TM58 => &Move::SkyDrop,
            TM::TM59 => &Move::Incinerate,
            TM::TM60 => &Move::Quash,
            TM::TM61 => &Move::WillOWisp,
            TM::TM62 => &Move::Acrobatics,
            TM::TM63 => &Move::Embargo,
            TM::TM64 => &Move::Explosion,
            TM::TM65 => &Move::ShadowClaw,
            TM::TM66 => &Move::Payback,
            TM::TM67 => &Move::Retaliate,
            TM::TM68 => &Move::GigaImpact,
            TM::TM69 => &Move::RockPolish,
            TM::TM70 => &Move::Flash,
            TM::TM71 => &Move::StoneEdge,
            TM::TM72 => &Move::VoltSwitch,
            TM::TM73 => &Move::ThunderWave,
            TM::TM74 => &Move::GyroBall,
            TM::TM75 => &Move::SwordsDance,
            TM::TM76 => &Move::StruggleBug,
            TM::TM77 => &Move::PsychUp,
            TM::TM78 => &Move::Bulldoze,
            TM::TM79 => &Move::FrostBreath,
            TM::TM80 => &Move::RockSlide,
            TM::TM81 => &Move::XScissor,
            TM::TM82 => &Move::DragonTail,
            TM::TM83 => &Move::WorkUp,
            TM::TM84 => &Move::PoisonJab,
            TM::TM85 => &Move::DreamEater,
            TM::TM86 => &Move::GrassKnot,
            TM::TM87 => &Move::Swagger,
            TM::TM88 => &Move::SleepTalk,
            TM::TM89 => &Move::UTurn,
            TM::TM90 => &Move::Substitute,
            TM::TM91 => &Move::FlashCannon,
            TM::TM92 => &Move::TrickRoom,
            TM::TM93 => &Move::WildCharge,
            TM::TM94 => &Move::SecretPower,
            TM::TM95 => &Move::Snarl,
            TM::TM96 => &Move::NaturePower,
            TM::TM97 => &Move::DarkPulse
        }
    }
}
impl From<TM> for Item {
    fn from(p: TM) -> Self { Item::TM(p) }
}

/// All HMs in the game, and their mappings
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HM {
    HM01, HM02, HM03, HM04, HM05, HM06, HM07, HM08
}
impl HM {
    pub fn get_move(&self) -> &Move {
        match self {
            HM::HM01 => &Move::Cut,
            HM::HM02 => &Move::Fly,
            HM::HM03 => &Move::Surf,
            HM::HM04 => &Move::Strength,
            HM::HM05 => &Move::Flash,
            HM::HM06 => &Move::RockSmash,
            HM::HM07 => &Move::Waterfall,
            HM::HM08 => &Move::Dive
        }
    }
}
impl From<HM> for Item {
    fn from(p: HM) -> Self { Item::HM(p) }
}
//endregion

/// All Mail in the game
/// Messages are attached via the included String
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mail {
    AirMail(String),
    BloomMail(String),
    BrickMail(String),
    BubbleMail(String),
    FlameMail(String),
    GrassMail(String),
    HeartMail(String),
    MosaicMail(String),
    SnowMail(String),
    SpaceMail(String),
    SteelMail(String),
    TunnelMail(String)
}
impl From<Mail> for Item {
    fn from(p: Mail) -> Self { Item::Mail(p) }
}

/// All Key items in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeyItem {

}
impl From<KeyItem> for Item {
    fn from(p: KeyItem) -> Self { Item::KeyItem(p) }
}

/// All Items in the game
/// Items from the above lists are wrapped, so they can still be considered items
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    Pokeball(Pokeball),
    Stone(EvolutionStone),
    Everstone,
    Incense(Incense),
    EvolutionHeldItem(EvolutionHeldItem),
    Berry(Berry),
    TM(TM),
    HM(HM),
    Mail(Mail),
    KeyItem(KeyItem),
    // Use Items
    // Health
    Potion, SuperPotion, HyperPotion, MaxPotion,
    BerryJuice, EnergyRoot, EnergyPowder, FreshWater, Lemonade, MoomooMilk, RageCandyBar,
    SacredAsh, ShellBell, SodaPop, SweetHeart, RevivalHerb,
    // Status
    Antidote, Awakening, BurnHeal, FullHeal, FullRestore, IceHeal, ParalyzeHeal, Revive, MaxRevive,
    LavaCookie, MentalHerb, OldGateau, Casteliacone,
    // PP
    Ether, MaxEther, Elixir, MaxElixir, PPUp, PPMax,
    // Stats
    HPUp, Protein, Iron, Calcium, Zinc, Carbos,
    // Held Items
    ShockDrive, BurnDrive, ChillDrive, DouseDrive,
    PowerAnklet, PowerBand, PowerBelt, PowerBracer, PowerLens, PowerWeight,
    ExpShare, LuckyEgg,
    DracoPlate, DreadPlate, EarthPlate, FistPlate, FlamePlate, IciclePlate, InsectPlate, IronPlate,
    MeadowPlate, MindPlate, PixiePlate, SkyPlate, SplashPlate, SpookyPlate, StonePlate, ToxicPlate, ZapPlate,
    Eviolite, Leek, LightBall, LuckyPunch, MetalPowder, QuickPowder, SoulDew, ThickClub,
    BlackBelt, BlackGlasses, Charcoal, DragonFang, HardStone, Magnet, MetalCoat, MiracleSeed, MysticWater,
    NeverMeltIce, PoisonBarb, SharpBeak, SilkScarf, SilverPowder, SoftSand, SpellTag, TwistedSpoon,
    AdamantOrb, LustrousOrb, GriseousOrb,
    BlueScarf, GreenScarf, PinkScarf, RedScarf, YellowScarf,
    SootheBell, CleanseTag, ChoiceBand, ChoiceScarf, ChoiceSpecs, BlackSludge, LifeOrb, RockyHelmet, StickyBarb,
    GripClaw, LightClay, TerrainExtender, AssaultVest, FlameOrb, IronBall, LaggingTail, MachoBrace,
    RingTarget, ToxicOrb, Leftovers, AbsorbBulb, CellBattery, EjectButton, LuminousMoss,
    Snowball, WeaknessPolicy, WideLens, ZoomLens, ScopeLens, BrightPowder, ExpertBelt, Metronome, FocusSash,
    FloatStone, BindingBand,
    HeatRock, DampRock, SmoothRock, IcyRock
}

#[derive(Debug)]
pub enum Pocket {
    Items,
    Medicine,
    PokeBalls,
    TMsAndHMs,
    Berries,
    Mail,
    BattleItems,
    KeyItems
}

#[derive(Debug)]
pub struct ItemData {
    pub cost: u16,
    pub pocket: Pocket,
    pub countable: bool,
    pub consumable: bool,
    pub holdable: bool,
    pub battle_usable: bool,
    pub overworld_usable: bool
}

//region Item constants
impl Item {
    pub fn data(&self) -> &'static ItemData {
        match self {
            Item::Pokeball(Pokeball::PokeBall) => &PokeBall,
            Item::Pokeball(Pokeball::GreatBall) => &GreatBall,
            Item::Pokeball(Pokeball::UltraBall) => &UltraBall,
            Item::Pokeball(Pokeball::MasterBall) => &MasterBall,
            Item::Pokeball(Pokeball::SafariBall) => &SafariBall,
            Item::Pokeball(Pokeball::FastBall) => &FastBall,
            Item::Pokeball(Pokeball::LevelBall) => &LevelBall,
            Item::Pokeball(Pokeball::LureBall) => &LureBall,
            Item::Pokeball(Pokeball::HeavyBall) => &HeavyBall,
            Item::Pokeball(Pokeball::LoveBall) => &LoveBall,
            Item::Pokeball(Pokeball::FriendBall) => &FriendBall,
            Item::Pokeball(Pokeball::MoonBall) => &MoonBall,
            Item::Pokeball(Pokeball::SportBall) => &SportBall,
            Item::Pokeball(Pokeball::NetBall) => &NetBall,
            Item::Pokeball(Pokeball::NestBall) => &NestBall,
            Item::Pokeball(Pokeball::RepeatBall) => &RepeatBall,
            Item::Pokeball(Pokeball::TimerBall) => &TimerBall,
            Item::Pokeball(Pokeball::LuxuryBall) => &LuxuryBall,
            Item::Pokeball(Pokeball::PremierBall) => &PremierBall,
            Item::Pokeball(Pokeball::DiveBall) => &DiveBall,
            Item::Pokeball(Pokeball::DuskBall) => &DuskBall,
            Item::Pokeball(Pokeball::HealBall) => &HealBall,
            Item::Pokeball(Pokeball::QuickBall) => &QuickBall,
            Item::Pokeball(Pokeball::CherishBall) => &CherishBall,
            Item::Pokeball(Pokeball::ParkBall) => &ParkBall,
            Item::Pokeball(Pokeball::DreamBall) => &DreamBall,
            Item::Stone(EvolutionStone::FireStone) => &FireStone,
            Item::Stone(EvolutionStone::WaterStone) => &WaterStone,
            Item::Stone(EvolutionStone::ThunderStone) => &ThunderStone,
            Item::Stone(EvolutionStone::LeafStone) => &LeafStone,
            Item::Stone(EvolutionStone::MoonStone) => &MoonStone,
            Item::Stone(EvolutionStone::SunStone) => &SunStone,
            Item::Stone(EvolutionStone::ShinyStone) => &ShinyStone,
            Item::Stone(EvolutionStone::DuskStone) => &DuskStone,
            Item::Stone(EvolutionStone::DawnStone) => &DawnStone,
            Item::Stone(EvolutionStone::IceStone) => &IceStone,
            Item::Incense(Incense::FullIncense) => &FullIncense,
            Item::Incense(Incense::LaxIncense) => &LaxIncense,
            Item::Incense(Incense::LuckIncense) => &LuckIncense,
            Item::Incense(Incense::OddIncense) => &OddIncense,
            Item::Incense(Incense::PureIncense) => &PureIncense,
            Item::Incense(Incense::RockIncense) => &RockIncense,
            Item::Incense(Incense::RoseIncense) => &RoseIncense,
            Item::Incense(Incense::SeaIncense) => &SeaIncense,
            Item::Incense(Incense::WaveIncense) => &WaveIncense,
            Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaTooth) => &DeepSeaTooth,
            Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaScale) => &DeepSeaScale,
            Item::EvolutionHeldItem(EvolutionHeldItem::DragonScale) => &DragonScale,
            Item::EvolutionHeldItem(EvolutionHeldItem::DubiousDisk) => &DubiousDisc,
            Item::EvolutionHeldItem(EvolutionHeldItem::Electirizer) => &Electirizer,
            Item::EvolutionHeldItem(EvolutionHeldItem::KingsRock) => &KingsRock,
            Item::EvolutionHeldItem(EvolutionHeldItem::Magmarizer) => &Magmarizer,
            Item::EvolutionHeldItem(EvolutionHeldItem::MetalCoat) => &MetalCoat,
            Item::EvolutionHeldItem(EvolutionHeldItem::OvalStone) => &OvalStone,
            Item::EvolutionHeldItem(EvolutionHeldItem::PrismScale) => &PrismScale,
            Item::EvolutionHeldItem(EvolutionHeldItem::Protector) => &Protector,
            Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw) => &RazorClaw,
            Item::EvolutionHeldItem(EvolutionHeldItem::RazorFang) => &RazorFang,
            Item::EvolutionHeldItem(EvolutionHeldItem::ReaperCloth) => &ReaperCloth,
            Item::EvolutionHeldItem(EvolutionHeldItem::UpGrade) => &UpGrade,
            Item::Berry(Berry::CheriBerry) => &CheriBerry,
            Item::Berry(Berry::ChestoBerry) => &ChestoBerry,
            Item::Berry(Berry::PechaBerry) => &PechaBerry,
            Item::Berry(Berry::RawstBerry) => &RawstBerry,
            Item::Berry(Berry::AspearBerry) => &AspearBerry,
            Item::Berry(Berry::LeppaBerry) => &LeppaBerry,
            Item::Berry(Berry::OranBerry) => &OranBerry,
            Item::Berry(Berry::PersimBerry) => &PersimBerry,
            Item::Berry(Berry::LumBerry) => &LumBerry,
            Item::Berry(Berry::SitrusBerry) => &SitrusBerry,
            Item::Berry(Berry::FigyBerry) => &FigyBerry,
            Item::Berry(Berry::WikiBerry) => &WikiBerry,
            Item::Berry(Berry::MagoBerry) => &MagoBerry,
            Item::Berry(Berry::AguavBerry) => &AguavBerry,
            Item::Berry(Berry::IapapaBerry) => &IapapaBerry,
            Item::Berry(Berry::RazzBerry) => &RazzBerry,
            Item::Berry(Berry::BlukBerry) => &BlukBerry,
            Item::Berry(Berry::NanabBerry) => &NanabBerry,
            Item::Berry(Berry::WepearBerry) => &WepearBerry,
            Item::Berry(Berry::PinapBerry) => &PinapBerry,
            Item::Berry(Berry::PomegBerry) => &PomegBerry,
            Item::Berry(Berry::KelpsyBerry) => &KelpsyBerry,
            Item::Berry(Berry::QualotBerry) => &QualotBerry,
            Item::Berry(Berry::HondewBerry) => &HondewBerry,
            Item::Berry(Berry::GrepaBerry) => &GrepaBerry,
            Item::Berry(Berry::TamatoBerry) => &TamatoBerry,
            Item::Berry(Berry::CornnBerry) => &CornnBerry,
            Item::Berry(Berry::MagostBerry) => &MagostBerry,
            Item::Berry(Berry::RabutaBerry) => &RabutaBerry,
            Item::Berry(Berry::NomelBerry) => &NomelBerry,
            Item::Berry(Berry::SpelonBerry) => &SpelonBerry,
            Item::Berry(Berry::PamtreBerry) => &PamtreBerry,
            Item::Berry(Berry::WatmelBerry) => &WatmelBerry,
            Item::Berry(Berry::DurinBerry) => &DurinBerry,
            Item::Berry(Berry::BelueBerry) => &BelueBerry,
            Item::Berry(Berry::OccaBerry) => &OccaBerry,
            Item::Berry(Berry::PasshoBerry) => &PasshoBerry,
            Item::Berry(Berry::WacanBerry) => &WacanBerry,
            Item::Berry(Berry::RindoBerry) => &RindoBerry,
            Item::Berry(Berry::YacheBerry) => &YacheBerry,
            Item::Berry(Berry::ChopleBerry) => &ChopleBerry,
            Item::Berry(Berry::KebiaBerry) => &KebiaBerry,
            Item::Berry(Berry::ShucaBerry) => &ShucaBerry,
            Item::Berry(Berry::CobaBerry) => &CobaBerry,
            Item::Berry(Berry::PayapaBerry) => &PayapaBerry,
            Item::Berry(Berry::TangaBerry) => &TangaBerry,
            Item::Berry(Berry::ChartiBerry) => &ChartiBerry,
            Item::Berry(Berry::KasibBerry) => &KasibBerry,
            Item::Berry(Berry::HabanBerry) => &HabanBerry,
            Item::Berry(Berry::ColburBerry) => &ColburBerry,
            Item::Berry(Berry::BabiriBerry) => &BabiriBerry,
            Item::Berry(Berry::ChilanBerry) => &ChilanBerry,
            Item::Berry(Berry::LiechiBerry) => &LiechiBerry,
            Item::Berry(Berry::GanlonBerry) => &GanlonBerry,
            Item::Berry(Berry::SalacBerry) => &SalacBerry,
            Item::Berry(Berry::PetayaBerry) => &PetayaBerry,
            Item::Berry(Berry::ApicotBerry) => &ApicotBerry,
            Item::Berry(Berry::LansatBerry) => &LansatBerry,
            Item::Berry(Berry::StarfBerry) => &StarfBerry,
            Item::Berry(Berry::EnigmaBerry) => &EnigmaBerry,
            Item::Berry(Berry::MicleBerry) => &MicleBerry,
            Item::Berry(Berry::CustapBerry) => &CustapBerry,
            Item::Berry(Berry::JabocaBerry) => &JabocaBerry,
            Item::Berry(Berry::RowapBerry) => &RowapBerry,
            Item::Berry(Berry::RoseliBerry) => &RoseliBerry,
            Item::Berry(Berry::KeeBerry) => &KeeBerry,
            Item::Berry(Berry::MarangaBerry) => &MarangaBerry,
            Item::Mail(Mail::AirMail(_)) => &AirMail,
            Item::Mail(Mail::BloomMail(_)) => &BloomMail,
            Item::Mail(Mail::BrickMail(_)) => &BrickMail,
            Item::Mail(Mail::BubbleMail(_)) => &BubbleMail,
            Item::Mail(Mail::FlameMail(_)) => &FlameMail,
            Item::Mail(Mail::GrassMail(_)) => &GrassMail,
            Item::Mail(Mail::HeartMail(_)) => &HeartMail,
            Item::Mail(Mail::MosaicMail(_)) => &MosaicMail,
            Item::Mail(Mail::SnowMail(_)) => &SnowMail,
            Item::Mail(Mail::SpaceMail(_)) => &SpaceMail,
            Item::Mail(Mail::SteelMail(_)) => &SteelMail,
            Item::Mail(Mail::TunnelMail(_)) => &TunnelMail,
            Item::Potion => &Potion,
            Item::SuperPotion => &SuperPotion,
            Item::HyperPotion => &HyperPotion,
            Item::MaxPotion => &MaxPotion,
            Item::BerryJuice => &BerryJuice,
            Item::EnergyRoot => &EnergyRoot,
            Item::EnergyPowder => &EnergyPowder,
            Item::FreshWater => &FreshWater,
            Item::Lemonade => &Lemonade,
            Item::MoomooMilk => &MoomooMilk,
            Item::RageCandyBar => &RageCandyBar,
            Item::SacredAsh => &SacredAsh,
            Item::ShellBell => &ShellBell,
            Item::SodaPop => &SodaPop,
            Item::SweetHeart => &SweetHeart,
            Item::RevivalHerb => &RevivalHerb,
            Item::Antidote => &Antidote,
            Item::Awakening => &Awakening,
            Item::BurnHeal => &BurnHeal,
            Item::FullHeal => &FullHeal,
            Item::FullRestore => &FullRestore,
            Item::IceHeal => &IceHeal,
            Item::ParalyzeHeal => &ParalyzeHeal,
            Item::Revive => &Revive,
            Item::MaxRevive => &MaxRevive,
            Item::LavaCookie => &LavaCookie,
            Item::MentalHerb => &MentalHerb,
            Item::OldGateau => &OldGateau,
            Item::Casteliacone => &Casteliacone,
            Item::Ether => &Ether,
            Item::MaxEther => &MaxEther,
            Item::Elixir => &Elixir,
            Item::MaxElixir => &MaxElixir,
            Item::PPUp => &PpUp,
            Item::PPMax => &PpMax,
            Item::HPUp => &HpUp,
            Item::Protein => &Protein,
            Item::Iron => &Iron,
            Item::Calcium => &Calcium,
            Item::Zinc => &Zinc,
            Item::Carbos => &Carbos,
            Item::ShockDrive => &ShockDrive,
            Item::BurnDrive => &BurnDrive,
            Item::ChillDrive => &ChillDrive,
            Item::DouseDrive => &DouseDrive,
            Item::PowerAnklet => &PowerAnklet,
            Item::PowerBand => &PowerBand,
            Item::PowerBelt => &PowerBelt,
            Item::PowerBracer => &PowerBracer,
            Item::PowerLens => &PowerLens,
            Item::PowerWeight => &PowerWeight,
            Item::ExpShare => &ExpShare,
            Item::LuckyEgg => &LuckyEgg,
            Item::DracoPlate => &DracoPlate,
            Item::DreadPlate => &DreadPlate,
            Item::EarthPlate => &EarthPlate,
            Item::FistPlate => &FistPlate,
            Item::FlamePlate => &FlamePlate,
            Item::IciclePlate => &IciclePlate,
            Item::InsectPlate => &InsectPlate,
            Item::IronPlate => &IronPlate,
            Item::MeadowPlate => &MeadowPlate,
            Item::MindPlate => &MindPlate,
            Item::PixiePlate => &PixiePlate,
            Item::SkyPlate => &SkyPlate,
            Item::SplashPlate => &SplashPlate,
            Item::SpookyPlate => &SpookyPlate,
            Item::StonePlate => &StonePlate,
            Item::ToxicPlate => &ToxicPlate,
            Item::ZapPlate => &ZapPlate,
            Item::Eviolite => &Eviolite,
            Item::Leek => &Leek,
            Item::LightBall => &LightBall,
            Item::LuckyPunch => &LuckyPunch,
            Item::MetalPowder => &MetalPowder,
            Item::QuickPowder => &QuickPowder,
            Item::SoulDew => &SoulDew,
            Item::ThickClub => &ThickClub,
            Item::BlackBelt => &BlackBelt,
            Item::BlackGlasses => &BlackGlasses,
            Item::Charcoal => &Charcoal,
            Item::DragonFang => &DragonFang,
            Item::HardStone => &HardStone,
            Item::Magnet => &Magnet,
            Item::MetalCoat => &MetalCoat,
            Item::MiracleSeed => &MiracleSeed,
            Item::MysticWater => &MysticWater,
            Item::NeverMeltIce => &NeverMeltIce,
            Item::PoisonBarb => &PoisonBarb,
            Item::SharpBeak => &SharpBeak,
            Item::SilkScarf => &SilkScarf,
            Item::SilverPowder => &SilverPowder,
            Item::SoftSand => &SoftSand,
            Item::SpellTag => &SpellTag,
            Item::TwistedSpoon => &TwistedSpoon,
            Item::AdamantOrb => &AdamantOrb,
            Item::LustrousOrb => &LustrousOrb,
            Item::GriseousOrb => &GriseousOrb,
            Item::BlueScarf => &BlueScarf,
            Item::GreenScarf => &GreenScarf,
            Item::PinkScarf => &PinkScarf,
            Item::RedScarf => &RedScarf,
            Item::YellowScarf => &YellowScarf,
            Item::SootheBell => &SootheBell,
            Item::CleanseTag => &CleanseTag,
            Item::ChoiceBand => &ChoiceBand,
            Item::ChoiceScarf => &ChoiceScarf,
            Item::ChoiceSpecs => &ChoiceSpecs,
            Item::BlackSludge => &BlackSludge,
            Item::LifeOrb => &LifeOrb,
            Item::RockyHelmet => &RockyHelmet,
            Item::StickyBarb => &StickyBarb,
            Item::GripClaw => &GripClaw,
            Item::LightClay => &LightClay,
            Item::TerrainExtender => &TerrainExtender,
            Item::AssaultVest => &AssaultVest,
            Item::FlameOrb => &FlameOrb,
            Item::IronBall => &IronBall,
            Item::LaggingTail => &LaggingTail,
            Item::MachoBrace => &MachoBrace,
            Item::RingTarget => &RingTarget,
            Item::ToxicOrb => &ToxicOrb,
            Item::Leftovers => &Leftovers,
            Item::AbsorbBulb => &AbsorbBulb,
            Item::CellBattery => &CellBattery,
            Item::EjectButton => &EjectButton,
            Item::LuminousMoss => &LuminousMoss,
            Item::Snowball => &Snowball,
            Item::WeaknessPolicy => &WeaknessPolicy,
            Item::WideLens => &WideLens,
            Item::Everstone => &cEverstone,
            Item::BrightPowder => &BrightPowder,
            Item::ZoomLens => &ZoomLens,
            Item::ScopeLens => &ScopeLens,
            Item::ExpertBelt => &ExpertBelt,
            Item::Metronome => &Metronome,
            Item::FocusSash => &FocusSash,
            Item::FloatStone => &FloatStone,
            Item::BindingBand => &BindingBand,
            Item::TM(_) | Item::HM(_) | Item::KeyItem(_) => panic!("No TM, HM, Key Item data filled in"),
            Item::HeatRock => &HeatRock,
            Item::DampRock => &DampRock,
            Item::SmoothRock => &SmoothRock,
            Item::IcyRock => &IcyRock
        }
    }

    pub fn is_mail(&self) -> bool {
        match self {
            Item::Mail(_) => true,
            _ => false
        }
    }

    pub fn is_plate(&self) -> bool {
        use Item::*;
        match self {
            DracoPlate | DreadPlate | EarthPlate | FistPlate | FlamePlate | IciclePlate | InsectPlate | IronPlate |
            MeadowPlate | MindPlate | PixiePlate | SkyPlate | SplashPlate | SpookyPlate | StonePlate | ToxicPlate | ZapPlate => true,
            _ => false
        }
    }

    pub fn is_drive(&self) -> bool {
        use Item::*;
        match self {
            DouseDrive | ShockDrive | BurnDrive | ChillDrive => true,
            _ => false
        }
    }
}

pub static PokeBall: ItemData = ItemData {
    cost: 200,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static GreatBall: ItemData = ItemData {
    cost: 600,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static UltraBall: ItemData = ItemData {
    cost: 800,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static MasterBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static SafariBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static FastBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LevelBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LureBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static HeavyBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LoveBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static FriendBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MoonBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SportBall: ItemData = ItemData {
    cost: 300,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static NetBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static NestBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static RepeatBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static TimerBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static LuxuryBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static PremierBall: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static DiveBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static DuskBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static HealBall: ItemData = ItemData {
    cost: 300,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static QuickBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static CherishBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: false,
};
pub static ParkBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static DreamBall: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::PokeBalls,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static FireStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WaterStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ThunderStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LeafStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MoonStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SunStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ShinyStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static DuskStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static DawnStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static IceStone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static FullIncense: ItemData = ItemData {
    cost: 5000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LaxIncense: ItemData = ItemData {
    cost: 5000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LuckIncense: ItemData = ItemData {
    cost: 11000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static OddIncense: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PureIncense: ItemData = ItemData {
    cost: 6000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static RockIncense: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static RoseIncense: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SeaIncense: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static WaveIncense: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static DeepSeaTooth: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static DeepSeaScale: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static DragonScale: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static DubiousDisc: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static Electirizer: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static KingsRock: ItemData = ItemData {
    cost: 5000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static Magmarizer: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MetalCoat: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static OvalStone: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PrismScale: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static Protector: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RazorClaw: ItemData = ItemData {
    cost: 5000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static RazorFang: ItemData = ItemData {
    cost: 5000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ReaperCloth: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static UpGrade: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static CheriBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChestoBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PechaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RawstBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static AspearBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LeppaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static OranBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PersimBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LumBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SitrusBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static FigyBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WikiBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MagoBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static AguavBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static IapapaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RazzBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BlukBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static NanabBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WepearBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PinapBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PomegBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static KelpsyBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static QualotBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static HondewBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static GrepaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static TamatoBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static CornnBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MagostBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RabutaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static NomelBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SpelonBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PamtreBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WatmelBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static DurinBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BelueBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static OccaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PasshoBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WacanBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RindoBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static YacheBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChopleBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static KebiaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ShucaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static CobaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PayapaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static TangaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChartiBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static KasibBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static HabanBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ColburBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BabiriBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChilanBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LiechiBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static GanlonBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SalacBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PetayaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ApicotBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LansatBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static StarfBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static EnigmaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MicleBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static CustapBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static JabocaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RowapBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RoseliBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static KeeBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MarangaBerry: ItemData = ItemData {
    cost: 20,
    pocket: Pocket::Berries,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static AirMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BloomMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BrickMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BubbleMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static FlameMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static GrassMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static HeartMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static MosaicMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SnowMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SpaceMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SteelMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static TunnelMail: ItemData = ItemData {
    cost: 50,
    pocket: Pocket::Mail,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static Potion: ItemData = ItemData {
    cost: 200,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static SuperPotion: ItemData = ItemData {
    cost: 700,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static HyperPotion: ItemData = ItemData {
    cost: 1500,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static MaxPotion: ItemData = ItemData {
    cost: 2500,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static BerryJuice: ItemData = ItemData {
    cost: 200,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static EnergyRoot: ItemData = ItemData {
    cost: 1200,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static EnergyPowder: ItemData = ItemData {
    cost: 500,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static FreshWater: ItemData = ItemData {
    cost: 200,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Lemonade: ItemData = ItemData {
    cost: 400,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static MoomooMilk: ItemData = ItemData {
    cost: 600,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static RageCandyBar: ItemData = ItemData {
    cost: 350,
    pocket: Pocket::KeyItems,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SacredAsh: ItemData = ItemData {
    cost: 50000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static ShellBell: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SodaPop: ItemData = ItemData {
    cost: 300,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static SweetHeart: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RevivalHerb: ItemData = ItemData {
    cost: 2800,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Antidote: ItemData = ItemData {
    cost: 200,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Awakening: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static BurnHeal: ItemData = ItemData {
    cost: 300,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static FullHeal: ItemData = ItemData {
    cost: 400,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static FullRestore: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static IceHeal: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static ParalyzeHeal: ItemData = ItemData {
    cost: 300,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Revive: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static MaxRevive: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static LavaCookie: ItemData = ItemData {
    cost: 350,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static MentalHerb: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static OldGateau: ItemData = ItemData {
    cost: 350,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Casteliacone: ItemData = ItemData {
    cost: 350,
    pocket: Pocket::Medicine,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static Ether: ItemData = ItemData {
    cost: 1200,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static MaxEther: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Elixir: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static MaxElixir: ItemData = ItemData {
    cost: 4500,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static PpUp: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static PpMax: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static HpUp: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Protein: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Iron: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Calcium: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Zinc: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static Carbos: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Medicine,
    countable: true,
    consumable: true,
    holdable: true,
    battle_usable: true,
    overworld_usable: true,
};
pub static ShockDrive: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BurnDrive: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChillDrive: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static DouseDrive: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PowerAnklet: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PowerBand: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PowerBelt: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PowerBracer: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PowerLens: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PowerWeight: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ExpShare: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LuckyEgg: ItemData = ItemData {
    cost: 10000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static DracoPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static DreadPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static EarthPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static FistPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static FlamePlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static IciclePlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static InsectPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static IronPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static MeadowPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static MindPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PixiePlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SkyPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SplashPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SpookyPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static StonePlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ToxicPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ZapPlate: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static Eviolite: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static Leek: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LightBall: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LuckyPunch: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static MetalPowder: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static QuickPowder: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SoulDew: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ThickClub: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static BlackBelt: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static BlackGlasses: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static Charcoal: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static DragonFang: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static HardStone: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static Magnet: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static MiracleSeed: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static MysticWater: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static NeverMeltIce: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static PoisonBarb: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SharpBeak: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SilkScarf: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SilverPowder: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SoftSand: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static SpellTag: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static TwistedSpoon: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static AdamantOrb: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LustrousOrb: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static GriseousOrb: ItemData = ItemData {
    cost: 0,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static BlueScarf: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static GreenScarf: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static PinkScarf: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static RedScarf: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static YellowScarf: ItemData = ItemData {
    cost: 100,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static SootheBell: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static CleanseTag: ItemData = ItemData {
    cost: 5000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChoiceBand: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChoiceScarf: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static ChoiceSpecs: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static BlackSludge: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LifeOrb: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static RockyHelmet: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static StickyBarb: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static GripClaw: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LightClay: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static TerrainExtender: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static AssaultVest: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static FlameOrb: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static IronBall: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static LaggingTail: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static MachoBrace: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static RingTarget: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static ToxicOrb: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static Leftovers: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static AbsorbBulb: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static CellBattery: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static EjectButton: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static LuminousMoss: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static Snowball: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WeaknessPolicy: ItemData = ItemData {
    cost: 1000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: false,
    battle_usable: false,
    overworld_usable: false,
};
pub static WideLens: ItemData = ItemData {
    cost: 4000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub static cEverstone: ItemData = ItemData {
    cost: 3000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false,
};
pub const ZoomLens: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const ScopeLens: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const BrightPowder: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const ExpertBelt: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const Metronome: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const FocusSash: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const FloatStone: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const BindingBand: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const HeatRock: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const DampRock: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const SmoothRock: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
pub const IcyRock: ItemData = ItemData {
    cost: 2000,
    pocket: Pocket::Items,
    countable: false,
    consumable: false,
    holdable: true,
    battle_usable: false,
    overworld_usable: false
};
//endregion