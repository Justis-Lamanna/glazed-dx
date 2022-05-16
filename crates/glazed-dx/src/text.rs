use bevy::{prelude::*, utils::HashSet, ecs::system::SystemParam};
use glyph_brush_layout::{ab_glyph::{FontRef, FontArc}, FontId};
use iyes_loopless::prelude::*;

use unicode_segmentation::UnicodeSegmentation;
use std::{ops::Range, time::Duration};

use crate::{controls::PlayerControls, SCREEN_WIDTH, util::despawn};
use crate::{Actions, UI};

const LEFT_MARGIN: f32 = 8.0;
const RIGHT_MARGIN: f32 = 16.0;
const TOP_MARGIN: f32 = 8.0;
const BOTTOM_MARGIN: f32 = 8.0;

pub const FONT: &'static str = "fonts/Font.ttf";
const FONT_BYTES: &'static[u8] = include_bytes!("../assets/fonts/Font.ttf");

const WAITING_1: &'static str = "icons/waiting0.png";
const WAITING_2: &'static str = "icons/waiting1.png";
const WAITING_3: &'static str = "icons/waiting2.png";
const CLOSE_1: &'static str = "icons/close0.png";
const CLOSE_2: &'static str = "icons/close1.png";

#[derive(Component)]
pub struct TextBox(EndOfTextAction);

#[derive(Component)]
pub struct InputMarker(usize, Timer);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TextState {
    None,
    Scrolling,
    WaitingForContinue,
    WaitingForComplete
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum EndOfTextAction {
    CloseOnButton
}
impl Default for EndOfTextAction {
    fn default() -> Self {
        EndOfTextAction::CloseOnButton
    }
}

#[derive(Debug, Clone)]
pub struct TextBoxOptions {
    pub string: String,
    pub width: f32,
    pub lines: usize,
    pub end_of_text_action: EndOfTextAction
}
impl TextBoxOptions {
    pub fn new(string: String) -> Self {
        Self {
            string,
            width: SCREEN_WIDTH,
            lines: 0,
            end_of_text_action: EndOfTextAction::default(),
        }
    }

    pub fn with_max_lines(mut self, count: usize) -> Self {
        self.lines = count;
        self
    }
}
impl Default for TextBoxOptions {
    fn default() -> Self {
        TextBoxOptions {
            string: "".into(),
            width: SCREEN_WIDTH,
            lines: 0,
            end_of_text_action: default()
        }
    }
}
impl From<String> for TextBoxOptions {
    fn from(s: String) -> Self {
        TextBoxOptions {
            string: s,
            ..default()
        }
    }
}
impl From<&str> for TextBoxOptions {
    fn from(s: &str) -> Self {
        TextBoxOptions {
            string: s.to_string(),
            ..default()
        }
    }
}

#[derive(Debug)]
pub struct EndOfText;

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EndOfText>()
            .add_loopless_state(TextState::None)
            .add_exit_system(TextState::WaitingForContinue, despawn::<InputMarker>)
            .add_exit_system(TextState::WaitingForComplete, despawn::<InputMarker>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TextState::Scrolling)
                    .with_system(TextPlugin::load_in_page)
                    .into()
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TextState::WaitingForContinue)
                    .with_system(TextPlugin::load_next_on_button_press)
                    .with_system(TextPlugin::animate_waiting)
                    .into()
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TextState::WaitingForComplete)
                    .with_system(TextPlugin::clear_text_on_button_press)
                    .with_system(TextPlugin::animate_wait_closing)
                    .into()
            )
            ;
    }
}

#[derive(Component)]
pub struct TextBoxState {
    lines: Vec<Page>,
    current_page: usize,
    current_line: usize,
    /// 0 indicates all lines should be visible.
    lines_per_page: usize,
    width: f32
}
impl TextBoxState {
    pub fn from(st: &TextBoxOptions, font: Handle<Font>) -> TextBoxState {
        let rt = Formatter::parse_to_graphemes(st.string.clone());
        let lines = rt.to_box(RichTextOptions {
            box_width: st.width - LEFT_MARGIN - RIGHT_MARGIN,
            font,
            font_size: 16.0,
            default_color: Color::WHITE,
        });

        TextBoxState {
            lines,
            current_line: 0,
            current_page: 0,
            lines_per_page: st.lines,
            width: st.width
        }
    }

