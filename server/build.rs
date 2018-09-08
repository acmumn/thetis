extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("src/auth/capabilities")
        .process()
        .unwrap();
}
