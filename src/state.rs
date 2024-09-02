pub mod compositor;
pub mod output;
pub mod window;

use smithay_client_toolkit::{
    delegate_compositor, delegate_registry,
    output::OutputState,
    registry::{self, ProvidesRegistryState, RegistryState},
    registry_handlers,
    shm::slot::Buffer,
};
use wayland_client::{
    globals::{registry_queue_init, GlobalList},
    Connection, EventQueue, QueueHandle,
};

pub struct GfState {
    pub registry_state: RegistryState,
    pub output_state: OutputState,
    buffer: Option<Buffer>,
    width: u32,
    height: u32,

    first_configure: bool,
}

impl GfState {
    pub fn new(global_list: &GlobalList, qh: &QueueHandle<Self>) -> Self {
        GfState {
            registry_state: RegistryState::new(global_list),
            output_state: OutputState::new(global_list, qh),
            first_configure: true,
            height: 256,
            width: 256,
            buffer: None,
        }
    }
}
delegate_registry!(GfState);
delegate_compositor!(GfState);

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