    pub fn get_current_page(&self) -> &Page {
        &self.lines[self.current_page]
    }

    pub fn get_current_page_size(&self) -> Size<Val> {
        let current_page = &self.lines[self.current_page];
        let lines = if self.lines_per_page == 0 {
            current_page.lines.len()
        } else {
            let total_lines_left = current_page.number_of_lines() - self.current_line;
            total_lines_left.min(self.lines_per_page)
        };
        Size::new(Val::Px(self.width), Val::Px(((lines + 1) as f32) * 16.0))
    }

    pub fn create_frame_node(&self) -> NodeBundle {
        NodeBundle {
            style: Style {
                size: self.get_current_page_size(),
                ..default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.75).into(),
            ..default()
        }
    }

    pub fn get_current_page_range(&self) -> Range<usize> {
        if self.lines_per_page == 0 {
            0..self.get_current_page().lines.len()
        } else {
            self.current_line..(self.current_line + self.lines_per_page)
        }
    }

    pub fn is_at_end(&self) -> bool {
        if self.lines_per_page == 0 {
            let next_page = self.current_page + 1;
            next_page >= self.lines.len()
        } else {
            let next_line = self.current_line + self.lines_per_page;
            if next_line >= self.get_current_page().lines.len() {
                let next_page = self.current_page + 1;
                next_page >= self.lines.len()
            } else {
                false
            }
        }
    }

    pub fn is_at_beginning(&self) -> bool {
        self.current_line == 0 && self.current_page == 0
    }

    pub fn advance(&mut self) {
        if self.lines_per_page == 0 {
            self.current_page += 1;
        } else {
            self.current_line += self.lines_per_page;
            if self.current_line >= self.get_current_page().lines.len() {
                self.current_line = 0;
                self.current_page += 1;
            }
        }
    }

    pub fn reverse(&mut self) {
        if self.lines_per_page == 0 {
            self.current_page = self.current_page.saturating_sub(1);
        } else {
            if self.current_line < self.lines_per_page {
                self.current_page = self.current_page.saturating_sub(1);
                let total_lines_for_page = self.get_current_page().lines.len();
                let last_chunk_line_size = total_lines_for_page % self.lines_per_page;
                if last_chunk_line_size == 0 {
                    self.current_line = total_lines_for_page - self.lines_per_page;
                } else {
                    self.current_line = total_lines_for_page - last_chunk_line_size;
                }
            } else {
                self.current_line -= self.lines_per_page;
            }
        }
    }
}

#[derive(SystemParam)]
pub struct TextBoxSystem<'w, 's> {
    commands: Commands<'w, 's>,
    assets: Res<'w, AssetServer>,
    query: Query<'w, 's, Entity, With<UI>>
}
impl<'w, 's> TextBoxSystem<'w, 's> {
    pub fn show<T: Into<TextBoxOptions>>(&mut self, text: T) {
        let text = text.into();
        let content = TextBoxState::from(&text, self.assets.load(FONT));
        info!("{}", text.string);
        self.commands.insert_resource(NextState(TextState::Scrolling));

        self.commands
            .entity(self.query.single())
            .with_children(|p| {
                info!("Creating child");
                p.spawn_bundle(content.create_frame_node())
                    .insert(TextBox(text.end_of_text_action))
                    .with_children(|p| {
                        p.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect {
                                    left: Val::Px(LEFT_MARGIN),
                                    right: Val::Px(RIGHT_MARGIN),
                                    top: Val::Px(TOP_MARGIN),
                                    bottom: Val::Px(BOTTOM_MARGIN)
                                },
                                max_size: Size::new(Val::Px(text.width), Val::Auto),
                                ..default()
                            },
                            text: Text { sections: vec![], alignment: default() },
                            ..default()
                        })
                            .insert(content);
                    });
            });
    }
}

