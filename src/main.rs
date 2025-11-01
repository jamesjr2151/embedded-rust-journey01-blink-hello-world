#![no_std]
#![no_main]

fn main() -> ! {
    // Create delay utility
    let mut delay = Delay::new_default();

    println!("ESP32 Blinky with Rust started!");
    println!("LED on GPIO2 will blink every 1 second");

    loop {
        led.set_high().unwrap(); // LED ON
        println!("LED ON");
        delay.delay_ms(1000u32);

        led.set_low().unwrap(); // LED OFF
        println!("LED OFF");
        delay.delay_ms(1000u32);
    }
}
