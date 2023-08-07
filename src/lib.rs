
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod cuda {
    include!("cuda_bindings.rs");
}

mod cudnn {
    include!("cudnn_bindings.rs");
}
