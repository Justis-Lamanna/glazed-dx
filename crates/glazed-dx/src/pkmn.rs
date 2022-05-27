use glazed_data::abilities::Ability;
use glazed_data::attack::Move;
use glazed_data::core::OneOrTwo;
use glazed_data::locations::Location;
use rand::Rng as o;
use serde::Deserialize;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_kira_audio::{Audio, AudioChannel, InstanceHandle, PlaybackState};
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

use glazed_data::pokemon::{Gender, Pokemon, SpeciesData, PokemonTemplate, GenderRatio, AbilitySlot, MoveTemplate, MoveSlot, PokemonPokerusStatus, StatSlot, NatureBoost, IVTemplate, EVTemplate, Nature, SpeciesTemplate};

use glazed_data::species::{ForcesOfNatureForm, KyuremForm, ShayminForm, Species, SpeciesDiscriminants, UnownForm, CastformForm, DeoxysForm, BurmyWormadamForm, CherrimForm, ShellosGastrodonForm, RotomForm, BasculinForm, DarmanitanForm, KeldeoForm, MeloettaForm, GenesectForm};
use crate::locale::Fluent;
use crate::player::{Player, PlayerService};
use crate::state::GlobalOptions;
use crate::util::{YamlLoader, Rng};

pub struct PkmnPlugin;
impl Plugin for PkmnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset_loader(YamlLoader::<SpeciesDataMapping, SpeciesDataLookup>::new("pkmn"))
            .add_asset::<SpeciesDataLookup>()
        ;
    }
}

#[derive(Deserialize)]
pub struct SpeciesDataMapping {
    id: Species,
    data: SpeciesData
}

#[derive(TypeUuid, Deref, DerefMut)]
#[uuid = "22f5c97d-2715-49c3-a7f8-18fdbc76060f"]
pub struct SpeciesDataLookup(HashMap<Species, SpeciesData>);
impl From<Vec<SpeciesDataMapping>> for SpeciesDataLookup {
    fn from(vec: Vec<SpeciesDataMapping>) -> Self {
        let d = vec.into_iter()
            .map(|p| (p.id, p.data))
            .collect::<HashMap<Species, SpeciesData>>();
        SpeciesDataLookup(d)
    }
}

/// Hold all Lookup files
pub struct PokemonDataFiles {
    pub species_data: Handle<SpeciesDataLookup>
}

#[derive(SystemParam)]
pub struct PokemonLookupService<'w, 's> {
    handles: Res<'w, PokemonDataFiles>,
    assets: Res<'w, Assets<SpeciesDataLookup>>,
    #[allow(dead_code)]
    marker: Commands<'w, 's>,
}
impl <'w, 's> PokemonLookupService<'w, 's> {
    pub fn lookup(&self, pkmn: Species) -> Option<&SpeciesData> {
        let handle = &self.handles.species_data;
        let asset = self.assets.get(handle);

        asset.and_then(|l| l.0.get(&pkmn))
    }
}

#[derive(SystemParam)]
pub struct CryService<'w, 's> {
    audio: Res<'w, Audio>,
    assets: Res<'w, AssetServer>,
    options: Res<'w, GlobalOptions>,
    commands: Commands<'w, 's>,
    handle: Option<Res<'w, CryHandle>>
}
pub struct CryHandle(InstanceHandle);
impl<'w, 's> CryService<'w, 's> {
    const BASE: &'static str = "pkmn/cries";
    const SUFFIX: &'static str = "ogg";

    fn channel() -> AudioChannel {
        AudioChannel::new("pkmn-cry".into())
    }

    fn get_cry_file(species: Species) -> String {
        let raw: &'static str = match species {
            Species::Kyurem(KyuremForm::Black) => "kyurem-black",
            Species::Kyurem(KyuremForm::White) => "kyurem-white",
            Species::Tornadus(ForcesOfNatureForm::Therian) => "tornadus-therian",
            Species::Thundurus(ForcesOfNatureForm::Therian) => "tornadus-therian",
            Species::Landorus(ForcesOfNatureForm::Therian) => "tornadus-therian",
            Species::Shaymin(ShayminForm::Sky) => "shaymin-sky",
            s => {
                let raw: SpeciesDiscriminants = s.into();
                raw.into()
            }
        };

