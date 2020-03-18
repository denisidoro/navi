use navi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = navi::handle_config(navi::config_from_iter("navi best trivial".split(' ').collect()));
        // assert_eq!(x, 3);
    }
}
