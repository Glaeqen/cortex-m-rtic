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
        bar::spawn().ok();
        baz::spawn().ok();

        (
            Shared {},
            Local {},
            init::Monotonics(Systick::new(cx.core.SYST, 12_000_000)),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // debug::exit(debug::EXIT_SUCCESS);
        loop {
            // hprintln!("idle");
            cortex_m::asm::wfi(); // put the MCU in sleep mode until interrupt occurs
        }
    }

    #[task]
    async fn foo(_cx: foo::Context) {
        hprintln!("hello from foo").ok();
        monotonics::delay(100.millis()).await;
        hprintln!("bye from foo").ok();
    }

    #[task]
    async fn bar(_cx: bar::Context) {
        hprintln!("hello from bar").ok();
        monotonics::delay(200.millis()).await;
        hprintln!("bye from bar").ok();
    }

    #[task]
    async fn baz(_cx: baz::Context) {
        hprintln!("hello from baz").ok();
        monotonics::delay(300.millis()).await;
        hprintln!("bye from baz").ok();

        debug::exit(debug::EXIT_SUCCESS);
    }
}
