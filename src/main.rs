extern crate image;

use std::time::Instant;
use image::GenericImageView;

const FOV: f32 = 3.14 / 3.0;
const W_WIDTH: i32 = 800;
const W_HEIGHT: i32 = 600;
const TEXTURE_SIZE: i32 = 64;
const OFFSET_TOP: i32 = 350;
const OFFSET_BOTTOM: i32 = 50;
const MAX_VIZIBILITY: i32 = 20 * 100;
const RADIX: u32 = 10;

struct Player {
    prev_x: f32,
    prev_y: f32,

    x: f32,
    y: f32,
    a: f32
}


fn paint_screen(mut player: Player) {

    let imgx = 800;
    let imgy = 600;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let img_textures = image::open("/home/artem/projects/golang/game/assets/textures.png").unwrap();

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
    let mut c:f32 = 0.0;
    let mut map_sign:char = ' ';

    if player.x < 1.0 {
        player.x = 1.0
    }
    if player.y < 1.0 {
        player.y = 1.0
    }

    if player.x > render_map.len() as f32 - 2.0 {
        player.x = render_map.len() as f32 - 2.0
    }

    if player.y > render_map[player.x as usize].len() as f32 - 2.0 {
        player.y = render_map[player.x as usize].len() as f32 - 2.0
    }

    // TODO: нужна оптимизация
    if render_map[player.x as usize].chars().nth(player.y as usize).unwrap() != ' ' {
        player.x = player.prev_x;
        player.y = player.prev_y;
        return
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        let mut x_wall: f32 = 0.0;
        let mut y_wall: f32 = 0.0;

        let angle = player.a - FOV/2.0 + x as f32 / W_WIDTH as f32;
        for i in 0..MAX_VIZIBILITY {
            c = i as f32 * 0.01;
            x_wall = player.x + c*angle.cos();
            y_wall = player.y + c*angle.sin();

            map_sign = render_map[x_wall as usize].chars().nth(y_wall as usize).unwrap();
            if map_sign != ' ' {
                break
            }
        }
        if map_sign == ' ' {
            continue
        }

        let mut size_y:i32 = (W_HEIGHT as f32 / (c*(angle-player.a).cos()) + (W_HEIGHT/5) as f32) as i32;
        if size_y > W_HEIGHT {
            size_y = W_HEIGHT
        }
        if size_y < 0 {
            size_y = 0
        }
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            if (y as i32) < W_HEIGHT - (size_y + OFFSET_TOP) {
                *pixel = image::Rgb([255, 255, 255]);
                continue
            }
            if (y as i32) > (size_y - OFFSET_BOTTOM) {
                *pixel = image::Rgb([255, 255, 255]);
                continue
            }
            let mut y_pic:i32 = y as i32 * TEXTURE_SIZE / (size_y - OFFSET_BOTTOM);
            let mut x_pic:i32 = ((x_wall - x_wall as i32 as f32) * TEXTURE_SIZE as f32) as i32;
            if x_pic == 0 || x_pic == TEXTURE_SIZE - 1 {
                x_pic = ((y_wall - y_wall as i32 as f32) * TEXTURE_SIZE as f32) as i32;
            } else {
                y_pic = y as i32 - (W_HEIGHT - (size_y + OFFSET_TOP)) * TEXTURE_SIZE / (size_y - OFFSET_BOTTOM - (W_HEIGHT - (size_y + OFFSET_TOP)))
            }
            if y_pic > 63 {
                y_pic = 63;
            }
            let mut koef:i32 = map_sign.to_digit(RADIX).unwrap() as i32;
            koef = koef * TEXTURE_SIZE;
            let color_rgba = img_textures.get_pixel((x_pic + koef) as u32, y_pic as u32);
            *pixel = image::Rgb([color_rgba[0], color_rgba[1], color_rgba[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("output0.png").unwrap();
}

fn main() {
    let now = Instant::now();
    let mut p:Player = Player{x: 2.0, y: 2.0, a: 0.0, prev_y: 0.0, prev_x: 0.0};
    paint_screen(p);
    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
}