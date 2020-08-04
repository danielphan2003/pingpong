use tetra::graphics::{self, Color, Texture, DrawParams};
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
        
        graphics::draw(ctx, &self.player1.texture, Vec2::new(
            self.player1.position.x,
            self.player1.position.y
        ));
        graphics::draw(ctx, &self.player2.texture, Vec2::new(
            self.player2.position.x,
            self.player2.position.y
        ));
        graphics::draw(ctx, &self.ball.texture, Vec2::new(
            self.ball.position.x,
            self.ball.position.y
        ));
        
        Ok(())
    }

    fn update(& mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        Ok(())
    }
}
