extern crate clap;
use clap::{Arg, App};
mod raytracing;

fn run_project(name: &str, func: fn() -> std::io::Result<()>)  {
    println!("\n================== Running: {} ==================\n", name);
    let result = func();
    let success = result.is_ok();
    println!("\n================== Success={} ==================\n", success);
}


fn main() {
    let matches = App::new("Hao's Raytracing in Rust")
        .version("1.0")
        .author("Hao Luo")
        .about("Learning Raytracing")
        .arg(Arg::with_name("INPUT")
            .help("Which file to run")
            .required(true)
            .index(1))
        .get_matches();
    
    let name = matches.value_of("INPUT").unwrap();

    match name {
        "2" => run_project(&name, raytracing::simple_ppm),
        "4" => run_project(&name, raytracing::simple_viewport),
        "5" => run_project(&name, raytracing::simple_sphere),
        "6.1" => run_project(&name, raytracing::surface_normals),
        _ => println!("sorry the function {} does not exist", name)
    }
}
