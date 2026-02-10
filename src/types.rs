use wayland_client::{
	protocol::{wl_compositor, wl_shm, wl_surface},
};

pub struct AppData{
	pub compositor: Option<wl_compositor::WlCompositor>,
	pub shm: Option<wl_shm::WlShm>,
	pub surface: Option<wl_surface::WlSurface>,
}
