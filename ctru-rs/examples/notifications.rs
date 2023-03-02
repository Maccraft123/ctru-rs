use ctru::prelude::*;
use ctru::services::srv::Srv;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let srv = Srv::init().expect("Couldn't obtain SRV controller");
    let bottom_screen = Console::init(gfx.bottom_screen.borrow_mut());
    let top_screen = Console::init(gfx.top_screen.borrow_mut());

    bottom_screen.select();
    println!("Press Home button or plug in/out charger!");
    println!("Press Start to exit");

    top_screen.select();

    let sem = srv.enable_notification().expect("Failed to enable notifications");


    sem.subscribe(0x20D).unwrap();
    sem.subscribe(0x20E).unwrap();
    sem.subscribe(0x20F).unwrap();
    sem.subscribe(0x210).unwrap();
    sem.subscribe(0x204).unwrap();
    sem.subscribe(0x205).unwrap();

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if let Some(n) = sem.poll_notification().expect("Failed to poll for notification") {
            match n {
                0x20D => println!("Charger plugged out!"),
                0x20E => println!("Charger plugged in!"),
                0x20F => println!("Started charging!"),
                0x210 => println!("Stopped charging!"),
                0x204 => println!("Home button pressed!"),
                0x205 => println!("Home button released!"),
                other => println!("Other notification: {other}"),
            }
        }

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
