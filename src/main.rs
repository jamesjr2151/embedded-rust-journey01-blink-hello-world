#![no_std]
#![no_main]

use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

#[no_mangle]
fn main() -> ! {
    // Temporary workaround for ESP32 entry point
    esp_idf_sys::link_patches();
    
    // Take peripherals
    let peripherals = Peripherals::take().unwrap();
    
    // Configure GPIO2 as output (built-in LED on most ESP32 dev boards)
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();
    
    // Create delay utility
    let mut delay = Delay::new_default();
    
    println!("ESP32 Blinky with Rust started!");
    println!("LED on GPIO2 will blink every 1 second");
    
    loop {
        led.set_high().unwrap();  // LED ON
        println!("LED ON");
        delay.delay_ms(1000u32);
        
        led.set_low().unwrap();   // LED OFF  
        println!("LED OFF");
        delay.delay_ms(1000u32);
    }
}