use crate::{App, Plugin};

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(delay::run_timer_check)
            .add_system(text::fuck_this)
            .add_system(text::run_show_text_check)
            .add_system(graphics::run_tween_translate_check)
            .add_system(audio::run_cry_check)
        ;
    }
}

pub mod delay {
    use bevy::prelude::*;
    use std::time::Duration;
    use bevy_sequential_actions::{Action, ActionCommands, EntityCommandsActionsExt, ModifyActionsExt};

    pub struct WaitAction(pub Duration);
    #[derive(Component, Deref, DerefMut)]
    pub struct WaitComponent(Timer);

    impl Action for WaitAction {
        fn start(&mut self, entity: Entity, world: &mut World, _commands: &mut ActionCommands) {
            info!("Starting WaitAction");
            world.entity_mut(entity).insert(WaitComponent(Timer::new(self.0, false)));
        }

        fn remove(&mut self, entity: Entity, world: &mut World) {
            info!("Deleted WaitAction");
            world.entity_mut(entity).remove::<WaitComponent>();
        }

        fn stop(&mut self, entity: Entity, world: &mut World) {
            self.remove(entity, world);
        }
    }

    pub fn run_timer_check(mut wait_q: Query<(Entity, &mut WaitComponent)>, time: Res<Time>, mut commands: Commands) {
        for (entity, mut wait) in wait_q.iter_mut() {
            wait.tick(time.delta());
            if wait.just_finished() {
                info!("Timer complete, advancing");
                commands.action(entity).next();
            }
        }
    }
}

pub mod text {
    use bevy::prelude::*;
    use bevy::ecs::system::SystemState;
    use bevy_sequential_actions::*;
    use crate::{Commands, Entity, World};
    use crate::text::{EndOfText, TextBoxOptions, TextBoxSystem};

    #[derive(Component, Clone)]
    pub struct ShowTextAction(pub TextBoxOptions);
    #[derive(Component)]
    pub struct WaitForText;

    impl Action for ShowTextAction {
        fn start(&mut self, entity: Entity, mut world: &mut World, _commands: &mut ActionCommands) {
            info!("Starting ShowTextAction");
            world.entity_mut(entity)
                .insert(self.clone())
                .insert(WaitForText);
        }

        fn remove(&mut self, entity: Entity, world: &mut World) {
            world.entity_mut(entity).remove::<WaitForText>();
        }

        fn stop(&mut self, entity: Entity, world: &mut World) {
            self.remove(entity, world);
        }
    }

    pub fn fuck_this(mut commands: Commands, mut text: TextBoxSystem, query: Query<(Entity, &ShowTextAction)>) {
        for (entity, action) in query.iter() {
            text.show(action.0.clone());
            commands.entity(entity).remove::<ShowTextAction>();
        }
    }

    pub fn run_show_text_check(mut commands: Commands, query: Query<Entity, With<WaitForText>>, mut reader: EventReader<EndOfText>) {
        for entity in query.iter() {
            if let Some(_) = reader.iter().last() {
                info!("Text complete, advancing");
                commands.action(entity).next();
            }
        }
    }
}

pub mod graphics {
    use std::marker::PhantomData;
    use std::time::Duration;
    use bevy::prelude::*;
    use bevy_sequential_actions::*;
    use bevy_tweening::{Animator, EaseMethod, Tween, TweeningType};
    use bevy_tweening::lens::TransformPositionLens;

    pub struct ShowSprite<T: Component> {
        pub visible: bool,
        m: PhantomData<T>
    }
    impl<T: Component> ShowSprite<T> {
        pub fn new(visible: bool) -> Self {
            Self {
                visible,
                m: PhantomData
            }
        }
    }
    impl<T: Component> Action for ShowSprite<T> {
        fn start(&mut self, entity: Entity, world: &mut World, commands: &mut ActionCommands) {
            info!("Starting ShowSprite");
            let mut l = world.query_filtered::<&mut Visibility, With<T>>();
            for mut i in l.iter_mut(world) {
                i.is_visible = self.visible
            }
            info!("Show Sprite complete, advancing");
            commands.action(entity).next();
        }

