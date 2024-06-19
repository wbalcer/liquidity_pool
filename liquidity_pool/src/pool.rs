use crate::models::{TokenAmount, StakedTokenAmount, LPTokenAmount};

#[derive(Debug)]
pub struct LpPool {
    pub price: u64,
    pub fee_min: u64,
    pub fee_max: u64,
    pub liquidity_target: TokenAmount,
    pub token_reserve: TokenAmount,
    pub staked_token_reserve: StakedTokenAmount,
    pub lp_token_supply: LPTokenAmount,
}

impl LpPool {
    pub fn init(price: u64, fee_min: u64, fee_max: u64, liquidity_target: TokenAmount) -> Self {
        Self {
            price,
            fee_min,
            fee_max,
            liquidity_target,
            token_reserve: TokenAmount(0),
            staked_token_reserve: StakedTokenAmount(0),
            lp_token_supply: LPTokenAmount(0),
        }
    }

    pub fn add_liquidity(&mut self, token_amount: TokenAmount) -> LPTokenAmount {
        self.token_reserve.0 += token_amount.0;
        let minted_lp_tokens = token_amount.0;
        self.lp_token_supply.0 += minted_lp_tokens;
        LPTokenAmount(minted_lp_tokens)
    }

    pub fn remove_liquidity(&mut self, lp_token_amount: LPTokenAmount) -> (TokenAmount, StakedTokenAmount) {
        let token_amount = (lp_token_amount.0 as u128 * self.token_reserve.0 as u128 / self.lp_token_supply.0 as u128) as u64;
        let staked_token_amount = (lp_token_amount.0 as u128 * self.staked_token_reserve.0 as u128 / self.lp_token_supply.0 as u128) as u64;

        self.token_reserve.0 -= token_amount;
        self.staked_token_reserve.0 -= staked_token_amount;
        self.lp_token_supply.0 -= lp_token_amount.0;

        (TokenAmount(token_amount), StakedTokenAmount(staked_token_amount))
    }

    pub fn swap(&mut self, staked_token_amount: StakedTokenAmount) -> TokenAmount {
        let fee_percentage = self.calculate_fee_percentage();
        let fee = (staked_token_amount.0 as u128 * fee_percentage as u128 / 10000) as u64;

        let token_amount = ((staked_token_amount.0 - fee) as u128 * self.price as u128 / 100) as u64;
        self.token_reserve.0 -= token_amount;
        self.staked_token_reserve.0 += staked_token_amount.0;

        TokenAmount(token_amount)
    }

    fn calculate_fee_percentage(&self) -> u64 {
        let liquidity_ratio = self.token_reserve.0 as f64 / self.liquidity_target.0 as f64;
        if liquidity_ratio > 1.0 {
            self.fee_min
        } else {
            self.fee_max
        }
    }
}
