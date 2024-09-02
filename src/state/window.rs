use smithay_client_toolkit::{
    delegate_xdg_shell, delegate_xdg_window, shell::xdg::window::WindowHandler,
};
use wayland_client::Connection;

use super::GfState;

delegate_xdg_shell!(GfState);
delegate_xdg_window!(GfState);

impl WindowHandler for GfState {
    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _window: &smithay_client_toolkit::shell::xdg::window::Window,
        configure: smithay_client_toolkit::shell::xdg::window::WindowConfigure,
        _serial: u32,
    ) {
        self.buffer = None;
        self.width = configure.new_size.0.map_or(256, |v| v.get());
        self.height = configure.new_size.1.map_or(256, |v| v.get());

        if self.first_configure {
            self.first_configure = false;
            // Draw
            {

            }
        }
    }
    fn request_close(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _window: &smithay_client_toolkit::shell::xdg::window::Window,
    ) {
    }
}
