fn main() {
    println!("cargo:rerun-if-changed=components/*.wit");
}
