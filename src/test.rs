#[cfg(test)]
mod test {
    use rocket::{local::blocking::Client, http::{Status}};

    use crate::{Name, Session};

    #[test]
    fn hello_world() {
        let rocket = crate::start();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/v1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!\n");
    }

    #[test]
    fn hello_name() {
        let rocket = crate::start();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/v1/hello/anka").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, anka!\n");
    }
    
    #[test]
    fn session() {
        let rocket = crate::start();
        let client = Client::tracked(rocket).unwrap();
        let a_name = Name { game_host: "John".to_string() };
        let response = client.post("/v1/session").json(&a_name).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let session = response.into_json::<Session>();
        assert_eq!(session.unwrap().game_host, a_name.game_host);
    }
}
