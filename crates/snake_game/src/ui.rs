pub mod game_ui {
    use crate::game::game_data::*;
    use bevy::prelude::*;

    pub fn init_ui(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        commands
            // texture
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    position: Rect {
                        left: Val::Percent(82.5),
                        bottom:Val::Percent(2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section (
                    "Score".to_string(),
                    TextStyle {
                        font_size: 30.0,
                        font: asset_server.load("fonts/Pixeboy.ttf"),
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },                    
                ),
                ..Default::default()
            })
            .insert(ScoreText);
        commands
            // texture
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position: Rect {
                        right: Val::Percent(10.0),
                        top:Val::Percent(2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section (
                    "High Scores \n\n1.  0 \n\n2.  0 \n\n3.  0".to_string(),
                    TextStyle {
                        font_size: 30.0,
                        font: asset_server.load("fonts/Pixeboy.ttf"),
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },  
                ),
                ..Default::default()
            })
            .insert(PrevScoreText);
    }

    pub fn update_high_scores (
        game: Res<GameState>,
        mut score_query: Query<(&mut Text, &PrevScoreText)>
    ) {
        for (mut text, _) in score_query.iter_mut() {
            let mut score_board = String::from("High Scores \n");
            let mut i = 1;
            for score in game.prev_scores.iter() {
                if i > 3 {
                    break;
                }
                score_board += &format!("\n{}. {}\n", i, score);
                i += 1;
                // println!("{}", score);
            }
            text.sections[0].value = score_board;
        }
    }
    pub fn update_score (
        game: Res<GameState>,
        mut score_query: Query<(&mut Text, &ScoreText)>
    ) {
        for (mut text, _) in score_query.iter_mut() {
            text.sections[0].value = format!("Score: {}", game.score);
        }
    }
}