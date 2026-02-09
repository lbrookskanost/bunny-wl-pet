use wayland_client::{
	Connection,
	Dispatch,
	QueueHandle,
	protocol::{wl_registry, wl_compositor, wl_surface, wl_shm, wl_buffer},
	globals::registry_queue_init,
};
//use wayland_protocols::wl_shm::clinet::wl_shm;
use memmap2::MmapMut;

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
			if interface == wl_compositor::WlCompositor::interface().name {
				state.compositor = Some(
					registry.bind(name, version, qh, ())
				);
			}
			if interface == wl_shm::WlShm::interface().name {
				state.shm = Some(
					registry.bind(name, version, qh, ())
				);
			}		
		}
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
	};
	event_queue.roundtrip(&mut app).unwrap();
	let surface = app
		.compositor
		.as_ref()
		.unwrap()
		.create_surface(&qh, ());
	//create shm buffer
	//attach buffer
	//keep event loop alive
	loop {
		event_queue.blocking_dispatch(&mut app).unwrap();
	}
}