        fn remove(&mut self, _entity: Entity, _world: &mut World) { }

        fn stop(&mut self, _entity: Entity, _world: &mut World) { }
    }

    pub struct ChangeFrame<T: Component> {
        pub idx: usize,
        m: PhantomData<T>
    }
    impl<T: Component> ChangeFrame<T> {
        pub fn new(idx: usize) -> Self {
            Self {
                idx,
                m: PhantomData
            }
        }
    }
    impl<T: Component> Action for ChangeFrame<T> {
        fn start(&mut self, entity: Entity, world: &mut World, commands: &mut ActionCommands) {
            info!("Starting ChangeFrame");
            let mut l = world.query_filtered::<&mut TextureAtlasSprite, With<T>>();
            for mut i in l.iter_mut(world) {
                i.index = self.idx;
            }
            info!("Change Frame complete, advancing");
            commands.action(entity).next();
        }

        fn remove(&mut self, _entity: Entity, _world: &mut World) {

        }

        fn stop(&mut self, _entity: Entity, _world: &mut World) {

        }
    }

    pub struct TweenTranslate<ID: Component> {
        start: Vec3,
        end: Vec3,
        duration: Duration,
        marker: PhantomData<ID>
    }
    #[derive(Component)]
    pub struct WatchForTweenTranslateEnd(Entity);
    impl<ID: Component> TweenTranslate<ID> {
        pub fn new(start: Vec3, end: Vec3, duration: Duration) -> Self {
            Self {
                start, end, duration, marker: PhantomData
            }
        }
    }
    impl<ID: Component> Action for TweenTranslate<ID> {
        fn start(&mut self, entity: Entity, world: &mut World, _commands: &mut ActionCommands) {
            info!("Starting TweenTranslate");
            let entities = world.query_filtered::<Entity, With<ID>>().iter(world).collect::<Vec<_>>();
            for a_entity in entities {
                let tween = Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    self.duration,
                    TransformPositionLens {
                        start: self.start,
                        end: self.end
                    }
                );

                world.entity_mut(a_entity)
                    .insert(Animator::new(tween))
                    .insert(WatchForTweenTranslateEnd(entity));
            }
        }

        fn remove(&mut self, _entity: Entity, _world: &mut World) { }

        fn stop(&mut self, _entity: Entity, _world: &mut World) { }
    }

    pub fn run_tween_translate_check(mut commands: Commands, query: Query<(Entity, &Animator<Transform>, &WatchForTweenTranslateEnd)>) {
        if let Some((entity, a, end)) = query.iter().last() {
            if a.progress() == 1.0 {
                commands.entity(entity)
                    .remove::<Animator<Transform>>()
                    .remove::<WatchForTweenTranslateEnd>();
                let WatchForTweenTranslateEnd(action_entity) = end;
                info!("TweenTranslate complete, advancing");
                commands.action(*action_entity).next();
            }
        }
    }
}

pub mod audio {
    use bevy::ecs::system::SystemState;
    use bevy::prelude::*;
    use bevy_sequential_actions::*;
    use glazed_data::species::Species;
    use crate::Cry;

    pub struct PlayCry(pub Species);
    #[derive(Component)]
    pub struct WaitForCry;

    impl Action for PlayCry {
        fn start(&mut self, entity: Entity, world: &mut World, _commands: &mut ActionCommands) {
            info!("Starting PlayCry");
            let mut audio: SystemState<(Cry)> = SystemState::new(world);
            let mut audio = audio.get_mut(world);
            audio.play_cry(self.0);

            world.entity_mut(entity).insert(WaitForCry);
        }

        fn remove(&mut self, entity: Entity, world: &mut World) {
            world.entity_mut(entity).remove::<WaitForCry>();
        }

        fn stop(&mut self, entity: Entity, world: &mut World) {
            self.remove(entity, world);
        }
    }

    pub fn run_cry_check(mut commands: Commands, cry: Cry, query: Query<Entity, With<WaitForCry>>) {
        if cry.is_cry_complete() {
            for entity in query.iter() {
                info!("PlayCry complete, advancing");
                commands.action(entity).next();
            }
        }
    }
}