use wifi_rs::{prelude::*, WiFi};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: None,
    });

    let mut wifi = WiFi::new(None);
    //let interface = WifiInterface::turn_on();
    match wifi.connect("iPhone13", "580580580") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successfull."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    Ok(())
}