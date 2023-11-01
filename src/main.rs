fn concatenate_strings(s1: &str, s2: &str) -> String {
    // The parameters s1 and s2 follow ownership rules. 
    // The function borrows these parameters and doesn't take ownership.

    let mut result = String::new(); // The result String takes ownership here.

    result.push_str(s1); // References to s1 & s2 are provided, and ownership isn't transferred.
    result.push_str(s2);

    // The result String is returned at the end of the function, transferring ownership.
    result // As a result, ownership is transferred and returns this String.
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, "); // string1 and string2 take ownership.
    let string2 = String::from("Rust!");

    // Call the concatenate_strings function with references to string1 and string2.
    let concatenated_string = concatenate_strings(&string1, &string2);
    // References to &string1 and &string2 are provided, so ownership isn't transferred.

    println!("{}", concatenated_string); // concatenated_string takes ownership and is printed.

}

