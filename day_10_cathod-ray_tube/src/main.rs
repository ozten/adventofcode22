use std::io::Error;

mod cathode_ray_tube;

mod cpu;

mod parser;
use cathode_ray_tube::CathodeRayTube;
use parser::parse;


fn main() -> Result<(), Error> {
    let mut cathod_ray_tube = CathodeRayTube::new();

    let instructions = parse("src/input.txt");
    println!("{:?}", &instructions);

    cathod_ray_tube.simulate(& instructions);


    Ok(())
}
