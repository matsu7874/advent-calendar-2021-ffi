fn main() {
    println!("cargo:rustc-link-search=native=/home/matsumoto/work/ffi_sample/legendary_c_lib");
    println!("cargo:rustc-link-lib=static=legend");
}
