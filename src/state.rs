use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Binary, BlockInfo, Order, StdResult, Storage};
use cw_storage_plus::{Bound, Map, Item};

use cw20::{Balance, Expiration};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub orders: Vec<BuyOrder>,
    pub owner: Addr,
}
#[cw_serde]
pub struct AtomicSwap {
    /// This is the sha-256 hash of the preimage
    pub hash: Binary,
    pub recipient: Addr,
    pub source: Addr,
    pub expires: Expiration,
    /// Balance in native tokens, or cw20 token
    pub balance: Balance,
}

pub struct BuyOrder {
    pub amount: Amount,
    pub price: u32,
    pub order_owner: Addr,
    pub operation_mode: u32,
}

impl AtomicSwap {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}
pub const STATE: Item<State> = Item::new("state");
pub const SWAPS: Map<&str, AtomicSwap> = Map::new("atomic_swap");
pub const ALL_ORDERS: Vec<BuyOrder> = Vec::new();

/// This returns the list of ids for all active swaps
pub fn all_swap_ids<'a>(
    storage: &dyn Storage,
    start: Option<Bound<'a, &'a str>>,
    limit: usize,
) -> StdResult<Vec<String>> {
    SWAPS
        .keys(storage, start, None, Order::Ascending)
        .take(limit)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::Binary;

    #[test]
    fn test_no_swap_ids() {
        let storage = MockStorage::new();
        let ids = all_swap_ids(&storage, None, 10).unwrap();
        assert_eq!(0, ids.len());
    }

    fn dummy_swap() -> AtomicSwap {
        AtomicSwap {
            recipient: Addr::unchecked("recip"),
            source: Addr::unchecked("source"),
            expires: Default::default(),
            hash: Binary("hash".into()),
            balance: Default::default(),
        }
    }

    #[test]
    fn test_all_swap_ids() {
        let mut storage = MockStorage::new();
        SWAPS.save(&mut storage, "lazy", &dummy_swap()).unwrap();
        SWAPS.save(&mut storage, "assign", &dummy_swap()).unwrap();
        SWAPS.save(&mut storage, "zen", &dummy_swap()).unwrap();

        let ids = all_swap_ids(&storage, None, 10).unwrap();
        assert_eq!(3, ids.len());
        assert_eq!(
            vec!["assign".to_string(), "lazy".to_string(), "zen".to_string()],
            ids
        )
    }
}
