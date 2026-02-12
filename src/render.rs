use memmap2::MmapMut;
use tempfile::tempfile;
use std::os::unix::io::AsFd;

use wayland_client::{
	QueueHandle,
	protocol::{wl_buffer, wl_shm},
};

//mod types;
use crate::types::AppData;
use crate::types::states;
use crate::animation;

use image::{RgbaImage, ImageReader};

//load sprite
pub fn load_sprite(path: &str) -> RgbaImage {
    ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8()
}

//draw sprite
pub fn draw_sprite(mmap: &mut MmapMut, sprite: &RgbaImage, width: i32, height: i32) {
    let stride = width * 4;
    
    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let pixel = sprite.get_pixel(x, y);
            let i = (y * stride as u32 + x * 4) as usize;
            
            // ARGB8888 format 
            mmap[i + 0] = pixel[2];  // Blue
            mmap[i + 1] = pixel[1];  // Green  
            mmap[i + 2] = pixel[0];  // Red
            mmap[i + 3] = pixel[3];  // Alpha
        }
    }
}


pub fn create_buffer(app: &AppData, qh: &QueueHandle<AppData>, initial_state: &states) -> wl_buffer::WlBuffer{
		//create tmpfile
		let file = tempfile().unwrap();
		//set length
		let width = 64; let height = 64;
		let stride = width * 4;
		let size = stride * height;
		file.set_len(size as u64).unwrap();
		//memory map
		let mut mmap = unsafe {
			MmapMut::map_mut(&file).unwrap()
		};
		//load sprite
		let path = animation::run(initial_state);
		draw_sprite(&mut mmap, &load_sprite(&path), width, height);
		//create pool
		let pool = app.shm.as_ref().unwrap().create_pool(
			file.as_fd(),
			size as i32,
			&qh,
			(),
		);
		//create buffer
		let buffer = pool.create_buffer(
			0,
			width,
			height,
			stride,
			wl_shm::Format::Argb8888,
			&qh,
			(),
		);
		return buffer;
}

