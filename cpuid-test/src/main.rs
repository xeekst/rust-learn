extern crate cpuid;

fn main () {
    match cpuid::identify() {
        Ok(info) => {
            println!("Found: {} CPU, model: {}", info.vendor, info.codename);
            println!("The full brand string is: {}", info.brand);
            println!("Hardware AES support: {}", if info.has_feature(cpuid::CpuFeature::AES) { "yes" } else { "no" });
        },
        Err(err) => println!("cpuid error: {}", err),
    };
    match cpuid::clock_frequency() {
        Some(frequency) => println!("CPU speed: {} MHz", frequency),
        None => println!("Couldn't get CPU speed."),
    };
}