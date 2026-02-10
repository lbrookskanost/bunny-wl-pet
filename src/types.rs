use wayland_client::{
	protocol::{wl_compositor, wl_shm, wl_surface},
};
use wayland_protocols::xdg::shell::client::{xdg_wm_base, xdg_surface, xdg_toplevel};


pub struct AppData{
	pub compositor: Option<wl_compositor::WlCompositor>,
	pub shm: Option<wl_shm::WlShm>,
	pub surface: Option<wl_surface::WlSurface>,
	pub xdg_wm_base: Option<xdg_wm_base::XdgWmBase>,
	pub xdg_surface: Option<xdg_surface::XdgSurface>,
	pub xdg_toplevel: Option<xdg_toplevel::XdgToplevel>,
}
