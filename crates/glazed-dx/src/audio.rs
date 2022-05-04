use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, InstanceHandle};

use glazed_data::species::{ForcesOfNatureForm, KyuremForm, ShayminForm, Species, SpeciesDiscriminants};
use crate::state::GlobalOptions;

pub struct Cry;
pub struct CryHandle(InstanceHandle);
pub struct PlayCry(pub Species);

impl Cry {
    const BASE: &'static str = "audio/pkmn-cries";
    const SUFFIX: &'static str = "ogg";

    pub fn channel() -> AudioChannel {
        AudioChannel::new("pkmn-cry".into())
    }

    pub fn get_cry_file(species: Species) -> String {
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

    fn watch_cry_system(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>, options: Res<GlobalOptions>, mut species: EventReader<PlayCry>) {
        if let Some(PlayCry(species)) = species.iter().last() {
            let channel = Self::channel();
            audio.stop_channel(&channel);
            audio.set_volume_in_channel(options.volume.get_sfx_volume(), &channel);
            let h = audio.play_in_channel(
                assets.load(Cry::get_cry_file(*species).as_str()),
                &channel
            );

            commands.insert_resource(CryHandle(h))
        }
    }
}
impl Plugin for Cry {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayCry>()
            .add_system(Cry::watch_cry_system);
    }
}