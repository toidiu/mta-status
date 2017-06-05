pub mod xml_client;
pub mod parse_xml;


#[cfg(test)]
mod tests {

    use super::xml_client;

    #[test]
    fn it_fails() {
        ::xml_client::connect();
    }
}