/// Represents an Ability a Pokemon has
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Ability {
    AirLock,
    ArenaTrap,
    BattleArmor,
    Blaze,
    Cacophony,
    Chlorophyll,
    ClearBody,
    CloudNine,
    ColorChange,
    CompoundEyes,
    CuteCharm,
    Damp,
    Drizzle,
    Drought,
    EarlyBird,
    EffectSpore,
    FlameBody,
    FlashFire,
    Forecast,
    Guts,
    HugePower,
    Hustle,
    HyperCutter,
    Illuminate,
    Immunity,
    InnerFocus,
    Insomnia,
    Intimidate,
    KeenEye,
    Levitate,
    LightningRod,
    Limber,
    LiquidOoze,
    MagmaArmor,
    MagnetPull,
    MarvelScale,
    Minus,
    NaturalCure,
    Oblivious,
    Overgrow,
    OwnTempo,
    Pickup,
    Plus,
    PoisonPoint,
    Pressure,
    PurePower,
    RainDish,
    RockHead,
    RoughSkin,
    RunAway,
    SandStream,
    SandVeil,
    SereneGrace,
    ShadowTag,
    ShedSkin,
    ShellArmor,
    ShieldDust,
    Soundproof,
    SpeedBoost,
    Static,
    Stench,
    StickyHold,
    Sturdy,
    SuctionCups,
    Swarm,
    SwiftSwim,
    Synchronize,
    ThickFat,
    Torrent,
    Trace,
    Truant,
    VitalSpirit,
    VoltAbsorb,
    WaterAbsorb,
    WaterVeil,
    WhiteSmoke,
    WonderGuard,
    Adaptability,
    Aftermath,
    AngerPoint,
    Anticipation,
    BadDreams,
    Download,
    DrySkin,
    Filter,
    FlowerGift,
    Forewarn,
    Frisk,
    Gluttony,
    Heatproof,
    HoneyGather,
    Hydration,
    IceBody,
    IronFist,
    Klutz,
    LeafGuard,
    MagicGuard,
    MoldBreaker,
    MotorDrive,
    Multitype,
    NoGuard,
    Normalize,
    PoisonHeal,
    QuickFeet,
    Reckless,
    Rivalry,
    Scrappy,
    Simple,
    SkillLink,
    SlowStart,
    Sniper,
    SnowCloak,
    SnowWarning,
    SolarPower,
    SolidRock,
    Stall,
    Steadfast,
    StormDrain,
    SuperLuck,
    TangledFeet,
    Technician,
    TintedLens,
    Unaware,
    Unburden,
    Analytic,
    BigPecks,
    Contrary,
    CursedBody,
    Defeatist,
    Defiant,
    FlareBoost,
    FriendGuard,
    Harvest,
    Healer,
    HeavyMetal,
    Illusion,
    Imposter,
    Infiltrator,
    IronBarbs,
    Justified,
    LightMetal,
    MagicBounce,
    Moody,
    Moxie,
    Multiscale,
    Mummy,
    Overcoat,
    Pickpocket,
    PoisonTouch,
    Prankster,
    Rattled,
    Regenerator,
    SandForce,
    SandRush,
    SapSipper,
    SheerForce,
    Telepathy,
    Teravolt,
    ToxicBoost,
    Turboblaze,
    Unnerve,
    VictoryStar,
    WeakArmor,
    WonderSkin,
    ZenMode,
    Competitive,
    SlushRush,
    Protean
}
impl Ability {
    pub fn is_ignore_ability_ability(&self) -> bool {
        match self {
            Ability::MoldBreaker | Ability::Teravolt | Ability::Turboblaze => true,
            _ => false
        }
    }
}

/// Represents the ability(s) for a Pokemon
#[derive(Debug)]
pub enum PokemonAbility {
    One(Ability),
    Two(Ability, Ability)
}