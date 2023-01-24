use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Deserialize, Serialize)]
enum Ttype {
    ENUM_1,
    ENUM_2,
}

#[derive(Serialize, Deserialize)]
struct BenchStruct {
    id: i32,
    value: String,
    ttype: Ttype,
}


/// A simple Spin HTTP component.
#[http_component]
fn wasm_bench(req: Request) -> Result<Response> {
    env_logger::init();

    if let Some(body) = req.body() {
        let mut parsed_body: BenchStruct = serde_json::from_str(std::str::from_utf8(body)?)?;
        parsed_body.id = parsed_body.id + 1;

        let new_body = serde_json::to_string(&parsed_body)?;

        return Ok(http::Response::builder()
                  .status(200)
                  .header("content-type", "application/json")
                  .body(Some(new_body.into()))?)
    }

    return Ok(http::Response::builder()
              .status(400)
              .body(Some("Error".into()))?)

}
