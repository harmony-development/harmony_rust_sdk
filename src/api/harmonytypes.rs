/// v1 of harmonytypes.
pub mod v1 {
    #![allow(missing_docs)]
    hrpc::include_proto!("protocol.harmonytypes.v1");
}
pub use v1::*;

impl ItemPosition {
    /// Create a place after another place.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::harmonytypes::ItemPosition;
    /// let place = ItemPosition::new_after(2);
    /// assert_eq!(place.after(), Some(2));
    /// ```
    pub fn new_after(item_id: u64) -> Self {
        Self {
            item_id,
            position: item_position::Position::After.into(),
        }
    }

    /// Get the ID of the item after where this position represents.
    pub fn after(&self) -> Option<u64> {
        matches!(self.position(), item_position::Position::After).then(|| self.item_id)
    }

    /// Create a place before another place.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::harmonytypes::ItemPosition;
    /// let place = ItemPosition::new_before(2);
    /// assert_eq!(place.before(), Some(2));
    /// ```
    pub fn new_before(item_id: u64) -> Self {
        Self {
            item_id,
            position: item_position::Position::BeforeUnspecified.into(),
        }
    }

    /// Get the ID of the item before where this position represents.
    pub fn before(&self) -> Option<u64> {
        matches!(self.position(), item_position::Position::BeforeUnspecified).then(|| self.item_id)
    }
}
