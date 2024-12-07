use std::ascii::escape_default;

use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();
    game.window_settings(Window {
        ..Default::default()
    });

    let player1 = game.add_sprite("player1", "sprite/racing/bus.png");
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;
    player1.scale = 0.5;

    game.audio_manager
        .play_music("music/Safari_Sunshine.mp3", 0.2);

    // savannah
    for i in 0..4 {
        let savannah = game.add_sprite(format!("savannah{}", i), "sprite/rolling/tlo2.png");
        savannah.layer = 0.01;
        savannah.scale = 2.0;
        savannah.translation.x = -600.0 + 800.0 * i as f32;
    }

    // road
    for i in 0..20 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // obstacles from presets
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // custom obstacles
    let palm_tree = "sprite/rolling/palm.png";
    let zebra = "sprite/rolling/zebra.png";
    let elephant = "sprite/rolling/elephant.png";
    let giraffe = "sprite/rolling/giraffe.png";
    let hippo = "sprite/rolling/hippo.png";
    let house_1 = "sprite/rolling/house_1.png";
    let house_2 = "sprite/rolling/house_2.png";
    let house_3 = "sprite/rolling/house_3.png";
    // let house_4 = "sprite/rolling/house_4.png";
    let animals = vec![zebra, elephant, giraffe, hippo];
    let houses = vec![house_1, house_2, house_3];
    let plants = vec![palm_tree];

    let boy = "sprite/rolling/boy.png";
    let girl = "sprite/rolling/girl.png";
    let boy_and_girl = vec![boy, girl];

    for (i, path) in boy_and_girl.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("children_obstacle{}", i), path);
        obstacle.layer = 5.0;
        obstacle.scale = 0.2;
        obstacle.collision = false;
        obstacle.translation.x = 800.0 + (i as f32 * 200.0) + thread_rng().gen_range(-50.0..50.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    for (i, path) in animals.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("animal_obstacle{}", i), path);
        obstacle.layer = 6.0;
        obstacle.scale = 0.3;
        obstacle.collision = true;
        obstacle.translation.x = 800.0 + (i as f32 * 200.0) + thread_rng().gen_range(-50.0..50.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    for (i, path) in houses.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("house_obstacle{}", i), path);
        obstacle.layer = 5.0;
        obstacle.scale = 0.4;
        obstacle.collision = true;
        obstacle.translation.x = 800.0 + (i as f32 * 200.0) + thread_rng().gen_range(-100.0..100.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    for (i, path) in plants.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("plant_obstacle{}", i), path);
        obstacle.layer = 7.0;
        obstacle.scale = 0.4;
        obstacle.collision = true;
        obstacle.translation.x = 800.0 + (i as f32 * 200.0) + thread_rng().gen_range(-15.0..15.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // health message
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // dont run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // move player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // move road
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
            if engine.keyboard_state.pressed(KeyCode::Back) {
                sprite.translation.x = ROAD_SPEED / 2.0 * engine.delta_f32;
            }
        }
        if sprite.label.starts_with("savannah") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x += 1600.0;
            }
        }
        if sprite.label.starts_with("animal_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(1800.0..2000.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
        if sprite.label.starts_with("house_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
        if sprite.label.starts_with("plant_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(2800.0..3600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
        if sprite.label.starts_with("children_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(2800.0..3600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }

    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
