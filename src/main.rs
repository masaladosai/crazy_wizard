use bevy::prelude::*;
use rand::Rng;


const grid_size:i32=10;
const tile:f32=48.0;

static start_player:i32 = grid_size/2;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Player;


#[derive(Component)]
struct GridPos{
    x:i32,
    y:i32
}

#[derive(Component)]
struct Coin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement,get_coin.after(player_movement),sync_grid_to_transform.after(get_coin)))
        .run();
}


fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());

    let half_grid=grid_size as f32*tile/2.0;
    let half_tile=tile/2.0;

    for x in 0..grid_size{
        for y in 0..grid_size{
                      commands.spawn((
                Tile,
                GridPos { x, y },
                SpriteBundle{
                    sprite: Sprite {
                        color: Color::rgb(0.2, 0.2, 0.2),
                        custom_size: Some(Vec2::splat(tile - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * tile-half_grid+half_tile,
                        y as f32 * tile-half_grid+half_tile,
                        0.0,
                    ),
                    ..default()
                },
            ));
        }
    }

    commands.spawn((
        Player,
        GridPos{x:start_player,y:start_player},
        SpriteBundle{
            sprite:Sprite { color: Color::rgb(0.3 , 0.8 , 0.2), custom_size: Some(Vec2::splat(tile-2.0)),..default() },

            transform: Transform::from_xyz(
            start_player as f32 * tile - half_grid + half_tile,
            start_player as f32 * tile - half_grid + half_tile,
            2.0, 
        ),
        ..default() 
        },
        
    ));
    spawn_coin(&mut commands);

}

fn player_movement(keys:Res<ButtonInput<KeyCode>>,mut query:Query<&mut GridPos,With<Player>>){

    let mut pos=query.single_mut();

    let mut dx=0;
    let mut dy=0;

    
    if keys.just_pressed(KeyCode::KeyW) {
        dy = 1;
    } else if keys.just_pressed(KeyCode::KeyS) {
        dy = -1;
    } else if keys.just_pressed(KeyCode::KeyA) {
        dx = -1;
    } else if keys.just_pressed(KeyCode::KeyD) {
        dx = 1;
    }

    let newx=pos.x+dx;
    let newy=pos.y+dy;

    if newx>=0&&newx<grid_size&&newy>=0&&newy<grid_size{
        pos.x=newx;
        pos.y=newy;
    }

}


fn sync_grid_to_transform(
    mut query: Query<(&GridPos, &mut Transform)>,
) {
    let half_grid = grid_size as f32 * tile / 2.0;
    let half_tile = tile / 2.0;

    for (pos, mut transform) in &mut query {
        transform.translation.x =
            pos.x as f32 * tile - half_grid + half_tile;
        transform.translation.y =
            pos.y as f32 * tile - half_grid + half_tile;
    }
}


fn get_coin(mut commands:Commands,player_a:Query<&GridPos,With<Player>>,coin_a:Query<(Entity,&GridPos),With<Coin>>){
    let player_pos=player_a.single();

    for (entity,coin_pos) in &coin_a{
        if player_pos.x==coin_pos.x && player_pos.y==coin_pos.y{
            commands.entity(entity).despawn();
            spawn_coin(&mut commands);
            break;
            
        }
         
    }
   
}

fn spawn_coin(mut commands:&mut Commands){
        let half_grid = grid_size as f32 * tile / 2.0;
    let half_tile = tile / 2.0;
    let mut rng = rand::thread_rng();
    let coinx=rng.random_range(0..grid_size);
    let coiny=rng.random_range(0..grid_size);

    commands.spawn((
        Coin,
        GridPos{x:coinx,y:coiny},
                SpriteBundle{
            sprite:Sprite { color: Color::rgb(0.9 , 0.1 , 0.1), custom_size: Some(Vec2::splat(tile-2.0)),..default() },

            transform: Transform::from_xyz(
            start_player as f32 * tile - half_grid + half_tile,
            start_player as f32 * tile - half_grid + half_tile,
            1.0, 
        ),
        ..default() 
        },


    ));
}