use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use bevy::prelude::*;

pub struct GlazedAnimator;
impl Plugin for GlazedAnimator {
    fn build(&self, app: &mut App) {
        app
            .add_system(run_ssanimation_systems)
        ;
    }
}

type PointId = usize;

/// Describe a complex set of steps for an animation
#[derive(Debug)]
pub enum AnimationStep {
    /// Wait for a duration of time
    Wait(Duration),
    /// Register a "point" that can be jumped to
    Point(PointId),
    /// Change the frame to a new frame
    AdvanceTo(usize),
    /// Jump to a "point" registered with Point
    JumpToPoint(PointId),
    /// Set the counter.
    SetCounter(usize),
    /// Decrements the counter by one. If counter > 0, jumps to PointId, otherwise continues.
    JumpWhileCounter(PointId),
    /// The animation is complete.
    Complete,
    /// The sprite should be shown/hidden
    Visible(bool)
}

enum TransformativeAction {
    SetFrame(usize),
    SetVisibility(bool)
}

/// Encapsulates an animation and its current state
#[derive(Debug, Component)]
pub struct SSAnimation {
    points: HashMap<PointId, usize>,
    frames: Vec<AnimationStep>,
    counter: usize,
    current_step: usize,
    time_left: Duration,
    complete: bool
}

impl SSAnimation {
    /// Create an animation from a sequence of steps
    /// Consider using SSAnimationBuilder to help create steps effortlessly.
    pub fn from_vec(v: Vec<AnimationStep>) -> SSAnimation {
        SSAnimation {
            points: HashMap::new(),
            frames: v,
            counter: 0,
            current_step: 0,
            time_left: Duration::ZERO,
            complete: false
        }
    }

    /// Advances the script to the next state
    /// Execution will advance until we hit a blocking step (Waiting or Complete).
    fn advance(&mut self) -> Vec<TransformativeAction> {
        let mut actions = Vec::new();
        // Advance continues until we hit Complete or Wait.
        loop {
            if self.current_step >= self.frames.len() {
                // If we've stepped outside of the list , we consider that an implicit complete.
                self.complete = true;
                return actions;
            }
            let step = &self.frames[self.current_step];
            match step {
                AnimationStep::Wait(duration) => {
                    self.time_left = *duration;
                    self.current_step += 1;
                    return actions;
                }
                AnimationStep::Point(id) => {
                    self.points.insert(*id, self.current_step);
                    self.current_step += 1;
                }
                AnimationStep::AdvanceTo(frame) => {
                    actions.push(TransformativeAction::SetFrame(*frame));
                    self.current_step += 1;
                }
                AnimationStep::JumpToPoint(point) => {
                    self.current_step = self.points[point] + 1;
                }
                AnimationStep::SetCounter(ctr) => {
                    self.counter = *ctr;
                    self.current_step += 1;
                }
                AnimationStep::JumpWhileCounter(point) => {
                    self.counter = self.counter.saturating_sub(1);
                    if self.counter == 0 {
                        self.current_step += 1;
                    } else {
                        self.current_step = self.points[point] + 1;
                    }
                }
                AnimationStep::Complete => {
                    self.complete = true;
                    return actions;
                }
                AnimationStep::Visible(visibility) => {
                    actions.push(TransformativeAction::SetVisibility(*visibility));
                    self.current_step += 1;
                }
            }
        }
    }

    /// Tick the animation counter. Returns true if ready to advance the script
    pub fn update(&mut self, tick: Duration) -> bool {
        if self.time_left > Duration::ZERO {
            self.time_left = self.time_left.saturating_sub(tick);
        }

        self.time_left <= Duration::ZERO
    }
}

/// Builder class to handle common animation functionality.
pub struct SSAnimationBuilder {
    next_point: usize,
    frames: VecDeque<AnimationStep>
}
impl SSAnimationBuilder {
    /// Create a builder from a sequence of Animation Steps
    pub fn from_vec(v: Vec<AnimationStep>) -> Self {
        SSAnimationBuilder {
            next_point: 0,
            frames: VecDeque::from(v)
        }
    }

    /// Create a builder from a sequence of frames and a duration
    pub fn from_iter<R: IntoIterator<Item=usize>>(iter: R, duration: Duration) -> Self {
        let mut v = Vec::new();
        for frame in iter.into_iter() {
            v.push(AnimationStep::AdvanceTo(frame));
            v.push(AnimationStep::Wait(duration));
        }
        Self::from_vec(v)
    }

    /// Build this into an SSAnimation for attaching
    pub fn build(self) -> SSAnimation {
        SSAnimation::from_vec(self.frames.into())
    }

    fn get_next_point(&mut self) -> usize {
        let r = self.next_point;
        self.next_point = self.next_point + 1;
        r
    }

    /// The current loop should loop forever
    /// Specifically, places a Point at the front of the animation, and a JumpToPoint
    /// at the end.
    pub fn loop_forever(mut self) -> Self {
        let p = self.get_next_point();
        self.frames.push_front(AnimationStep::Point(p));
        self.frames.push_back(AnimationStep::JumpToPoint(p));
        self
    }
}
impl Into<SSAnimation> for SSAnimationBuilder {
    fn into(self) -> SSAnimation {
        self.build()
    }
}

type SpriteBundle<'a> = (Option<&'a mut TextureAtlasSprite>, Option<&'a mut Visibility>);

fn run_ssanimation_systems(time: Res<'_, Time>, mut animations: Query<(&mut SSAnimation, SpriteBundle)>) {
    for (mut animation, (mut sprite, mut visibility)) in animations.iter_mut() {
        if animation.update(time.delta()) {
            let transformations = animation.advance();

            for transformation in transformations {
                match transformation {
                    TransformativeAction::SetFrame(frame) => {
                        if let Some(s) = sprite.as_mut() {
                            s.index = frame;
                        }
                    }
                    TransformativeAction::SetVisibility(visible) => {
                        if let Some(v) = visibility.as_mut() {
                            v.is_visible = visible;
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

/// A more basic timeline which simply iterates down a list of durations
#[derive(Component)]
pub struct Timeline {
    timeline: Vec<Duration>,
    index: usize,
    time_left: Duration,
    complete: bool
}
impl Timeline {
    pub fn new(timeline: Vec<Duration>) -> Timeline {
        let empty = timeline.is_empty();
        let first_duration = *timeline.get(0).unwrap_or(&Duration::MAX);
        Timeline {
            timeline,
            index: 0,
            time_left: first_duration,
            complete: empty
        }
    }

    pub fn from_iter<F: IntoIterator<Item=G>, G: Into<u64>>(timeline: F) -> Timeline {
        let t = timeline.into_iter()
            .map(|i| Duration::from_millis(i.into()))
            .collect::<Vec<Duration>>();
        Self::new(t)
    }

    pub fn total_time(&self) -> Duration {
        self.timeline.iter()
            .sum()
    }

    pub fn update(&mut self, tick: Duration) -> bool {
        if self.time_left == Duration::MAX {
            return false;
        }

        if self.time_left > Duration::ZERO {
            self.time_left = self.time_left.saturating_sub(tick);
        }

        self.time_left <= Duration::ZERO
    }

    pub fn advance(&mut self) -> usize {
        self.index = self.index + 1;
        self.time_left = match self.timeline.get(self.index) {
            Some(d) => *d,
            None => {
                self.complete = true;
                Duration::MAX
            }
        };
        self.index
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

#[derive(Component)]
pub struct DespawnOnStateChange;

pub fn despawn<T: Component>(mut commands: Commands, marked: Query<Entity, With<T>>) {
    marked.for_each(|e| commands.entity(e).despawn_recursive())
}