        return format!("{}/{}.{}", CryService::BASE, raw, CryService::SUFFIX);
    }

    pub fn play_cry(&mut self, species: Species) {
        let channel = Self::channel();
        self.audio.stop_channel(&channel);
        self.audio.set_volume_in_channel(self.options.volume.get_sfx_volume(), &channel);
        let h = self.audio.play_in_channel(
            self.assets.load(CryService::get_cry_file(species).as_str()),
            &channel
        );

        self.commands.insert_resource(CryHandle(h))
    }

    pub fn is_cry_complete(&self) -> bool {
        if let Some(a) = &self.handle {
            let state = self.audio.state(a.0.clone());
            matches!(state, PlaybackState::Stopped)
        } else {
            true
        }
    }
}

pub struct SpriteRequest {
    pub species: Species,
    pub gender: Gender,
    pub shiny: bool
}
impl From<&Pokemon> for SpriteRequest {
    fn from(p: &Pokemon) -> Self {
        SpriteRequest {
            species: p.species,
            gender: p.gender,
            shiny: p.is_shiny()
        }
    }
}
impl From<Species> for SpriteRequest {
    fn from(s: Species) -> Self {
        SpriteRequest {
            species: s,
            gender: Gender::Male,
            shiny: false
        }
    }
}

#[derive(SystemParam)]
pub struct PokemonSpriteService<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    #[allow(dead_code)]
    marker: Query<'w, 's, ()>
}
impl<'w, 's> PokemonSpriteService<'w, 's> {
    const BASE: &'static str = "pkmn/battle";
    const FRONT: &'static str = "front";
    const BACK: &'static str = "front";
    const NORMAL: &'static str = "normal";
    const SHINY: &'static str = "shiny";
    const SUFFIX: &'static str = "png";

    pub fn get_front_sprite<T: Into<SpriteRequest>>(&self, request: T) -> Handle<Image> {
        let request = request.into();
        self.get_battle_sprite(request.species, request.gender, request.shiny, true)
    }

    // pub fn get_back_sprite<T: Into<SpriteRequest>>(&self, request: T) -> Handle<Image> {
    //     let request = request.into();
    //     self.get_battle_sprite(request.species, request.gender, request.shiny, false)
    // }

