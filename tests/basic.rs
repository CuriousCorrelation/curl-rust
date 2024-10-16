use curl::easy::Easy;
use std::time::Instant;

#[test]
fn test_basic_https_connection() {
    let mut easy = Easy::new();

    easy.url("https://www.example.com").unwrap();
    easy.perform().unwrap();

    assert_eq!(easy.response_code().unwrap(), 200);
}

#[test]
fn test_tls_versions() {
    for version in &[
        curl::easy::SslVersion::Tlsv10,
        curl::easy::SslVersion::Tlsv11,
        curl::easy::SslVersion::Tlsv12,
        curl::easy::SslVersion::Tlsv13,
    ] {
        let mut easy = Easy::new();

        easy.url("https://www.example.com").unwrap();
        easy.ssl_version(*version).unwrap();
        easy.perform().unwrap();

        assert_eq!(easy.response_code().unwrap(), 200);
    }
}

#[test]
fn test_invalid_certificate() {
    let mut easy = Easy::new();

    easy.url("https://expired.badssl.com/").unwrap();
    easy.ssl_verify_peer(true).unwrap();

    assert!(easy.perform().is_err());
}

#[test]
fn test_performance() {
    let mut easy = Easy::new();

    easy.url("https://www.example.com").unwrap();

    let start = Instant::now();

    easy.perform().unwrap();

    let duration = start.elapsed();

    assert!(duration.as_secs() < 5);
}

#[test]
fn test_cipher_selection() {
    let mut easy = Easy::new();

    easy.url("https://www.example.com").unwrap();
    easy.ssl_cipher_list("ECDHE-RSA-AES256-GCM-SHA384").unwrap();
    easy.perform().unwrap();

    assert_eq!(easy.response_code().unwrap(), 200);
}

#[test]
fn test_timeout() {
    let mut easy = Easy::new();

    easy.url("https://httpbin.org/delay/10").unwrap();
    easy.timeout(std::time::Duration::from_secs(5)).unwrap();

    assert!(easy.perform().is_err());
}

#[test]
fn test_alpn_http2() {
    let mut easy = Easy::new();

    easy.url("https://http2.golang.org").unwrap();

    if let Ok(_) = easy.http_version(curl::easy::HttpVersion::V2) {
        easy.perform().unwrap();

        assert_eq!(easy.response_code().unwrap(), 200);
    } else {
        println!("HTTP/2 not supported, skipping test");
    }
}

#[test]
fn test_sni() {
    let mut easy = Easy::new();

    easy.url("https://www.cloudflare.com").unwrap();
    easy.perform().unwrap();

    assert_eq!(easy.response_code().unwrap(), 200);
}

#[test]
fn test_large_data_transfer() {
    let mut easy = Easy::new();

    easy.url("https://speed.cloudflare.com/__down?bytes=104857600")
        .unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    assert_eq!(data.len(), 100 * 1024 * 1024);
}
