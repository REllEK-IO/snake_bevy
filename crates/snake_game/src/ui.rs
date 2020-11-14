pub mod game_ui {
    use crate::game::game_data::*;
    use bevy::prelude::*;

    pub fn init_ui(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        commands
            // texture
            .spawn(TextComponents {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    position: Rect {
                        left: Val::Percent(82.5),
                        bottom:Val::Percent(2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "Score".to_string(),
                    font: asset_server.load("fonts/Pixeboy.ttf"),
                    style: TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            })
            .with(ScoreText);
        commands
            // texture
            .spawn(TextComponents {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position: Rect {
                        right: Val::Percent(10.0),
                        top:Val::Percent(2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "High Scores \n\n1.  0 \n\n2.  0 \n\n3.  0".to_string(),
                    font: asset_server.load("fonts/Pixeboy.ttf"),
                    style: TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            })
            .with(PrevScoreText);
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
            text.value = score_board;
        }
    }
    pub fn update_score (
        game: Res<GameState>,
        mut score_query: Query<(&mut Text, &ScoreText)>
    ) {
        for (mut text, _) in score_query.iter_mut() {
            text.value = format!("Score: {}", game.score);
        }
    }
}