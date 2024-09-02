use smithay_client_toolkit::{
    delegate_xdg_shell, delegate_xdg_window,
    shell::{xdg::window::WindowHandler, WaylandSurface},
};
use wayland_client::{protocol::wl_shm, Connection};

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
                let width = self.width;
                let height = self.height;
                self.buffer = match self.slot_pool().create_buffer(
                    width as i32,
                    height as i32,
                    width as i32 * 4,
                    wl_shm::Format::Argb8888,
                ) {
                    Ok(buffer) => Some(buffer.0),
                    Err(err) => panic!("Failed to create buffer.\nErr: {err}"),
                };
                let buffer = self.buffer.as_ref().expect("Set to Some above.");
                // Handle return!
                buffer
                    .attach_to(self.window().wl_surface())
                    .expect("Attaching Buffer failed.");
                self.window().commit();
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
