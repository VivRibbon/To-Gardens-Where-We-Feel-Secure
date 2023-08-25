use rand::prelude::*;

pub fn star_generator(columns: u8, rows: u8) -> (String, String) {
    let mut rng = rand::thread_rng(); 
    let mut star_count = rng.gen_range(1..=100);
    let mut white_stars = String::new();
    let mut black_stars = String::new();
    for _ in 1..=rows {
       for _ in 1..=columns {
           match star_count {
               1..=92 => white_stars.push(' '),
               93..=100 => white_stars.push('✦'),
               _ => white_stars.push(' ')
           }
       star_count = rng.gen_range(1..=100);
       }
       white_stars.push('\n');
    }
    for i in white_stars.chars() {
        match i {
            '✦' => black_stars.push('✧'),
            ' ' => black_stars.push(' '),
            '\n'=> black_stars.push('\n'),
            _ => black_stars.push(' ')
        }
    }
    (white_stars, black_stars)
}

