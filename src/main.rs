use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;



fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Bouncing DVD Logo")
        .build();
    let dvd_logo_texture: Texture2D = match rl.load_texture(&thread, "dvd_logo.png") {
                                                Ok(texture) => texture,
                                                Err(e) => panic!("Failed to load texture: {}", e),
                                            };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_texture(&dvd_logo_texture, 0, 0, Color::WHITE);
    }
}