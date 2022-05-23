#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter};

use crate::attack::Move;
use crate::types::Type;

/// All Pokeballs in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, Serialize, Deserialize)]
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
    fn from(p: Pokeball) -> Self { Item::Ball(p) }
}

/// All Evolution Stones in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
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
/// For convenience, MailDiscriminants is provided, which is all the same Mail
/// types without attached messages.
#[derive(Debug, Clone, Eq, PartialEq, EnumDiscriminants, Serialize, Deserialize)]
pub enum Mail {
    AirMail(Option<String>),
    BloomMail(Option<String>),
    BrickMail(Option<String>),
    BubbleMail(Option<String>),
    FlameMail(Option<String>),
    GrassMail(Option<String>),
    HeartMail(Option<String>),
    MosaicMail(Option<String>),
    SnowMail(Option<String>),
    SpaceMail(Option<String>),
    SteelMail(Option<String>),
    TunnelMail(Option<String>)
}
impl From<Mail> for Item {
    fn from(p: Mail) -> Self { Item::Mail(p) }
}
impl From<MailDiscriminants> for Mail {
    fn from(p: MailDiscriminants) -> Self {
        match p {
            MailDiscriminants::AirMail => Mail::AirMail(None),
            MailDiscriminants::BloomMail => Mail::BloomMail(None),
            MailDiscriminants::BrickMail => Mail::BrickMail(None),
            MailDiscriminants::BubbleMail => Mail::BubbleMail(None),
            MailDiscriminants::FlameMail => Mail::FlameMail(None),
            MailDiscriminants::GrassMail => Mail::GrassMail(None),
            MailDiscriminants::HeartMail => Mail::HeartMail(None),
            MailDiscriminants::MosaicMail => Mail::MosaicMail(None),
            MailDiscriminants::SnowMail => Mail::SnowMail(None),
            MailDiscriminants::SpaceMail => Mail::SpaceMail(None),
            MailDiscriminants::SteelMail => Mail::SteelMail(None),
            MailDiscriminants::TunnelMail => Mail::TunnelMail(None)
        }
    }
}

/// All Key items in the game
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum KeyItem {

}
impl From<KeyItem> for Item {
    fn from(p: KeyItem) -> Self { Item::KeyItem(p) }
}

/// All Items in the game
/// Items from the above lists are wrapped, so they can still be considered items
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Ball(Pokeball),
    Stone(EvolutionStone),
    Everstone,
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
    HeatRock, DampRock, SmoothRock, IcyRock,
    DestinyKnot,
    // Evolution-inducing
    DeepSeaTooth, DeepSeaScale, DragonScale, DubiousDisk, Electirizer, KingsRock,
    Magmarizer, OvalStone, PrismScale, Protector, RazorClaw, RazorFang, ReaperCloth, UpGrade,
    // Incenses
    FullIncense, LaxIncense, LuckIncense, OddIncense,
    PureIncense, RockIncense, RoseIncense, SeaIncense, WaveIncense
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Item {
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