    fn get_battle_sprite(&self, species: Species, gender: Gender, shiny: bool, front: bool) -> Handle<Image> {
        let species_path: String = match species {
            // Forms
            Species::Unown(letter) => {
                match letter {
                    UnownForm::QuestionMark => "unown-qm".into(),
                    UnownForm::ExclamationMark => "unown-em".into(),
                    l => {
                        let l: &str = l.into();
                        format!("unown-{}", l.to_ascii_lowercase())
                    }
                }
            },
            Species::Castform(CastformForm::Sunny) => "castform-sunny".into(),
            Species::Castform(CastformForm::Rainy) => "castform-rainy".into(),
            Species::Castform(CastformForm::Snowy) => "castform-snowy".into(),
            Species::Deoxys(DeoxysForm::Attack) => "deoxys-attack".into(),
            Species::Deoxys(DeoxysForm::Defense) => "deoxys-defense".into(),
            Species::Deoxys(DeoxysForm::Speed) => "deoxys-speed".into(),
            Species::Burmy(BurmyWormadamForm::Plant) => "burmy-plant".into(),
            Species::Burmy(BurmyWormadamForm::Sandy) => "burmy-sandy".into(),
            Species::Burmy(BurmyWormadamForm::Trash) => "burmy-trash".into(),
            Species::Wormadam(BurmyWormadamForm::Plant) => "wormadam-plant".into(),
            Species::Wormadam(BurmyWormadamForm::Sandy) => "wormadam-sandy".into(),
            Species::Wormadam(BurmyWormadamForm::Trash) => "wormadam-trash".into(),
            Species::Cherrim(CherrimForm::Sunshine) => "cherrim-sunshine".into(),
            Species::Shellos(ShellosGastrodonForm::EastSea) => "shellos-east".into(),
            Species::Shellos(ShellosGastrodonForm::WestSea) => "shellos-west".into(),
            Species::Gastrodon(ShellosGastrodonForm::EastSea) => "gastrodon-east".into(),
            Species::Gastrodon(ShellosGastrodonForm::WestSea) => "gastrodon-west".into(),
            Species::Rotom(RotomForm::Heat) => "rotom-heat".into(),
            Species::Rotom(RotomForm::Wash) => "rotom-wash".into(),
            Species::Rotom(RotomForm::Frost) => "rotom-frost".into(),
            Species::Rotom(RotomForm::Fan) => "rotom-fan".into(),
            Species::Rotom(RotomForm::Mow) => "rotom-mow".into(),
            Species::Shaymin(ShayminForm::Sky) => "shaymin-sky".into(),
            Species::Arceus(t) => {
                let t: &str = t.into();
                format!("arceus-{}", t.to_ascii_lowercase())
            },
            Species::Basculin(BasculinForm::BlueStriped) => "basculin-bluestriped".into(),
            Species::Darmanitan(DarmanitanForm::Zen) => "darmanitan-zen".into(),
            Species::Deerling(season) => {
                let season: &str = season.into();
                format!("deerling-{}", season.to_ascii_lowercase())
            },
            Species::Sawsbuck(season) => {
                let season: &str = season.into();
                format!("sawsbuck-{}", season.to_ascii_lowercase())
            },
            Species::Tornadus(ForcesOfNatureForm::Therian) => "tornadus-therian".into(),
            Species::Thundurus(ForcesOfNatureForm::Therian) => "thundurus-therian".into(),
            Species::Landorus(ForcesOfNatureForm::Therian) => "landorus-therian".into(),
            Species::Kyurem(KyuremForm::Black) => "kyurem-black".into(),
            Species::Kyurem(KyuremForm::White) => "kyurem-white".into(),
            Species::Keldeo(KeldeoForm::Resolute) => "keldeo-resolute".into(),
            Species::Meloetta(MeloettaForm::Pirouette) => "meloetta-pirouette".into(),
            Species::Genesect(GenesectForm::Burn) => "genesect-burn".into(),
            Species::Genesect(GenesectForm::Douse) => "genesect-douse".into(),
            Species::Genesect(GenesectForm::Chill) => "genesect-chill".into(),
            Species::Genesect(GenesectForm::Shock) => "genesect-shock".into(),
            // Female variants
            Species::Venusaur | Species::Butterfree | Species::Rattata | Species::Raticate |
            Species::Pikachu | Species::Raichu | Species::Zubat | Species::Golbat |
            Species::Gloom | Species::Vileplume | Species::Kadabra | Species::Alakazam |
            Species::Doduo | Species::Dodrio | Species::Hypno | Species::Rhyhorn |
            Species::Rhydon | Species::Goldeen | Species::Seaking | Species::Scyther |
            Species::Magikarp | Species::Gyarados | Species::Meganium | Species::Ledyba |
            Species::Ledian | Species::Xatu | Species::Sudowoodo | Species::Politoed |
            Species::Aipom | Species::Wooper | Species::Quagsire | Species::Murkrow |
            Species::Wobbuffet | Species::Girafarig | Species::Gligar | Species::Steelix |
            Species::Scizor | Species::Heracross | Species::Sneasel | Species::Ursaring |
            Species::Piloswine | Species::Octillery | Species::Houndoom | Species::Donphan |
            Species::Torchic | Species::Combusken | Species::Blaziken | Species::Beautifly |
            Species::Dustox | Species::Ludicolo | Species::Nuzleaf | Species::Shiftry |
            Species::Meditite | Species::Medicham | Species::Roselia | Species::Gulpin |
            Species::Swalot | Species::Numel | Species::Camerupt | Species::Cacturne |
            Species::Milotic | Species::Relicanth | Species::Starly | Species::Staravia |
            Species::Staraptor | Species::Bidoof | Species::Bibarel | Species::Kricketot |
            Species::Kricketune | Species::Shinx | Species::Luxio | Species::Luxray |
            Species::Roserade | Species::Combee | Species::Pachirisu | Species::Buizel |
            Species::Floatzel | Species::Ambipom | Species::Gible | Species::Gabite |
            Species::Garchomp | Species::Hippopotas | Species::Hippowdon | Species::Croagunk |
            Species::Toxicroak | Species::Finneon | Species::Lumineon | Species::Snover |
            Species::Abomasnow | Species::Weavile | Species::Rhyperior | Species::Tangrowth |
            Species::Mamoswine | Species::Unfezant | Species::Frillish | Species::Jellicent
            if gender == Gender::Female => {
                let raw: SpeciesDiscriminants = species.into();
                let raw: &str  = raw.into();
                format!("{}-f", raw)
            }
            s => {
                let raw: SpeciesDiscriminants = s.into();
                let raw: &str = raw.into();
                raw.into()
            }
        };

        let shiny_path = match shiny {
            true => Self::SHINY,
            false => Self::NORMAL
        };

        let front_path = match front {
            true => Self::FRONT,
            false => Self::BACK
        };

        let path = format!("{}/{}/{}/{}.{}", Self::BASE, front_path, shiny_path, species_path, Self::SUFFIX);

        self.asset_server.load(path.as_str())
    }
}

