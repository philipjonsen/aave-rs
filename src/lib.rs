#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate bigdecimal;

use bigdecimal::BigDecimal;
use core::str;
use napi::{CallContext, JsNumber, JsObject, JsString, Result};
use std::{convert::TryInto, ops::Div};
use std::{ops::Mul, str::FromStr};

const SECONDS_PER_YEAR: i32 = 31536000;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("normalize", normalize)?;

    exports.create_named_method(
        "calculateHealthFactorFromBalances",
        calculate_health_factor_from_balances,
    )?;

    exports.create_named_method("calculateLinearInterest", calculate_linear_interest)?;

    exports.create_named_method("getLinearBalance", get_linear_balance)?;
    Ok(())
}

fn native_normalize(base: BigDecimal, decimals: u32) -> BigDecimal {
    return base.div(10_i64.pow(decimals));
}

#[js_function(2)]
fn normalize(ctx: CallContext) -> Result<JsString> {
    let base = ctx.get::<JsString>(0)?.into_utf8()?.into_owned().unwrap();
    let base_big_d = BigDecimal::from_str(&base).unwrap();
    let decimals: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
    let result = native_normalize(base_big_d, decimals);
    ctx.env.create_string(&result.to_string())
}

fn native_calculate_health_factor_from_balances(
    total_collateral_eth: BigDecimal,
    total_borrows_eth: BigDecimal,
    current_liquidation_threshold: BigDecimal,
) -> BigDecimal {
    if total_borrows_eth.eq(&BigDecimal::from(0)) {
        return BigDecimal::from(-1);
    }
    return total_collateral_eth
        .mul(current_liquidation_threshold)
        .div(10_i32.pow(4))
        .div(total_borrows_eth);
}

#[js_function(3)]
fn calculate_health_factor_from_balances(ctx: CallContext) -> Result<JsString> {
    let total_collateral = ctx.get::<JsString>(0)?.into_utf8()?.into_owned().unwrap();
    let total_collateral_big_d = BigDecimal::from_str(&total_collateral).unwrap();
    let total_borrows = ctx.get::<JsString>(1)?.into_utf8()?.into_owned().unwrap();
    let total_borrows_big_d = BigDecimal::from_str(&total_borrows).unwrap();
    let current_liquidation_threshold = ctx.get::<JsString>(2)?.into_utf8()?.into_owned().unwrap();
    let current_liquidation_threshold_big_d =
        BigDecimal::from_str(&current_liquidation_threshold).unwrap();
    let result = native_calculate_health_factor_from_balances(
        total_collateral_big_d,
        total_borrows_big_d,
        current_liquidation_threshold_big_d,
    );
    ctx.env.create_string(&result.to_string())
}

fn native_calculate_linear_interest(
    rate: BigDecimal,
    current_timestamp: u32,
    last_updated_timestamp: u32,
) -> BigDecimal {
    let delta = BigDecimal::from(current_timestamp - last_updated_timestamp);
    let delta_per_year = delta.div(SECONDS_PER_YEAR);
    return rate.mul(delta_per_year);
}

#[js_function(3)]
fn calculate_linear_interest(ctx: CallContext) -> Result<JsString> {
    let rate = ctx.get::<JsString>(0)?.into_utf8()?.into_owned().unwrap();
    let rate_big_d = BigDecimal::from_str(&rate).unwrap();
    let current_timestamp: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
    let last_updated_timestamp: u32 = ctx.get::<JsNumber>(2)?.try_into()?;
    let result =
        native_calculate_linear_interest(rate_big_d, current_timestamp, last_updated_timestamp);
    ctx.env.create_string(&result.to_string())
}

fn native_calculate_reserve_normalize_income(
    rate: BigDecimal,
    index: BigDecimal,
    last_updated_timestamp: u32,
    current_timestamp: u32,
) -> BigDecimal {
    let cumulated_interest =
        native_calculate_linear_interest(rate, current_timestamp, last_updated_timestamp);

    return cumulated_interest.mul(index);
}

fn native_get_linear_balance(
    balance: BigDecimal,
    index: BigDecimal,
    rate: BigDecimal,
    last_updated_timestamp: u32,
    current_timestamp: u32,
) -> BigDecimal {
    let reserve_normalized_income = native_calculate_reserve_normalize_income(
        rate,
        index,
        last_updated_timestamp,
        current_timestamp,
    );
    return balance.mul(reserve_normalized_income);
}

#[js_function(5)]
fn get_linear_balance(ctx: CallContext) -> Result<JsString> {
    let balance = ctx.get::<JsString>(0)?.into_utf8()?.into_owned().unwrap();
    let balance_big_d = BigDecimal::from_str(&balance).unwrap();
    let index = ctx.get::<JsString>(1)?.into_utf8()?.into_owned().unwrap();
    let index_big_d = BigDecimal::from_str(&index).unwrap();
    let rate = ctx.get::<JsString>(2)?.into_utf8()?.into_owned().unwrap();
    let rate_big_d = BigDecimal::from_str(&rate).unwrap();
    let last_updated_timestamp: u32 = ctx.get::<JsNumber>(4)?.try_into()?;
    let current_timestamp: u32 = ctx.get::<JsNumber>(3)?.try_into()?;
    let result = native_get_linear_balance(
        balance_big_d,
        index_big_d,
        rate_big_d,
        last_updated_timestamp,
        current_timestamp,
    );
    ctx.env.create_string(&result.to_string())
}
