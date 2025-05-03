/// Takes a URI and a label, returns a UNIX-compliant hyperlink
/// See the definition of this function for more details.
pub fn new(uri: &String, label: &String) -> String {
    // This is a temporary way of creating a hyperlink until we find a suitable crate
    // to handle this kind of use cases -> A maintained crated specialized in this use case will be
    //                                       safer to use than inserting ourselves the escape sequences.
    // For more details, see:
    //  - Terminal hyperlink -> https://askubuntu.com/questions/1391071/creating-a-hyperlink-from-command-line-output-on-a-terminal
    //  - Rust Hexadecimal escape characters -> https://stackoverflow.com/a/33139393/13025136

    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\\n", uri, label)
}