impl<'w, 's> Fluent<'w, 's> {
    /// Get the translated version of a Pokemon's name
    /// Form is completely disregarded; A Deoxys(Attack) and Deoxys(Defense) both return
    /// "Deoxys" in English.
    pub fn get_pokemon_species_name(&self, species: Species) -> String {
        let species: SpeciesDiscriminants = species.into();
        let species: &str = species.into();
        let species_key = format!("{}-species", species.to_ascii_lowercase());

        self.translate(species_key.as_str())
            .expect(species_key.as_str())
    }

    /// Retrieve the Pokemon's species name, and put it in the buffer.
    pub fn buffer_species_name<K: ToString>(&mut self, key: K, species: Species) {
        let name = self.get_pokemon_species_name(species);
        self.buffer_string(key, name);
    }
}

impl<'w, 's> PlayerService<'w, 's> {
    pub fn resolve_pokemon_template(&self, template: PokemonTemplate, rng: &mut Rng) -> Pokemon {
        let species = self.create_species(template.species, rng);

        // We will use this data a lot.
        let data = self.pkmn_lookup.lookup(species)
            .expect("Missing Pokemon Species data");

        // Calculate the movepool.
        let mut available_pool = data.get_knowable_moves_for_level(template.level).into_iter();
        let mut final_moves = Vec::new();
        Self::resolve_move_template(template.move_1, &mut final_moves, &mut available_pool);
        Self::resolve_move_template(template.move_2, &mut final_moves, &mut available_pool);
        Self::resolve_move_template(template.move_3, &mut final_moves, &mut available_pool);
        Self::resolve_move_template(template.move_4, &mut final_moves, &mut available_pool);
        let mut final_moves = final_moves.into_iter()
            .map(|m| MoveSlot {
                attack: m,
                pp: 5, // <- Look this up
                pp_bonus: 0,
            });

        // Use the player's data if none is provided otherwise.
        let (trainer_id, trainer_secret, trainer_name) = template.original_trainer
            .map(|t| (t.trainer_id, t.secret_id, t.name))
            .unwrap_or((self.player.trainer_id, self.player.secret_id, self.player.name.clone()));

        // Calculate nature now, so we can calculate stats in the next step.
        let nature = template.nature.unwrap_or(rng.gen());
        // Calculate stats
        let (hp, atk, def, spa, spd, spe) 
            = Self::create_stats(data, template.level, nature, template.ivs, template.evs, rng);

        // Calculate personality value, taking a force shiny into account.
        let personality = match template.personality {
            Some(p) => p,
            None if template.force_shiny => {
                // The simplest way to generate a Pokemon is to create a personality value which, when
                // the first half is xor'ed with the second, is equal to trainer_portion. This will cause the
                // full expression to equal 0, which matches as a shiny
                let personality_hb = rng.gen::<u16>();
                let personality_lb = trainer_id ^ trainer_secret ^ personality_hb;

                let personality_hb = personality_hb as u32;
                let personality_lb = personality_lb as u32;

                (personality_hb << 16) | personality_lb
            }
            _ => rng.gen()
        };

        // Construct the entire Pokemon finally
        Pokemon {
            species: species,
            gender: template.gender.unwrap_or(self.create_gender(data, rng)),
            egg: template.level == 0,
            level_met: template.level_met.unwrap_or(template.level),
            nature,
            ability: template.ability.unwrap_or(rng.gen()),
            poke_ball: template.poke_ball.unwrap_or(glazed_data::item::Pokeball::PokeBall),
            held_item: template.held_item,
            move_1: final_moves.next(),
            move_2: final_moves.next(),
            move_3: final_moves.next(),
            move_4: final_moves.next(),
            experience: data.level_rate.experience_for_level(template.level),
            personality,
            friendship: template.friendship.unwrap_or(data.base_friendship),
            original_trainer_id: trainer_id,
            original_trainer_secret_id: trainer_secret,
            original_trainer_name: trainer_name,
            nickname: template.nickname,
            level: template.level,
            markings: template.markings,
            status: template.status.unwrap_or_default(),
            pokerus: template.pokerus.unwrap_or(PokemonPokerusStatus::None),
            current_hp: hp.value,
            hp,
            attack: atk,
            defense: def,
            special_attack: spa,
            special_defense: spd,
            speed: spe,
            contest: template.contest.unwrap_or_default(),
            fateful_encounter: template.fateful_encounter,
            date_caught: template.date_caught.unwrap_or(0), // <- Insert Timestamp
            location_caught: template.location_caught.unwrap_or(Location::FarawayPlace), // <- Retrieve actual location
        }
    }

