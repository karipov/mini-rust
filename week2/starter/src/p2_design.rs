//! In this file, you will design functions to implement a high-level specification.
//! The goal is to have you explore the different possible implementations of a spec in Rust,
//! and to articulate the trade-offs in terms of generality, performance, and usability.

// EXAMPLE: below is a completed function that demonstrates how each problem will work.
// Each problem contains a specification above the function. Your goal is to design the function
// signature and implementation. For each parameter and the return type, you should describe
// (a) a reasonable space of possible types, and (b) your rationale for picking a particular type.
// Make sure to add at least one unit test for your particular implementation.

/// round_all is a function that takes:
///   * v: representing a collection of numbers
/// and rounds every number in-place in v to the nearest integer.
pub fn round_all(
    // (1) v could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &mut[_] because
    //     we do not need to change the size or order of the collection, but do need to change the elements.
    // (2) v could be a &mut [{number type}], and "round to the nearest integer" implies the use of floats.
    // (3) The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
    //     function that works for both types, so we arbitrarily pick f32 for now.
    v: &mut [f32],
)
// No return value, since this function only mutates an input.
{
    for n in v.iter_mut() {
        *n = n.round();
    }
}

#[test]
fn round_all_test() {
    let mut v = vec![0.3, 0.7];
    round_all(&mut v);
    assert_eq!(v, vec![0., 1.]);
}

// Now you try!

/// P2a: find_contains is a function that takes:
///   * haystack: representing a collection of strings
///   * needle: representing a particular string
/// and returns a value:
///   * representing which strings in the collection contain the needle
pub fn find_contains<'a>(
    // (1) haystack could be a Vec<String>, &Vec<String>, &Vec<&str>, &[&str]. I choose &[&str]
    //     because I do not need ownership of the string and we do not need to change the size of the
    //     input collection, therefore a slice.
    // (2) needle could be String, &str. I choose &str due to above reasons!
    // also i'm getting a bunch of lifetime problems... i did what the rust compiler told me
    haystack: &'a [&str],
    needle: &str,
) -> Vec<&'a str> /* We return pointers to the string literals containing the needle. I use a Vec
                    type here because arrays are created on the stack and we also don't know how
                    many such valid strings we may get */
{
    let mut hays: Vec<&str> = Vec::new();
    for &hay in haystack.iter() {
        if hay.contains(needle) {
            hays.push(hay);
        }
    }

    hays
}

#[test]
fn find_contains_test() {
    let v = ["abc", "de", "ab", "a", "f"];
    let out = find_contains(&v, "a");

    assert_eq!(out, vec!["abc", "ab", "a"]);
}

/// P2b: fill_progress_bar is a function that takes:
///   * buf: a string to fill
///   * delims: a pair of delimiters to wrap the bar
///   * frac: the fraction of the bar to display
/// Then places a textual representation of the progress bar into `buf`.
/// For example, at a progress of 20% with bracketed delimiters, the bar would be:
///   [==        ]
pub fn fill_progress_bar(
    // (1) buf could be a Vec<String>, &mut Vec<String> [question: does this mutably borrow, ref, or consume String?]
    //     String, &mut String, &mut str, &mut char. I choose &mut char because I do not need to change the length
    //     of the string buffer I receive. Using a &mut str is confusing and gives me errors...
    // (2) delims could be a String, &String, &str, &Vec<char>, &[char]. I choose &[char] because that seems
    //     most reasonable from given
    // (3) frac can be any f-type, i-type, u-type. I chose f32 for simplicity of programming,
    //     so percentages have to be represented as floats between 0 and 1 and can be easily multiplied
    buf: &mut [char],
    delims: &[char],
    frac: f32,
)
// No return value, since this function only mutates an input.
{
    let buf_len = buf.len();
    let mut num_to_fill: u32 = (((buf_len - 2) as f32) * frac) as u32;
    println!("n: {num_to_fill}");

    for (i, a_char) in buf.iter_mut().enumerate() {
        *a_char = if i == 0 {
            delims[0]
        } else if i == buf_len - 1 {
            delims[1]
        } else {
            if num_to_fill > 0 {
                num_to_fill -= 1;
                '='
            } else {
                ' '
            }
        }
    }
}

#[test]
fn test_fill_progress_bar() {
    let mut bar = ['_', '_', '_', '_', '_', '_', '_'];
    let mut big_bar = ['_'; 12];

    fill_progress_bar(&mut bar, &['[', ']'], 0.2);
    fill_progress_bar(&mut big_bar, &['{', '}'], 0.6);

    assert_eq!(String::from_iter(bar), String::from("[=    ]"));
    assert_eq!(String::from_iter(big_bar), String::from("{======    }"));
}
