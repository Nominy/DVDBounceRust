use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
const DVD_LOGO_SPEED: Vector2 = Vector2::new(128.0, 128.0);
const INITIAL_DVD_POS: Vector2 = Vector2::new(20.0,20.0);

fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Bouncing DVD Logo")
        .build();
    let dvd_logo_texture: Texture2D = match rl.load_texture(&thread, "dvd_logo.png") {
                                                Ok(texture) => texture,
                                                Err(e) => panic!("Failed to load texture: {}", e),
                                            };

    let mut pos = INITIAL_DVD_POS;
    let mut vel = DVD_LOGO_SPEED;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let dt = d.get_frame_time();
        let next_pos = pos + vel * dt;

        if next_pos.x < 0.0 || next_pos.x + dvd_logo_texture.width as f32 > SCREEN_WIDTH as f32 {
            vel.x *= -1.0;
        }
        if next_pos.y < 0.0 || next_pos.y + dvd_logo_texture.height as f32 > SCREEN_HEIGHT as f32 {
            vel.y *= -1.0;
        }

        pos += vel * dt;
        d.draw_texture_v(&dvd_logo_texture, pos, Color::WHITE);
    }
}