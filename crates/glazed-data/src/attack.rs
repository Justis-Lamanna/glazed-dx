#![allow(non_upper_case_globals)]

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use crate::item::{EvolutionStone, Item};
use crate::types::Type;

/// Represents an Attack a Pokemon can have
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumCountMacro, Serialize, Deserialize, Hash, Eq)]
pub enum Move {
    Pound,
    KarateChop,
    DoubleSlap,
    CometPunch,
    MegaPunch,
    PayDay,
    FirePunch,
    IcePunch,
    ThunderPunch,
    Scratch,
    ViseGrip,
    Guillotine,
    RazorWind,
    SwordsDance,
    Cut,
    Gust,
    WingAttack,
    Whirlwind,
    Fly,
    Bind,
    Slam,
    VineWhip,
    Stomp,
    DoubleKick,
    MegaKick,
    JumpKick,
    RollingKick,
    SandAttack,
    Headbutt,
    HornAttack,
    FuryAttack,
    HornDrill,
    Tackle,
    BodySlam,
    Wrap,
    TakeDown,
    Thrash,
    DoubleEdge,
    TailWhip,
    PoisonSting,
    Twineedle,
    PinMissile,
    Leer,
    Bite,
    Growl,
    Roar,
    Sing,
    Supersonic,
    SonicBoom,
    Disable,
    Acid,
    Ember,
    Flamethrower,
    Mist,
    WaterGun,
    HydroPump,
    Surf,
    IceBeam,
    Blizzard,
    Psybeam,
    BubbleBeam,
    AuroraBeam,
    HyperBeam,
    Peck,
    DrillPeck,
    Submission,
    LowKick,
    Counter,
    SeismicToss,
    Strength,
    Absorb,
    MegaDrain,
    LeechSeed,
    Growth,
    RazorLeaf,
    SolarBeam,
    PoisonPowder,
    StunSpore,
    SleepPowder,
    PetalDance,
    StringShot,
    DragonRage,
    FireSpin,
    ThunderShock,
    Thunderbolt,
    ThunderWave,
    Thunder,
    RockThrow,
    Earthquake,
    Fissure,
    Dig,
    Toxic,
    Confusion,
    Psychic,
    Hypnosis,
    Meditate,
    Agility,
    QuickAttack,
    Rage,
    Teleport,
    NightShade,
    Mimic,
    Screech,
    DoubleTeam,
    Recover,
    Harden,
    Minimize,
    Smokescreen,
    ConfuseRay,
    Withdraw,
    DefenseCurl,
    Barrier,
    LightScreen,
    Haze,
    Reflect,
    FocusEnergy,
    Bide,
    Metronome,
    MirrorMove,
    SelfDestruct,
    EggBomb,
    Lick,
    Smog,
    Sludge,
    BoneClub,
    FireBlast,
    Waterfall,
    Clamp,
    Swift,
    SkullBash,
    SpikeCannon,
    Constrict,
    Amnesia,
    Kinesis,
    SoftBoiled,
    HighJumpKick,
    Glare,
    DreamEater,
    PoisonGas,
    Barrage,
    LeechLife,
    LovelyKiss,
    SkyAttack,
    Transform,
    Bubble,
    DizzyPunch,
    Spore,
    Flash,
    Psywave,
    Splash,
    AcidArmor,
    Crabhammer,
    Explosion,
    FurySwipes,
    Bonemerang,
    Rest,
    RockSlide,
    HyperFang,
    Sharpen,
    Conversion,
    TriAttack,
    SuperFang,
    Slash,
    Substitute,
    // Struggle,
    Sketch,
    TripleKick,
    Thief,
    SpiderWeb,
    MindReader,
    Nightmare,
    FlameWheel,
    Snore,
    Curse,
    Flail,
    Conversion2,
    Aeroblast,
    CottonSpore,
    Reversal,
    Spite,
    PowderSnow,
    Protect,
    MachPunch,
    ScaryFace,
    FeintAttack,
    SweetKiss,
    BellyDrum,
    SludgeBomb,
    MudSlap,
    Octazooka,
    Spikes,
    ZapCannon,
    Foresight,
    DestinyBond,
    PerishSong,
    IcyWind,
    Detect,
    BoneRush,
    LockOn,
    Outrage,
    Sandstorm,
    GigaDrain,
    Endure,
    Charm,
    Rollout,
    FalseSwipe,
    Swagger,
    MilkDrink,
    Spark,
    FuryCutter,
    SteelWing,
    MeanLook,
    Attract,
    SleepTalk,
    HealBell,
    Return,
    Present,
    Frustration,
    Safeguard,
    PainSplit,
    SacredFire,
    Magnitude,
    DynamicPunch,
    Megahorn,
    DragonBreath,
    BatonPass,
    Encore,
    Pursuit,
    RapidSpin,
    SweetScent,
    IronTail,
    MetalClaw,
    VitalThrow,
    MorningSun,
    Synthesis,
    Moonlight,
    HiddenPower,
    CrossChop,
    Twister,
    RainDance,
    SunnyDay,
    Crunch,
    MirrorCoat,
    PsychUp,
    ExtremeSpeed,
    AncientPower,
    ShadowBall,
    FutureSight,
    RockSmash,
    Whirlpool,
    BeatUp,
    FakeOut,
    Uproar,
    Stockpile,
    SpitUp,
    Swallow,
    HeatWave,
    Hail,
    Torment,
    Flatter,
    WillOWisp,
    Memento,
    Facade,
    FocusPunch,
    SmellingSalts,
    FollowMe,
    NaturePower,
    Charge,
    Taunt,
    HelpingHand,
    Trick,
    RolePlay,
    Wish,
    Assist,
    Ingrain,
    Superpower,
    MagicCoat,
    Recycle,
    Revenge,
    BrickBreak,
    Yawn,
    KnockOff,
    Endeavor,
    Eruption,
    SkillSwap,
    Imprison,
    Refresh,
    Grudge,
    Snatch,
    SecretPower,
    Dive,
    ArmThrust,
    Camouflage,
    TailGlow,
    LusterPurge,
    MistBall,
    FeatherDance,
    TeeterDance,
    BlazeKick,
    MudSport,
    IceBall,
    NeedleArm,
    SlackOff,
    HyperVoice,
    PoisonFang,
    CrushClaw,
    BlastBurn,
    HydroCannon,
    MeteorMash,
    Astonish,
    WeatherBall,
    Aromatherapy,
    FakeTears,
    AirCutter,
    Overheat,
    OdorSleuth,
    RockTomb,
    SilverWind,
    MetalSound,
    GrassWhistle,
    Tickle,
    CosmicPower,
    WaterSpout,
    SignalBeam,
    ShadowPunch,
    Extrasensory,
    SkyUppercut,
    SandTomb,
    SheerCold,
    MuddyWater,
    BulletSeed,
    AerialAce,
    IcicleSpear,
    IronDefense,
    Block,
    Howl,
    DragonClaw,
    FrenzyPlant,
    BulkUp,
    Bounce,
    MudShot,
    PoisonTail,
    Covet,
    VoltTackle,
    MagicalLeaf,
    WaterSport,
    CalmMind,
    LeafBlade,
    DragonDance,
    RockBlast,
    ShockWave,
    WaterPulse,
    DoomDesire,
    PsychoBoost,
    Roost,
    Gravity,
    MiracleEye,
    WakeUpSlap,
    HammerArm,
    GyroBall,
    HealingWish,
    Brine,
    NaturalGift,
    Feint,
    Pluck,
    Tailwind,
    Acupressure,
    MetalBurst,
    UTurn,
    CloseCombat,
    Payback,
    Assurance,
    Embargo,
    Fling,
    PsychoShift,
    TrumpCard,
    HealBlock,
    WringOut,
    PowerTrick,
    GastroAcid,
    LuckyChant,
    MeFirst,
    Copycat,
    PowerSwap,
    GuardSwap,
    Punishment,
    LastResort,
    WorrySeed,
    SuckerPunch,
    ToxicSpikes,
    HeartSwap,
    AquaRing,
    MagnetRise,
    FlareBlitz,
    ForcePalm,
    AuraSphere,
    RockPolish,
    PoisonJab,
    DarkPulse,
    NightSlash,
    AquaTail,
    SeedBomb,
    AirSlash,
    XScissor,
    BugBuzz,
    DragonPulse,
    DragonRush,
    PowerGem,
    DrainPunch,
    VacuumWave,
    FocusBlast,
    EnergyBall,
    BraveBird,
    EarthPower,
    Switcheroo,
    GigaImpact,
    NastyPlot,
    BulletPunch,
    Avalanche,
    IceShard,
    ShadowClaw,
    ThunderFang,
    IceFang,
    FireFang,
    ShadowSneak,
    MudBomb,
    PsychoCut,
    ZenHeadbutt,
    MirrorShot,
    FlashCannon,
    RockClimb,
    Defog,
    TrickRoom,
    DracoMeteor,
    Discharge,
    LavaPlume,
    LeafStorm,
    PowerWhip,
    RockWrecker,
    CrossPoison,
    GunkShot,
    IronHead,
    MagnetBomb,
    StoneEdge,
    Captivate,
    StealthRock,
    GrassKnot,
    Chatter,
    Judgment,
    BugBite,
    ChargeBeam,
    WoodHammer,
    AquaJet,
    AttackOrder,
    DefendOrder,
    HealOrder,
    HeadSmash,
    DoubleHit,
    RoarOfTime,
    SpacialRend,
    LunarDance,
    CrushGrip,
    MagmaStorm,
    DarkVoid,
    SeedFlare,
    OminousWind,
    ShadowForce,
    HoneClaws,
    WideGuard,
    GuardSplit,
    PowerSplit,
    WonderRoom,
    Psyshock,
    Venoshock,
    Autotomize,
    RagePowder,
    Telekinesis,
    MagicRoom,
    SmackDown,
    StormThrow,
    FlameBurst,
    SludgeWave,
    QuiverDance,
    HeavySlam,
    Synchronoise,
    ElectroBall,
    Soak,
    FlameCharge,
    Coil,
    LowSweep,
    AcidSpray,
    FoulPlay,
    SimpleBeam,
    Entrainment,
    AfterYou,
    Round,
    EchoedVoice,
    ChipAway,
    ClearSmog,
    StoredPower,
    QuickGuard,
    AllySwitch,
    Scald,
    ShellSmash,
    HealPulse,
    Hex,
    SkyDrop,
    ShiftGear,
    CircleThrow,
    Incinerate,
    Quash,
    Acrobatics,
    ReflectType,
    Retaliate,
    FinalGambit,
    Bestow,
    Inferno,
    WaterPledge,
    FirePledge,
    GrassPledge,
    VoltSwitch,
    StruggleBug,
    Bulldoze,
    FrostBreath,
    DragonTail,
    WorkUp,
    Electroweb,
    WildCharge,
    DrillRun,
    DualChop,
    HeartStamp,
    HornLeech,
    SacredSword,
    RazorShell,
    HeatCrash,
    LeafTornado,
    Steamroller,
    CottonGuard,
    NightDaze,
    Psystrike,
    TailSlap,
    Hurricane,
    HeadCharge,
    GearGrind,
    SearingShot,
    TechnoBlast,
    RelicSong,
    SecretSword,
    Glaciate,
    BoltStrike,
    BlueFlare,
    FieryDance,
    FreezeShock,
    IceBurn,
    Snarl,
    IcicleCrash,
    VCreate,
    FusionFlare,
    FusionBolt
}

