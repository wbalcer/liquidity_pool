#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let pool = LpPool::init(150, 10, 900, TokenAmount(9000));
        assert_eq!(pool.price, 150);
        assert_eq!(pool.fee_min, 10);
        assert_eq!(pool.fee_max, 900);
        assert_eq!(pool.liquidity_target.0, 9000);
        assert_eq!(pool.token_reserve.0, 0);
        assert_eq!(pool.staked_token_reserve.0, 0);
        assert_eq!(pool.lp_token_supply.0, 0);
    }

    #[test]
    fn test_add_liquidity() {
        let mut pool = LpPool::init(150, 10, 900, TokenAmount(9000));
        let lp_tokens = pool.add_liquidity(TokenAmount(10000));
        assert_eq!(lp_tokens.0, 10000);
        assert_eq!(pool.token_reserve.0, 10000);
        assert_eq!(pool.lp_token_supply.0, 10000);
    }

    #[test]
    fn test_remove_liquidity() {
        let mut pool = LpPool::init(150, 10, 900, TokenAmount(9000));
        pool.add_liquidity(TokenAmount(10000));
        let (tokens, staked_tokens) = pool.remove_liquidity(LPTokenAmount(5000));
        assert_eq!(tokens.0, 5000);
        assert_eq!(staked_tokens.0, 0);
        assert_eq!(pool.token_reserve.0, 5000);
        assert_eq!(pool.lp_token_supply.0, 5000);
    }

    #[test]
    fn test_swap() {
        let mut pool = LpPool::init(150, 10, 900, TokenAmount(9000));
        pool.add_liquidity(TokenAmount(10000));
        let received_tokens = pool.swap(StakedTokenAmount(600));
        assert_eq!(received_tokens.0, 897);
        assert_eq!(pool.token_reserve.0, 9103);
        assert_eq!(pool.staked_token_reserve.0, 600);
    }
}
