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
    /// assert_eq!(place.item_id, 2);
    /// ```
    pub fn new_after(item_id: u64) -> Self {
        Self {
            item_id,
            position: item_position::Position::After.into(),
        }
    }

    /// Create a place before another place.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::harmonytypes::ItemPosition;
    /// let place = ItemPosition::new_before(2);
    /// assert_eq!(place.item_id, 2);
    /// ```
    pub fn new_before(item_id: u64) -> Self {
        Self {
            item_id,
            position: item_position::Position::BeforeUnspecified.into(),
        }
    }

    /// Does the position comes before the specified ID?
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::harmonytypes::ItemPosition;
    /// let place = ItemPosition::new_before(2);
    /// assert_eq!(place.is_before(), true);
    /// ```
    pub fn is_before(&self) -> bool {
        matches!(self.position(), item_position::Position::BeforeUnspecified)
    }

    /// Does the position comes after the specified ID?
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::harmonytypes::ItemPosition;
    /// let place = ItemPosition::new_after(2);
    /// assert_eq!(place.is_after(), true);
    /// ```
    pub fn is_after(&self) -> bool {
        matches!(self.position(), item_position::Position::After)
    }
}