    fn create_species(&self, t: SpeciesTemplate, rng: &mut Rng) -> Species {
        match t {
            SpeciesTemplate::HardCoded(s) => s,
            SpeciesTemplate::RandomUnown => Species::Unown(rng.gen())
        }
    }

    // Construct a random gender this Pokemon can be.
    fn create_gender(&self, data: &SpeciesData, rng: &mut Rng) -> Gender {
        match data.gender_ratio {
            GenderRatio::None | GenderRatio::Proportion(0, 0) => Gender::None,
            GenderRatio::Proportion(0, _) => Gender::Female,
            GenderRatio::Proportion(_, 0) => Gender::Male,
            GenderRatio::Proportion(m, f) => {
                let ratio = (m as f64) / ((m + f) as f64);
                if rng.gen_bool(ratio) { Gender::Male } else { Gender::Female }
            }
        }
    }

    // Calculate the move for the given move template.
    // If the move is hard-coded, it is immediately added, provided the Pokemon does not already have it.
    // If the move is natural, the next non-known level-up move is added. Does nothing if there are no other moves
    // No move does nothing.
    fn resolve_move_template<T: Iterator<Item = Move>>(template: MoveTemplate, final_moves: &mut Vec<Move>, available_pool: &mut T) {
        match template {
            MoveTemplate::HardCoded(m) => {
                if !final_moves.contains(&m) {
                    final_moves.push(m);
                }
            }
            MoveTemplate::NaturalMove => {
                while let Some(m) = available_pool.next() {
                    if !final_moves.contains(&m) {
                        final_moves.push(m);
                        break;
                    }
                }
            }
            MoveTemplate::None => {}
        }
    }

    /// Calculate the stat block of a Pokemon.
    /// Also resolves IV and EV templates. 
    fn create_stats(data: &SpeciesData, level: u8, nature: Nature, ivs: IVTemplate, evs: EVTemplate, rng: &mut Rng) -> (StatSlot, StatSlot, StatSlot, StatSlot, StatSlot, StatSlot) {
        let ivs = match ivs {
            IVTemplate::Random => [
                rng.gen_range(0..=31),
                rng.gen_range(0..=31),
                rng.gen_range(0..=31),
                rng.gen_range(0..=31),
                rng.gen_range(0..=31),
                rng.gen_range(0..=31)
            ],
            IVTemplate::HardCoded(hp, atk, def, spa, spd, spe) => [hp, atk, def, spa, spd, spe],
            IVTemplate::All(val) => [val; 6],
            IVTemplate::Rare => {
                let mut lucky_slots = [false; 6];
                let mut counter = 0;
                while counter < 3 {
                    let slot = rng.gen_range(0..6);
                    if !lucky_slots[slot] {
                        lucky_slots[slot] = true;
                        counter += 1;
                    }
                }
                let mut stats = [0u8; 6];
                for (idx, slot) in stats.iter_mut().enumerate() {
                    if lucky_slots[idx] {
                        *slot = 31u8;
                    } else {
                        *slot = rng.gen_range(0u8..=31u8);
                    }
                }
                stats
            }
        };
        let evs = match evs {
            EVTemplate::HardCoded(a, b, c, d, e, f) => [a, b, c, d, e, f],
            EVTemplate::All(v) => [v; 6]
        };
        (
            StatSlot::hp(data.stats.0.base_stat, level, ivs[0], evs[0]),
            StatSlot::stat(data.stats.1.base_stat, level, ivs[1], evs[1], nature.get_attack_boost()),
            StatSlot::stat(data.stats.2.base_stat, level, ivs[2], evs[2], nature.get_defense_boost()),
            StatSlot::stat(data.stats.3.base_stat, level, ivs[3], evs[3], nature.get_special_attack_boost()),
            StatSlot::stat(data.stats.4.base_stat, level, ivs[4], evs[4], nature.get_special_defense_boost()),
            StatSlot::stat(data.stats.5.base_stat, level, ivs[5], evs[5], nature.get_speed_boost()),
        )
    }
}