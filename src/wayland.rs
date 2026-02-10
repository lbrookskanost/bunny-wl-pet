use wayland_client::{
	Connection,
	Dispatch,
	QueueHandle,
	protocol::{wl_registry, wl_compositor, wl_surface, wl_shm, wl_display, wl_buffer, wl_shm_pool, wl_callback},
	globals::registry_queue_init,
};

use wayland_protocols::xdg::shell::client::{xdg_wm_base, xdg_surface, xdg_toplevel};

//mod render;
//mod types;
use crate::types::AppData;
use crate::render;


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
				"xdg_wm_base" => {  
                    let xdg = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, version, qh, ());
                    state.xdg_wm_base = Some(xdg);
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
		//empty
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
    ) {
		//empty
	}
}

impl Dispatch<wl_callback::WlCallback, ()> for AppData {
    fn event(
        state: &mut Self,
        _: &wl_callback::WlCallback,
        event: wl_callback::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_callback::Event::Done { .. } => {
                // 1. update animation state
                // 2. draw new sprite to buffer  
                // 3. attach buffer to surface
                // 4. request NEXT frame callback
                //surface.frame(qh, ());
                //surface.commit();
            }
            _ => {}
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &xdg_wm_base::XdgWmBase,
        _: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
		//empty
	}
}

impl Dispatch<xdg_surface::XdgSurface, ()> for AppData {
    fn event(
        _: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
		match event {
            xdg_surface::Event::Configure { serial } => {
                xdg_surface.ack_configure(serial);
            }
            _ => {}
        }
	}
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        _: xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
		//empty
	}
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
		surface: None,
		xdg_wm_base: None,
		xdg_surface: None,
	    xdg_toplevel: None,
	};
	event_queue.roundtrip(&mut app).unwrap();
	let surface = app
		.compositor
		.as_ref()
		.unwrap()
		.create_surface(&qh, ());
	let xdg_surface = app
		.xdg_wm_base
		.as_ref()
		.unwrap()
		.get_xdg_surface(&surface, &qh, ());
	let xdg_toplevel = xdg_surface.get_toplevel(&qh, ());
	xdg_toplevel.set_title("ferris".to_string());
	//create shm buffer
	app.surface = Some(surface.clone());
	app.xdg_surface = Some(xdg_surface.clone());
	surface.commit();
	event_queue.roundtrip(&mut app).unwrap();
	
	let buffer = render::create_buffer(&app, &qh);
	surface.attach(Some(&buffer), 0, 0);
	surface.frame(&qh, ());
	surface.commit();
	//keep event loop alive
	loop {
		event_queue.blocking_dispatch(&mut app).unwrap();
	}
}
