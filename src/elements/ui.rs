use bevy::{ecs::query, prelude::*};


#[derive(Resource)]
pub struct PlayerData{
   pub health:u32,
  pub score:u32

}

#[derive(Component)]
pub struct ScoreText;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            padding: UiRect::horizontal(Val::Px(16.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "| | |",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        parent.spawn((TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 20.0,
                color: Color::GOLD,
                ..default()
            },
        ),
            
        ScoreText
        ));
    });
}

pub fn update_score_ui(mut score:Res<PlayerData>,mut query:Query<&mut Text,With<ScoreText>>){

    if !score.is_changed(){
        return;

    }
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", score.score);
}