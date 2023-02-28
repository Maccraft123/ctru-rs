use ctru::prelude::*;
use ctru::services::ptmu::Ptmu;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let ptmu = Ptmu::init().expect("Couldn't obtain Ptmu controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    // Main loop
    let mut i: u8 = 0;
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        // Refresh only every 16 frames
        if i % 16 == 0 {
            // Clear console
            print!("\x1b[2J");

            // Get and print out power data
            let shell_state = ptmu.get_shell_state().expect("Failed to get shell state");
            let battery_level = ptmu.get_battery_level().expect("Failed to get battery level");
            let battery_charging = ptmu.is_battery_charging().expect("Failed to get battery charge status");
            let adapter_plugged_in = ptmu.is_adapter_plugged_in().expect("Failed to get adapter status");
            let pedometer_counting = ptmu.is_pedometer_counting().expect("Failed to get pedometer status");
            let steps = ptmu.get_total_step_count().expect("Failed to get step count");

            println!("Shell state: {:?}", shell_state);
            println!("Battery level(0-5): {}", battery_level);
            if battery_charging {
                println!("Battery is charging")
            } else {
                println!("Battery is not charging")
            }
            
            if adapter_plugged_in {
                println!("Power supply plugged in")
            } else {
                println!("Power supply not plugged in")
            }

            if pedometer_counting {
                println!("Pedometer is counting")
            } else {
                println!("Pedometer is not counting")
            }

            println!("Total step count: {steps}");
            println!("Press Start to exit");
        }


        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();

        i = i.wrapping_add(1);
    }
}
