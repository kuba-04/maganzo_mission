use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
const CELEBRATIONS: [(&str, &str); 3] = [
    ("sfx/asante.mp3", "Asante!"),
    ("sfx/habari2.mp3", "Habari!"),
    ("sfx/yaay1.mp3", "Yaay!"),
];

const INTRO_SLIDES: [&str; 7] = [
    "sprite/img/SMX.png",
    "sprite/img/SM0.png",
    "sprite/img/SM1.png",
    "sprite/img/SM3.png",
    "sprite/img/SM4.png",
    "sprite/img/SM5.png",
    "sprite/img/SM6.png",
];

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
    score: u8,
    asante_timer: Option<f32>,
    current_slide: usize,
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

    game.audio_manager.play_music("music/be_happy.mp3", 0.2);

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
        let obstacle = game.add_sprite(format!("children{}", i), path);
        obstacle.layer = 5.0;
        obstacle.scale = 0.2;
        obstacle.collision = true;
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
    health_message.layer = 101.0;

    // Add score message
    let score_message = game.add_text("score_message", "Score: 0");
    score_message.translation = Vec2::new(550.0, 280.0);
    score_message.layer = 101.0;

    // Add first slide
    let slide = game.add_sprite("intro_slide_0", INTRO_SLIDES[0]);
    slide.layer = 102.0;
    slide.scale = 1.0;

    // Add control buttons (using temporary sprites)
    let up_button = game.add_sprite("button_up", "sprite/img/up.png");
    up_button.translation = Vec2::new(-600.0, 100.0);
    up_button.layer = 101.0;
    up_button.scale = 0.3;

    let down_button = game.add_sprite("button_down", "sprite/img/up.png");
    down_button.translation = Vec2::new(-600.0, -100.0);
    down_button.layer = 101.0;
    down_button.scale = 0.3;
    down_button.rotation = -std::f32::consts::PI / 1.0; // Rotate to point down

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
        score: 0,
        asante_timer: None,
        current_slide: 0,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Handle asante text timer
    if let Some(timer) = &mut game_state.asante_timer {
        *timer -= engine.delta_f32;
        if *timer <= 0.0 {
            engine.texts.remove("asante");
            game_state.asante_timer = None;
        }
    }

    // Handle slideshow
    if game_state.current_slide < INTRO_SLIDES.len() {
        if engine.keyboard_state.just_pressed(KeyCode::Space)
            || engine.keyboard_state.just_pressed(KeyCode::Return)
            || engine.keyboard_state.just_pressed(KeyCode::Right)
            || engine.mouse_state.just_pressed(MouseButton::Left)
        {
            println!("Key pressed! Current slide: {}", game_state.current_slide);
            engine
                .sprites
                .remove(&format!("intro_slide_{}", game_state.current_slide));

            if game_state.current_slide + 1 < INTRO_SLIDES.len() {
                game_state.current_slide += 1;
                println!(
                    "Adding new slide: {}",
                    INTRO_SLIDES[game_state.current_slide]
                );
                let new_slide = engine.add_sprite(
                    format!("intro_slide_{}", game_state.current_slide),
                    INTRO_SLIDES[game_state.current_slide],
                );
                new_slide.layer = 102.0;
                new_slide.scale = 1.0;
            } else {
                game_state.current_slide = INTRO_SLIDES.len();
            }
        }
        return;
    }

    // dont run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up)
        || (engine.mouse_state.pressed(MouseButton::Left)
            && engine
                .mouse_state
                .location()
                .unwrap_or(Vec2::ZERO)
                .distance(engine.sprites.get("button_up").unwrap().translation)
                < 50.0)
    {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down)
        || (engine.mouse_state.pressed(MouseButton::Left)
            && engine
                .mouse_state
                .location()
                .unwrap_or(Vec2::ZERO)
                .distance(engine.sprites.get("button_down").unwrap().translation)
                < 50.0)
    {
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
            if sprite.translation.x < -1200.0 {
                sprite.translation.x += 2400.0;
            }
            if engine.keyboard_state.pressed(KeyCode::Back) {
                sprite.translation.x = ROAD_SPEED / 2.0 * engine.delta_f32;
            }
        }
        if sprite.label.starts_with("savannah") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -1200.0 {
                sprite.translation.x += 2400.0;
            }
        }
        if sprite.label.starts_with("animal_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -1200.0 {
                sprite.translation.x = thread_rng().gen_range(1800.0..2400.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
        if sprite.label.starts_with("house_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -1200.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
        if sprite.label.starts_with("plant_obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -1200.0 {
                sprite.translation.x = thread_rng().gen_range(2800.0..3600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
        if sprite.label.starts_with("children") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -1200.0 {
                sprite.translation.x = thread_rng().gen_range(2800.0..3600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Update messages
    let health_message = engine.texts.get_mut("health_message").unwrap();
    health_message.value = format!("Health: {}", game_state.health_amount);

    let score_message = engine.texts.get_mut("score_message").unwrap();
    score_message.value = format!("Score: {}", game_state.score);

    for event in engine.collision_events.clone().into_iter() {
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }

        if event.pair.either_contains("children") {
            game_state.score += 1;

            // Find which child was hit and remove it
            let child_id = if event.pair.0 == "player1" {
                &event.pair.1
            } else {
                &event.pair.0
            };
            engine.sprites.remove(child_id);

            // Respawn the child immediately
            let new_child = engine.add_sprite(
                child_id.clone(),
                if thread_rng().gen_bool(0.5) {
                    "sprite/rolling/boy.png"
                } else {
                    "sprite/rolling/girl.png"
                },
            );
            new_child.layer = 5.0;
            new_child.scale = 0.2;
            new_child.collision = true;
            new_child.translation.x = thread_rng().gen_range(2800.0..3600.0);
            new_child.translation.y = thread_rng().gen_range(-300.0..300.0);

            // Play celebration sound and show message
            let (sound, message) = CELEBRATIONS.choose(&mut thread_rng()).unwrap();
            engine.audio_manager.play_sfx(sound.to_string(), 0.5);
            let asante = engine.add_text("asante", message.to_string());
            asante.font_size = 128.0;
            game_state.asante_timer = Some(1.0);
            continue;
        }

        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.5);
        }
    }

    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text(
            "game over",
            format!("Game Over\nScore: {}", game_state.score),
        );
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
