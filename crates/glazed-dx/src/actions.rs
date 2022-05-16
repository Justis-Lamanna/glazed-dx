use crate::{App, Plugin};

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(delay::run)
            .add_system(text::fuck_this)
            .add_system(text::run)
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
            world.entity_mut(entity).insert(WaitComponent(Timer::new(self.0, false)));
        }

        fn remove(&mut self, entity: Entity, world: &mut World) {
            world.entity_mut(entity).remove::<WaitComponent>();
        }

        fn stop(&mut self, entity: Entity, world: &mut World) {
            self.remove(entity, world);
        }
    }

    pub fn run(mut wait_q: Query<(Entity, &mut WaitComponent)>, time: Res<Time>, mut commands: Commands) {
        for (entity, mut wait) in wait_q.iter_mut() {
            wait.tick(time.delta());
            if wait.just_finished() {
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

    pub fn run(mut commands: Commands, query: Query<Entity, With<WaitForText>>, mut reader: EventReader<EndOfText>) {
        for entity in query.iter() {
            if let Some(_) = reader.iter().last() {
                commands.action(entity).next();
            }
        }
    }
}

pub mod graphics {
    use std::marker::PhantomData;
    use bevy::prelude::*;
    use bevy_sequential_actions::*;

    pub struct ChangeFrame<T: Component> {
        pub idx: usize,
        pub m: PhantomData<T>
    }
    impl<T: Component> ChangeFrame<T> {
        pub fn new(idx: usize) -> Self {
            Self {
                idx,
                m: PhantomData::default()
            }
        }
    }

    impl<T: Component> Action for ChangeFrame<T> {
        fn start(&mut self, entity: Entity, world: &mut World, commands: &mut ActionCommands) {
            let mut l = world.query_filtered::<&mut TextureAtlasSprite, With<T>>();
            for mut i in l.iter_mut(world) {
                i.index = self.idx;
            }
            commands.action(entity).next();
        }

        fn remove(&mut self, entity: Entity, world: &mut World) {

        }

        fn stop(&mut self, entity: Entity, world: &mut World) {

        }
    }
}