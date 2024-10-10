use curl::easy::Easy;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_multiple_concurrent_https_requests() {
    let urls = vec![
        "https://www.example.com",
        "https://www.google.com",
        "https://www.github.com",
        "https://www.rust-lang.org",
    ];

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for url in urls {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let mut easy = Easy::new();

            easy.url(url).unwrap();
            easy.follow_location(true).unwrap();
            easy.perform().unwrap();

            let code = easy.response_code().unwrap();

            results.lock().unwrap().push((url, code));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!(results.len(), 4);
    for (_, code) in results.iter() {
        assert_eq!(*code, 200);
    }
}

#[test]
fn test_ssl_info_callback() {
    let mut easy = Easy::new();
    easy.url("https://www.example.com").unwrap();
    let ssl_data = Arc::new(Mutex::new(Vec::new()));
    let ssl_data_clone = ssl_data.clone();

    easy.verbose(true).unwrap();

    easy.debug_function(move |info_type, data| {
        println!("Debug: {:?}", info_type);
        match info_type {
            curl::easy::InfoType::SslDataIn | curl::easy::InfoType::SslDataOut => {
                println!("Capturing SSL data: {} bytes", data.len());
                ssl_data_clone
                    .lock()
                    .unwrap()
                    .push((info_type, data.to_vec()));
            }
            _ => {}
        }
    })
    .unwrap();

    println!("Performing request...");
    match easy.perform() {
        Ok(_) => println!("Request completed successfully"),
        Err(e) => println!("Request failed: {:?}", e),
    }

    let ssl_data = ssl_data.lock().unwrap();
    println!("Captured SSL data entries: {}", ssl_data.len());

    assert!(!ssl_data.is_empty(), "No SSL data captured");
}
