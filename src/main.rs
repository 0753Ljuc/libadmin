use libadmin::launch;

#[cfg(not(feature="mock"))]
fn main() {
    launch().unwrap();
}
