mod helperfunctions;
mod graphics;
use helperfunctions::{clear, printout};
use graphics::star_generator;
//use std::arch::x86_64::_mm_test_epi32_mask;
use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use log_update::LogUpdate;

const DEFAULTTEXTSPEED: u64 = 70;

fn main() {
    let mut log_update = LogUpdate::new(stdout()).unwrap();
   // clear();
    printout("To Gardens Where We Feel Secure\nAn Evocation\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\nMoira Oona Morse", 125);
    sleep(Duration::from_secs(2));
   // clear();
    printout("If we stand on a hilltop swallowing stars and picking grass from dirt, when we talk to ourselves let us ask why we still write the way we do.\n\n", DEFAULTTEXTSPEED);
    let (white_stars, black_stars) = star_generator(95, 11);
    for _ in 0..25 {
        log_update.render(&white_stars).unwrap();
        sleep(Duration::from_millis(320));
        log_update.render(&black_stars).unwrap();
        sleep(Duration::from_millis(320));
    }
    let mut sky_writing: [char; 5] = ['t', 'e', 's', 't', '!'];
    for mut i in white_stars.chars() {
        if let mut i = ' ' {
            i = 't';
        }
    }
    for _ in 0..25 {
        log_update.render(&white_stars).unwrap();
        sleep(Duration::from_millis(320));
        log_update.render(&black_stars).unwrap();
        sleep(Duration::from_millis(320));
    }
}

