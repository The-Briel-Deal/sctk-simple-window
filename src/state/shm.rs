use smithay_client_toolkit::{
    delegate_shm,
    shm::{Shm, ShmHandler},
};

use super::GfState;

delegate_shm!(GfState);
impl ShmHandler for GfState {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}
