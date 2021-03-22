use package_manager::statics::LIB_DIR;

fn main() {
    println!("{}", LIB_DIR.to_str().unwrap());
}
