use std::marker::PhantomData;
use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_kira_audio::{Audio, AudioChannel, InstanceHandle};
use glazed_data::pokemon::{Gender, Pokemon};

use glazed_data::species::{ForcesOfNatureForm, KyuremForm, ShayminForm, Species, SpeciesDiscriminants};
use crate::state::GlobalOptions;

#[derive(SystemParam)]
pub struct Cry<'w, 's> {
    audio: Res<'w, Audio>,
    assets: Res<'w, AssetServer>,
    options: Res<'w, GlobalOptions>,
    commands: Commands<'w, 's>
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

    pub fn get_back_sprite<T: Into<SpriteRequest>>(&self, request: T) -> Handle<Image> {
        let request = request.into();
        self.get_battle_sprite(request.species, request.gender, request.shiny, false)
    }

    fn get_battle_sprite(&self, species: Species, _gender: Gender, shiny: bool, front: bool) -> Handle<Image> {
        let species_path: &str = match species {
            s => {
                let raw: SpeciesDiscriminants = s.into();
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