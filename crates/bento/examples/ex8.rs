// A function that returns the longer of two string slices.
// The lifetime annotation 'a tells the compiler:
// "the returned reference will live at least as long as the shorter of x and y"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result); // ✅ Works fine
    }

    // println!("{}", result); // ❌ Would fail — string2 is dropped, result can't outlive it
}
