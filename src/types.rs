use wayland_client::{
	protocol::{wl_compositor, wl_shm, wl_surface},
};

use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1,
    zwlr_layer_surface_v1,
};

pub struct AppData{
	pub compositor: Option<wl_compositor::WlCompositor>,
	pub shm: Option<wl_shm::WlShm>,
	pub surface: Option<wl_surface::WlSurface>,
	pub layer_shell: Option<zwlr_layer_shell_v1::ZwlrLayerShellV1>, 
    pub layer_surface: Option<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>
}
