mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    // my::function();
    // function();
    // my::indirect_access();
    // my::nested::function();

    // my::public_function();  // not accessible
    // my::inaccessible::public_function();  // requires pub mod
}
