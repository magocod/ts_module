#![allow(dead_code)]

#[napi(constructor)]
pub struct AnimalWithDefaultConstructor {
  pub name: String,
  pub kind: u32,
}

// A complex struct which cannot be exposed to JavaScript directly.
struct QueryEngine {}

impl QueryEngine {
  fn new() -> Self {
    QueryEngine {}
  }

  pub fn status(&self) -> u32 {
    1
  }

  pub async fn query(&self, sentence: String) -> String {
    format!("query -> {}", sentence)
  }
}

#[napi(js_name = "QueryEngine")]
struct JsQueryEngine {
  engine: QueryEngine,
}

#[napi]
impl JsQueryEngine {
  #[napi(constructor)]
  pub fn new() -> Self {
    JsQueryEngine {
      engine: QueryEngine::new(),
    }
  }

  /// Class method
  #[napi]
  pub async fn query(&self, query: String) -> napi::Result<String> {
    Ok(self.engine.query(query).await)
  }

  #[napi]
  pub fn status(&self) -> napi::Result<u32> {
    Ok(self.engine.status())
  }
}
