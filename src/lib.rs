
extern crate cuda_sys;
extern crate cudnn_sys;

mod cuda {
    pub use cuda_sys::*;
}

mod cudnn {
    pub use cudnn_sys::*;
}
