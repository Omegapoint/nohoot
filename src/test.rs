#[cfg(test)]
mod test {
    use rocket::{local::blocking::Client, http::{Status}};

    use crate::{Name};

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
        let response = client.get("/v1/hello/kalle").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, kalle!\n");
    }
    
    #[test]
    fn session() {
        let rocket = crate::start();
        let client = Client::tracked(rocket).unwrap();
        let a_name = Name { name: "John".to_string() };
        let response = client.post("/v1/session").json(&a_name).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), format!("You get a session for {}!\n", a_name.name.to_string()));
    }
}