impl TextPlugin {
    fn load_in_page(mut commands: Commands, assets: Res<AssetServer>,
                    mut query: Query<(&Parent, &mut Text, &mut TextBoxState)>,
                    mut p_query: Query<&mut Style, With<TextBox>>
    ) {
        info!("Here");
        for (parent, mut text, mut content) in query.iter_mut() {
            let sections = content.get_current_page_range()
                .filter_map(|idx| content.get_current_page().lines.get(idx))
                .flat_map(|section| section.content.iter())
                .map(|v| v.clone())
                .collect::<Vec<_>>();

            let mut style = p_query.get_mut(parent.0).unwrap();
            style.size = content.get_current_page_size();

            text.sections = sections;

            let next_state = if content.is_at_end() {
                commands.entity(parent.0)
                    .with_children(|p| {
                        p.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                                ..default()
                            },
                            image: UiImage(assets.load(CLOSE_1)),
                            ..default()
                        })
                        .insert(InputMarker(1, Timer::from_seconds(1.0, true)));
                    });
                TextState::WaitingForComplete
            } else {
                commands.entity(parent.0)
                    .with_children(|p| {
                        p.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(24.0), Val::Px(16.0)),
                                ..default()
                            },
                            image: UiImage(assets.load(WAITING_1)),
                            ..default()
                        })
                        .insert(InputMarker(1, Timer::from_seconds(1.0, true)));
                    });
                TextState::WaitingForContinue
            };

            commands.insert_resource(NextState(next_state));
        }
    }

    fn load_next_on_button_press(mut commands: Commands, controls: PlayerControls, mut query: Query<&mut TextBoxState>) {
        let controls = controls.single();
        let mut tc = query.single_mut();
        if controls.just_released(Actions::Accept) {
            tc.advance();
            commands.insert_resource(NextState(TextState::Scrolling));
        } else if controls.just_released(Actions::Cancel) {
            if !tc.is_at_beginning() {
                tc.reverse();
                commands.insert_resource(NextState(TextState::Scrolling));  
            }
        }
    }

    fn clear_text_on_button_press(mut commands: Commands, controls: PlayerControls,
                                  query: Query<Entity, With<TextBox>>, mut scroll: Query<&mut TextBoxState>, mut finished: EventWriter<EndOfText>) {
        let controls = controls.single();
        if controls.just_released(Actions::Accept) {
            let id = query.single();
            commands.entity(id).despawn_recursive();
            commands.insert_resource(NextState(TextState::None));
            finished.send(EndOfText);
        } else if controls.just_released(Actions::Cancel) {
            let mut tc = scroll.single_mut();
            if !tc.is_at_beginning() {
                tc.reverse();
                commands.insert_resource(NextState(TextState::Scrolling));
            }
        }
    }

    fn animate_waiting(time: Res<Time>, assets: Res<AssetServer>,
        mut query: Query<(&mut InputMarker, &mut UiImage)>) {
        for (mut marker, mut image) in query.iter_mut() {
            marker.1.tick(time.delta());
            if marker.1.just_finished() {
                let (file, next) = match marker.0 {
                    0 => (WAITING_1, 1),
                    1 => (WAITING_2, 2),
                    _ => (WAITING_3, 0),
                };
                marker.0 = next;
                *image = UiImage(assets.load(file));
            }
        }
    }

    fn animate_wait_closing(time: Res<Time>, assets: Res<AssetServer>,
        mut query: Query<(&mut InputMarker, &mut UiImage)>) {
        for (mut marker, mut image) in query.iter_mut() {
            marker.1.tick(time.delta());
            if marker.1.just_finished() {
                let (file, next) = match marker.0 {
                    0 => (CLOSE_1, 1),
                    _ => (CLOSE_2, 0),
                };
                marker.0 = next;
                *image = UiImage(assets.load(file));
            }
        }
    }
}

pub fn text_box_up(state: CurrentState<TextState>) -> bool {
    matches!(state, CurrentState(TextState::None)) == false
}

// Below this is all the fancy logic for supporting "rich" text. 
struct Formatter;
impl Formatter {
    pub fn parse_to_graphemes<S: ToString>(string: S) -> RichText {
        let string = string.to_string();
        let mut chars = RichText::default();

        let mut color = None;
        let mut escaped = false;

        let mut iter = string.graphemes(true);
        while let Some(g) = iter.next() {
            if escaped {
                 match g {
                    lit @ ("<" | ">") => chars.push(lit, color),
                    "c" => match iter.next() {
                        Some("0") => color = Some(Color::NONE),
                        Some("1") => color = Some(Color::WHITE),
                        Some("2") => color = Some(Color::BLACK),
                        _ => {}
                    },
                    "r" => color = None,
                    _ => {}
                 }
                 escaped = false;
            } else {
                match g {
                    "<" => escaped = true,
                    ">" => color = None,
                    a => chars.push(a, color)
                }
            }
        }
        chars
    }
}

