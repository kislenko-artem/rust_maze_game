//! An example of generating julia fractals.
extern crate image;

const FOV: f32 = 3.14 / 3.0;
const W_WIDTH: i32 = 800;
const W_HEIGHT: i32 = 600;
const TEXTURE_SIZE: i32 = 64;
const OFFSET_TOP: i32 = 350;
const OFFSET_BOTTOM: i32 = 50;
const MAX_VIZIBILITY: f32 = 20.0;

struct Player {
    prev_x: f32,
    prev_y: f32,

    x: f32,
    y: f32,
    a: f32
}


fn paint_screen(mut player: Player) {

    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let render_map: [String; 16] =  [
        "0000000000000000".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0  11111       0".to_string(),
        "0  1           0".to_string(),
        "0111           0000000".to_string(),
        "0              3     0".to_string(),
        "0              0000000".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0              0".to_string(),
        "0222222222222220".to_string(),
    ];
    //c:f32 = 0.0;
    //mapSign:&str = "";
    if player.x < 1.0 {
        player.x = 1.0
    }
    if player.y < 1.0 {
        player.y = 1.0
    }

    if player.x < render_map.len() as f32 - 2.0 {
        player.x = render_map.len() as f32 - 2.0
    }

    if player.y < render_map[player.x as usize].len() as f32 - 2.0 {
        player.y = render_map[player.x as usize].len() as f32 - 2.0
    }

    // TODO: нужна оптимизация
    if render_map[player.x as usize].chars().nth(player.y as usize).unwrap() == ' ' {
        player.x = player.prev_x;
        player.y = player.prev_y;
        return
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

fn main() {
    let mut p:Player = Player{x: 2.0, y: 2.0, a: 0.0, prev_y: 0.0, prev_x: 0.0};
    paint_screen(p)
}