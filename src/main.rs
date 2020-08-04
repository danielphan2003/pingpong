use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PADDLE_SPEED: f32 = 8.0;


fn main() -> tetra::Result {
    ContextBuilder::new("PingPong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position 
        }
    }
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Player 1
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
                16.0,
                (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0
        );
        let player1 = Entity::new(player1_texture, player1_position);

        // Player 2
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
                WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
                (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );
        let player2 = Entity::new(player2_texture, player2_position);

        // Ball 
        let ball_texture = Texture::new(ctx, "./resources/ballBlue.png")?;
        let ball_position = Vec2::new(
            (WINDOW_WIDTH - ball_texture.width() as f32) / 2.0,
            (WINDOW_HEIGHT - ball_texture.height() as f32) / 2.0,
        );
        let ball = Entity::new(ball_texture, ball_position);

        Ok(GameState { 
            player1,
            player2,
            ball
        })
    }
}



impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        draw_texture(ctx, &self.player1);
        draw_texture(ctx, &self.player2);
        draw_texture(ctx, &self.ball);
        
        Ok(())
    }

    fn update(& mut self, ctx: &mut Context) -> tetra::Result {
        for key in input::get_keys_down(ctx) {
            match key {
                // Player 1
                Key::W => move_up(&mut self.player1), 
                Key::S => move_down(&mut self.player1), 

                // Player 2
                Key::Up => move_up(&mut self.player2), 
                Key::Down => move_down(&mut self.player2),

                // Other keys
                _ => {},
            }
        }

        Ok(())
    }
}

// Simply draw texture from entity with position
fn draw_texture(ctx: &mut Context, entity: &Entity) {
    graphics::draw(ctx, &entity.texture, entity.position);
}

fn move_up(entity: &mut Entity) {
    if entity.position.y > 0.0 {
        entity.position.y -= PADDLE_SPEED;
    } else {
        entity.position.y = 0.0;
    }
}

fn move_down(entity: &mut Entity) {
    let half_width: f32 = entity.texture.width() as f32/2.0;
    if entity.position.y + half_width < WINDOW_HEIGHT {
        entity.position.y += PADDLE_SPEED;
    } else {
        entity.position.y = WINDOW_HEIGHT - half_width;
    }
}