use std::collections::{HashMap, VecDeque};
use std::ops::RangeBounds;
use std::time::Duration;
use bevy::prelude::*;

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
    Complete
}

/// Encapsulates an animation and its current state
#[derive(Debug, Component)]
pub struct SSAnimation {
    points: HashMap<PointId, usize>,
    frames: Vec<AnimationStep>,
    counter: usize,
    current_step: usize,
    current_frame: usize,
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
            current_frame: 0,
            time_left: Duration::ZERO,
            complete: false
        }
    }

    /// Advances the script to the next state
    /// Execution will advance until we hit a blocking step (Waiting or Complete).
    pub fn advance(&mut self) {
        // Advance continues until we hit Complete or Wait.
        loop {
            if self.current_step >= self.frames.len() {
                // If we've stepped outside of the list , we consider that an implicit complete.
                self.complete = true;
                return;
            }
            let step = &self.frames[self.current_step];
            match step {
                AnimationStep::Wait(duration) => {
                    self.time_left = *duration;
                    self.current_step += 1;
                    return;
                }
                AnimationStep::Point(id) => {
                    self.points.insert(*id, self.current_step);
                    self.current_step += 1;
                }
                AnimationStep::AdvanceTo(frame) => {
                    self.current_frame = *frame;
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
                    return;
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

    /// The current loop should loop n times
    /// Specifically, this places:
    /// * A SetCounter to n, followed by a Point
    /// * A JumpWhileCounter at the end.
    pub fn loop_n_times(mut self, n: usize) -> Self {
        let p = self.get_next_point();
        self.frames.push_front(AnimationStep::Point(p));
        self.frames.push_front(AnimationStep::SetCounter(n));
        self.frames.push_back(AnimationStep::JumpWhileCounter(p));
        self
    }

    /// Appends further frames at the end of the animation
    pub fn then_from_iter<F: IntoIterator<Item=usize>>(mut self, iter: F, duration: Duration) -> Self {
        for frame in iter.into_iter() {
            self.frames.push_back(AnimationStep::AdvanceTo(frame));
            self.frames.push_back(AnimationStep::Wait(duration));
        }
        self
    }
}

fn animate (time: Res<'_, Time>, mut animations: Query<(&mut TextureAtlasSprite, &mut SSAnimation)>) {
    for (mut sprite, mut animation) in animations.iter_mut() {
        if animation.update(time.delta()) {
            animation.advance();
            sprite.index = animation.current_frame;
        }
    }
}

pub struct GlazedAnimator;
impl Plugin for GlazedAnimator {
    fn build(&self, app: &mut App) {
        app.add_system(animate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_advance() {
        let animation = vec![AnimationStep::AdvanceTo(1)];
        let mut animation = SSAnimation::from_vec(animation);
        animation.advance();

        assert_eq!(1, animation.current_frame);
    }

    #[test]
    pub fn test_advance_two() {
        let animation = vec![AnimationStep::AdvanceTo(1), AnimationStep::Wait(Duration::ZERO), AnimationStep::AdvanceTo(2)];
        let mut animation = SSAnimation::from_vec(animation);

        animation.advance();
        assert_eq!(1, animation.current_frame);

        animation.advance();
        assert_eq!(2, animation.current_frame);
    }

    #[test]
    pub fn test_advance_implicit_complete() {
        let animation = vec![AnimationStep::AdvanceTo(1)];
        let mut animation = SSAnimation::from_vec(animation);
        animation.advance();

        assert!(animation.complete);
    }
}
