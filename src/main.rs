use rand::rngs::ThreadRng;
use rand::*;
use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

struct Wall {
    position: Vector2f,
    size_y: i32,
}

struct WallSet {
    top: Wall,
    bottom: Wall,
}

struct Player {
    position: Vector2f,
}

fn main() {
    // Random number gen.
    let mut rng = rand::thread_rng();

    let text_font = Font::from_file("/home/markus/Code/Rust/flappy_rust/Assets/Vera.ttf").unwrap();

    let mut wall_set = Vec::new();
    let mut player = Player {
        position: Vector2f::new(100.0, 250.0),
    };
    let mut game_over = false;
    let mut score = 0;

    let mut text_score: Text = Text::new(&score.to_string(), &text_font, 22);

    text_score.set_fill_color(&Color::BLACK);
    text_score.set_position((700.0, 10.0));
    let mut window = init();

    for i in 0..10 {
        wall_set.push(WallSet {
            top: Wall {
                position: Vector2f::new(i as f32 * 100.0, 0.0),
                size_y: rng.gen_range(100, 250),
            },
            bottom: Wall {
                position: Vector2f::new(i as f32 * 100.0, rng.gen_range(300.0, 400.0)),
                size_y: 500,
            },
        });
    }

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        if !game_over {
            draw(&mut window, &wall_set, &player, &text_score);
            update(
                &mut wall_set,
                &mut player,
                &mut text_score,
                &mut score,
                &mut game_over,
                &mut rng,
            );
        }
    }
}

fn init() -> RenderWindow {
    RenderWindow::new(
        (800, 600),
        "Flappy Rust",
        Style::CLOSE,
        &ContextSettings::default(),
    )
}

fn draw(window: &mut RenderWindow, wall_set: &Vec<WallSet>, player: &Player, score_text: &Text) {
    window.clear(&Color::CYAN);
    // Draw the walls.
    for i in wall_set {
        let mut temp: RectangleShape = RectangleShape::new();
        temp.set_size((50.0, i.top.size_y as f32));
        temp.set_position((i.top.position.x, i.top.position.y));
        window.draw(&temp);
        let mut temp_bottom: RectangleShape = RectangleShape::new();
        temp_bottom.set_size((50.0, i.bottom.size_y as f32));
        temp_bottom.set_position((i.bottom.position.x, i.bottom.position.y));
        window.draw(&temp_bottom);
    }

    // Draw the player.
    let mut temp: CircleShape = CircleShape::new(15.0, 30);
    temp.set_position((player.position.x, player.position.y));
    temp.set_fill_color(&Color::YELLOW);
    window.draw(&temp);
    // Draw the score.
    window.draw(score_text);
    window.display();
}

fn update(
    wall_set: &mut Vec<WallSet>,
    player: &mut Player,
    text_score: &mut Text,
    score: &mut i32,
    game_over: &mut bool,
    rng: &mut ThreadRng,
) {
    for i in 0..wall_set.len() {
        wall_set[i].bottom.position.x -= 0.01;
        wall_set[i].top.position.x -= 0.01;

        if wall_set[i].top.position.x < -50.0 {
            *score += 10;
            text_score.set_string(&score.to_string());
            wall_set[i].top.position.x = 850.0;
            wall_set[i].bottom.position.x = wall_set[i].top.position.x;
            wall_set[i].top.size_y = rng.gen_range(100, 250);
            wall_set[i].bottom.position.y = rng.gen_range(300.0, 400.0);
        }

        if is_point_touching(
            &player.position,
            &15.0,
            &wall_set[i].top.position,
            &Vector2f::new(50.0, *&wall_set[i].top.size_y as f32),
        ) || is_point_touching(
            &player.position,
            &15.0,
            &wall_set[i].bottom.position,
            &Vector2f::new(50.0, *&wall_set[i].bottom.size_y as f32),
        ) {
            *game_over = true;
        }
    }

    if Key::Space.is_pressed() {
        player.position.y -= 0.02;
    }

    if !Key::Space.is_pressed() {
        player.position.y += 0.02;
    }
}

fn is_point_touching(
    position_one: &Vector2f,
    size_one: &f32,
    position_two: &Vector2f,
    size_two: &Vector2f,
) -> bool {
    position_one.x < (position_two.x + size_two.x)
        && (position_one.x + size_one) > position_two.x
        && position_one.y < (position_two.y + size_two.y)
        && (position_one.y + size_one) > position_two.y
}
