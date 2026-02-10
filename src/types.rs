use wayland_client::{
	protocol::{wl_compositor, wl_shm},
};

pub struct AppData{
	pub compositor: Option<wl_compositor::WlCompositor>,
	pub shm: Option<wl_shm::WlShm>,
}
