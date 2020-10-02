use crate::server::{helper, HttpResponse, Payload};
use test_server as server;

#[test]
fn start_server_at_given_port() {
    let server = server::new("127.0.0.1:65432", HttpResponse::Ok).unwrap();

    assert!(&server.url().ends_with(":65432"));

    let response = ureq::get(&server.url()).call();

    assert!(response.ok());
}

// thread '<unnamed>' panicked at 'Failed to bind!: Os { code: 98, kind: AddrInUse, message: "Address already in use" }',
#[test]
#[ignore]
fn restart_server_at_same_port() {
    {
        let server = server::new("127.0.0.1:65433", HttpResponse::Ok).unwrap();
        let response = ureq::get(&server.url()).call();

        assert!(response.ok());

        server.stop();
    }

    {
        let server = server::new("127.0.0.1:65432", HttpResponse::BadRequest).unwrap();
        let response = ureq::get(&server.url()).call();

        assert!(response.client_error());
    }
}

#[test]
fn validate_client_request() {
    let server = server::new("127.0.0.1:0", HttpResponse::Ok).unwrap();

    let request_content = helper::random_string(100);
    let _ = ureq::post(&server.url()).send_string(&request_content);

    assert_eq!(server.requests.len(), 1);

    let req = server.requests.next();
    assert!(req.is_some());

    let req = req.unwrap();
    assert_eq!(request_content.as_bytes(), &req.body()[..]);
    assert_eq!("100", req.headers().get("content-length").unwrap());
    assert_eq!("POST", req.method());
    assert_eq!("/", req.uri().path());
    assert!(req.uri().query().is_none());
}

#[test]
fn validate_client_response() {
    let server = server::new("127.0.0.1:0", |payload: Payload| {
        HttpResponse::Ok().streaming(payload)
    })
    .unwrap();

    let request_content = helper::random_string(100);
    let response = ureq::post(&server.url()).send_string(&request_content);

    assert!(response.ok());
    assert_eq!(response.into_string().unwrap(), request_content);
}

#[test]
fn not_necessary_to_fetch_request_from_server() {
    let server = server::new("127.0.0.1:0", || {
        let content = helper::read_file("tests/sample.json").unwrap();
        HttpResponse::Ok().body(content)
    })
    .unwrap();
    let response = ureq::get(&server.url()).call();

    assert_eq!(
        helper::read_file("tests/sample.json").unwrap(),
        response.into_string().unwrap()
    );
}

#[test]
fn fetch_2nd_request_from_server() {
    let server = server::new("127.0.0.1:0", HttpResponse::Ok).unwrap();

    let _ = ureq::get(&server.url()).call();
    let _ = ureq::post(&server.url()).send_string("2");

    assert_eq!(server.requests.len(), 2);

    let _ = server.requests.next();
    let request = server.requests.next();

    assert!(request.is_some());
    assert_eq!(b"2", &request.unwrap().body()[..]);
}
