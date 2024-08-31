// use core::future::Future;

// use crate::port::INT8U;

// pub struct Semaphore{
    
// }

// impl Future for Semaphore{
//     type Output = ();
//     fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
//         todo!()
//     }
// }

// pub trait SemDriver{
//     fn new(count: INT8U) -> Self;
//     fn acquire(&self);
//     fn release(&self);
// }

// use crate::port::INT8U;


// pub struct SemHandle {
//     id: INT8U,
// }

// impl SemHandle{
//     pub fn new(id:INT8U)->Self{
//         Self{
//             id
//         }
//     }
// }