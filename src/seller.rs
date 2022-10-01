#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Seller {
    pub export_power: f32,
    pub export_base_price: f32,
    pub is_external: bool,

    /// Seller becomes ready only when all the fields have value.
    ///
    /// MQTT topics send messages asynchronously therefore the seller
    /// objects must make sure the initial data filled is correct.
    pub is_ready: bool,
}

impl Default for Seller {
    fn default() -> Self {
        Self {
            export_power: 0.0,
            export_base_price: -1.0,
            is_external: false,
            is_ready: false,
        }
    }
}

impl Seller {
    fn base_cash_inflow(&self) -> f32 {
        self.export_power * self.export_base_price
    }
}
