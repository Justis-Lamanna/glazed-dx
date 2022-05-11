use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::Lens;

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

/// Describes some form of curve that can be interpolated
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BezierCurve {
    /// A linear "curve" which smoothly travels between two points
    Linear(Vec2, Vec2),
    /// A quadratic curve which travels from p0 to p2.
    /// Being a Bezier curve, the curve does not necessarily hit p1. Instead, this point
    /// determines the direction and sharpness of the curve. 
    /// Formally, for any parameter T, the output point is equal to:
    /// lerp(lerp(p0, p1, T), lerp(p1, p2, T), T)
    /// Where lerp uses the same formula as the Linear curve
    Quadratic(Vec2, Vec2, Vec2),
    /// A cubic curve which travels from p0 to p3.
    /// Being a Bezier curve, the curve does not necessarily hit p1 or p2. Instead, these
    /// points determine the direction and sharpness of the curve.
    /// Formally, for any parameter T, the output point is equal to:
    /// lerp3(lerp3(p0, p1, p2, T), lerp3(p1, p2, p3, T), lerp3(p2, p3, p4, T), T)
    /// Where lerp3 uses the same formula as the Quadratic curve
    Cubic(Vec2, Vec2, Vec2, Vec2)
}
impl BezierCurve {
    pub fn lerp(&self, ratio: f32) -> Vec2 {
        match self {
            BezierCurve::Linear(start, end) => {
                Self::bound(ratio, *start, *end, || start.lerp(*end, ratio))
            },
            BezierCurve::Quadratic(start, middle, end) => {
                Self::bound(ratio, *start, *end, 
                    || Self::lerp_three(*start, *middle, *end, ratio))
            },
            BezierCurve::Cubic(one, two, three, four) => {
                Self::bound(ratio, *one, *four, 
                || Self::lerp_three(
                    one.lerp(*two, ratio),
                    two.lerp(*three, ratio),
                    three.lerp(*four, ratio),
                    ratio
                ))
            }
        }
    }

    pub fn start(&self) -> Vec2 {
        match self {
            BezierCurve::Linear(start, _) | 
            BezierCurve::Quadratic(start, _, _) | 
            BezierCurve::Cubic(start, _, _, _) => *start
        }
    }

    pub fn end(&self) -> Vec2 {
        match self {
            BezierCurve::Linear(_, end) | 
            BezierCurve::Quadratic(_, _, end) | 
            BezierCurve::Cubic(_, _, _, end) => *end
        }
    }

    pub fn path(self) -> Path {
        Path {
            path: vec![self]
        }
    }

    fn bound<T, F: Fn() -> T>(ratio: f32, lb: T, ub: T, mid: F) -> T {
        if ratio <= 0.0 { lb }
        else if ratio >= 1.0 { ub }
        else { mid() }
    }

    fn lerp_three(one: Vec2, two: Vec2, three: Vec2, ratio: f32) -> Vec2 {
        let one = one.lerp(two, ratio);
        let two = two.lerp(three, ratio);
        one.lerp(two, ratio)
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    path: Vec<BezierCurve>
}
impl Path {
    pub fn then_linear(mut self, to: Vec2) -> Path {
        let p = self.path.last().unwrap().end();
        self.path.push(BezierCurve::Linear(p, to));
        self
    }

    pub fn then_quadratic(mut self, mid: Vec2, to: Vec2) -> Path {
        let p = self.path.last().unwrap().end();
        self.path.push(BezierCurve::Quadratic(p, mid, to));
        self
    }

    pub fn then_cubic(mut self, mid1: Vec2, mid2: Vec2, to: Vec2) -> Path {
        let p = self.path.last().unwrap().end();
        self.path.push(BezierCurve::Cubic(p, mid1, mid2, to));
        self
    }

    // To do: then_* functions don't enforce smoothness. We should be able to smartly
    // determine the mid or mid1 parameters to create a smooth path, but
    // my brain is too tired right now.

    pub fn lerp(&self, ratio: f32) -> Vec2 {
        if ratio <= 0.0 { self.path.first().unwrap().start() }
        else if ratio >= 1.0 { self.path.last().unwrap().end() }
        else {
            let multiply = ratio * (self.path.len() as f32);
            let index = multiply.trunc();
            let inner_ratio = multiply - index;

            // Math makes it so index must be [0,path length)
            // inner_ratio must be [0, 1)
            self.path[index as usize].lerp(inner_ratio)
        }
    }
}
impl From<BezierCurve> for Path {
    fn from(b: BezierCurve) -> Self {
        b.path()
    }
}

impl Lens<Transform> for Path {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let point = Path::lerp(&self, ratio);
        target.translation.x = point.x;
        target.translation.y = point.y;
    }
}