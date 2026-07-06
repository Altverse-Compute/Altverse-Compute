use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct JoinProps {
  pub name: String,
  pub id: u64,
}

#[napi]
impl JoinProps {
  #[napi(constructor)]
  pub fn new(name: String, id: u64) -> JoinProps {
    JoinProps { name, id }
  }
}
