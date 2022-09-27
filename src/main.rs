#[cfg(feature = "smawk")] {
let text = "textwrap: an efficient and powerful library for wrapping text.";
assert_eq!(
    textwrap::wrap(text, 28),
    vec![
        "textwrap: an efficient",
        "and powerful library for",
        "wrapping text.",
    ]
);
}