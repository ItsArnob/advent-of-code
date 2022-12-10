pub fn run(data: &String) {
    let lines = data.split("\n");
    
    let mut cpu_cycle = 0;
    let mut x_register = 1;
    let mut signal_strength = 0;
    let mut last_pixel_pos = 0;
    
    println!("part 1:");
    for line in lines {
        let instruction = line.trim().split(" ").collect::<Vec<&str>>();
        
        if instruction[0] == "noop" {
            cpu_cycle += 1;
            execute_instruction(cpu_cycle, &mut x_register, 0, true, false, &mut signal_strength, &mut last_pixel_pos);
        } else {

            // addx requires two cpu cycle.
            for i in 0..2 {
                cpu_cycle += 1;
                execute_instruction(cpu_cycle, &mut x_register, instruction[1].parse::<i32>().unwrap(),false, i == 1, &mut signal_strength, &mut last_pixel_pos);
            }
        }

    }
    println!("part 1: {}", signal_strength);
}
fn execute_instruction(cpu_cycle: i32, x_register: &mut i32, to_add: i32 ,is_noop: bool, is_second_cycle: bool, signal_strength: &mut i32, last_pixel_pos: &mut i32) {
    
    

    if [220, 180, 140, 100, 60, 20].contains(&cpu_cycle) {
        *signal_strength += cpu_cycle * *x_register;
    }

    // sprites are 3 pixel wide and the x_register representws the *MIDDLE* pixel of a sprite.
    // so lets say, we have a crt row that starts from x = 0 and ends at x = 6
    // if x_register = 2, then that sprites' starting pixel would be at x_register - 1 = 1 and end pixel would be at x_register + 1 = 3 because of the reason stated above.
    // so in that plane the sprites position would be from x = 1 to x = 3.
    // we apply the same logic here to create a range for that sprites' position. instead of +1 we are doing +2 because this is a range because,
    // x = 3 doesn't count as part of 1..3 range. so we make a 1..4 range.
    let sprite_pos = *x_register - 1..*x_register + 2;

    // initial solution to get the pixel position without last_pixel_pos
    // let pixel_pos_possibly_negative = cpu_cycle - ((cpu_cycle / 40) * 40 + 1);
    // let pixel_pos = if  pixel_pos_possibly_negative == -1 { 39 } else { pixel_pos_possibly_negative };


    let pixel_pos = cpu_cycle - *last_pixel_pos - 1; // -1 because pixel position starts at 0.
    if sprite_pos.contains(&pixel_pos) {
        print!("#");
    } else {
        print!(".");
    }
    // 39 is the last pixel position of a row.
    if pixel_pos == 39 {
        print!("\n");
        *last_pixel_pos = cpu_cycle;
    }

    // value of addx is only added in the cycle after the one it started executing in.
    if !is_noop && is_second_cycle {
        *x_register += to_add;
    }
    
}