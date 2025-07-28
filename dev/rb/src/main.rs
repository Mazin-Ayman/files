use pico_args::Arguments;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Args {
    name: String,
    age: u8,
    gender: Gender,
    is_useless: bool,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = Arguments::from_env();
    let args = Args {
        name: pargs.free_from_str()?,
        age: pargs.value_from_str(["--age", "-a"])?,
        gender: {
            if pargs.contains(["-m", "--male"]) {
                Gender::Male
            } else if pargs.contains(["-f", "--female"]) {
                Gender::Female
            } else {
                Gender::Male
            }
        },
        is_useless: pargs.contains(["-i", "--useless"]),
    };
    Ok(args)
}

fn main() {
    let args: Args = parse_args().unwrap();
    println!("{:?}", args);
}
