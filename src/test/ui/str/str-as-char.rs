// run-rustfix

fn main() {
    println!('●●'); //~ ERROR character literal may only contain one codepoint
    //~^ ERROR format argument must be a string literal
}
