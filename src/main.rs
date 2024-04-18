mod header_new;
mod header_old;

fn main() {
    println!("generate header using old and new implementation");

    let file = "./test.bin";
    let f = std::fs::read(file).expect("cannot open file");

    let o = header_old::spl_create_hdr(f.clone());
    let n = header_new::spl_create_hdr(f);

    if o == n {
        println!("results are equal");
    } else {
        println!("results differ");
    }
}
