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
            self.draw();
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

impl GfState {
    pub fn draw(&mut self) {
        let width = self.width as i32;
        let height = self.height as i32;
        let stride = width * 4;
        self.buffer =
            match self
                .slot_pool()
                .create_buffer(width, height, stride, wl_shm::Format::Argb8888)
            {
                Ok(buffer) => Some(buffer.0),
                Err(err) => panic!("Failed to create buffer.\nErr: {err}"),
            };
        let pool = self.pool.as_mut().unwrap();
        let buffer = self.buffer.as_ref().expect("Set to Some above.");
        // TODO: Create a fallback for this.
        let canvas = pool.canvas(buffer).expect("Getting Canvas Failed.");
        canvas.chunks_exact_mut(4).for_each(|chunk| {
            chunk[0] = 0xFF;
            chunk[1] = rand::random();
            chunk[2] = 0xFF;
            chunk[3] = 0xFF;
        });
        buffer
            .attach_to(self.window().wl_surface())
            .expect("Attaching Buffer failed.");
        self.window()
            .wl_surface()
            .damage_buffer(0, 0, width, height);
        self.window()
            .wl_surface()
            .frame(&self.queue_handle, self.window().wl_surface().clone());
        self.window().commit();
    }
}
