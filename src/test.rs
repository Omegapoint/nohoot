#[cfg(test)]
mod test {
    use rocket::{local::blocking::Client, http::{Status}, serde::json::Json};

    use crate::{Name};

    #[test]
    fn hello_world() {
        let rocket = crate::rocket();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/v1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!\n");
    }
    
    #[test]
    fn session() {
        let rocket = crate::rocket();
        let client = Client::tracked(rocket).unwrap();
        let aName = Name { name: "John".to_string() };
        let response = client.post("/v1/session").json(&aName).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), format!("You get a session for {}!\n", aName.name.to_string()));
    }
}
