use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, OverflowError, Uint128};
use cw_storage_plus::{Item, Map};

use crate::ContractError;
use cw_denom::{CheckedDenom, UncheckedDenom};

use wynd_utils::Curve;

#[cw_serde]
pub struct UncheckedVestingParams {
    pub recipient: String,       // address
    pub amount: Uint128,         // non-zero
    pub denom: UncheckedDenom,   // valid
    pub vesting_schedule: Curve, // high == amount && low == 0
    pub title: Option<String>,
    pub description: Option<String>,
}

#[cw_serde]
pub struct CheckedVestingParams {
    pub recipient: Addr,
    pub amount: Uint128,
    pub denom: CheckedDenom,
    pub vesting_schedule: Curve,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl UncheckedVestingParams {
    pub fn into_checked(self, deps: Deps) -> Result<CheckedVestingParams, ContractError> {
        if self.amount.is_zero() {
            return Err(ContractError::NothingToVest);
        }

        // Check vesting schedule
        self.assert_schedule_vests_amount()?;

        // Check valid recipient address
        let recipient = deps.api.addr_validate(&self.recipient)?;

        // Check denom
        let checked_denom = self.denom.into_checked(deps)?;

        // If title is included, validate title length (max 280 characters)
        if let Some(ref title) = self.title {
            if title.len() > 280 || title.is_empty() {
                return Err(ContractError::InvalidTitle);
            }
        }

        Ok(CheckedVestingParams {
            recipient,
            amount: self.amount,
            denom: checked_denom,
            vesting_schedule: self.vesting_schedule,
            title: self.title,
            description: self.description,
        })
    }

    /// Asserts the vesting schedule decreases to 0 eventually, 2and is never more than the
    /// amount being sent. If it doesn't match these conditions, returns an error.
    pub fn assert_schedule_vests_amount(&self) -> Result<(), ContractError> {
        self.vesting_schedule.validate_monotonic_decreasing()?;
        let (low, high) = self.vesting_schedule.range();
        if low == 0 && high == self.amount.u128() {
            Ok(())
        } else if low != 0 {
            Err(ContractError::NeverFullyVested)
        } else {
            // high != amount
            Err(ContractError::VestsDifferently)
        }
    }
}

#[cw_serde]
pub enum VestingPaymentStatus {
    Active,
    Canceled,
    CanceledAndUnbonding,
    FullyVested,
    Unfunded,
}

#[cw_serde]
pub struct VestingPayment {
    /// The recipient for the vesting payment
    pub recipient: Addr,
    /// Vesting amount in Native and Cw20 tokens
    pub amount: Uint128,
    /// Amount claimed so far
    pub claimed_amount: Uint128,
    /// Canceled at time in seconds, only set if contract is canceled
    pub canceled_at_time: Option<u64>,
    /// Vesting schedule
    pub vesting_schedule: Curve,
    /// The denom of a token (cw20 or native)
    pub denom: CheckedDenom,
    /// Title of the payroll item, for example for a bug bounty "Fix issue in contract.rs"
    pub title: Option<String>,
    /// Description of the payroll item, a more in depth description of how to meet the payroll conditions
    pub description: Option<String>,
    /// The status of the vesting payment
    pub status: VestingPaymentStatus,
    /// The amount of the vesting payment that has been staked
    pub staked_amount: Uint128,
}

impl VestingPayment {
    /// Create a new VestingPayment from CheckedVestingParams
    pub fn new(checked_vesting_params: CheckedVestingParams) -> Result<Self, ContractError> {
        let vesting_payment = Self {
            status: VestingPaymentStatus::Active,
            claimed_amount: Uint128::zero(),
            staked_amount: Uint128::zero(),
            canceled_at_time: None,
            recipient: checked_vesting_params.recipient,
            amount: checked_vesting_params.amount,
            denom: checked_vesting_params.denom,
            vesting_schedule: checked_vesting_params.vesting_schedule,
            title: checked_vesting_params.title,
            description: checked_vesting_params.description,
        };

        Ok(vesting_payment)
    }

    pub fn get_vested_amount_by_seconds(&self, time: u64) -> Result<Uint128, OverflowError> {
        let vesting_funds = self.vesting_schedule.value(time);
        self.amount.checked_sub(vesting_funds)
    }
}

/// Holds information about the vesting payment
pub const VESTING_PAYMENT: Item<VestingPayment> = Item::new("vesting_payment");
/// A map of staked vesting claims by validator
pub const STAKED_VESTING_BY_VALIDATOR: Map<&str, Uint128> = Map::new("staked_vesting_by_validator");

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use wynd_utils::PiecewiseLinear;

    use super::*;

