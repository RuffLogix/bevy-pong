use crate::logic::{AiScore, PlayerScore, Score};
use bevy::prelude::*;

fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.0 = score.player.to_string();
        }

        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.0 = score.ai.to_string();
        }
    }
}

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        PlayerScore,
        Text::new("0"),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
    ));

    commands.spawn((
        AiScore,
        Text::new("0"),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));
}

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scoreboard);
        app.add_systems(Update, update_scoreboard);
    }
}
