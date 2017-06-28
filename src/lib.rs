//#![deny(warnings)]
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate html5ever;

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