    #[test]
    fn test_catches_vests_more_than_sent() {
        let deps = mock_dependencies();
        let amount = Uint128::new(10);
        let start = mock_env().block.time.seconds();
        let end = start + 10_000;
        let vesting_schedule = Curve::saturating_linear((start, 69), (end, 0));
        let params = UncheckedVestingParams {
            recipient: "test".to_string(),
            amount,
            denom: UncheckedDenom::Native("ujuno".to_string()),
            vesting_schedule,
            title: None,
            description: None,
        };
        let err = params.into_checked(deps.as_ref()).unwrap_err();
        assert_eq!(err, ContractError::VestsDifferently);
    }

    #[test]
    fn test_catches_never_fully_vesting() {
        let deps = mock_dependencies();
        let amount = Uint128::new(11223344);
        let start = mock_env().block.time.seconds();
        let end = start + 10_000;
        let vesting_schedule = Curve::saturating_linear((start, amount.into()), (end, 1));
        let params = UncheckedVestingParams {
            recipient: "test".to_string(),
            amount,
            denom: UncheckedDenom::Native("ujuno".to_string()),
            vesting_schedule,
            title: None,
            description: None,
        };
        let err = params.into_checked(deps.as_ref()).unwrap_err();
        assert_eq!(err, ContractError::NeverFullyVested);
    }

    #[test]
    fn test_catches_non_decreasing_curve() {
        let deps = mock_dependencies();
        let amount = Uint128::new(11223344);
        let start = mock_env().block.time.seconds();
        let end = start + 10_000;
        let vesting_schedule = Curve::saturating_linear((start, 0), (end, amount.into()));
        let params = UncheckedVestingParams {
            recipient: "test".to_string(),
            amount,
            denom: UncheckedDenom::Native("ujuno".to_string()),
            vesting_schedule,
            title: None,
            description: None,
        };
        let err = params.into_checked(deps.as_ref()).unwrap_err();
        assert_eq!(
            err,
            ContractError::Curve(wynd_utils::CurveError::MonotonicIncreasing)
        );
    }

    #[test]
    fn test_valdiate_title() {
        let deps = mock_dependencies();
        let amount = Uint128::new(11223344);
        let start = mock_env().block.time.seconds();
        let end = start + 10_000;
        let vesting_schedule = Curve::saturating_linear((start, amount.into()), (end, 0));

        // Catch empty string title
        let params = UncheckedVestingParams {
            recipient: "test".to_string(),
            amount,
            denom: UncheckedDenom::Native("ujuno".to_string()),
            vesting_schedule: vesting_schedule.clone(),
            title: Some("".to_string()),
            description: None,
        };
        let err = params.into_checked(deps.as_ref()).unwrap_err();
        assert_eq!(err, ContractError::InvalidTitle);

        // Catch long title
        let params = UncheckedVestingParams {
            recipient: "test".to_string(),
            amount,
            denom: UncheckedDenom::Native("ujuno".to_string()),
            vesting_schedule,
            title: Some(
                "
                58

                If a country is governed with tolerance,
                the people are comfortable and honest.
                If a country is governed with repression,
                the people are depressed and crafty.

                When the will to power is in charge,
                the higher the ideals, the lower the results.
                Try to make people happy,
                and you lay the groundwork for misery.
                Try to make people moral,
                and you lay the groundwork for vice.

                Thus the Master is content
                to serve as an example
                and not to impose her will.
                She is pointed, but doesn't pierce.
                Straightforward, but supple.
                Radiant, but easy on the eyes.

                59

                For governing a country well
                there is nothing better than moderation.

                The mark of a moderate man
                is freedom from his own ideas.
                Tolerant like the sky,
                all-pervading like sunlight,
                firm like a mountain,
                supple like a tree in the wind,
                he has no destination in view
                and makes use of anything
                life happens to bring his way.

                Nothing is impossible for him.
                Because he has let go,
                he can care for the people's welfare
                as a mother cares for her child.

                (Text sourced from https://www.organism.earth/library/document/tao-te-ching)
                "
                .to_string(),
            ),
            description: None,
        };
        let err = params.into_checked(deps.as_ref()).unwrap_err();
        assert_eq!(err, ContractError::InvalidTitle);
    }

    #[test]
    fn test_validate_piecewise_vessting_schedule() {
        let deps = mock_dependencies();
        let amount = Uint128::new(11223344);
        let start = mock_env().block.time.seconds();
        let complexity = 100;
        let steps: Vec<_> = (0..complexity)
            .map(|x| (start + x, amount - Uint128::from(x)))
            .chain(std::iter::once((start + complexity, Uint128::new(0)))) // fully vest
            .collect();
        let vesting_schedule = Curve::PiecewiseLinear(PiecewiseLinear { steps });
        let params = UncheckedVestingParams {
            recipient: "test".to_string(),
            amount,
            denom: UncheckedDenom::Native("ujuno".to_string()),
            vesting_schedule,
            title: None,
            description: None,
        };
        params.into_checked(deps.as_ref()).unwrap();
    }
}