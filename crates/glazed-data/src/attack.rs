#![allow(non_upper_case_globals)]

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumIter, EnumCount as EnumCountMacro};

use crate::item::{EvolutionHeldItem, EvolutionStone, Item};
use crate::types::Type;

/// Represents an Attack a Pokemon can have
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumCountMacro)]
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
impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        let idx = rng.gen_range(0usize..Move::COUNT);
        Move::iter().nth(idx).unwrap()
    }
}
impl Move {
    /// Retrieve a random Move for Metronome. Will not return certain blacklisted moves.
    pub fn metronome() -> Move {
        loop {
            let attack = rand::thread_rng().gen::<Move>();
            match attack {
                Move::AfterYou | Move::Assist | Move::Bestow | Move::Chatter | Move::Copycat |
                Move::Counter | Move::Covet | Move::DestinyBond | Move::Detect | Move::Endure |
                Move::Feint | Move::FocusPunch | Move::FollowMe | Move::FreezeShock | Move::HelpingHand |
                Move::IceBurn | Move::MeFirst | Move::Mimic | Move::MirrorCoat | Move::MirrorMove |
                Move::NaturePower | Move::Protect | Move::Sketch | Move::SleepTalk | Move::Snatch |
                Move::Snore | Move::Switcheroo | Move::Thief | Move::Transform |
                Move::Trick | Move::VCreate | Move::WideGuard => {},
                m => return m
            }
        }
    }

    /// If true, this Move can be used while frozen, and will remove the Frozen condition.
    pub fn can_thaw_user(&self) -> bool {
        match self {
            Move::FlameWheel | Move::FlareBlitz | Move::FusionFlare | Move::SacredFire | Move::Scald => true,
            _ => false
        }
    }

    /// If true, this Move can only be used when the user is asleep.
    pub fn can_only_be_used_while_sleeping(&self) -> bool {
        match self {
            Move::SleepTalk | Move::Snore => true,
            _ => false
        }
    }

    /// If true, this Move can only be used when the *target* is asleep.
    pub fn can_only_be_used_on_sleeping_target(&self) -> bool {
        match self {
            Move::DreamEater | Move::Nightmare => true,
            _ => false
        }
    }

    /// If true, this attack can be learned via Mimic
    pub fn can_be_mimicked(&self) -> bool {
        match self {
            Move::Sketch | Move::Transform | Move::Metronome => false,
            _ => true
        }
    }

    /// If true, this attack can be learned via Sketch
    pub fn can_be_sketched(&self) -> bool {
        match self {
            Move::MirrorMove | Move::SleepTalk | Move::Sketch => false,
            _ => true
        }
    }

    /// If true, this attack can be called by Sleep Talk
    pub fn can_be_sleep_talked(&self) -> bool {
        match self {
            Move::Assist | Move::Bide | Move::Bounce | Move::Copycat | Move::Dig |
            Move::Dive | Move::FreezeShock | Move::Fly | Move::FocusPunch | Move::IceBurn |
            Move::MeFirst | Move::Metronome | Move::MirrorMove | Move::Mimic | Move::RazorWind |
            Move::ShadowForce | Move::Sketch | Move::SkullBash | Move::SkyAttack | Move::SkyDrop |
            Move::SolarBeam | Move::Uproar | Move::SleepTalk => false,
            _ => true
        }
    }

    /// If true, this attack can be the subject of an encore
    pub fn can_be_encored(&self) -> bool {
        match self {
            Move::Transform | Move::Mimic | Move::Sketch | Move::MirrorMove | Move::SleepTalk | Move::Encore => false,
            _ => true
        }
    }

    /// If true, this Move does double damage on a target that has used Minimize
    pub fn double_damage_on_minimized_target(&self) -> bool {
        match self {
            Move::BodySlam | Move::Stomp | Move::DragonRush | Move::ShadowForce | Move::Steamroller | Move::HeatCrash | Move::HeavySlam => true,
            _ => false
        }
    }

    /// If true, this move increments the protection counter. If false, this move resets the protection counter to 0.
    pub fn is_protection_move(&self) -> bool {
        match self {
            Move::Protect | Move::Detect | Move::Endure | Move::QuickGuard | Move::WideGuard => true,
            _ => false
        }
    }

    /// If true, this attack can bypass Protect/Detect
    pub fn bypasses_protect(&self) -> bool {
        match self.data().target {
            Target::User | Target::UserAndAlly | Target::All => true,
            _ => match self {
                Move::Acupressure | Move::Conversion2 | Move::Curse | Move::DoomDesire | Move::Feint |
                Move::FutureSight | Move::PerishSong | Move::PsychUp | Move::RolePlay | Move::ShadowForce |
                Move::Sketch | Move::Transform | Move::Spikes | Move::ToxicSpikes | Move::StealthRock => true,
                _ => false
            }
        }
    }
}

/// Represents the Accuracy of a move
#[derive(Debug)]
pub enum Accuracy {
    AlwaysHits,
    Percentage(u8),
    Variable
}

pub type FractionPlaceholder = (u8, u8);

/// Represents the Power of a move
#[derive(Debug)]
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

/// The potential types of Multi Hit
#[derive(Debug)]
pub enum MultiHitFlavor {
    Variable(u8),
    Accumulating(u8, u8, u8),
    Fixed(u8, u8),
    BeatUp
}

/// Represents the type of Move, for which attack/defense is used
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DamageType {
    Physical,
    Special,
    Status
}

/// Represents the stats that are maintained in battle
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub enum NonVolatileBattleAilment {
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison(PoisonType)
}

/// Represents a weather condition in battle
#[derive(Debug, Copy, Clone)]
pub enum Weather {
    HarshSun,
    Rain,
    Sandstorm,
    Hail,
    Fog
}

/// Represents a target, or targets of an attack
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug)]
pub enum StatChangeTarget {
    User,
    Target
}

/// Possible effects of an attack
#[derive(Debug)]
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
    Predicated(EffectPredicate, &'static Effect, &'static Effect),
    Custom
}

#[derive(Debug)]
pub enum EffectPredicate {
    Sunny
}

#[derive(Debug, Copy, Clone)]
pub enum PoisonType {
    Poison,
    BadlyPoisoned
}

#[derive(Debug)]
pub enum ScreenType {
    LightScreen,
    Reflect
}

#[derive(Debug)]
pub enum EntryHazardType {
    Spikes,
    ToxicSpikes,
    StealthRock
}

/// Represents data for a specific attack
#[derive(Debug)]
pub struct MoveData {
    pub pp: u8,
    pub priority: i8,
    pub power: Power,
    pub accuracy: Accuracy,
    pub _type: Type,
    pub damage_type: DamageType,
    pub target: Target,
    pub crit_rate: Option<u8>,
    pub effects: &'static[Effect]
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

#[derive(Debug, Copy, Clone)]
pub enum SemiInvulnerableLocation {
    Underground,
    Underwater,
    InAir,
    Vanished
}

impl Item {
    pub fn fling_power(&self) -> u8 {
        match self {
            Item::Berry(_) | Item::Incense(_) | Item::RedScarf | Item::YellowScarf | Item::PinkScarf | Item::GreenScarf | Item::BlueScarf |
            Item::ChoiceBand | Item::ChoiceScarf | Item::ChoiceSpecs | Item::LaggingTail | Item::Leftovers | Item::MentalHerb |
            Item::MetalPowder | Item::QuickPowder | Item::EvolutionHeldItem(EvolutionHeldItem::ReaperCloth) | Item::RingTarget |
            Item::SilkScarf | Item::SilverPowder | Item::SoftSand | Item::SootheBell | Item::WideLens | Item::BrightPowder | Item::ZoomLens => 10,
            Item::Eviolite | Item::LuckyPunch => 40,
            Item::EvolutionHeldItem(EvolutionHeldItem::DubiousDisk) | Item::SharpBeak => 50,
            Item::AdamantOrb | Item::GriseousOrb | Item::Leek | Item::LustrousOrb | Item::MachoBrace | Item::RockyHelmet => 60,
            Item::ShockDrive | Item::BurnDrive | Item::ChillDrive | Item::DouseDrive |
            Item::PowerAnklet | Item::PowerBand | Item::PowerBelt | Item::PowerBracer | Item::PowerLens | Item::PowerWeight |
            Item::DragonFang | Item::PoisonBarb => 70,
            Item::AssaultVest | Item::Stone(EvolutionStone::DawnStone) | Item::Stone(EvolutionStone::DuskStone) | Item::EvolutionHeldItem(EvolutionHeldItem::Electirizer) |
            Item::EvolutionHeldItem(EvolutionHeldItem::Magmarizer) | Item::EvolutionHeldItem(EvolutionHeldItem::OvalStone) | Item::EvolutionHeldItem(EvolutionHeldItem::Protector) |
            Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw) | Item::Stone(EvolutionStone::ShinyStone) | Item::StickyBarb | Item::WeaknessPolicy => 80,
            Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaTooth) | Item::GripClaw | Item::ThickClub |
            Item::DracoPlate | Item::DreadPlate | Item::EarthPlate | Item::FistPlate |
            Item::FlamePlate | Item::IciclePlate | Item::InsectPlate | Item::IronPlate |
            Item::MeadowPlate | Item::MindPlate | Item::PixiePlate | Item::SkyPlate |
            Item::SplashPlate | Item::SpookyPlate | Item::StonePlate | Item::ToxicPlate | Item::ZapPlate => 90,
            Item::HardStone => 100,
            Item::IronBall => 130,
            Item::TM(tm) => tm.get_move().get_power().unwrap_or(10),
            Item::HM(hm) => hm.get_move().get_power().unwrap_or(10),
            _ => 30
        }
    }
}

