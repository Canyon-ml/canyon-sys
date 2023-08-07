
extern crate canyon_cuda_sys;
extern crate canyon_cudnn_sys;

mod cuda {
    pub use cuda_sys::*;
}

mod cudnn {
    pub use cudnn_sys::*;
}
