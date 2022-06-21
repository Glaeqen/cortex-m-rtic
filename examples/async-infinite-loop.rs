#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, UART0], peripherals = true)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init").unwrap();

        foo::spawn().ok();

        (
            Shared {},
            Local {},
            init::Monotonics(Systick::new(cx.core.SYST, 12_000_000)),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi(); // put the MCU in sleep mode until interrupt occurs
        }
    }

    // Infinite loops are not allowed in RTIC, however in async tasks they are - in there is an
    // await inside the loop.
    #[task]
    async fn foo(_cx: foo::Context) {
        let mut i = 0;
        loop {
            if i == 5 {
                debug::exit(debug::EXIT_SUCCESS);
            }

            hprintln!("hello from async {}", i).ok();
            monotonics::delay(100.millis()).await; // This makes it okey!

            i += 1;
        }
    }
}