/// Represents the Accuracy of a move
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Accuracy {
    AlwaysHits,
    Percentage(u8),
    Variable
}

pub type FractionPlaceholder = (u8, u8);

/// Represents the Power of a move
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Power {
    None,
    Base(u8),
    BaseWithRecoil(u8, FractionPlaceholder),
    BaseWithMercy(u8),
    BaseWithCharge(u8, Option<SemiInvulnerableLocation>),
    BaseWithCrash(u8),
    BaseWithFaint(u8),
    BaseWithDrain(u8),
    BaseWithTurnMultiplier(u8),
    BaseAfterNTurns(u8, u8),
    WeightBased,
    WeightRatioBased,
    MultiHit(MultiHitFlavor),
    MultiTurn(u8, u8),
    OneHitKnockout,
    Exact(u8),
    Percentage(FractionPlaceholder),
    Variable,
    Revenge(FractionPlaceholder, Option<DamageType>)
}
impl Default for Power {
    fn default() -> Self {
        Power::None
    }
}

/// The potential types of Multi Hit
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum MultiHitFlavor {
    Variable(u8),
    Accumulating(u8, u8, u8),
    Fixed(u8, u8),
    BeatUp
}

/// Represents the type of Move, for which attack/defense is used
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
pub enum DamageType {
    Physical,
    Special,
    Status
}

