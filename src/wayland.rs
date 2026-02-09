use wayland_client::{
	Connection,
	Dispatch,
	QueueHandle,
	protocol::{wl_registry, wl_compositor, wl_surface, wl_shm, wl_display, wl_buffer, wl_shm_pool},
	globals::registry_queue_init,
};
//use wayland_protocols::wl_shm::clinet::wl_shm;
use memmap2::MmapMut;
use tempfile::tempfile;
use std::os::unix::io::AsFd;
use std::io::Write;

struct AppData{
	compositor: Option<wl_compositor::WlCompositor>,
	shm: Option<wl_shm::WlShm>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
	fn event(
		state: &mut Self,
		registry: &wl_registry::WlRegistry,
		event: wl_registry::Event,
		_: &(),
		_: &Connection,
		qh: &QueueHandle<AppData>,
	) {
		if let wl_registry::Event::Global { name, interface, version } = event {
			match interface.as_str() {
				"wl_compositor" => {
					let compositor = registry
						.bind::<wl_compositor::WlCompositor, _, _> (name, version, qh, ());
					state.compositor = Some(compositor);
				}
				"wl_shm" => {
					let shm = registry
						.bind::<wl_shm::WlShm, _, _> (name, version, qh, ());
					state.shm = Some(shm);
				}
				_ => {}
			}		
		}
	}
}

impl Dispatch<wl_display::WlDisplay, ()> for AppData {
	fn event(
		_: &mut Self,
		_: &wl_display::WlDisplay,
		_: wl_display::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		//empty
	}
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // empty
    }
}

impl Dispatch<wl_shm::WlShm, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_shm::WlShm,
        _: wl_shm::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // sempty
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        event: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wl_surface::Event::Enter { .. } => {
                // enters output
            }
            wl_surface::Event::Leave { .. } => {
                // leaves output
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for AppData {
    fn event(
        _state: &mut Self,
        _buffer: &wl_buffer::WlBuffer,
        event: wl_buffer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
		/*match event {
			wl_buffer::Event::Release => {}
			_ => {}
		}
		*/
	}
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_shm_pool::WlShmPool,
        _: wl_shm_pool::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {}
}

fn create_buffer(app: &AppData, qh: &QueueHandle<AppData>) -> wl_buffer::WlBuffer{
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


pub fn run() {
	//connects to compositor
	let conn = Connection::connect_to_env().unwrap();
	//creates root Wayland object, event queue
	let display = conn.display();
	let mut event_queue = conn.new_event_queue();
	let qh = event_queue.handle();
	//finds global objects
	let _registry = display.get_registry(&qh, ());
	let mut app = AppData {
		compositor: None,
		shm: None,
	};
	event_queue.roundtrip(&mut app).unwrap();
	let surface = app
		.compositor
		.as_ref()
		.unwrap()
		.create_surface(&qh, ());
	//create shm buffer- create tmp, set size, mmap it, write rgba pixels, create buffer
	let buffer = create_buffer(&app, &qh);
	surface.attach(Some(&buffer), 0, 0);
	surface.commit();
	
	//keep event loop alive
	loop {
		event_queue.blocking_dispatch(&mut app).unwrap();
	}
}
