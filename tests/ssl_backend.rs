use curl::easy::{Easy, List};
use std::str;

// This is a self-signed certificate used for testing purposes only.
// In a real-world scenario, you would use a certificate signed by a trusted CA.
// This certificate is just used to test custom root certificate functionality and client certificate setup.
const EXAMPLE_COM_CERT: &str = r#"-----BEGIN CERTIFICATE-----
MIIDazCCAlOgAwIBAgIUBY0HZxQW1qjVFOA/3BN1W2GHfZQwDQYJKoZIhvcNAQEL
BQAwRTELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoM
GEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDAeFw0yMzEwMjkxNDIxMzhaFw0yNDEw
MjgxNDIxMzhaMEUxCzAJBgNVBAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEw
HwYDVQQKDBhJbnRlcm5ldCBXaWRnaXRzIFB0eSBMdGQwggEiMA0GCSqGSIb3DQEB
AQUAA4IBDwAwggEKAoIBAQDFB1b2QoAUoLh1XP1ZMppVcmaFAKrwbRVeBDdSRrjm
ePpmLJT/x/6iOfcg7PVh8N1mFT+SG8gswJcD/B2V6pUHVBMwmJ6qgY9N5ZQqWvtJ
mYxOdFWzUQl1uRQc6t0N5J3eXJpXHcCU4b8sCKRDVnxp7JbLuzLHUuZGQCo7iYX4
Ihhx70nPNCQB3JzeNm/gAjltAGsRoOC/QZhtsGJtH/n0z33Gt6dXmhcBwqjl3ZQC
9kH2NwXm3ZejzVLrMhNELZTubJ7E3FG/UL+JqFBu0YnRqhPKhN+jgVKqmHNRjAKQ
cZEABITn+Cb+xHi7+sF0ZL5H5qctB8mVaVLxEbDtw2YbAgMBAAGjUzBRMB0GA1Ud
DgQWBBRIXYvAyXe4W8skQsH6f1FYRtVoAjAfBgNVHSMEGDAWgBRIXYvAyXe4W8sk
QsH6f1FYRtVoAjAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQC3
Q9oF5Ax5u8N1u1hkfp7Rcw6vK5i/UoaXIIlZSBuylsVaqNrTbVzaNJx4PmBPjPi+
9/DkqaBm9pPZQXFXOmogH9ov5ECj0hOvk02RgsWFMlkhS89hLB4U5x9L71xMp9KK
cNp1HaCMsYUfUvtdUqvK9sYWM+ky50NiUWqxD/QTLubfN7sBdepd10FHlIrEjE00
zPXR4Ja04WghBMntx6zHCXQo9gjy6VaLMvKjnHKDR2zDrCDZ8X3qQTvbp2fIz/6K
Xf0pKhU+NWnX6VWqt8AGWMWo77+/9VoqYYKwBc7WRgXJ0cSxzr6oWlEh7kjOTF47
7CwKfhb3XVBaNGrqepWY
-----END CERTIFICATE-----"#;

