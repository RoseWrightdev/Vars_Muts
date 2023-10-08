fn main() {
    let spaces = "";
    let spaces = spaces.len();
    // works bceause we're changing types^ w/ shadowing

    // dosen't work because we're changing types without redeclaring
    /*
    let spaces = "";
    spaces = spaces.len();
     */
    println!("{}",spaces)

}