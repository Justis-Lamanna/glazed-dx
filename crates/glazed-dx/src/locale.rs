use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::str;

use bevy::ecs::system::SystemParam;
use bevy::reflect::TypeUuid;
use bevy::{prelude::*, asset::LoadedAsset};
use bevy::asset::AssetLoader;

use fluent::{FluentBundle, FluentResource, FluentMessage, FluentArgs, FluentValue};
use unic_langid::{LanguageIdentifier, langid};
use fluent_langneg::negotiate_languages;
use fluent_langneg::NegotiationStrategy;

use crate::player::Player;

/// Plugin to initialize Translation data.
/// This will give Bevy the ability to read Fluent files (.ftl),
/// and initialize Fluent files on startup. 
/// 
/// Fluent files should be present in the locales/<locale code> directory
/// wherever you keep your assets. There can be as many .ftl files in this
/// directory, but no subdirectories or other files (yet).
/// 
/// Translations from a more specific locale will "override" those of a more
/// general locale. Conversely, if a more specific locale does not have the provided
/// message key, it will "fall back" to less general locales. This is done as intelligently
/// as possible, using the English translations if no other locales have it.
#[derive(Default)]
pub struct TranslationPlugin {
    /// The default language to use. If absent, English is used.
    pub default_lang: Option<LanguageIdentifier>
}
impl Plugin for TranslationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<FluentResourceWrapper>()
            .init_asset_loader::<FluentFileLoader>()
            .add_startup_system(detect_and_load_fluent_files)
            .insert_resource(FluentParams::default())
        ;

        let default_lang = match &self.default_lang {
            Some(l) => l.clone(),
            None => langid!("en-US")
        };
        app.insert_resource(Locale(default_lang));
    }
}

/// Scan the locales/ directory for Fluent files, and compile their handles
/// into a single resource (FluentData)
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

/// A resource which holds all supported languages, and all the handles for each translation
/// file.
#[derive(Default, Debug)]
pub struct FluentData {
    langs: Vec<LanguageIdentifier>,
    files: HashMap<LanguageIdentifier, Vec<Handle<FluentResourceWrapper>>>
}
impl FluentData {
    /// Add the resources to support a language
    pub fn add_resources(&mut self, lang: LanguageIdentifier, files: Vec<Handle<FluentResourceWrapper>>) {
        self.langs.push(lang.clone());
        self.files.insert(lang, files);
    }

    /// Get handles for all Fluent files. Used for preloading
    pub fn get_handles(&self) -> Vec<Handle<FluentResourceWrapper>> {
        self.files.values()
            .flat_map(|v| v.iter())
            .map(|handle| handle.clone())
            .collect::<Vec<_>>()
    }

    /// Get all the supported locales
    pub fn get_locales(&self) -> &Vec<LanguageIdentifier> {
        &self.langs
    }

    /// Get handles for all Fluent files of a specific locale.
    pub fn get_handles_for_locale(&self, locale: &LanguageIdentifier) -> Vec<Handle<FluentResourceWrapper>> {
        match self.files.get(locale) {
            Some(handles) => handles.clone(),
            None => Vec::new()
        }
    }
}

/// A resource which holds the current player's locale.
pub struct Locale(pub LanguageIdentifier);

/// A resource which contains dynamic parameters for Fluent translations
#[derive(Default, Debug, Deref, DerefMut)]
pub struct FluentParams {
    params: HashMap<String, Param>
}

#[derive(Debug)]
pub enum Param {
    PString(String)
}
impl<'a> From<&Param> for FluentValue<'a> {
    fn from(p: &Param) -> Self {
        match p {
            Param::PString(s) => s.clone().into()
        }
    }
}

/// The Fluent orchestrator
/// This performs all the work to do a translation.
/// More specifically, this does the following:
/// 1. Get the current locale via the Locale resource
/// 2. Calculate the fallback path for that locale
/// 3. Resolve all translation files for all locales in the fallback path
/// 4. Search for the Fluent expression across all files, respecting the fallback path
/// 5. Inject specific resources (i.e. Player name) as arguments into the expression
/// 6. Resolve the expression into a string.
#[derive(SystemParam)]
pub struct Fluent<'w, 's> {
    locale: Res<'w, Locale>,
    data: Res<'w, FluentData>,
    loader: Res<'w, Assets<FluentResourceWrapper>>,
    player: Res<'w, Player>,
    params: ResMut<'w, FluentParams>,
    #[allow(dead_code)]
    marker: Query<'w, 's, ()>
}
impl<'w, 's> Fluent<'w, 's> {
    /// Retrieve a FluentBundles object, which represents the fallback order
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

            bundle.set_use_isolating(false);

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

    /// Create the FluentArgs object from other resources
    fn build_fluent_args(&self) -> Option<FluentArgs> {
        let mut args = FluentArgs::default();
        args.set("player", self.player.name.as_str());

        for (key, value) in self.params.params.iter() {
            args.set(key, FluentValue::from(value))
        }

        println!("{:?}", args);
        Some(args)
    }

    /// The magic method which resolves a key into a string.
    /// This method returns None if no bundles in the fallback chain have the provided key.
    /// One day, I may make it so None to allow for more lazy loading.
    /// Otherwise, the fully translated string is returned.
    /// 
    /// If there is a parsing error, the errors are printed to console, but otherwise
    /// do not affect what is returned.
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

    /// Add a string to the parameter buffer.
    /// Implementations for other specific items (for example, Species name)
    /// are done in their respective classes.
    pub fn buffer_string<T: ToString, U: ToString>(&mut self, key: T, value: U) {
        let key = key.to_string();
        let value = value.to_string();
        self.params.params.insert(key, Param::PString(value));
    }

    /// Scrub the entire buffer
    pub fn clear_buffer(&mut self) {
        self.params.params.clear();
    }
}

/// Helper component to search a list of FluentBundles for the Bundle with the specified key.
struct FluentBundles<'a> {
    bundles: Vec<FluentBundle<&'a FluentResource>>
}
impl<'a> FluentBundles<'a> {
    /// Get the bundle and message for a specific ID, if one is found.
    pub fn get_bundle_for_id(&self, id: &str) -> Option<(&FluentBundle<&FluentResource>, FluentMessage)> {
        for bundle in &self.bundles {
            if let Some(m) = bundle.get_message(id) {
                return Some((bundle, m))
            }
        }
        None
    }
}

/// A newtype which allows for a FluentResource to be read as an asset.
#[derive(Deref, DerefMut, TypeUuid)]
#[uuid = "b3116e5c-952e-4771-bec6-8fee9ad49604"]
pub struct FluentResourceWrapper(FluentResource);

/// A newtype which allows for compatibility between anyhow and Fluent's error message
#[derive(Default, Debug)]
struct FluentErrorWrapper;
impl fmt::Display for FluentErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Fluent Error")
    }
}
impl Error for FluentErrorWrapper {}

/// An assetloader which parses a file into a Fluent Resource.
/// Failure occurs if the file is not in UTF-8, or could not be parsed into a Resource.
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
                .map_err(|(_, e)| {
                    error!("{:?}", e);
                    FluentErrorWrapper
                })?;
            let resource = FluentResourceWrapper(resource);
            load_context.set_default_asset(LoadedAsset::new(resource));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}