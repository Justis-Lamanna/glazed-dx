use serde::Deserialize;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_kira_audio::{Audio, AudioChannel, InstanceHandle, PlaybackState};
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

use glazed_data::pokemon::{Gender, Pokemon, SpeciesData};

use glazed_data::species::{ForcesOfNatureForm, KyuremForm, ShayminForm, Species, SpeciesDiscriminants, UnownForm, CastformForm, DeoxysForm, BurmyWormadamForm, CherrimForm, ShellosGastrodonForm, RotomForm, BasculinForm, DarmanitanForm, KeldeoForm, MeloettaForm, GenesectForm};
use crate::locale::Fluent;
use crate::state::GlobalOptions;
use crate::util::YamlLoader;

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
pub struct PokemonLookup<'w, 's> {
    handles: Res<'w, PokemonDataFiles>,
    assets: Res<'w, Assets<SpeciesDataLookup>>,
    #[allow(dead_code)]
    marker: Commands<'w, 's>,
}
impl <'w, 's> PokemonLookup<'w, 's> {
    pub fn lookup(&self, pkmn: Species) -> Option<&SpeciesData> {
        let handle = &self.handles.species_data;
        let asset = self.assets.get(handle);

        asset.and_then(|l| l.0.get(&pkmn))
    }
}

#[derive(SystemParam)]
pub struct Cry<'w, 's> {
    audio: Res<'w, Audio>,
    assets: Res<'w, AssetServer>,
    options: Res<'w, GlobalOptions>,
    commands: Commands<'w, 's>,
    handle: Option<Res<'w, CryHandle>>
}
pub struct CryHandle(InstanceHandle);
impl<'w, 's> Cry<'w, 's> {
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

        return format!("{}/{}.{}", Cry::BASE, raw, Cry::SUFFIX);
    }

    pub fn play_cry(&mut self, species: Species) {
        let channel = Self::channel();
        self.audio.stop_channel(&channel);
        self.audio.set_volume_in_channel(self.options.volume.get_sfx_volume(), &channel);
        let h = self.audio.play_in_channel(
            self.assets.load(Cry::get_cry_file(species).as_str()),
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
pub struct PokemonSprite<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    #[allow(dead_code)]
    marker: Query<'w, 's, ()>
}
impl<'w, 's> PokemonSprite<'w, 's> {
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