#[derive(Debug, Default)]
struct CharData {
    grapheme: String,
    color: Option<Color>
}
impl CharData {
    pub fn is_section_boundary(&self, next: &CharData) -> bool {
        next.color != self.color
    }
}

#[derive(Default, Debug)]
pub struct RichTextOptions {
    pub box_width: f32,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub default_color: Color
}

#[derive(Default, Debug)]
pub struct Line {
    content: Vec<TextSection>
}
impl Line {
    fn append(&mut self, word: &str) {
        let ts = self.content.last_mut().unwrap();
        ts.value.push_str(word);
    }

    fn next_section(&mut self, section: TextSection) {
        self.content.push(section);
    }
}

#[derive(Default, Debug)]
pub struct Page {
    lines: Vec<Line>
}
impl Page {
    fn last_line(&mut self) -> &mut Line {
        self.lines.last_mut().unwrap()
    }

    fn new_line(&mut self) {
        self.lines.push(Line::default())
    }

    fn number_of_lines(&self) -> usize {
        self.lines.len()
    }
}

#[derive(Default, Debug)]
struct RichText {
    unformatted: String,
    graphemes: Vec<CharData>,
    hard_breaks: HashSet<usize>
}
impl RichText {
    pub fn push<S: ToString>(&mut self, grapheme: S, color: Option<Color>) {
        let grapheme = grapheme.to_string();
        self.unformatted.push_str(grapheme.as_str());
        
        // Hard break is a tricky case.
        // We leave it in the unformatted for calculations, but we don't
        // keep it in the CharData list. 
        if grapheme != "\n" {
            let cd = CharData {
                grapheme,
                color,
            };
            self.graphemes.push(cd);
        } else {
            self.hard_breaks.insert(self.graphemes.len());
        }
    }

    pub fn to_box(&self, options: RichTextOptions) -> Vec<Page> {
        let RichTextOptions { 
            box_width: width, 
            font_size, 
            default_color, 
            font } = options;

        let g_font = FontRef::try_from_slice(FONT_BYTES)
            .unwrap();

        let mut brush = bevy::text::GlyphBrush::default();
        brush.add_font(font.clone(), FontArc::from(g_font));

        let calc_graphemes = brush.compute_glyphs(&[
                glyph_brush_layout::SectionText {
                    text: self.unformatted.as_str(),
                    scale: font_size.into(),
                    font_id: FontId(0),
                }
            ], 
            Size::new(width, f32::INFINITY), 
            TextAlignment::default())
            .unwrap();

        let mut prev_line_no = usize::MAX;
        let mut pages: Vec<Page> = vec![Page::default()];
        let mut last_data: Option<&CharData> = None;
        for (idx, (data, glyph)) in self.graphemes.iter().zip(calc_graphemes.iter()).enumerate() {
            let line_no = glyph.glyph.position.y.trunc() as usize;
            if prev_line_no != line_no {
                if prev_line_no != usize::MAX {
                    if self.hard_breaks.contains(&idx) {
                        // A hard break indicates we want a new page
                        pages.push(Page::default());
                    } else {
                        // A soft break stays on the same page
                        pages.last_mut().unwrap().last_line().append("\n");
                    }
                }
                pages.last_mut().unwrap().new_line();
                last_data = None;
            }

            let is_boundary = if let Some(l) = last_data {
                l.is_section_boundary(data)
            } else {
                true // The first section is always a "boundary"
            };

            if is_boundary {
                pages.last_mut().unwrap().last_line().next_section(TextSection {
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color: data.color.unwrap_or(default_color)
                    },
                    ..default()
                });
            }
            
            pages.last_mut().unwrap().last_line().append(data.grapheme.as_str());

            prev_line_no = line_no;
            last_data = Some(data);
        }

        pages
    }
}