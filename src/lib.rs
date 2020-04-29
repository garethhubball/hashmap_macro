/// Creates a [`std::collections::HashMap`] containing the arguments.
/// 
/// `hashmap!` allows `HashMap`s to be defined with simple `=>` syntax much line in ruby.
///
/// Sample usage:
/// ```
/// let result = hashmap!["Foo" => "Bar", "Baz" => "Quux"];
/// assert_eq!(result["Foo"], "Bar");
/// assert_eq!(result["Baz"], "Quux");
/// assert_eq!(result.len(), 2);
/// ```
#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($key:expr => $value:expr),+) => {{
        let mut hm = ::std::collections::HashMap::with_capacity(count_items!(@COUNT, $($key),*));
        $(hm.insert($key, $value);)+
        hm
    }};

    ($($key:expr => $value:expr,)+) => { hashmap![$($key => $value),+] };
}

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! count_items {
    (@COUNT, $($item:expr),*) => {
        <[()]>::len(&[$(count_items!(@SUBST, $item, ())),*])
    };

    (@SUBST, $_x:expr, $sub:expr) => {
        ()
    };
}

#[test]
pub fn empty() {
    use std::collections::HashMap;

    let result: HashMap<(), ()> = hashmap!();
    assert_eq!(result.len(), 0);
}

#[test]
pub fn single() {
    let result = hashmap!["Foo" => "Bar"];
    assert_eq!(result.len(), 1);
    assert_eq!(result["Foo"], "Bar");
}

#[test]
pub fn double() {
    let result = hashmap!["Foo" => "Bar", "Baz" => "Quux"];
    assert_eq!(result.len(), 2);
    assert_eq!(result["Foo"], "Bar");
    assert_eq!(result["Baz"], "Quux");
}

#[test]
pub fn double_with_trailing_comma() {
    let result = hashmap!["Foo" => "Bar", "Baz" => "Quux",];
    assert_eq!(result.len(), 2);
    assert_eq!(result["Foo"], "Bar");
    assert_eq!(result["Baz"], "Quux");
}