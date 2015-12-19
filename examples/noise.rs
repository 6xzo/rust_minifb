extern crate minifb;

use minifb::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    //let mut noise;
    //let mut carry;
    //let mut seed = 0xbeefu32;

    let mut buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    let window = match macos::Window::new("Noise Test - Press ESC to exit", 
                                   WIDTH, 
                                   HEIGHT, 
                                   Scale::X1, 
                                   Vsync::No) {
        Ok(window) => window,
        Err(info) => {
            println!("{}", info);
            return;
        }
    };

    loop {
        window.update(&buffer);
        /*
        for i in buffer.iter_mut() {
            noise = seed;
            noise >>= 3;
            noise ^= seed;
            carry = noise & 1;
            noise >>= 1;
            seed >>= 1;
            seed |= carry << 30;
            noise &= 0xFF;
            *i = (noise << 16) | (noise << 8) | noise;
        }
        */
    }
}
