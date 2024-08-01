#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
use ucosii as _;
use defmt::Format; // <- derive attribute
#[derive(Format)]
struct S1<T> {
    x: u8,
    y: T,
}

#[derive(Format)]
struct S2 {
    z: u8,
}
// #[ucosii_executor_macro::task]
async fn hello() {
    defmt::info!("Hello, world!");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let s = S1 {
        x: 42,
        y: S2 { z: 43 },
    };
    hello();
    defmt::println!("s={:?}", s);
    let x = 42;
    defmt::println!("x={=u8}", x);

    ucosii::lang_items::exit()
}