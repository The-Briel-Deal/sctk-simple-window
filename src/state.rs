pub mod compositor;
pub mod output;
pub mod shm;
pub mod window;

use core::panic;

use smithay_client_toolkit::{
    compositor::CompositorState,
    delegate_compositor, delegate_registry,
    output::OutputState,
    registry::{self, ProvidesRegistryState, RegistryState},
    registry_handlers,
    shell::{
        xdg::{
            window::{Window, WindowDecorations},
            XdgShell,
        },
        WaylandSurface,
    },
    shm::{
        slot::{Buffer, SlotPool},
        Shm,
    },
};
use wayland_client::{
    globals::{registry_queue_init, GlobalList},
    Connection, EventQueue, QueueHandle,
};

use crate::snake::{Game, Snake};

pub struct GfState {
    queue_handle: QueueHandle<Self>,

    registry_state: RegistryState,
    output_state: OutputState,
    compositor_state: CompositorState,

    xdg_shell: XdgShell,
    shm: Shm,
    pool: Option<SlotPool>,
    window: Option<Window>,
    buffer: Option<Buffer>,

    first_configure: bool,
    width: u32,
    height: u32,

    game: Box<dyn Game>,
}

impl GfState {
    pub fn new(global_list: &GlobalList, qh: &QueueHandle<Self>) -> Self {
        GfState {
            queue_handle: qh.clone(),

            registry_state: RegistryState::new(global_list),
            output_state: OutputState::new(global_list, qh),
            compositor_state: match CompositorState::bind(global_list, qh) {
                Ok(compositor) => compositor,
                Err(err) => panic!("Failed to bind compositor.\nErr: {err}"),
            },

            xdg_shell: match XdgShell::bind(global_list, qh) {
                Ok(xdg_shell) => xdg_shell,
                Err(err) => panic!("Failed to bind XdgShell.\nErr: {err} "),
            },
            shm: match Shm::bind(global_list, qh) {
                Ok(xdg_shell) => xdg_shell,
                Err(err) => panic!("Failed to bind XdgShell.\nErr: {err} "),
            },
            pool: None,
            window: None,
            buffer: None,

            first_configure: true,
            height: 256,
            width: 256,

            game: Box::new(Snake::new()),
        }
    }
    pub fn init_window(&mut self, title: &str, app_id: &str, min_size: (u32, u32)) {
        self.window = Some(self.xdg_shell.create_window(
            self.compositor_state.create_surface(&self.queue_handle),
            WindowDecorations::None,
            &self.queue_handle,
        ));
        let window = self.window.as_ref().expect("Created directly above.");

        self.pool = Some(SlotPool::new(256 * 256 * 4, &self.shm).expect("Failed to create pool."));

        window.set_title(title);
        window.set_app_id(app_id);
        window.set_min_size(Some(min_size));

        window.commit();
    }
    pub fn compositor_state(&self) -> &CompositorState {
        &self.compositor_state
    }
    pub fn xdg_shell(&self) -> &XdgShell {
        &self.xdg_shell
    }
    pub fn slot_pool(&mut self) -> &mut SlotPool {
        match self.pool.as_mut() {
            Some(pool) => pool,
            None => panic!("No slot pool in state. Have you called `init_window()`?"),
        }
    }
    pub fn window(&self) -> &Window {
        match self.window.as_ref() {
            Some(window) => window,
            None => panic!("No window in state. Have you called `init_window()`?"),
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