//region MoveConstants
impl Move {
    pub fn data(&self) -> &'static MoveData {
        match self {
            Move::Pound => &Pound,
            Move::KarateChop => &KarateChop,
            Move::DoubleSlap => &DoubleSlap,
            Move::CometPunch => &CometPunch,
            Move::MegaPunch => &MegaPunch,
            Move::PayDay => &PayDay,
            Move::FirePunch => &FirePunch,
            Move::IcePunch => &IcePunch,
            Move::ThunderPunch => &ThunderPunch,
            Move::Scratch => &Scratch,
            Move::ViseGrip => &ViseGrip,
            Move::Guillotine => &Guillotine,
            Move::RazorWind => &RazorWind,
            Move::SwordsDance => &SwordsDance,
            Move::Cut => &Cut,
            Move::Gust => &Gust,
            Move::WingAttack => &WingAttack,
            Move::Whirlwind => &Whirlwind,
            Move::Fly => &Fly,
            Move::Bind => &Bind,
            Move::Slam => &Slam,
            Move::VineWhip => &VineWhip,
            Move::Stomp => &Stomp,
            Move::DoubleKick => &DoubleKick,
            Move::MegaKick => &MegaKick,
            Move::JumpKick => &JumpKick,
            Move::RollingKick => &RollingKick,
            Move::SandAttack => &SandAttack,
            Move::Headbutt => &Headbutt,
            Move::HornAttack => &HornAttack,
            Move::FuryAttack => &FuryAttack,
            Move::HornDrill => &HornDrill,
            Move::Tackle => &Tackle,
            Move::BodySlam => &BodySlam,
            Move::Wrap => &Wrap,
            Move::TakeDown => &TakeDown,
            Move::Thrash => &Thrash,
            Move::DoubleEdge => &DoubleEdge,
            Move::TailWhip => &TailWhip,
            Move::PoisonSting => &PoisonSting,
            Move::Twineedle => &Twineedle,
            Move::PinMissile => &PinMissile,
            Move::Leer => &Leer,
            Move::Bite => &Bite,
            Move::Growl => &Growl,
            Move::Roar => &Roar,
            Move::Sing => &Sing,
            Move::Supersonic => &Supersonic,
            Move::SonicBoom => &SonicBoom,
            Move::Disable => &Disable,
            Move::Acid => &Acid,
            Move::Ember => &Ember,
            Move::Flamethrower => &Flamethrower,
            Move::Mist => &Mist,
            Move::WaterGun => &WaterGun,
            Move::HydroPump => &HydroPump,
            Move::Surf => &Surf,
            Move::IceBeam => &IceBeam,
            Move::Blizzard => &Blizzard,
            Move::Psybeam => &Psybeam,
            Move::BubbleBeam => &BubbleBeam,
            Move::AuroraBeam => &AuroraBeam,
            Move::HyperBeam => &HyperBeam,
            Move::Peck => &Peck,
            Move::DrillPeck => &DrillPeck,
            Move::Submission => &Submission,
            Move::LowKick => &LowKick,
            Move::Counter => &Counter,
            Move::SeismicToss => &SeismicToss,
            Move::Strength => &Strength,
            Move::Absorb => &Absorb,
            Move::MegaDrain => &MegaDrain,
            Move::LeechSeed => &LeechSeed,
            Move::Growth => &Growth,
            Move::RazorLeaf => &RazorLeaf,
            Move::SolarBeam => &SolarBeam,
            Move::PoisonPowder => &PoisonPowder,
            Move::StunSpore => &StunSpore,
            Move::SleepPowder => &SleepPowder,
            Move::PetalDance => &PetalDance,
            Move::StringShot => &StringShot,
            Move::DragonRage => &DragonRage,
            Move::FireSpin => &FireSpin,
            Move::ThunderShock => &ThunderShock,
            Move::Thunderbolt => &Thunderbolt,
            Move::ThunderWave => &ThunderWave,
            Move::Thunder => &Thunder,
            Move::RockThrow => &RockThrow,
            Move::Earthquake => &Earthquake,
            Move::Fissure => &Fissure,
            Move::Dig => &Dig,
            Move::Toxic => &Toxic,
            Move::Confusion => &Confusion,
            Move::Psychic => &Psychic,
            Move::Hypnosis => &Hypnosis,
            Move::Meditate => &Meditate,
            Move::Agility => &Agility,
            Move::QuickAttack => &QuickAttack,
            Move::Rage => &Rage,
            Move::Teleport => &Teleport,
            Move::NightShade => &NightShade,
            Move::Mimic => &Mimic,
            Move::Screech => &Screech,
            Move::DoubleTeam => &DoubleTeam,
            Move::Recover => &Recover,
            Move::Harden => &Harden,
            Move::Minimize => &Minimize,
            Move::Smokescreen => &Smokescreen,
            Move::ConfuseRay => &ConfuseRay,
            Move::Withdraw => &Withdraw,
            Move::DefenseCurl => &DefenseCurl,
            Move::Barrier => &Barrier,
            Move::LightScreen => &LightScreen,
            Move::Haze => &Haze,
            Move::Reflect => &Reflect,
            Move::FocusEnergy => &FocusEnergy,
            Move::Bide => &Bide,
            Move::Metronome => &Metronome,
            Move::MirrorMove => &MirrorMove,
            Move::SelfDestruct => &SelfDestruct,
            Move::EggBomb => &EggBomb,
            Move::Lick => &Lick,
            Move::Smog => &Smog,
            Move::Sludge => &Sludge,
            Move::BoneClub => &BoneClub,
            Move::FireBlast => &FireBlast,
            Move::Waterfall => &Waterfall,
            Move::Clamp => &Clamp,
            Move::Swift => &Swift,
            Move::SkullBash => &SkullBash,
            Move::SpikeCannon => &SpikeCannon,
            Move::Constrict => &Constrict,
            Move::Amnesia => &Amnesia,
            Move::Kinesis => &Kinesis,
            Move::SoftBoiled => &SoftBoiled,
            Move::HighJumpKick => &HighJumpKick,
            Move::Glare => &Glare,
            Move::DreamEater => &DreamEater,
            Move::PoisonGas => &PoisonGas,
            Move::Barrage => &Barrage,
            Move::LeechLife => &LeechLife,
            Move::LovelyKiss => &LovelyKiss,
            Move::SkyAttack => &SkyAttack,
            Move::Transform => &Transform,
            Move::Bubble => &Bubble,
            Move::DizzyPunch => &DizzyPunch,
            Move::Spore => &Spore,
            Move::Flash => &Flash,
            Move::Psywave => &Psywave,
            Move::Splash => &Splash,
            Move::AcidArmor => &AcidArmor,
            Move::Crabhammer => &Crabhammer,
            Move::Explosion => &Explosion,
            Move::FurySwipes => &FurySwipes,
            Move::Bonemerang => &Bonemerang,
            Move::Rest => &Rest,
            Move::RockSlide => &RockSlide,
            Move::HyperFang => &HyperFang,
            Move::Sharpen => &Sharpen,
            Move::Conversion => &Conversion,
            Move::TriAttack => &TriAttack,
            Move::SuperFang => &SuperFang,
            Move::Slash => &Slash,
            Move::Substitute => &Substitute,
            // Move::Struggle => &Struggle,
            Move::Sketch => &Sketch,
            Move::TripleKick => &TripleKick,
            Move::Thief => &Thief,
            Move::SpiderWeb => &SpiderWeb,
            Move::MindReader => &MindReader,
            Move::Nightmare => &Nightmare,
            Move::FlameWheel => &FlameWheel,
            Move::Snore => &Snore,
            Move::Curse => &Curse,
            Move::Flail => &Flail,
            Move::Conversion2 => &Conversion2,
            Move::Aeroblast => &Aeroblast,
            Move::CottonSpore => &CottonSpore,
            Move::Reversal => &Reversal,
            Move::Spite => &Spite,
            Move::PowderSnow => &PowderSnow,
            Move::Protect => &Protect,
            Move::MachPunch => &MachPunch,
            Move::ScaryFace => &ScaryFace,
            Move::FeintAttack => &FeintAttack,
            Move::SweetKiss => &SweetKiss,
            Move::BellyDrum => &BellyDrum,
            Move::SludgeBomb => &SludgeBomb,
            Move::MudSlap => &MudSlap,
            Move::Octazooka => &Octazooka,
            Move::Spikes => &Spikes,
            Move::ZapCannon => &ZapCannon,
            Move::Foresight => &Foresight,
            Move::DestinyBond => &DestinyBond,
            Move::PerishSong => &PerishSong,
            Move::IcyWind => &IcyWind,
            Move::Detect => &Detect,
            Move::BoneRush => &BoneRush,
            Move::LockOn => &LockOn,
            Move::Outrage => &Outrage,
            Move::Sandstorm => &Sandstorm,
            Move::GigaDrain => &GigaDrain,
            Move::Endure => &Endure,
            Move::Charm => &Charm,
            Move::Rollout => &Rollout,
            Move::FalseSwipe => &FalseSwipe,
            Move::Swagger => &Swagger,
            Move::MilkDrink => &MilkDrink,
            Move::Spark => &Spark,
            Move::FuryCutter => &FuryCutter,
            Move::SteelWing => &SteelWing,
            Move::MeanLook => &MeanLook,
            Move::Attract => &Attract,
            Move::SleepTalk => &SleepTalk,
            Move::HealBell => &HealBell,
            Move::Return => &Return,
            Move::Present => &Present,
            Move::Frustration => &Frustration,
            Move::Safeguard => &Safeguard,
            Move::PainSplit => &PainSplit,
            Move::SacredFire => &SacredFire,
            Move::Magnitude => &Magnitude,
            Move::DynamicPunch => &DynamicPunch,
            Move::Megahorn => &Megahorn,
            Move::DragonBreath => &DragonBreath,
            Move::BatonPass => &BatonPass,
            Move::Encore => &Encore,
            Move::Pursuit => &Pursuit,
            Move::RapidSpin => &RapidSpin,
            Move::SweetScent => &SweetScent,
            Move::IronTail => &IronTail,
            Move::MetalClaw => &MetalClaw,
            Move::VitalThrow => &VitalThrow,
            Move::MorningSun => &MorningSun,
            Move::Synthesis => &Synthesis,
            Move::Moonlight => &Moonlight,
            Move::HiddenPower => &HiddenPower,
            Move::CrossChop => &CrossChop,
            Move::Twister => &Twister,
            Move::RainDance => &RainDance,
            Move::SunnyDay => &SunnyDay,
            Move::Crunch => &Crunch,
            Move::MirrorCoat => &MirrorCoat,
            Move::PsychUp => &PsychUp,
            Move::ExtremeSpeed => &ExtremeSpeed,
            Move::AncientPower => &AncientPower,
            Move::ShadowBall => &ShadowBall,
            Move::FutureSight => &FutureSight,
            Move::RockSmash => &RockSmash,
            Move::Whirlpool => &Whirlpool,
            Move::BeatUp => &BeatUp,
            Move::FakeOut => &FakeOut,
            Move::Uproar => &Uproar,
            Move::Stockpile => &Stockpile,
            Move::SpitUp => &SpitUp,
            Move::Swallow => &Swallow,
            Move::HeatWave => &HeatWave,
            Move::Hail => &Hail,
            Move::Torment => &Torment,
            Move::Flatter => &Flatter,
            Move::WillOWisp => &WillOWisp,
            Move::Memento => &Memento,
            Move::Facade => &Facade,
            Move::FocusPunch => &FocusPunch,
            Move::SmellingSalts => &SmellingSalts,
            Move::FollowMe => &FollowMe,
            Move::NaturePower => &NaturePower,
            Move::Charge => &Charge,
            Move::Taunt => &Taunt,
            Move::HelpingHand => &HelpingHand,
            Move::Trick => &Trick,
            Move::RolePlay => &RolePlay,
            Move::Wish => &Wish,
            Move::Assist => &Assist,
            Move::Ingrain => &Ingrain,
            Move::Superpower => &Superpower,
            Move::MagicCoat => &MagicCoat,
            Move::Recycle => &Recycle,
            Move::Revenge => &Revenge,
            Move::BrickBreak => &BrickBreak,
            Move::Yawn => &Yawn,
            Move::KnockOff => &KnockOff,
            Move::Endeavor => &Endeavor,
            Move::Eruption => &Eruption,
            Move::SkillSwap => &SkillSwap,
            Move::Imprison => &Imprison,
            Move::Refresh => &Refresh,
            Move::Grudge => &Grudge,
            Move::Snatch => &Snatch,
            Move::SecretPower => &SecretPower,
            Move::Dive => &Dive,
            Move::ArmThrust => &ArmThrust,
            Move::Camouflage => &Camouflage,
            Move::TailGlow => &TailGlow,
            Move::LusterPurge => &LusterPurge,
            Move::MistBall => &MistBall,
            Move::FeatherDance => &FeatherDance,
            Move::TeeterDance => &TeeterDance,
            Move::BlazeKick => &BlazeKick,
            Move::MudSport => &MudSport,
            Move::IceBall => &IceBall,
            Move::NeedleArm => &NeedleArm,
            Move::SlackOff => &SlackOff,
            Move::HyperVoice => &HyperVoice,
            Move::PoisonFang => &PoisonFang,
            Move::CrushClaw => &CrushClaw,
            Move::BlastBurn => &BlastBurn,
            Move::HydroCannon => &HydroCannon,
            Move::MeteorMash => &MeteorMash,
            Move::Astonish => &Astonish,
            Move::WeatherBall => &WeatherBall,
            Move::Aromatherapy => &Aromatherapy,
            Move::FakeTears => &FakeTears,
            Move::AirCutter => &AirCutter,
            Move::Overheat => &Overheat,
            Move::OdorSleuth => &OdorSleuth,
            Move::RockTomb => &RockTomb,
            Move::SilverWind => &SilverWind,
            Move::MetalSound => &MetalSound,
            Move::GrassWhistle => &GrassWhistle,
            Move::Tickle => &Tickle,
            Move::CosmicPower => &CosmicPower,
            Move::WaterSpout => &WaterSpout,
            Move::SignalBeam => &SignalBeam,
            Move::ShadowPunch => &ShadowPunch,
            Move::Extrasensory => &Extrasensory,
            Move::SkyUppercut => &SkyUppercut,
            Move::SandTomb => &SandTomb,
            Move::SheerCold => &SheerCold,
            Move::MuddyWater => &MuddyWater,
            Move::BulletSeed => &BulletSeed,
            Move::AerialAce => &AerialAce,
            Move::IcicleSpear => &IcicleSpear,
            Move::IronDefense => &IronDefense,
            Move::Block => &Block,
            Move::Howl => &Howl,
            Move::DragonClaw => &DragonClaw,
            Move::FrenzyPlant => &FrenzyPlant,
            Move::BulkUp => &BulkUp,
            Move::Bounce => &Bounce,
            Move::MudShot => &MudShot,
            Move::PoisonTail => &PoisonTail,
            Move::Covet => &Covet,
            Move::VoltTackle => &VoltTackle,
            Move::MagicalLeaf => &MagicalLeaf,
            Move::WaterSport => &WaterSport,
            Move::CalmMind => &CalmMind,
            Move::LeafBlade => &LeafBlade,
            Move::DragonDance => &DragonDance,
            Move::RockBlast => &RockBlast,
            Move::ShockWave => &ShockWave,
            Move::WaterPulse => &WaterPulse,
            Move::DoomDesire => &DoomDesire,
            Move::PsychoBoost => &PsychoBoost,
            Move::Roost => &Roost,
            Move::Gravity => &Gravity,
            Move::MiracleEye => &MiracleEye,
            Move::WakeUpSlap => &WakeUpSlap,
            Move::HammerArm => &HammerArm,
            Move::GyroBall => &GyroBall,
            Move::HealingWish => &HealingWish,
            Move::Brine => &Brine,
            Move::NaturalGift => &NaturalGift,
            Move::Feint => &Feint,
            Move::Pluck => &Pluck,
            Move::Tailwind => &Tailwind,
            Move::Acupressure => &Acupressure,
            Move::MetalBurst => &MetalBurst,
            Move::UTurn => &UTurn,
            Move::CloseCombat => &CloseCombat,
            Move::Payback => &Payback,
            Move::Assurance => &Assurance,
            Move::Embargo => &Embargo,
            Move::Fling => &Fling,
            Move::PsychoShift => &PsychoShift,
            Move::TrumpCard => &TrumpCard,
            Move::HealBlock => &HealBlock,
            Move::WringOut => &WringOut,
            Move::PowerTrick => &PowerTrick,
            Move::GastroAcid => &GastroAcid,
            Move::LuckyChant => &LuckyChant,
            Move::MeFirst => &MeFirst,
            Move::Copycat => &Copycat,
            Move::PowerSwap => &PowerSwap,
            Move::GuardSwap => &GuardSwap,
            Move::Punishment => &Punishment,
            Move::LastResort => &LastResort,
            Move::WorrySeed => &WorrySeed,
            Move::SuckerPunch => &SuckerPunch,
            Move::ToxicSpikes => &ToxicSpikes,
            Move::HeartSwap => &HeartSwap,
            Move::AquaRing => &AquaRing,
            Move::MagnetRise => &MagnetRise,
            Move::FlareBlitz => &FlareBlitz,
            Move::ForcePalm => &ForcePalm,
            Move::AuraSphere => &AuraSphere,
            Move::RockPolish => &RockPolish,
            Move::PoisonJab => &PoisonJab,
            Move::DarkPulse => &DarkPulse,
            Move::NightSlash => &NightSlash,
            Move::AquaTail => &AquaTail,
            Move::SeedBomb => &SeedBomb,
            Move::AirSlash => &AirSlash,
            Move::XScissor => &XScissor,
            Move::BugBuzz => &BugBuzz,
            Move::DragonPulse => &DragonPulse,
            Move::DragonRush => &DragonRush,
            Move::PowerGem => &PowerGem,
            Move::DrainPunch => &DrainPunch,
            Move::VacuumWave => &VacuumWave,
            Move::FocusBlast => &FocusBlast,
            Move::EnergyBall => &EnergyBall,
            Move::BraveBird => &BraveBird,
            Move::EarthPower => &EarthPower,
            Move::Switcheroo => &Switcheroo,
            Move::GigaImpact => &GigaImpact,
            Move::NastyPlot => &NastyPlot,
            Move::BulletPunch => &BulletPunch,
            Move::Avalanche => &Avalanche,
            Move::IceShard => &IceShard,
            Move::ShadowClaw => &ShadowClaw,
            Move::ThunderFang => &ThunderFang,
            Move::IceFang => &IceFang,
            Move::FireFang => &FireFang,
            Move::ShadowSneak => &ShadowSneak,
            Move::MudBomb => &MudBomb,
            Move::PsychoCut => &PsychoCut,
            Move::ZenHeadbutt => &ZenHeadbutt,
            Move::MirrorShot => &MirrorShot,
            Move::FlashCannon => &FlashCannon,
            Move::RockClimb => &RockClimb,
            Move::Defog => &Defog,
            Move::TrickRoom => &TrickRoom,
            Move::DracoMeteor => &DracoMeteor,
            Move::Discharge => &Discharge,
            Move::LavaPlume => &LavaPlume,
            Move::LeafStorm => &LeafStorm,
            Move::PowerWhip => &PowerWhip,
            Move::RockWrecker => &RockWrecker,
            Move::CrossPoison => &CrossPoison,
            Move::GunkShot => &GunkShot,
            Move::IronHead => &IronHead,
            Move::MagnetBomb => &MagnetBomb,
            Move::StoneEdge => &StoneEdge,
            Move::Captivate => &Captivate,
            Move::StealthRock => &StealthRock,
            Move::GrassKnot => &GrassKnot,
            Move::Chatter => &Chatter,
            Move::Judgment => &Judgment,
            Move::BugBite => &BugBite,
            Move::ChargeBeam => &ChargeBeam,
            Move::WoodHammer => &WoodHammer,
            Move::AquaJet => &AquaJet,
            Move::AttackOrder => &AttackOrder,
            Move::DefendOrder => &DefendOrder,
            Move::HealOrder => &HealOrder,
            Move::HeadSmash => &HeadSmash,
            Move::DoubleHit => &DoubleHit,
            Move::RoarOfTime => &RoarOfTime,
            Move::SpacialRend => &SpacialRend,
            Move::LunarDance => &LunarDance,
            Move::CrushGrip => &CrushGrip,
            Move::MagmaStorm => &MagmaStorm,
            Move::DarkVoid => &DarkVoid,
            Move::SeedFlare => &SeedFlare,
            Move::OminousWind => &OminousWind,
            Move::ShadowForce => &ShadowForce,
            Move::HoneClaws => &HoneClaws,
            Move::WideGuard => &WideGuard,
            Move::GuardSplit => &GuardSplit,
            Move::PowerSplit => &PowerSplit,
            Move::WonderRoom => &WonderRoom,
            Move::Psyshock => &Psyshock,
            Move::Venoshock => &Venoshock,
            Move::Autotomize => &Autotomize,
            Move::RagePowder => &RagePowder,
            Move::Telekinesis => &Telekinesis,
            Move::MagicRoom => &MagicRoom,
            Move::SmackDown => &SmackDown,
            Move::StormThrow => &StormThrow,
            Move::FlameBurst => &FlameBurst,
            Move::SludgeWave => &SludgeWave,
            Move::QuiverDance => &QuiverDance,
            Move::HeavySlam => &HeavySlam,
            Move::Synchronoise => &Synchronoise,
            Move::ElectroBall => &ElectroBall,
            Move::Soak => &Soak,
            Move::FlameCharge => &FlameCharge,
            Move::Coil => &Coil,
            Move::LowSweep => &LowSweep,
            Move::AcidSpray => &AcidSpray,
            Move::FoulPlay => &FoulPlay,
            Move::SimpleBeam => &SimpleBeam,
            Move::Entrainment => &Entrainment,
            Move::AfterYou => &AfterYou,
            Move::Round => &Round,
            Move::EchoedVoice => &EchoedVoice,
            Move::ChipAway => &ChipAway,
            Move::ClearSmog => &ClearSmog,
            Move::StoredPower => &StoredPower,
            Move::QuickGuard => &QuickGuard,
            Move::AllySwitch => &AllySwitch,
            Move::Scald => &Scald,
            Move::ShellSmash => &ShellSmash,
            Move::HealPulse => &HealPulse,
            Move::Hex => &Hex,
            Move::SkyDrop => &SkyDrop,
            Move::ShiftGear => &ShiftGear,
            Move::CircleThrow => &CircleThrow,
            Move::Incinerate => &Incinerate,
            Move::Quash => &Quash,
            Move::Acrobatics => &Acrobatics,
            Move::ReflectType => &ReflectType,
            Move::Retaliate => &Retaliate,
            Move::FinalGambit => &FinalGambit,
            Move::Bestow => &Bestow,
            Move::Inferno => &Inferno,
            Move::WaterPledge => &WaterPledge,
            Move::FirePledge => &FirePledge,
            Move::GrassPledge => &GrassPledge,
            Move::VoltSwitch => &VoltSwitch,
            Move::StruggleBug => &StruggleBug,
            Move::Bulldoze => &Bulldoze,
            Move::FrostBreath => &FrostBreath,
            Move::DragonTail => &DragonTail,
            Move::WorkUp => &WorkUp,
            Move::Electroweb => &Electroweb,
            Move::WildCharge => &WildCharge,
            Move::DrillRun => &DrillRun,
            Move::DualChop => &DualChop,
            Move::HeartStamp => &HeartStamp,
            Move::HornLeech => &HornLeech,
            Move::SacredSword => &SacredSword,
            Move::RazorShell => &RazorShell,
            Move::HeatCrash => &HeatCrash,
            Move::LeafTornado => &LeafTornado,
            Move::Steamroller => &Steamroller,
            Move::CottonGuard => &CottonGuard,
            Move::NightDaze => &NightDaze,
            Move::Psystrike => &Psystrike,
            Move::TailSlap => &TailSlap,
            Move::Hurricane => &Hurricane,
            Move::HeadCharge => &HeadCharge,
            Move::GearGrind => &GearGrind,
            Move::SearingShot => &SearingShot,
            Move::TechnoBlast => &TechnoBlast,
            Move::RelicSong => &RelicSong,
            Move::SecretSword => &SecretSword,
            Move::Glaciate => &Glaciate,
            Move::BoltStrike => &BoltStrike,
            Move::BlueFlare => &BlueFlare,
            Move::FieryDance => &FieryDance,
            Move::FreezeShock => &FreezeShock,
            Move::IceBurn => &IceBurn,
            Move::Snarl => &Snarl,
            Move::IcicleCrash => &IcicleCrash,
            Move::VCreate => &VCreate,
            Move::FusionFlare => &FusionFlare,
            Move::FusionBolt => &FusionBolt,
        }
    }

    pub fn is_sound_based(&self) -> bool {
        match self {
            Move::BugBuzz | Move::Chatter | Move::EchoedVoice | Move::GrassWhistle | Move::Growl |
                Move::HealBell | Move::Howl | Move::HyperVoice | Move::MetalSound | Move::PerishSong |
                Move::RelicSong | Move::Roar | Move::Round | Move::Screech | Move::Sing | Move::Snarl |
                Move::Snore | Move::Supersonic | Move::Uproar => true,
            _ => false
        }
    }

    pub fn is_contact(&self) -> bool {
        let data = self.data();
        match data.damage_type {
            DamageType::Physical => match self {
                Move::AttackOrder | Move::Barrage | Move::BeatUp | Move::BoneRush | Move::Bonemerang |
                Move::Bulldoze | Move::BulletSeed | Move::Earthquake | Move::EggBomb | Move::Explosion |
                Move::Feint | Move::Fissure | Move::Fling | Move::FreezeShock | Move::FusionBolt |
                Move::GunkShot | Move::IceShard | Move::IcicleCrash | Move::IcicleSpear | Move::MagnetBomb |
                Move::Magnitude | Move::MetalBurst | Move::NaturalGift | Move::PayDay | Move::PinMissile |
                Move::PoisonSting | Move::Present | Move::PsychoCut | Move::RazorLeaf | Move::RockBlast |
                Move::RockSlide | Move::RockThrow | Move::RockTomb | Move::RockWrecker | Move::SacredFire |
                Move::SandTomb | Move::SecretPower | Move::SeedBomb | Move::SelfDestruct | Move::SkyAttack |
                Move::SmackDown | Move::SpikeCannon | Move::StoneEdge | Move::Twineedle => false,
                _ => true
            },
            DamageType::Special => match self {
                Move::PetalDance | Move::TrumpCard | Move::WringOut | Move::GrassKnot => true,
                _ => false
            },
            DamageType::Status => false
        }
    }

    pub fn is_powder(&self) -> bool {
        match self {
            Move::CottonSpore | Move::PoisonPowder | Move::RagePowder | Move::SleepPowder | Move::Spore | Move::StunSpore => true,
            _ => false
        }
    }

    pub fn is_trapping(&self) -> bool {
        match self {
            Move::Block | Move::Ingrain | Move::MeanLook | Move::SpiderWeb => true,
            _ => false
        }
    }

    pub fn bypasses_substitute(&self) -> bool {
        self.is_sound_based()
    }

    pub fn get_power(&self) -> Option<u8> {
        match self.data().power {
            Power::Base(a) => Some(a),
            _ => None
        }
    }
}
impl Into<&'static MoveData> for Move {
    fn into(self) -> &'static MoveData {
        self.data()
    }
}

