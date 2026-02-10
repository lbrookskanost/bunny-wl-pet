use memmap2::MmapMut;
use tempfile::tempfile;
use std::os::unix::io::AsFd;

use wayland_client::{
	QueueHandle,
	protocol::{wl_buffer, wl_shm, wl_shm_pool},
};

pub mod wayland;
use wayland::AppData;

use image::{RgbaImage, ImageBuffer};

pub fn create_buffer(app: &AppData, qh: &QueueHandle<AppData>) -> wl_buffer::WlBuffer{
		//create tmpfile
		let mut file = tempfile().unwrap();
		//set length
		let width = 32; let height = 32;
		let stride = width * 4;
		let size = stride * height;
		file.set_len(size as u64).unwrap();
		//memory map
		let mut mmap = unsafe {
			MmapMut::map_mut(&file).unwrap()
		};
		//draw pixels
		for y in 0..height{
			for x in 0..width {
				let i = (y * stride + x * 4) as usize;
				mmap[i + 0] = 0xFF;
				mmap[i + 1] = 0x00;
				mmap[i + 2] = 0xFF;
				mmap[i + 3] = 0x80;
			}
		}
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
		buffer
}

