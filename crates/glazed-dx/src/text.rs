use std::ops::Range;

use bevy::prelude::*;
use glyph_brush_layout::{GlyphPositioner, Layout, SectionGeometry, ab_glyph::FontRef, FontId};
use iyes_loopless::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::UI;

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
impl ShowText {
    pub fn create_frame_node(&self) -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(self.width), Val::Px(((self.lines + 1) as f32) * 16.0)),
                ..default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.75).into(),
            ..default()
        }
    }
}

#[derive(Component)]
pub struct TextBoxContent {
    lines: Vec<Vec<TextSection>>,
    current_line: usize,
    lines_per_page: usize
}
impl TextBoxContent {
    pub fn from(st: &ShowText) -> TextBoxContent {
        let rt = Formatter::parse_to_graphemes(st.string.clone());
        let lines = rt.to_box(RichTextOptions {
            box_width: st.width - 32.0, // Determined empirically to play well with the margins
            font: st.font.clone(),
            font_size: 16.0,
            default_color: Color::WHITE,
        });

        TextBoxContent {
            lines,
            current_line: 0,
            lines_per_page: st.lines
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
                    .with_system(test)
                    .with_system(TextPlugin::create_text_frame)
                    .into()
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TextState::Scrolling)
                    .with_system(TextPlugin::load_in_page)
                    .into()
            )
            ;
    }
}

impl TextPlugin {
    fn create_text_frame(mut commands: Commands, mut events: EventReader<ShowText>, query: Query<Entity, With<UI>>) {
        if let Some(e) = events.iter().last() {
            commands.insert_resource(NextState(TextState::Scrolling));

            commands
                .entity(query.single())
                .with_children(|p| {
                    p.spawn_bundle(e.create_frame_node()).insert(TextBox)
                    .with_children(|p| {
                        p.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect {
                                    left: Val::Px(8.0),
                                    right: Val::Px(16.0),
                                    top: Val::Px(8.0),
                                    bottom: Val::Px(8.0)
                                },
                                min_size: Size::new(Val::Px(232.0), Val::Px(32.0)),
                                max_size: Size::new(Val::Px(232.0), Val::Px(32.0)),
                                ..default()
                            },
                            text: Text { sections: vec![], alignment: default() },
                            ..default()
                        })
                        .insert(TextBoxContent::from(e));
                    });
                });
        }
    }

    fn load_in_page(mut commands: Commands, mut query: Query<(&mut Text, &mut TextBoxContent)>) {
        for (mut text, mut content) in query.iter_mut() {
            let sections = content.get_current_page_range()
                .flat_map(|idx| content.lines[idx].iter())
                .map(|v| v.clone())
                .collect::<Vec<_>>();

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

    //fn load_next_on_button_press(mut commands: Commands, )
}

fn test(mut controls: Res<Input<KeyCode>>, mut speak: EventWriter<ShowText>, assets: Res<AssetServer>) {
    if controls.just_pressed(KeyCode::Space) {
        speak.send(ShowText {
            string: "Honestly, I'm pretty anxious about this text box. Things seem to be going smoothly, but...".to_string(),
            width: 256.0,
            lines: 2,
            font: assets.load("fonts/RobotoMono-Regular.ttf")
        });
    }
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
                        Some("1") => color = Some(Color::BLACK),
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
        
        let cd = CharData {
            grapheme,
            color,
        };
        self.graphemes.push(cd);
    }

    pub fn to_box(&self, options: RichTextOptions) -> Vec<Vec<TextSection>> {
        let RichTextOptions { 
            box_width: width, 
            font_size, 
            default_color, 
            font } = options;

        let g_font = FontRef::try_from_slice(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf"))
            .unwrap();

        let calc_graphemes = Layout::default().calculate_glyphs(
            &[g_font], 
            &SectionGeometry {
                bounds: (width, f32::INFINITY),
                ..default()
            }, 
            &[
                glyph_brush_layout::SectionText {
                    text: self.unformatted.as_str(),
                    scale: font_size.into(),
                    font_id: FontId(0),
                }
            ]);

        let mut prev_line_no = usize::MAX;
        let mut lines = Vec::new();
        let mut last_data: Option<&CharData> = None;
        for (data, glyph) in self.graphemes.iter().zip(calc_graphemes.iter()) {
            let line_no = glyph.glyph.position.y.trunc() as usize;
            if prev_line_no != line_no {
                lines.push(Vec::new());
                last_data = None;
            }

            if let Some(l) = last_data {
                if l.is_section_boundary(data) {
                    lines.last_mut().unwrap().push(TextSection {
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: data.color.unwrap_or(default_color)
                        },
                        ..default()
                    });
                }
            } else {
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

// mod tests {
//     use bevy::prelude::Color;

//     use crate::text::Formatter;

//     use super::RichTextOptions;

//     #[test]
//     fn parsing_a_string() {
//         let s = "I'd like to know... can you tell me what <c13²> is?";
        
//         let g = Formatter::parse_to_graphemes(s);
//         g.to_box(RichTextOptions {
//             box_width: 256.0,
//             font: Default::default(),
//             font_size: 16.0,
//             default_color: Color::WHITE,
//         });
//     }
// }