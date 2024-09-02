pub mod compositor;
pub mod output;
pub mod window;

use smithay_client_toolkit::{
    delegate_compositor, delegate_output, delegate_registry, delegate_xdg_shell,
    output::OutputState,
    registry::{self, ProvidesRegistryState, RegistryState},
    registry_handlers,
};
use wayland_client::{
    globals::{registry_queue_init, GlobalList},
    Connection, EventQueue,
};

pub struct GfState {
    pub registry_state: RegistryState,
    pub output_state: OutputState,
}
delegate_registry!(GfState);
delegate_output!(GfState);
delegate_compositor!(GfState);
delegate_xdg_shell!(GfState);

impl ProvidesRegistryState for GfState {
    fn registry(&mut self) -> &mut registry::RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState];
}

/// Get wayland globals from registry
pub fn get_globals(conn: &Connection) -> (GlobalList, EventQueue<GfState>) {
    match registry_queue_init(conn) {
        Ok((globals, event_queue)) => (globals, event_queue),
        Err(err) => panic!("Failed to initialize registry.\nErr is: {err}"),
    }
}
