use smithay_client_toolkit::output::{OutputHandler, OutputState};
use wayland_client::Connection;

use super::GfState;


impl OutputHandler for GfState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }
    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _output: wayland_client::protocol::wl_output::WlOutput,
    ) {
        println!("New output Advertised!!!");
    }
    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _output: wayland_client::protocol::wl_output::WlOutput,
    ) {
    }
    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &wayland_client::QueueHandle<Self>,
        _output: wayland_client::protocol::wl_output::WlOutput,
    ) {
    }
}
