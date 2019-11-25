use ptagslib::bin::run;

// ---------------------------------------------------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------------------------------------------------

fn main() {
    match run() {
        Err(x) => {
            println!("{}", x);
            for x in x.iter_chain() {
                println!("{}", x);
            }
        }
        _ => (),
    }
}
