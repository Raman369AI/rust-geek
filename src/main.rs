fn main() {
    ruler(5,6)
}



fn draw_tick(length:u32) {
    let mut tick = "-";
    let repeated = tick.repeat(length as usize);
    println!("{}", repeated);
    
}



fn draw_interval(length:u32) {
    let mut tick = "-";
    if length > 0{draw_interval(length-1);
    draw_tick(length);
    draw_interval(length-1);
    }
}





fn ruler(length:u32,ent:u32) {
    let tick = "-";
    let repeated = tick.repeat(length as usize);
    println!("{}", repeated);
    for i in 0..ent {
        draw_interval(length);
    }

}