/// Represents the stats that are maintained in battle
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum BattleStat {
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Accuracy,
    Evasion,
    CriticalHitRatio
}

/// Represents a persistent status ailment
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum NonVolatileBattleAilment {
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison(PoisonType)
}

/// Represents a weather condition in battle
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Weather {
    HarshSun,
    Rain,
    Sandstorm,
    Hail,
    Fog
}

/// Represents a target, or targets of an attack
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Target {
    User,
    Ally,
    UserAndAlly,
    UserOrAlly,
    Opponent,
    Opponents,
    AllyOrOpponent,
    RandomOpponent,
    LastAttacker(Option<DamageType>),
    Any,
    AllExceptUser,
    All,
    Implicit
}

/// Represents if a stat change affects the user, or the target
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum StatChangeTarget {
    User,
    Target
}

/// Possible effects of an attack
#[derive(Debug, Clone, Deserialize)]
pub enum Effect {
    StatChange(BattleStat, i8, u8, StatChangeTarget),
    StatReset,
    NonVolatileStatus(NonVolatileBattleAilment, u8, StatChangeTarget),
    TriAttack,
    Confuse(u8, StatChangeTarget),
    Infatuate(u8),
    Heal(u8),
    WeatherHeal {
        no_weather: (u8, u8),
        sun: (u8, u8),
        other_weather: (u8, u8)
    },
    Flinch(u8),
    ChangeWeather(Weather),
    DispelWeather,
    ForceSwitch(StatChangeTarget),
    DropCoins,
    Bind,
    Thrash,
    Disable,
    Mist,
    Recharge,
    Leech,
    Rage,
    Mimic,
    Minimize,
    Curl,
    Screen(ScreenType),
    Bide,
    Transform,
    Rest,
    Conversion,
    Conversion2,
    Substitute,
    Sketch,
    StealItem,
    Trap,
    LockOn,
    Nightmare,
    Curse,
    Spite,
    Protect,
    BellyDrum,
    Foresight,
    DestinyBond,
    PerishSong,
    Safeguard,
    PainSplit,
    BatonPass,
    Encore,
    EntryHazard(EntryHazardType),
    ClearHazards,
    Predicated(EffectPredicate, Box<Effect>, Box<Effect>),
    Custom
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum EffectPredicate {
    Sunny
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PoisonType {
    Poison,
    BadlyPoisoned
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ScreenType {
    LightScreen,
    Reflect
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum EntryHazardType {
    Spikes,
    ToxicSpikes,
    StealthRock
}

/// Represents data for a specific attack
#[derive(Debug, Deserialize)]
pub struct MoveData {
    pub id: Move,
    pub pp: u8,
    #[serde(default)]
    pub priority: i8,
    #[serde(default)]
    pub power: Power,
    pub accuracy: Accuracy,
    #[serde(rename="type")]
    pub _type: Type,
    pub damage_type: DamageType,
    pub target: Target,
    #[serde(default)]
    pub crit_rate: u8,
    #[serde(default)]
    pub effects: Vec<Effect>
}
impl MoveData {
    pub fn is_no_power_move(&self) -> bool {
        if let Power::None = self.power {
            true
        } else {
            false
        }
    }

    pub fn is_charging_move(&self) -> bool {
        if let Power::BaseWithCharge(_, _) = self.power {
            true
        } else {
            false
        }
    }

    pub fn is_rage(&self) -> bool {
        self.effects.iter()
            .any(|e| {
                if let Effect::Rage = e {
                    true
                } else {
                    false
                }
            })
    }

    /// Check whether this attack is affected by Kings Rock or Razor Fang
    /// All damage-dealing attacks which don't already cause flinching are affected.
    pub fn is_affected_by_flinch_items(&self) -> bool {
        if let Power::None = self.power {
            false
        } else {
            self.effects.iter().all(|e| {
                if let Effect::Flinch(_) = e {
                    false
                } else {
                    true
                }
            })
        }
    }

    /// Check whether this attack is affected by Serene Grace
    /// All damage-dealing attacks with secondary effects are affected.
    pub fn is_affected_by_serene_grace(&self) -> bool {
        if let Power::None = self.power {
            false
        } else {
            self.effects.len() > 0
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum SemiInvulnerableLocation {
    Underground,
    Underwater,
    InAir,
    Vanished
}

impl Item {
    pub fn fling_power(&self) -> u8 {
        match self {
            Item::Berry(_) | Item::RedScarf | Item::YellowScarf | Item::PinkScarf | Item::GreenScarf | Item::BlueScarf |
            Item::ChoiceBand | Item::ChoiceScarf | Item::ChoiceSpecs | Item::LaggingTail | Item::Leftovers | Item::MentalHerb |
            Item::MetalPowder | Item::QuickPowder | Item::ReaperCloth | Item::RingTarget |
            Item::SilkScarf | Item::SilverPowder | Item::SoftSand | Item::SootheBell | Item::WideLens | Item::BrightPowder | Item::ZoomLens => 10,
            Item::Eviolite | Item::LuckyPunch => 40,
            Item::DubiousDisk | Item::SharpBeak => 50,
            Item::AdamantOrb | Item::GriseousOrb | Item::Leek | Item::LustrousOrb | Item::MachoBrace | Item::RockyHelmet => 60,
            Item::ShockDrive | Item::BurnDrive | Item::ChillDrive | Item::DouseDrive |
            Item::PowerAnklet | Item::PowerBand | Item::PowerBelt | Item::PowerBracer | Item::PowerLens | Item::PowerWeight |
            Item::DragonFang | Item::PoisonBarb => 70,
            Item::AssaultVest | Item::Stone(EvolutionStone::DawnStone) | Item::Stone(EvolutionStone::DuskStone) | Item::Electirizer |
            Item::Magmarizer | Item::OvalStone | Item::Protector |
            Item::RazorClaw | Item::Stone(EvolutionStone::ShinyStone) | Item::StickyBarb | Item::WeaknessPolicy => 80,
            Item::DeepSeaTooth | Item::GripClaw | Item::ThickClub |
            Item::DracoPlate | Item::DreadPlate | Item::EarthPlate | Item::FistPlate |
            Item::FlamePlate | Item::IciclePlate | Item::InsectPlate | Item::IronPlate |
            Item::MeadowPlate | Item::MindPlate | Item::PixiePlate | Item::SkyPlate |
            Item::SplashPlate | Item::SpookyPlate | Item::StonePlate | Item::ToxicPlate | Item::ZapPlate => 90,
            Item::HardStone => 100,
            Item::IronBall => 130,
            Item::TM(tm) => 10,
            Item::HM(hm) => 10,
            _ => 30
        }
    }
}