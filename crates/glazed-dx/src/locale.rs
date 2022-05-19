use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::str;

use bevy::ecs::system::SystemParam;
use bevy::reflect::TypeUuid;
use bevy::{prelude::*, asset::LoadedAsset};
use bevy::asset::AssetLoader;

use fluent::{FluentBundle, FluentResource, FluentMessage, FluentArgs};
use unic_langid::{LanguageIdentifier, langid};
use fluent_langneg::negotiate_languages;
use fluent_langneg::NegotiationStrategy;

use crate::player::Player;

#[derive(Default)]
pub struct TranslationPlugin {
    pub default_lang: Option<LanguageIdentifier>
}
impl Plugin for TranslationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<FluentResourceWrapper>()
            .init_asset_loader::<FluentFileLoader>()
            .add_startup_system(detect_and_load_fluent_files)
        ;

        let default_lang = match &self.default_lang {
            Some(l) => l.clone(),
            None => langid!("en-US")
        };
        app.insert_resource(Locale(default_lang));
    }
}

fn detect_and_load_fluent_files(mut commands: Commands, assets: Res<AssetServer>) {
    let io = assets.asset_io();
    let locales_root = Path::new("locales");
    if io.is_directory(locales_root) {
        debug!("Looking for locales in {}", locales_root.display());
        let directories = match io.read_directory(locales_root) {
            Ok(d) => d,
            Err(e) => {
                error!("Error reading from {}: {}", locales_root.display(), e);
                return;
            }
        };

        let mut r = FluentData::default();
        for directory in directories {
            debug!("Scanning {}", directory.display());
            let lang = directory
                .file_name().expect("Locale directory should have name")
                .to_str().expect("Should be in valid unicode");
            let lang_id: LanguageIdentifier = lang.parse()
                .expect("Directory name should be valid language id");
            let handles = match assets.load_folder(&directory) {
                Ok(t_files) => {
                    t_files.into_iter()
                    .map(|h| h.typed::<FluentResourceWrapper>())
                    .collect::<Vec<_>>()
                }
                Err(e) => {
                    error!("Error reading from directory: {}", e);
                    return;
                }
            };
        
            debug!("Got language {:?} with {} handle(s)", lang, handles.len());
            r.add_resources(lang_id, handles);
        }

        info!("Found {} languages: {:?}", r.langs.len(), r.langs);
        commands.insert_resource(r);
    } else {
        error!("No translations available")
    }
}

#[derive(Default, Debug)]
pub struct FluentData {
    langs: Vec<LanguageIdentifier>,
    files: HashMap<LanguageIdentifier, Vec<Handle<FluentResourceWrapper>>>
}
impl FluentData {
    pub fn add_resources(&mut self, lang: LanguageIdentifier, files: Vec<Handle<FluentResourceWrapper>>) {
        self.langs.push(lang.clone());
        self.files.insert(lang, files);
    }

    pub fn get_handles(&self) -> Vec<Handle<FluentResourceWrapper>> {
        self.files.values()
            .flat_map(|v| v.iter())
            .map(|handle| handle.clone())
            .collect::<Vec<_>>()
    }

    pub fn get_locales(&self) -> &Vec<LanguageIdentifier> {
        &self.langs
    }

    pub fn get_handles_for_locale(&self, locale: &LanguageIdentifier) -> Vec<Handle<FluentResourceWrapper>> {
        match self.files.get(locale) {
            Some(handles) => handles.clone(),
            None => Vec::new()
        }
    }
}

pub struct Locale(pub LanguageIdentifier);

#[derive(SystemParam)]
pub struct Fluent<'w, 's> {
    locale: Res<'w, Locale>,
    data: Res<'w, FluentData>,
    loader: Res<'w, Assets<FluentResourceWrapper>>,
    player: Res<'w, Player>,
    #[allow(dead_code)]
    marker: Query<'w, 's, ()>
}
impl<'w, 's> Fluent<'w, 's> {
    fn get_bundles(&self) -> Option<FluentBundles> {
        let en = langid!("en-US");
        let locale = &self.locale.0;
        // Retrieve a list of working languages. 
        let matching_languages = negotiate_languages(
            &[locale],
            self.data.get_locales().as_slice(),
            Some(&en),
            NegotiationStrategy::Filtering
        );

        let mut bundles = Vec::new();
        for locale in matching_languages {
            let mut bundle = 
                FluentBundle::new(vec![locale.clone()]);

            let handles = self.data.get_handles_for_locale(locale);

            for handle in handles {
                let wrapper = self.loader.get(handle);
                if let Some(FluentResourceWrapper(w)) = wrapper {
                    bundle.add_resource_overriding(w);
                }
            }

            bundles.push(bundle);
        }
        
        Some(FluentBundles { bundles })
    }

    fn build_fluent_args(&self) -> Option<FluentArgs> {
        let mut args = FluentArgs::default();
        args.set("player", self.player.name.as_str());
        Some(args)
    }

    pub fn translate(&self, id: &str) -> Option<String> {
        let bundles = self.get_bundles()?;
        let (bundle, message) = bundles.get_bundle_for_id(id)?;
        let message = message.value()?;
        let mut errors = Vec::new();

        let args = self.build_fluent_args();

        let cow = bundle.format_pattern(
            message, 
            args.as_ref(),
            &mut errors);

        if errors.len() > 0 {
            for error in errors {
                warn!("Parsing error for id {}: {}", id, error);
            }
        }

        Some(cow.to_string())
    }
}

pub struct FluentBundles<'a> {
    bundles: Vec<FluentBundle<&'a FluentResource>>
}
impl<'a> FluentBundles<'a> {
    pub fn get_bundle_for_id(&self, id: &str) -> Option<(&FluentBundle<&FluentResource>, FluentMessage)> {
        for bundle in &self.bundles {
            if let Some(m) = bundle.get_message(id) {
                return Some((bundle, m))
            }
        }
        None
    }
}

#[derive(Deref, DerefMut, TypeUuid)]
#[uuid = "b3116e5c-952e-4771-bec6-8fee9ad49604"]
pub struct FluentResourceWrapper(FluentResource);

#[derive(Default, Debug)]
struct FluentErrorWrapper;
impl fmt::Display for FluentErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Fluent Error")
    }
}
impl Error for FluentErrorWrapper {}

#[derive(Default)]
pub struct FluentFileLoader;
impl AssetLoader for FluentFileLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let content = str::from_utf8(bytes)?;
            let resource = FluentResource::try_new(content.into())
                .map_err(|_| FluentErrorWrapper)?;
            let resource = FluentResourceWrapper(resource);
            load_context.set_default_asset(LoadedAsset::new(resource));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}