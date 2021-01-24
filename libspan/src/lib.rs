mod types;
pub use crate::types::{Error, PanelState, InverterState};

use serde::Deserialize;
use reqwest;

struct LibSpan {
   // IP or hostname of panel
   address: String,
}

impl LibSpan {
   pub fn new(address: Option<&str>) -> Result<LibSpan, Error> {
      Ok(LibSpan {
         address: match address {
            Some(a) => a.to_string(),
            None => "span-gateway.local".to_string(),
         }
      })
   }

   pub async fn get_panel_state(&self) -> Result<PanelState, Error> {
      let url = format!("http://{}:19240/panel/status", &self.address);
      let response = reqwest::get(&url).await?;
      let ps: PanelState = response.json().await?;
      Ok(ps)
   }

   pub async fn get_inverter_state(&self) -> Result<InverterState, Error> {
      Err(Error::Unknown)
   }
}

#[cfg(test)]
mod tests {
   use crate::LibSpan;

   #[test]
   fn get_state() {
      let ls = LibSpan::new(None).unwrap();
      let ps = tokio_test::block_on(ls.get_panel_state());
      assert!(ps.is_ok(), "Failed to get a panel state response");
   }

   #[test]
   fn get_inverter() {
      let ls = LibSpan::new(None).unwrap();
      let is= tokio_test::block_on(ls.get_inverter_state());
      assert!(is.is_ok(), "Failed to get an inverter state response");
   }
}

