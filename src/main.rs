use std::time::Duration;

use sctk_simple_window::state::{get_globals, GfState};
use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    reexports::{calloop::EventLoop, calloop_wayland_source::WaylandSource},
    registry::RegistryState,
    shell::xdg::XdgShell,
};
use wayland_client::Connection;

fn main() {
    env_logger::init();

    let conn = match Connection::connect_to_env() {
        Ok(conn) => conn,
        Err(err) => {
            panic!("Unable to connect to wayland compositor, is it running?\nErr: {err}")
        }
    };

    let (global_list, event_queue) = get_globals(&conn);
    let qh = event_queue.handle();

    let mut event_loop: EventLoop<GfState> =
        EventLoop::try_new().expect("Failed to initialize event loop!");
    let loop_handle = event_loop.handle();

    WaylandSource::new(conn.clone(), event_queue)
        .insert(loop_handle)
        .unwrap();

    let mut gf_state = GfState {
        registry_state: RegistryState::new(&global_list),
        output_state: OutputState::new(&global_list, &qh),
    };

    let compositor = match CompositorState::bind(&global_list, &qh) {
        Ok(compositor) => compositor,
        Err(err) => panic!("Failed to bind compositor.\nErr: {err}"),
    };
    compositor.create_surface(&qh);

    let xdg_shell = match XdgShell::bind(&global_list, &qh) {
        Ok(xdg_shell) => xdg_shell,
        Err(err) => panic!("Failed to bind XdgShell.\nErr: {err} "),
    };

    loop {
        match event_loop.dispatch(Duration::from_millis(16), &mut gf_state) {
            Ok(_) => println!("Loop"),
            Err(err) => panic!("Failed to dispatch event.\nErr: {err}"),
        };
    }
}
