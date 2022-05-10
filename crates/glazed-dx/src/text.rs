use std::ops::Range;

use bevy::prelude::*;
use glyph_brush_layout::{ab_glyph::{FontRef, FontArc}, FontId};
use iyes_loopless::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::controls::PlayerControls;
use crate::{Actions, UI};

const LEFT_MARGIN: f32 = 8.0;
const RIGHT_MARGIN: f32 = 16.0;
const TOP_MARGIN: f32 = 8.0;
const BOTTOM_MARGIN: f32 = 8.0;

#[derive(Component)]
pub struct TextBox;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TextState {
    None,
    Scrolling,
    WaitingForContinue,
    WaitingForComplete
}

#[derive(Default, Debug)]
pub struct ShowText {
    pub string: String,
    pub width: f32,
    pub lines: usize,
    pub font: Handle<Font>
}

#[derive(Component)]
pub struct TextBoxContent {
    lines: Vec<Vec<TextSection>>,
    current_line: usize,
    /// 0 indicates all lines should be visible.
    lines_per_page: usize,
    width: f32
}
impl TextBoxContent {
    pub fn from(st: &ShowText) -> TextBoxContent {
        let rt = Formatter::parse_to_graphemes(st.string.clone());
        let lines = rt.to_box(RichTextOptions {
            box_width: st.width - LEFT_MARGIN - RIGHT_MARGIN,
            font: st.font.clone(),
            font_size: 16.0,
            default_color: Color::WHITE,
        });

        let lines_per_page = if st.lines == 0 {
            lines.len()
        } else {
            st.lines
        };

        TextBoxContent {
            lines,
            current_line: 0,
            lines_per_page,
            width: st.width
        }
    }

    pub fn get_current_page_size(&self) -> Size<Val> {
        let total_lines_left = self.lines.len() - self.current_line;
        let lines = total_lines_left.min(self.lines_per_page);
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
        self.current_line..(self.current_line + self.lines_per_page)
    }

    pub fn is_at_end(&self) -> bool {
        self.current_line >= self.lines.len()
    }
}

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShowText>()
            .add_loopless_state(TextState::None)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TextState::None)
                    .with_system(TextPlugin::create_text_frame)
                    .into()
            )
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
                    .into()
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TextState::WaitingForComplete)
                    .with_system(TextPlugin::clear_text_on_button_press)
                    .into()
            )
            ;
    }
}

impl TextPlugin {
    fn create_text_frame(mut commands: Commands, mut events: EventReader<ShowText>, query: Query<Entity, With<UI>>) {
        if let Some(e) = events.iter().last() {
            let e = TextBoxContent::from(e);
            commands.insert_resource(NextState(TextState::Scrolling));

            commands
                .entity(query.single())
                .with_children(|p| {
                    p.spawn_bundle(e.create_frame_node()).insert(TextBox)
                    .with_children(|p| {
                        p.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect {
                                    left: Val::Px(LEFT_MARGIN),
                                    right: Val::Px(RIGHT_MARGIN),
                                    top: Val::Px(TOP_MARGIN),
                                    bottom: Val::Px(BOTTOM_MARGIN)
                                },
                                max_size: Size::new(Val::Px(e.width), Val::Auto),
                                ..default()
                            },
                            text: Text { sections: vec![], alignment: default() },
                            ..default()
                        })
                        .insert(e);
                    });
                });
        }
    }

    fn load_in_page(mut commands: Commands, 
        mut query: Query<(&Parent, &mut Text, &mut TextBoxContent)>,
        mut p_query: Query<&mut Style, With<TextBox>>
    ) {
        for (parent, mut text, mut content) in query.iter_mut() {
            let sections = content.get_current_page_range()
                .filter_map(|idx| content.lines.get(idx))
                .flat_map(|section| section.iter())
                .map(|v| v.clone())
                .collect::<Vec<_>>();

            let mut style = p_query.get_mut(parent.0).unwrap();
            style.size = content.get_current_page_size();

            text.sections = sections;
            content.current_line += content.lines_per_page;

            let next_state = if content.is_at_end() {
                TextState::WaitingForComplete
            } else {
                TextState::WaitingForContinue
            };

            commands.insert_resource(NextState(next_state));
        }
    }

    fn load_next_on_button_press(mut commands: Commands, controls: PlayerControls) {
        if controls.single().just_released(Actions::Accept) {
            commands.insert_resource(NextState(TextState::Scrolling));
        }
    }

    fn clear_text_on_button_press(mut commands: Commands, controls: PlayerControls, query: Query<Entity, With<TextBox>>) {
        if controls.single().just_released(Actions::Accept) {
            let id = query.single();
            commands.entity(id).despawn_recursive();
            commands.insert_resource(NextState(TextState::None));
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
struct RichText {
    unformatted: String,
    graphemes: Vec<CharData>
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
        }   
    }

    pub fn to_box(&self, options: RichTextOptions) -> Vec<Vec<TextSection>> {
        let RichTextOptions { 
            box_width: width, 
            font_size, 
            default_color, 
            font } = options;

        let g_font = FontRef::try_from_slice(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf"))
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
        let mut lines: Vec<Vec<TextSection>> = Vec::new();
        let mut last_data: Option<&CharData> = None;
        for (data, glyph) in self.graphemes.iter().zip(calc_graphemes.iter()) {
            let line_no = glyph.glyph.position.y.trunc() as usize;
            if prev_line_no != line_no {
                if prev_line_no != usize::MAX {
                    // For ease, every new line gets marked with a \n. 
                    let ts = lines.last_mut().unwrap().last_mut().unwrap();
                    ts.value.push_str("\n");
                }
                lines.push(Vec::new());
                last_data = None;
            }

            let is_boundary = if let Some(l) = last_data {
                l.is_section_boundary(data)
            } else {
                true // The first section is always a "boundary"
            };

            if is_boundary {
                lines.last_mut().unwrap().push(TextSection {
                    style: TextStyle {
                        font: font.clone(),
                        font_size,
                        color: data.color.unwrap_or(default_color)
                    },
                    ..default()
                });
            }
            
            let ts = lines.last_mut().unwrap().last_mut().unwrap();
            ts.value.push_str(data.grapheme.as_str());

            prev_line_no = line_no;
            last_data = Some(data);
        }

        lines
    }
}