extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 700;
const HEIGHT: u32 = 700;
const TOPPLE: u32 = 4;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Sandpiles", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut vec_a = vec![
        0, 0, 0,
        0, 100000, 0,
        0, 0, 0,
    ];

    let mut vec_b = vec![
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
    ];
    
    let mut is_a = true;
    let mut cont = false;

    renderer.set_draw_color(Color::RGB(20, 20, 20));
    renderer.clear();

    present_square(&mut renderer, if is_a { &vec_b } else { &vec_a });

    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                | Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running
                },
                Event::Window {..} => {
                    renderer.present();
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    cont = !cont;
                },
                _ => {
                    // println!("{:?}", event);
                    // renderer.present();
                }
            }
        }

        if !cont {
            continue;
        }

        if is_a {
            cont = next_step(&mut vec_a, &mut vec_b);
        } else {
            cont = next_step(&mut vec_b, &mut vec_a);
        }

        print!("{:?}", cont);

        present_square(&mut renderer, if is_a { &vec_b } else { &vec_a });

        is_a = !is_a;

        // cont = false;
    }
}

fn next_step(src: &mut Vec<u32>, dst: &mut Vec<u32>) -> bool {
    let len = (src.len() as f64).sqrt() as usize;
    
    let mut need_expand = false;
    
    for i in 0..len {
        if src[i]                   >= TOPPLE || // top
           src[(i + 1) * len - 1]   >= TOPPLE || // right
           src[i * len]             >= TOPPLE || // left
           src[len * (len - 1) + i] >= TOPPLE {  // bottom
            need_expand = true;
            break;
        }
    }
    
    /*
    0 1 2
    3 4 5
    6 7 8
    
    to

                      new_i = old_i + ((old_i / old_len) * 2) + new_len + 1;
                      old_i = new_i - (((new_i / new_len) - 1) * 2) - new_len - 1;

    0  1     2     3     4
    5  6 (0) 7 (1) 8 (2) 9  ; 6  = 0 + (row(0) * 2) + new_len(5) + 1; 0 = 6 - (((6 / new_len(5)) - 1) * 2) - new_len(5) - 1
    10 11(3) 12(4) 13(5) 14 ; 12 = 4 + (row(1) * 2) + new_len(5) + 1; 4 = 
    15 16(6) 17(7) 18(8) 19 ; 18 = 8 + (row(2) * 2) + new_len(5) + 1
    20 21    22    23    24
    */
    
    if need_expand {
        let new_len = len + 2;
        dst.resize(new_len * new_len, 0);
        
        for i in 0..dst.len() {
            let mut adjustment: i64 = 0;
            
            if right_edge(new_len, i) {
                adjustment += 1;
            }
            
            if left_edge(new_len, i) {
                if adjustment != 0 {
                    dst[i] = 0;
                    continue;
                }
                adjustment -= 1;
            }
            
            if top_edge(new_len, i) {
                if adjustment != 0 {
                    dst[i] = 0;
                    continue;
                }
                adjustment += new_len as i64;
            }
            
            if bottom_edge(new_len, i) {
                if adjustment != 0 {
                    dst[i] = 0;
                    continue;
                }
                adjustment -= new_len as i64;
            }
            
            if adjustment != 0 {
                dst[i] = src[old_index(i as i64 + adjustment, new_len as i64)] / TOPPLE;
            } else {
                let old_i = old_index(i as i64, new_len as i64);

                dst[i] = if !right_edge(len, old_i)  { src[old_i + 1  ] / TOPPLE } else { 0 } +
                         if !left_edge(len, old_i)   { src[old_i - 1  ] / TOPPLE } else { 0 } +
                         if !bottom_edge(len, old_i) { src[old_i + len] / TOPPLE } else { 0 } +
                         if !top_edge(len, old_i)    { src[old_i - len] / TOPPLE } else { 0 } +
                         src[old_i] % TOPPLE;
            }
        }
        
        src.resize(new_len * new_len, 0);
    } else {
        for i in 0..src.len() {
            dst[i] = if !right_edge(len, i)  { src[i + 1  ] / TOPPLE } else { 0 } +
                     if !left_edge(len, i)   { src[i - 1  ] / TOPPLE } else { 0 } +
                     if !bottom_edge(len, i) { src[i + len] / TOPPLE } else { 0 } +
                     if !top_edge(len, i)    { src[i - len] / TOPPLE } else { 0 } +
                     src[i] % TOPPLE;
        }
    }
    
    src.iter().any(|i| i > &3)
}

fn old_index(i: i64, new_len: i64) -> usize {
    (i - (((i / new_len) - 1) * 2) - new_len - 1) as usize
}

fn right_edge(len: usize, i: usize) -> bool {
    i % len + 1 == len
}

fn left_edge(len: usize, i: usize) -> bool {
    i % len == 0
}

fn top_edge(len: usize, i: usize) -> bool {
    i < len
}

fn bottom_edge(len: usize, i: usize) -> bool {
    i / len + 1 == len
}

fn print_square(arr: &[u32]) {
    let sqrt = (arr.len() as f64).sqrt() as usize;
    
    let max_len = (*arr.iter().max().unwrap() as f64).log10().trunc() as usize + 1;
    
    for i in 0..sqrt {
        for item in &arr[i * sqrt .. (i + 1) * sqrt] {
            print!("{:width$} ", item, width = max_len)
        }
        
        println!("");
    }
    
    println!("");
}

fn present_square(rend: &mut sdl2::render::Renderer, arr: &[u32]) {
    let sqrt = (arr.len() as f64).sqrt() as usize;
    let w = (WIDTH as f64 / sqrt as f64);
    let w_r = (WIDTH as f64 / sqrt as f64).round() as u32 + 1;
    let max = *arr.iter().max().unwrap();

    for row in 0..sqrt {
        for (col, num) in arr[row * sqrt .. (row + 1) * sqrt].iter().enumerate() {
            let v = 240.0 - ((*num as f64 / max as f64) * 240.0);
            rend.set_draw_color( if v != 240.0 { cool_color_palle(v) } else { Color::RGB(20, 20, 20) });
            rend.fill_rect(Some(((col as f64 * w).round() as i32, (row as f64 * w).round() as i32, w_r, w_r).into()));
        }
    }

    rend.present();
}

fn cool_color_palle(v: f64) -> Color {
    let (r,g,b) = hsl_to_rgb(v);
    Color::RGB(r, g, b)
}

// Heavily modified
fn hsl_to_rgb(h: f64) -> (u8, u8, u8) {
    let h = h / 360.0; // treat this as 0..1 instead of degrees

    (percent_to_byte(hue_to_rgb(h + 1.0 / 3.0)),
     percent_to_byte(hue_to_rgb(h)),
     percent_to_byte(hue_to_rgb(h - 1.0 / 3.0)))
}

fn percent_to_byte(percent: f64) -> u8 {
    (percent * 255.0).round() as u8
}

fn hue_to_rgb(t: f64) -> f64 {
    // Normalize
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        6.0 * t
    } else if t < 1.0 / 2.0 {
        1.0
    } else if t < 2.0 / 3.0 {
        4.0 - 6.0 * t 
    } else {
        0.0
    }
}