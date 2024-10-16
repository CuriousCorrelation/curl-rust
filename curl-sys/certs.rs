const CACERT_PEM: &str = include_str!("cacert.pem");

pub fn get_cert_content() -> &'static str {
    CACERT_PEM
}
