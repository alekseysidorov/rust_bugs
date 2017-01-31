#[cfg(feature="crazy")]
pub fn i_am_crazy() {
    println!("I am crazy");
}

#[cfg(not(feature="crazy"))]
pub fn i_am_normal() {
    println!("I am normal");
}
