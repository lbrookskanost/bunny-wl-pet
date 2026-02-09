use wayland_client::{
	Connection,
	Dispatch,
	QueueHandle,
	protocol::{wl_registry, wl_compositor, wl_surface, wl_shm, wl_buffer},
	globals::registry_queue_init,
};
//use wayland_protocols::wl_shm::clinet::wl_shm;
use memmap2::MmapMut;

struct AppData;

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
	fn event(
		_state: &mut Self,
		_: &wl_registry::WlRegistry,
		event: wl_registry::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<AppData>,
	) {
		if let wl_registry::Event::Global { name, interface, version } = event {
			println!("[{}] {} (v{})", name, interface, version);
		}
	}
}
pub fn run() {
	let conn = Connection::connect_to_env().unwrap();
	let display = conn.display();
	let mut event_queue = conn.new_event_queue();
	let qh = event_queue.handle();
	let _registry = display.get_registry(&qh, ());
	println!("Advertised globals:");
	event_queue.roundtrip(&mut AppData).unwrap();
}