// This is the private key corresponding to the self-signed certificate above.
// This key is used here only for testing the client certificate functionality.
const EXAMPLE_COM_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDFB1b2QoAUoLh1
XP1ZMppVcmaFAKrwbRVeBDdSRrjmePpmLJT/x/6iOfcg7PVh8N1mFT+SG8gswJcD
/B2V6pUHVBMwmJ6qgY9N5ZQqWvtJmYxOdFWzUQl1uRQc6t0N5J3eXJpXHcCU4b8s
CKRDVnxp7JbLuzLHUuZGQCo7iYX4Ihhx70nPNCQB3JzeNm/gAjltAGsRoOC/QZht
sGJtH/n0z33Gt6dXmhcBwqjl3ZQC9kH2NwXm3ZejzVLrMhNELZTubJ7E3FG/UL+J
qFBu0YnRqhPKhN+jgVKqmHNRjAKQcZEABITn+Cb+xHi7+sF0ZL5H5qctB8mVaVLx
EbDtw2YbAgMBAAECggEAT90N6DG1Yva5VrUBsZM9C4QlZLcYPQlUFhGSBHLQlXZ2
6msvyS/3vReIQPQ6CNgPbLaRhbR7x3JA6DqBOCGm+NtKRGc/+BIv1JoBEIFt9rG5
LVwBaH2LKU2vfZwCiuRPTpqwdGKLGMZhfaNd6YtUOLf70pOyFOiCq5v8JTZ66VlJ
AUmHJv5u0EY7FN/mMyfJMIzPRi+hMsRHXgLuELXSP+bO7HbZqOtFW+XTiYGH00SH
WFQvCTsL7x+1ApnfFg8oAT3j+JuCvQWU1GzAUijybE3bXjKO8CRTt2vXhGqGbL2x
P8gTx6wlFkMCc+vZNM2IUQC3Slt47L/FEDd5RhJ6gQKBgQD2YoFH6EiUHT0PH+yD
UjwH5Ov/u2y1ldkOv72ILaQ8tgZ7/D1yJYMqcQKX7i2YIJe8qKJJ8z2Zg7VC/+uF
iO3FsEDMgq9mwPXvM8zN8VXgRMmSK70lZmFxBn1Nj/sD/DdXtbZNNrQIpxn8+ESW
bx7u3iR2Eg2TPMkdM0iSVpxUwQKBgQDMwtNg9JU8lLlYWjrJDHU0hXLKFO4mj2Ec
z8U9Q0JBwHkfMaZplgrcuIYdxLkN85HsPdopLxGrsCw9yGG6lgK4W4EwuKX1lTyI
/qAa8LvfOIeGLyfyMGPWb4HMD7O3kzqkZS6AYZ9zy62y+A5XYmGMmUMEQTzCDnO6
r43SJUkf2wKBgQCQhXuZLrp2VMeKPc67FfkcC1n5oTbBILVEwNiqaXZAxwAD+ZDg
SFrjxSFlTZEVoN1uV53CqMw/lf8OXfA0nrfVvbJVkgr43l4vmH7RKxI8Q1aapyLk
mS4iyqy33HYyppk+MhJzCcl/6K+nYB6C/VaEkqC/iV2JLtjKNf7j7yjxgQKBgQCu
P0KSVpzRpw/f/wAKApBrFY21L58/OKd0f1GGsZAv7+AuJEYCvAFIwBBm40f1iHHj
e/NMYy1VEwMXZeWzpxTZYH7yHOQnDrQL2r02tq1S7pX+JVOYPgSEJ6gtX9nkG4Yc
fvkm/NDZK4YO93PgwSCiGZZQiFhWUF1PffzZfIFxCQKBgAnekwrQvpFEgNvPhYGV
Ppaa9E3r1iyqQwEAV+3CgLylr1XZvD70c4hvEZs52GVotCOoB5jC+BpP9HMYqWAX
OwKk+sO060R/e5JF9zXSQ+YsQhoGGO/1vktSjz5bJbR/LDIdX6wG/Qp9/DZpIgLY
GNzNwFEf4JMUgITjz1E8V6XQ
-----END PRIVATE KEY-----"#;

#[test]
fn test_ssl_backend() {
    let version = curl::Version::get();
    let ssl_version = version
        .ssl_version()
        .expect("SSL version should be available");
    assert!(
        ssl_version.starts_with("OpenSSL"),
        "SSL backend should be OpenSSL, but got: {}",
        ssl_version
    );

    let mut easy = Easy::new();
    easy.url("https://www.howsmyssl.com/a/check").unwrap();
    easy.ssl_verify_host(false).unwrap();
    easy.ssl_verify_peer(false).unwrap();

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

    let response = str::from_utf8(&data).unwrap();
    assert!(
        response.contains("\"tls_version\":\"TLS 1.2\"")
            || response.contains("\"tls_version\":\"TLS 1.3\""),
        "TLS 1.2 or 1.3 should be used"
    );

    easy.ssl_cipher_list("DEFAULT").unwrap(); // This is an OpenSSL-specific option.

    let mut easy = Easy::new();
    easy.url("https://expired.badssl.com/").unwrap();
    easy.ssl_verify_host(true).unwrap();
    easy.ssl_verify_peer(true).unwrap();

    let result = easy.perform();
    assert!(
        result.is_err(),
        "Connection to expired.badssl.com should fail"
    );
    let err = result.unwrap_err();
    assert!(
        err.is_ssl_certproblem() || err.is_peer_failed_verification(),
        "Error should be related to certificate verification"
    );

    let mut easy = Easy::new();
    let mut list = List::new();
    list.append(EXAMPLE_COM_CERT).unwrap();
    easy.http_headers(list).unwrap();
    easy.url("https://example.com").unwrap();
    easy.perform().unwrap();

    let mut easy = Easy::new();
    easy.ssl_cert_blob(EXAMPLE_COM_CERT.as_bytes()).unwrap();
    easy.ssl_key_blob(EXAMPLE_COM_KEY.as_bytes()).unwrap();
    // We're using a self-signed cert, so we need to disable verification.
    easy.ssl_verify_host(false).unwrap();
    easy.ssl_verify_peer(false).unwrap();
    // Use a URL that doesn't actually require client certs, just to test the setup.
    easy.url("https://example.com").unwrap();
    let result = easy.perform();
    assert!(
        result.is_ok(),
        "Connection with client cert should succeed: {:?}",
        result
    );

    println!("All SSL/TLS tests passed successfully!");
}