pub static Pound: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
    crit_rate: None,
    effects: &[],
};
pub static KarateChop: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: Some(1),
	effects: &[],
};
pub static DoubleSlap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(15)),
	crit_rate: None,
	effects: &[],
};
pub static CometPunch: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(18)),
	crit_rate: None,
	effects: &[],
};
pub static MegaPunch: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static PayDay: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::DropCoins],
};
pub static FirePunch: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static IcePunch: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Freeze, 10, StatChangeTarget::Target)],
};
pub static ThunderPunch: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 10, StatChangeTarget::Target)],
};
pub static Scratch: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static ViseGrip: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(55),
	crit_rate: None,
	effects: &[],
};
pub static Guillotine: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::OneHitKnockout,
    crit_rate: None,
    accuracy: Accuracy::Variable,
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static RazorWind: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::BaseWithCharge(80, None),
	crit_rate: Some(1),
	effects: &[],
};
pub static SwordsDance: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 2, 100, StatChangeTarget::User)],
};
pub static Cut: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[],
};
pub static Gust: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static WingAttack: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static Whirlwind: MoveData = MoveData {
    pp: 20,
    priority: -6,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::ForceSwitch(StatChangeTarget::Target)],
};
pub static Fly: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(90, Some(SemiInvulnerableLocation::InAir)),
	crit_rate: None,
	effects: &[],
};
pub static Bind: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(15),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static Slam: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static VineWhip: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(45),
	crit_rate: None,
	effects: &[],
};
pub static Stomp: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static DoubleKick: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Fixed(2, 30)),
	crit_rate: None,
	effects: &[],
};
pub static MegaKick: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[],
};
pub static JumpKick: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCrash(100),
	crit_rate: None,
	effects: &[],
};
pub static RollingKick: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static SandAttack: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 100, StatChangeTarget::Target)],
};
pub static Headbutt: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static HornAttack: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[],
};
pub static FuryAttack: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(15)),
	crit_rate: None,
	effects: &[],
};
pub static HornDrill: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::OneHitKnockout,
    crit_rate: None,
    accuracy: Accuracy::Percentage(30),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Tackle: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static BodySlam: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static Wrap: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(15),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static TakeDown: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(90, (1u8, 4u8)),
	crit_rate: None,
	effects: &[],
};
pub static Thrash: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::RandomOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::Thrash],
};
pub static DoubleEdge: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(120, (1u8, 3u8)),
	crit_rate: None,
	effects: &[],
};
pub static TailWhip: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::Defense, -1, 100, StatChangeTarget::Target)],
};
pub static PoisonSting: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(15),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 30, StatChangeTarget::Target)],
};
pub static Twineedle: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Fixed(2, 25)),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 20, StatChangeTarget::Target)],
};
pub static PinMissile: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(25)),
	crit_rate: None,
	effects: &[],
};
pub static Leer: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::Defense, -1, 100, StatChangeTarget::Target)],
};
pub static Bite: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static Growl: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::Attack, -1, 100, StatChangeTarget::Target)],
};
pub static Roar: MoveData = MoveData {
    pp: 20,
    priority: -6,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::ForceSwitch(StatChangeTarget::Target)],
};
pub static Sing: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(55),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static Supersonic: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(55),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Confuse(0, StatChangeTarget::Target)],
};
pub static SonicBoom: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::Exact(20),
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Disable: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Disable],
};
pub static Acid: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static Ember: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static Flamethrower: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static Mist: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ice,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[Effect::Mist],
};
pub static WaterGun: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static HydroPump: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(110),
	crit_rate: None,
	effects: &[],
};
pub static Surf: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllExceptUser,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[],
};
pub static IceBeam: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Freeze, 10, StatChangeTarget::Target)],
};
pub static Blizzard: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(70),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(110),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Freeze, 10, StatChangeTarget::Target)],
};
pub static Psybeam: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Confuse(10, StatChangeTarget::Target)],
};
pub static BubbleBeam: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 10, StatChangeTarget::Target)],
};
pub static AuroraBeam: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, -1, 10, StatChangeTarget::Target)],
};
pub static HyperBeam: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[Effect::Recharge],
};
pub static Peck: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(35),
	crit_rate: None,
	effects: &[],
};
pub static DrillPeck: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static Submission: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(80, (1u8, 4u8)),
	crit_rate: None,
	effects: &[],
};
pub static LowKick: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::WeightBased,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Counter: MoveData = MoveData {
    pp: 20,
    priority: -5,
    power: Power::Revenge((2u8, 1u8), Some(DamageType::Physical)),
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::Implicit,
    effects: &[],
};
pub static SeismicToss: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Strength: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static Absorb: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(20),
	crit_rate: None,
	effects: &[],
};
pub static MegaDrain: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(40),
	crit_rate: None,
	effects: &[],
};
pub static LeechSeed: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Leech],
};
pub static Growth: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[
        Effect::Predicated(EffectPredicate::Sunny,
                           &Effect::StatChange(BattleStat::Attack, 2, 100, StatChangeTarget::User),
                           &Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User)),
        Effect::Predicated(EffectPredicate::Sunny,
                           &Effect::StatChange(BattleStat::SpecialAttack, 2, 100, StatChangeTarget::User),
                           &Effect::StatChange(BattleStat::SpecialAttack, 1, 100, StatChangeTarget::User)),
    ],
};
pub static RazorLeaf: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::Opponents,
    power: Power::Base(55),
	crit_rate: Some(1),
	effects: &[],
};
pub static SolarBeam: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(120, None),
	crit_rate: None,
	effects: &[],
};
pub static PoisonPowder: MoveData = MoveData {
    pp: 35,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 0, StatChangeTarget::Target)],
};
pub static StunSpore: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 0, StatChangeTarget::Target)],
};
pub static SleepPowder: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static PetalDance: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::RandomOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::Thrash],
};
pub static StringShot: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::Speed, -2, 100, StatChangeTarget::Target)],
};
pub static DragonRage: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::Exact(40),
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static FireSpin: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(35),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static ThunderShock: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 10, StatChangeTarget::Target)],
};
pub static Thunderbolt: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 10, StatChangeTarget::Target)],
};
pub static ThunderWave: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Electric,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 0, StatChangeTarget::Target)],
};
pub static Thunder: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(70),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(110),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static RockThrow: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[],
};
pub static Earthquake: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllExceptUser,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[],
};
pub static Fissure: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::OneHitKnockout,
    crit_rate: None,
    accuracy: Accuracy::Percentage(30),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Dig: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(80, Some(SemiInvulnerableLocation::Underground)),
	crit_rate: None,
	effects: &[],
};
pub static Toxic: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::BadlyPoisoned), 0, StatChangeTarget::Target)],
};
pub static Confusion: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::Confuse(10, StatChangeTarget::Target)],
};
pub static Psychic: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static Hypnosis: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(60),
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static Meditate: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User)],
};
pub static Agility: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Speed, 2, 100, StatChangeTarget::User)],
};
pub static QuickAttack: MoveData = MoveData {
    pp: 30,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static Rage: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(20),
	crit_rate: None,
	effects: &[Effect::Rage],
};
pub static Teleport: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::ForceSwitch(StatChangeTarget::User)],
};
pub static NightShade: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Mimic: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Mimic],
};
pub static Screech: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Defense, -2, 100, StatChangeTarget::Target)],
};
pub static DoubleTeam: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Evasion, 1, 100, StatChangeTarget::User)],
};
pub static Recover: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(50)],
};
pub static Harden: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User)],
};
pub static Minimize: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Minimize, Effect::StatChange(BattleStat::Evasion, 2, 100, StatChangeTarget::User)],
};
pub static Smokescreen: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 100, StatChangeTarget::Target)],
};
pub static ConfuseRay: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Confuse(0, StatChangeTarget::Target)],
};
pub static Withdraw: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Water,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User)],
};
pub static DefenseCurl: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User), Effect::Curl],
};
pub static Barrier: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 2, 100, StatChangeTarget::User)],
};
pub static LightScreen: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[Effect::Screen(ScreenType::LightScreen)],
};
pub static Haze: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ice,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatReset],
};
pub static Reflect: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[Effect::Screen(ScreenType::Reflect)],
};
pub static FocusEnergy: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::CriticalHitRatio, 2, 0, StatChangeTarget::User)],
};
pub static Bide: MoveData = MoveData {
    pp: 10,
    priority: 1,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::User,
    effects: &[Effect::Bide],
};
pub static Metronome: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static MirrorMove: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Flying,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static SelfDestruct: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllExceptUser,
    power: Power::BaseWithFaint(200),
	crit_rate: None,
	effects: &[],
};
pub static EggBomb: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[],
};
pub static Lick: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(30),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static Smog: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(70),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(30),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 40, StatChangeTarget::Target)],
};
pub static Sludge: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 30, StatChangeTarget::Target)],
};
pub static BoneClub: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Flinch(10)],
};
pub static FireBlast: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(110),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static Waterfall: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::Flinch(20)],
};
pub static Clamp: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(35),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static Swift: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static SkullBash: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(130, None),
	crit_rate: None,
	effects: &[],
};
pub static SpikeCannon: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(20)),
	crit_rate: None,
	effects: &[],
};
pub static Constrict: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(10),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 10, StatChangeTarget::Target)],
};
pub static Amnesia: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::SpecialDefense, 2, 100, StatChangeTarget::User)],
};
pub static Kinesis: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 100, StatChangeTarget::Target)],
};
pub static SoftBoiled: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(50)],
};
pub static HighJumpKick: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCrash(130),
	crit_rate: None,
	effects: &[],
};
pub static Glare: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 0, StatChangeTarget::Target)],
};
pub static DreamEater: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(100),
	crit_rate: None,
	effects: &[],
};
pub static PoisonGas: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 0, StatChangeTarget::Target)],
};
pub static Barrage: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(15)),
	crit_rate: None,
	effects: &[],
};
pub static LeechLife: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(80),
	crit_rate: None,
	effects: &[],
};
pub static LovelyKiss: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static SkyAttack: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(140, None),
	crit_rate: Some(1),
	effects: &[Effect::Flinch(30)],
};
pub static Transform: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Transform],
};
pub static Bubble: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 10, StatChangeTarget::Target)],
};
pub static DizzyPunch: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::Confuse(20, StatChangeTarget::Target)],
};
pub static Spore: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static Flash: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 100, StatChangeTarget::Target)],
};
pub static Psywave: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Splash: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static AcidArmor: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 2, 100, StatChangeTarget::User)],
};
pub static Crabhammer: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: Some(1),
	effects: &[],
};
pub static Explosion: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllExceptUser,
    power: Power::BaseWithFaint(250),
	crit_rate: None,
	effects: &[],
};
pub static FurySwipes: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(18)),
	crit_rate: None,
	effects: &[],
};
pub static Bonemerang: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Fixed(2, 50)),
	crit_rate: None,
	effects: &[],
};
pub static Rest: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Rest],
};
pub static RockSlide: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::Opponents,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static HyperFang: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::Flinch(10)],
};
pub static Sharpen: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User)],
};
pub static Conversion: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Conversion],
};
pub static TriAttack: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::TriAttack],
};
pub static SuperFang: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::Percentage((1u8, 2u8)),
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Slash: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: Some(1),
	effects: &[],
};
pub static Substitute: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Substitute],
};
// pub static Struggle: MoveData = MoveData {
//     pp: 1,
//     priority: 0,
//     accuracy: Accuracy::AlwaysHits,
//     _type: Type::Normal,
//     contest_type: ContestType::Cool,
//     damage_type: DamageType::Physical,
//     target: Target::RandomOpponent,
//     power: Power::BaseWithRecoil(50, (1u8, 4u8)),
// 	crit_rate: None,
// 	effects: &[],
// };
pub static Sketch: MoveData = MoveData {
    pp: 1,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Sketch],
};
pub static TripleKick: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Accumulating(10, 20, 30)),
	crit_rate: None,
	effects: &[],
};
pub static Thief: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::StealItem],
};
pub static SpiderWeb: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Trap],
};
pub static MindReader: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::LockOn],
};
pub static Nightmare: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Nightmare],
};
pub static FlameWheel: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static Snore: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static Curse: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ghost,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Curse],
};
pub static Flail: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Conversion2: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Conversion2],
};
pub static Aeroblast: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Flying,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: Some(1),
	effects: &[],
};
pub static CottonSpore: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::Speed, -2, 100, StatChangeTarget::Target)],
};
pub static Reversal: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Spite: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Spite],
};
pub static PowderSnow: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Freeze, 10, StatChangeTarget::Target)],
};
pub static Protect: MoveData = MoveData {
    pp: 10,
    priority: 4,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Variable,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Protect],
};
pub static MachPunch: MoveData = MoveData {
    pp: 30,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static ScaryFace: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Speed, -2, 100, StatChangeTarget::Target)],
};
pub static FeintAttack: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static SweetKiss: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Fairy,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Confuse(0, StatChangeTarget::Target)],
};
pub static BellyDrum: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::BellyDrum],
};
pub static SludgeBomb: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 30, StatChangeTarget::Target)],
};
pub static MudSlap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(20),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 100, StatChangeTarget::Target)],
};
pub static Octazooka: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 50, StatChangeTarget::Target)],
};
pub static Spikes: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ground,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::EntryHazard(EntryHazardType::Spikes)],
};
pub static ZapCannon: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(50),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 100, StatChangeTarget::Target)],
};
pub static Foresight: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Foresight],
};
pub static DestinyBond: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ghost,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::DestinyBond],
};
pub static PerishSong: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::All,
    effects: &[Effect::PerishSong],
};
pub static IcyWind: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(55),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static Detect: MoveData = MoveData {
    pp: 5,
    priority: 4,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fighting,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Protect],
};
pub static BoneRush: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(25)),
	crit_rate: None,
	effects: &[],
};
pub static LockOn: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::LockOn],
};
pub static Outrage: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dragon,damage_type: DamageType::Physical,
    target: Target::RandomOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::Thrash],
};
pub static Sandstorm: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Rock,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::ChangeWeather(Weather::Sandstorm)],
};
pub static GigaDrain: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(75),
	crit_rate: None,
	effects: &[],
};
pub static Endure: MoveData = MoveData {
    pp: 10,
    priority: 4,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Protect],
};
pub static Charm: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fairy,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Attack, -2, 100, StatChangeTarget::Target)],
};
pub static Rollout: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiTurn(30, 5),
	crit_rate: None,
	effects: &[],
};
pub static FalseSwipe: MoveData = MoveData {
    pp: 40,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithMercy(40),
	crit_rate: None,
	effects: &[],
};
pub static Swagger: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Attack, 2, 100, StatChangeTarget::Target), Effect::Confuse(0, StatChangeTarget::Target)],
};
pub static MilkDrink: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(50)],
};
pub static Spark: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static FuryCutter: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithTurnMultiplier(40),
	crit_rate: None,
	effects: &[],
};
pub static SteelWing: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, 1, 10, StatChangeTarget::User)],
};
pub static MeanLook: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Trap],
};
pub static Attract: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Infatuate(0)],
};
pub static SleepTalk: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static HealBell: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[],
};
pub static Return: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Present: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Frustration: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Safeguard: MoveData = MoveData {
    pp: 25,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[Effect::Safeguard],
};
pub static PainSplit: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::PainSplit],
};
pub static SacredFire: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 50, StatChangeTarget::Target)],
};
pub static Magnitude: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllExceptUser,
    effects: &[],
};
pub static DynamicPunch: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(50),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::Confuse(100, StatChangeTarget::Target)],
};
pub static Megahorn: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[],
};
pub static DragonBreath: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static BatonPass: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::BatonPass],
};
pub static Encore: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Encore],
};
pub static Pursuit: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static RapidSpin: MoveData = MoveData {
    pp: 40,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::ClearHazards],
};
pub static SweetScent: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::Evasion, -2, 100, StatChangeTarget::Target)],
};
pub static IronTail: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 30, StatChangeTarget::Target)],
};
pub static MetalClaw: MoveData = MoveData {
    pp: 35,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, 1, 10, StatChangeTarget::User)],
};
pub static VitalThrow: MoveData = MoveData {
    pp: 10,
    priority: -1,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static MorningSun: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::WeatherHeal{
        no_weather: (1, 2),
        sun: (2, 3),
        other_weather: (1,4)
    }],
};
pub static Synthesis: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::WeatherHeal{
        no_weather: (1, 2),
        sun: (2, 3),
        other_weather: (1,4)
    }],
};
pub static Moonlight: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fairy,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::WeatherHeal{
        no_weather: (1, 2),
        sun: (2, 3),
        other_weather: (1,4)
    }],
};
pub static HiddenPower: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Variable,
	crit_rate: None,
	effects: &[],
};
pub static CrossChop: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: Some(1),
	effects: &[],
};
pub static Twister: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::Flinch(20)],
};
pub static RainDance: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Water,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::ChangeWeather(Weather::Rain)],
};
pub static SunnyDay: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fire,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::ChangeWeather(Weather::HarshSun)],
};
pub static Crunch: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 20, StatChangeTarget::Target)],
};
pub static MirrorCoat: MoveData = MoveData {
    pp: 20,
    priority: -5,
    power: Power::Revenge((2u8, 1u8), Some(DamageType::Special)),
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::Implicit,
    effects: &[],
};
pub static PsychUp: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static ExtremeSpeed: MoveData = MoveData {
    pp: 5,
    priority: 2,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static AncientPower: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Rock,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::Defense, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialAttack, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, 1, 10, StatChangeTarget::User)],
};
pub static ShadowBall: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 20, StatChangeTarget::Target)],
};
pub static FutureSight: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseAfterNTurns(120, 3),
	crit_rate: None,
	effects: &[],
};
pub static RockSmash: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 50, StatChangeTarget::Target)],
};
pub static Whirlpool: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(35),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static BeatUp: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::MultiHit(MultiHitFlavor::BeatUp),
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static FakeOut: MoveData = MoveData {
    pp: 10,
    priority: 3,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::Flinch(100)],
};
pub static Uproar: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::RandomOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[],
};
pub static Stockpile: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static SpitUp: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Swallow: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(25)],
};
pub static HeatWave: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(95),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static Hail: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ice,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::ChangeWeather(Weather::Hail)],
};
pub static Torment: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Custom],
};
pub static Flatter: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::SpecialAttack, 1, 100, StatChangeTarget::Target), Effect::Confuse(0, StatChangeTarget::Target)],
};
pub static WillOWisp: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Fire,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 0, StatChangeTarget::Target)],
};
pub static Memento: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Facade: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static FocusPunch: MoveData = MoveData {
    pp: 20,
    priority: -3,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static SmellingSalts: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static FollowMe: MoveData = MoveData {
    pp: 20,
    priority: 2,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static NaturePower: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Charge: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Electric,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::SpecialDefense, 1, 100, StatChangeTarget::User)],
};
pub static Taunt: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static HelpingHand: MoveData = MoveData {
    pp: 20,
    priority: 5,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Ally,
    effects: &[],
};
pub static Trick: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static RolePlay: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Wish: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Assist: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Ingrain: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Custom],
};
pub static Superpower: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, -1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Defense, -1, 100, StatChangeTarget::User)],
};
pub static MagicCoat: MoveData = MoveData {
    pp: 15,
    priority: 4,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Recycle: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Revenge: MoveData = MoveData {
    pp: 10,
    priority: -4,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static BrickBreak: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[],
};
pub static Yawn: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Custom],
};
pub static KnockOff: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[],
};
pub static Endeavor: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Eruption: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static SkillSwap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Imprison: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Refresh: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Grudge: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ghost,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Snatch: MoveData = MoveData {
    pp: 10,
    priority: 4,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static SecretPower: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static Dive: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(80, Some(SemiInvulnerableLocation::Underwater)),
	crit_rate: None,
	effects: &[],
};
pub static ArmThrust: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(15)),
	crit_rate: None,
	effects: &[],
};
pub static Camouflage: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static TailGlow: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::SpecialAttack, 3, 100, StatChangeTarget::User)],
};
pub static LusterPurge: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 50, StatChangeTarget::Target)],
};
pub static MistBall: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -1, 50, StatChangeTarget::Target)],
};
pub static FeatherDance: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Attack, -2, 100, StatChangeTarget::Target)],
};
pub static TeeterDance: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllExceptUser,
    effects: &[Effect::Confuse(0, StatChangeTarget::Target)],
};
pub static BlazeKick: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: Some(1),
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static MudSport: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ground,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static IceBall: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(30),
	crit_rate: None,
	effects: &[],
};
pub static NeedleArm: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static SlackOff: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(50)],
};
pub static HyperVoice: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[],
};
pub static PoisonFang: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 50, StatChangeTarget::Target)],
};
pub static CrushClaw: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 50, StatChangeTarget::Target)],
};
pub static BlastBurn: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static HydroCannon: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static MeteorMash: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, 1, 20, StatChangeTarget::User)],
};
pub static Astonish: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(30),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static WeatherBall: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[],
};
pub static Aromatherapy: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[],
};
pub static FakeTears: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::SpecialDefense, -2, 100, StatChangeTarget::Target)],
};
pub static AirCutter: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Flying,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(60),
	crit_rate: Some(1),
	effects: &[],
};
pub static Overheat: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(130),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -2, 100, StatChangeTarget::User)],
};
pub static OdorSleuth: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Custom],
};
pub static RockTomb: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static SilverWind: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::Defense, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialAttack, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, 1, 10, StatChangeTarget::User)],
};
pub static MetalSound: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Steel,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::SpecialDefense, -2, 100, StatChangeTarget::Target)],
};
pub static GrassWhistle: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(55),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static Tickle: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::StatChange(BattleStat::Attack, -1, 100, StatChangeTarget::Target), Effect::StatChange(BattleStat::Defense, -1, 100, StatChangeTarget::Target)],
};
pub static CosmicPower: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 100, StatChangeTarget::User)],
};
pub static WaterSpout: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static SignalBeam: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::Confuse(10, StatChangeTarget::Target)],
};
pub static ShadowPunch: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Ghost,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static Extrasensory: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::Flinch(10)],
};
pub static SkyUppercut: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: None,
	effects: &[],
};
pub static SandTomb: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(35),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static SheerCold: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::OneHitKnockout,
    crit_rate: None,
    accuracy: Accuracy::Percentage(30),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static MuddyWater: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 30, StatChangeTarget::Target)],
};
pub static BulletSeed: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(25)),
	crit_rate: None,
	effects: &[],
};
pub static AerialAce: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static IcicleSpear: MoveData = MoveData {
    pp: 30,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(25)),
	crit_rate: None,
	effects: &[],
};
pub static IronDefense: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Steel,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 2, 100, StatChangeTarget::User)],
};
pub static Block: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Howl: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User)],
};
pub static DragonClaw: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dragon,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static FrenzyPlant: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static BulkUp: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fighting,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User)],
};
pub static Bounce: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(85, Some(SemiInvulnerableLocation::InAir)),
	crit_rate: None,
    effects: &[],
};
pub static MudShot: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Ground,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(55),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static PoisonTail: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: Some(1),
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 10, StatChangeTarget::Target)],
};
pub static Covet: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static VoltTackle: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(120, (1u8, 3u8)),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 10, StatChangeTarget::Target)],
};
pub static MagicalLeaf: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static WaterSport: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Water,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static CalmMind: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::SpecialAttack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 100, StatChangeTarget::User)],
};
pub static LeafBlade: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: Some(1),
	effects: &[],
};
pub static DragonDance: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Dragon,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, 1, 100, StatChangeTarget::User)],
};
pub static RockBlast: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(25)),
	crit_rate: None,
	effects: &[],
};
pub static ShockWave: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static WaterPulse: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::Confuse(20, StatChangeTarget::Target)],
};
pub static DoomDesire: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseAfterNTurns(140, 3),
	crit_rate: None,
	effects: &[],
};
pub static PsychoBoost: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(140),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -2, 100, StatChangeTarget::User)],
};
pub static Roost: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Flying,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(50)],
};
pub static Gravity: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static MiracleEye: MoveData = MoveData {
    pp: 40,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Custom],
};
pub static WakeUpSlap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static HammerArm: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::User)],
};
pub static GyroBall: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static HealingWish: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Brine: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[],
};
pub static NaturalGift: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Feint: MoveData = MoveData {
    pp: 10,
    priority: 2,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(30),
	crit_rate: None,
	effects: &[],
};
pub static Pluck: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static Tailwind: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Flying,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[],
};
pub static Acupressure: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::UserOrAlly,
    effects: &[],
};
pub static MetalBurst: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::Revenge((3u8, 2u8), None),
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::Implicit,
    effects: &[],
};
pub static UTurn: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::ForceSwitch(StatChangeTarget::User)],
};
pub static CloseCombat: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, -1, 100, StatChangeTarget::User)],
};
pub static Payback: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[],
};
pub static Assurance: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static Embargo: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Custom],
};
pub static Fling: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static PsychoShift: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static TrumpCard: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static HealBlock: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::Custom],
};
pub static WringOut: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static PowerTrick: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static GastroAcid: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static LuckyChant: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[],
};
pub static MeFirst: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Opponent,
    effects: &[],
};
pub static Copycat: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static PowerSwap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static GuardSwap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Punishment: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static LastResort: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(140),
	crit_rate: None,
	effects: &[],
};
pub static WorrySeed: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static SuckerPunch: MoveData = MoveData {
    pp: 5,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static ToxicSpikes: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static HeartSwap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static AquaRing: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Water,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static MagnetRise: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Electric,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static FlareBlitz: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(120, (1u8, 3u8)),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static ForcePalm: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static AuraSphere: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fighting,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static RockPolish: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Rock,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Speed, 2, 100, StatChangeTarget::User)],
};
pub static PoisonJab: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 30, StatChangeTarget::Target)],
};
pub static DarkPulse: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::Flinch(20)],
};
pub static NightSlash: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: Some(1),
	effects: &[],
};
pub static AquaTail: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[],
};
pub static SeedBomb: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static AirSlash: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Flying,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static XScissor: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static BugBuzz: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static DragonPulse: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: None,
	effects: &[],
};
pub static DragonRush: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Dragon,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::Flinch(20)],
};
pub static PowerGem: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Rock,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static DrainPunch: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(75),
	crit_rate: None,
	effects: &[],
};
pub static VacuumWave: MoveData = MoveData {
    pp: 30,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static FocusBlast: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(70),
    _type: Type::Fighting,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static EnergyBall: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static BraveBird: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(120, (1u8, 3u8)),
	crit_rate: None,
	effects: &[],
};
pub static EarthPower: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static Switcheroo: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static GigaImpact: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static NastyPlot: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::SpecialAttack, 2, 100, StatChangeTarget::User)],
};
pub static BulletPunch: MoveData = MoveData {
    pp: 30,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static Avalanche: MoveData = MoveData {
    pp: 10,
    priority: -4,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static IceShard: MoveData = MoveData {
    pp: 30,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static ShadowClaw: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: Some(1),
	effects: &[],
};
pub static ThunderFang: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Flinch(10), Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 10, StatChangeTarget::Target)],
};
pub static IceFang: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Flinch(10), Effect::NonVolatileStatus(NonVolatileBattleAilment::Freeze, 10, StatChangeTarget::Target)],
};
pub static FireFang: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Flinch(10), Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 10, StatChangeTarget::Target)],
};
pub static ShadowSneak: MoveData = MoveData {
    pp: 30,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static MudBomb: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Ground,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 30, StatChangeTarget::Target)],
};
pub static PsychoCut: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: Some(1),
	effects: &[],
};
pub static ZenHeadbutt: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Psychic,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::Flinch(20)],
};
pub static MirrorShot: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Steel,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 30, StatChangeTarget::Target)],
};
pub static FlashCannon: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -1, 10, StatChangeTarget::Target)],
};
pub static RockClimb: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[Effect::Confuse(20, StatChangeTarget::Target)],
};
pub static Defog: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Flying,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::DispelWeather],
};
pub static TrickRoom: MoveData = MoveData {
    pp: 5,
    priority: -7,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static DracoMeteor: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(130),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -2, 100, StatChangeTarget::User)],
};
pub static Discharge: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllExceptUser,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static LavaPlume: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllExceptUser,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 30, StatChangeTarget::Target)],
};
pub static LeafStorm: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(130),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -2, 100, StatChangeTarget::User)],
};
pub static PowerWhip: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[],
};
pub static RockWrecker: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static CrossPoison: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: Some(1),
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 10, StatChangeTarget::Target)],
};
pub static GunkShot: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Poison,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 30, StatChangeTarget::Target)],
};
pub static IronHead: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static MagnetBomb: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static StoneEdge: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: Some(1),
	effects: &[],
};
pub static Captivate: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::StatChange(BattleStat::SpecialAttack, -2, 100, StatChangeTarget::Target)],
};
pub static StealthRock: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Rock,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static GrassKnot: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::WeightBased,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Chatter: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Confuse(100, StatChangeTarget::Target)],
};
pub static Judgment: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[],
};
pub static BugBite: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static ChargeBeam: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, 1, 70, StatChangeTarget::User)],
};
pub static WoodHammer: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(120, (1u8, 3u8)),
	crit_rate: None,
	effects: &[],
};
pub static AquaJet: MoveData = MoveData {
    pp: 20,
    priority: 1,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static AttackOrder: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: Some(1),
	effects: &[],
};
pub static DefendOrder: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 100, StatChangeTarget::User)],
};
pub static HealOrder: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::Heal(50)],
};
pub static HeadSmash: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(80),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(150, (1u8, 2u8)),
	crit_rate: None,
	effects: &[],
};
pub static DoubleHit: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Fixed(2, 35)),
	crit_rate: None,
	effects: &[],
};
pub static RoarOfTime: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(150),
	crit_rate: None,
	effects: &[],
};
pub static SpacialRend: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Dragon,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: Some(1),
	effects: &[],
};
pub static LunarDance: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static CrushGrip: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static MagmaStorm: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(75),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::Bind],
};
pub static DarkVoid: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(50),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 0, StatChangeTarget::Target)],
};
pub static SeedFlare: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -2, 40, StatChangeTarget::Target)],
};
pub static OminousWind: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Attack, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::Defense, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialAttack, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 10, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, 1, 10, StatChangeTarget::User)],
};
pub static ShadowForce: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(120, Some(SemiInvulnerableLocation::Vanished)),
	crit_rate: None,
	effects: &[],
};
pub static HoneClaws: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Accuracy, 1, 100, StatChangeTarget::User)],
};
pub static WideGuard: MoveData = MoveData {
    pp: 10,
    priority: 3,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Rock,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[],
};
pub static GuardSplit: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static PowerSplit: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static WonderRoom: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static Psyshock: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static Venoshock: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[],
};
pub static Autotomize: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Steel,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Speed, 2, 100, StatChangeTarget::User)],
};
pub static RagePowder: MoveData = MoveData {
    pp: 20,
    priority: 2,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Telekinesis: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Custom],
};
pub static MagicRoom: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::Opponents,
    effects: &[],
};
pub static SmackDown: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Rock,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::Custom],
};
pub static StormThrow: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: Some(6),
	effects: &[],
};
pub static FlameBurst: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static SludgeWave: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllExceptUser,
    power: Power::Base(95),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Poison(PoisonType::Poison), 10, StatChangeTarget::Target)],
};
pub static QuiverDance: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Bug,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::SpecialAttack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, 1, 100, StatChangeTarget::User)],
};
pub static HeavySlam: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::WeightRatioBased,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Synchronoise: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllExceptUser,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[],
};
pub static ElectroBall: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Soak: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static FlameCharge: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, 1, 100, StatChangeTarget::User)],
};
pub static Coil: MoveData = MoveData {
    pp: 20,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Poison,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Defense, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Accuracy, 1, 100, StatChangeTarget::User)],
};
pub static LowSweep: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static AcidSpray: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialDefense, -2, 100, StatChangeTarget::Target)],
};
pub static FoulPlay: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(95),
	crit_rate: None,
	effects: &[],
};
pub static SimpleBeam: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Entrainment: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static AfterYou: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Round: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static EchoedVoice: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(40),
	crit_rate: None,
	effects: &[],
};
pub static ChipAway: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static ClearSmog: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Poison,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[],
};
pub static StoredPower: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(20),
	crit_rate: None,
	effects: &[],
};
pub static QuickGuard: MoveData = MoveData {
    pp: 15,
    priority: 3,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Fighting,damage_type: DamageType::Status,
    target: Target::UserAndAlly,
    effects: &[],
};
pub static AllySwitch: MoveData = MoveData {
    pp: 15,
    priority: 2,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static Scald: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 30, StatChangeTarget::Target)],
};
pub static ShellSmash: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[],
};
pub static HealPulse: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Psychic,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[Effect::Heal(50)],
};
pub static Hex: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ghost,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[],
};
pub static SkyDrop: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static ShiftGear: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Steel,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, 2, 100, StatChangeTarget::User)],
};
pub static CircleThrow: MoveData = MoveData {
    pp: 10,
    priority: -6,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::ForceSwitch(StatChangeTarget::Target)],
};
pub static Incinerate: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[],
};
pub static Quash: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Dark,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Acrobatics: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Flying,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(55),
	crit_rate: None,
	effects: &[],
};
pub static ReflectType: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Retaliate: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[],
};
pub static FinalGambit: MoveData = MoveData {
    pp: 5,
    priority: 0,
    power: Power::Variable,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Bestow: MoveData = MoveData {
    pp: 15,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static Inferno: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(50),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 100, StatChangeTarget::Target)],
};
pub static WaterPledge: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Water,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static FirePledge: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static GrassPledge: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[],
};
pub static VoltSwitch: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(70),
	crit_rate: None,
	effects: &[Effect::ForceSwitch(StatChangeTarget::User)],
};
pub static StruggleBug: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(50),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -1, 100, StatChangeTarget::Target)],
};
pub static Bulldoze: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllExceptUser,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static FrostBreath: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: Some(6),
	effects: &[],
};
pub static DragonTail: MoveData = MoveData {
    pp: 10,
    priority: -6,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Dragon,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::ForceSwitch(StatChangeTarget::Target)],
};
pub static WorkUp: MoveData = MoveData {
    pp: 30,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Normal,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Attack, 1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialAttack, 1, 100, StatChangeTarget::User)],
};
pub static Electroweb: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Electric,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(55),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static WildCharge: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(90, (1u8, 4u8)),
	crit_rate: None,
	effects: &[],
};
pub static DrillRun: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Ground,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: Some(1),
	effects: &[],
};
pub static DualChop: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Dragon,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Fixed(2, 40)),
	crit_rate: None,
	effects: &[],
};
pub static HeartStamp: MoveData = MoveData {
    pp: 25,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(60),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static HornLeech: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Grass,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithDrain(75),
	crit_rate: None,
	effects: &[],
};
pub static SacredSword: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(90),
	crit_rate: None,
	effects: &[],
};
pub static RazorShell: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Water,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 50, StatChangeTarget::Target)],
};
pub static HeatCrash: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::WeightRatioBased,
    crit_rate: None,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    effects: &[],
};
pub static LeafTornado: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Grass,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 50, StatChangeTarget::Target)],
};
pub static Steamroller: MoveData = MoveData {
    pp: 20,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Bug,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static CottonGuard: MoveData = MoveData {
    pp: 10,
    priority: 0,
    power: Power::None,
    crit_rate: None,
    accuracy: Accuracy::AlwaysHits,
    _type: Type::Grass,damage_type: DamageType::Status,
    target: Target::User,
    effects: &[Effect::StatChange(BattleStat::Defense, 3, 100, StatChangeTarget::User)],
};
pub static NightDaze: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Dark,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Accuracy, -1, 40, StatChangeTarget::Target)],
};
pub static Psystrike: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Psychic,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[],
};
pub static TailSlap: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Variable(25)),
	crit_rate: None,
	effects: &[],
};
pub static Hurricane: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(70),
    _type: Type::Flying,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(110),
	crit_rate: None,
	effects: &[Effect::Confuse(30, StatChangeTarget::Target)],
};
pub static HeadCharge: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithRecoil(120, (1u8, 4u8)),
	crit_rate: None,
	effects: &[],
};
pub static GearGrind: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Steel,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::MultiHit(MultiHitFlavor::Fixed(2, 50)),
	crit_rate: None,
	effects: &[],
};
pub static SearingShot: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllExceptUser,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 30, StatChangeTarget::Target)],
};
pub static TechnoBlast: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(120),
	crit_rate: None,
	effects: &[],
};
pub static RelicSong: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Normal,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(75),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Sleep, 10, StatChangeTarget::Target)],
};
pub static SecretSword: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fighting,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: None,
	effects: &[],
};
pub static Glaciate: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(65),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::Target)],
};
pub static BoltStrike: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(130),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 20, StatChangeTarget::Target)],
};
pub static BlueFlare: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(85),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(130),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 20, StatChangeTarget::Target)],
};
pub static FieryDance: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(80),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, 1, 50, StatChangeTarget::User)],
};
pub static FreezeShock: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(140, None),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Paralysis, 30, StatChangeTarget::Target)],
};
pub static IceBurn: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ice,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::BaseWithCharge(140, None),
	crit_rate: None,
	effects: &[Effect::NonVolatileStatus(NonVolatileBattleAilment::Burn, 30, StatChangeTarget::Target)],
};
pub static Snarl: MoveData = MoveData {
    pp: 15,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Dark,damage_type: DamageType::Special,
    target: Target::Opponents,
    power: Power::Base(55),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::SpecialAttack, -1, 100, StatChangeTarget::Target)],
};
pub static IcicleCrash: MoveData = MoveData {
    pp: 10,
    priority: 0,
    accuracy: Accuracy::Percentage(90),
    _type: Type::Ice,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(85),
	crit_rate: None,
	effects: &[Effect::Flinch(30)],
};
pub static VCreate: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(95),
    _type: Type::Fire,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(180),
	crit_rate: None,
	effects: &[Effect::StatChange(BattleStat::Defense, -1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::SpecialDefense, -1, 100, StatChangeTarget::User), Effect::StatChange(BattleStat::Speed, -1, 100, StatChangeTarget::User)],
};
pub static FusionFlare: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Fire,damage_type: DamageType::Special,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[],
};
pub static FusionBolt: MoveData = MoveData {
    pp: 5,
    priority: 0,
    accuracy: Accuracy::Percentage(100),
    _type: Type::Electric,damage_type: DamageType::Physical,
    target: Target::AllyOrOpponent,
    power: Power::Base(100),
	crit_rate: None,
	effects: &[],
};
//endregion