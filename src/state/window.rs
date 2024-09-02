use smithay_client_toolkit::shell::xdg::window::WindowHandler;
use wayland_client::Connection;

use super::GfState;

impl WindowHandler for GfState {
    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _window: &smithay_client_toolkit::shell::xdg::window::Window,
        _configure: smithay_client_toolkit::shell::xdg::window::WindowConfigure,
        _serial: u32,
    ) {
        println!("Recieved Configure Request For Window")
    }
    fn request_close(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _window: &smithay_client_toolkit::shell::xdg::window::Window,
    ) {
    }
}
