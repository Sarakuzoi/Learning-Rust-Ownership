fn main() {
    let mut s1: String = String::from("hello");

    //Borrowing the value of s1 on as many immutable references
    let r1: &String = &s1;
    let r2: &String = &s1;

    println!("r1: {}, r2: {}", r1, r2);
    //r1, r2 are now out of scope, so I can use a mutable reference
    //to the same value s1

    let mut len: usize = calculate_length(&s1);
    println!("length of s1: {}", len);

    //Value of s1 is changed as it is borrowed mutably by the function,
    //thus the length is also updated
    change(&mut s1);
    len = calculate_length(&s1);
    println!("new length of s1: {}", len);

    //As all the immutable references have gone out of scope,
    //we can use a mutable references

    //We cannot have more than one mutable reference in the same scope
    {
        let tmp_r3: &mut String = &mut s1;
        change(tmp_r3);
        println!("tmp_r3: {}", tmp_r3);
    }
    //tmp_r3 is now out of scope and dropped

    let r3: &mut String = &mut s1;

    println!("r3: {}", r3);
    //r3 is now out of scope, and also dropped
}

//With a mutable reference, it is clear that the "change"
//function modifies the value it borrows
fn change(str: &mut String) {
    str.push_str(", world");
}

//We here use a reference, as String does not implement
//the Copy trait and we don't want s1 moved to this scope
fn calculate_length(s: &String) -> usize {
    s.len()
}

//It is also important not to let through dangling pointers,
//so the compiler makes sure such behaviour does not appear.
//Thus, the following code will not run:
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
//Here s goes out of scope, yet we will still have a reference
//to the now dropped memory. Thus, the following function is favored:
fn _no_dangle() -> String {
    let s: String = String::from("hello");

    s
}
//Here we return the full String
