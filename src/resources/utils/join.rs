use napi_derive::napi;

use crate::external::Role;

#[napi]
#[derive(Clone)]
pub struct JoinProps {
  pub name: String,
  pub id: i64,
  pub role: u32,
}

#[napi]
impl JoinProps {
  #[napi(constructor)]
  pub fn new(name: String, id: i64, role: u32) -> JoinProps {
    JoinProps { name, id, role }
  }

  pub fn role_to_internal(self: &Self) -> Role {
    match self.role {
      1 => Role::Mod,
      2 => Role::Dev,
      _ => Role::User,
    }
  }
}
