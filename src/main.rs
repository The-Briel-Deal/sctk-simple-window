use std::time::Duration;

use sctk_simple_window::state::{get_globals, GfState};
use smithay_client_toolkit::{
    reexports::{calloop::EventLoop, calloop_wayland_source::WaylandSource},
    shell::{xdg::window::WindowDecorations, WaylandSurface},
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

    let mut gf_state = GfState::new(&global_list, &qh);
    gf_state.init_window(
        "Gabriel's First Window (:",
        "gabriel.loves.cute.dogs",
        (256, 256),
    );

    loop {
        match event_loop.dispatch(Duration::from_millis(16), &mut gf_state) {
            Ok(_) => println!("Loop"),
            Err(err) => panic!("Failed to dispatch event.\nErr: {err}"),
        };
    }
}
