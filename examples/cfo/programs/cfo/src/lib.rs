use anchor_lang::associated_seeds;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, system_program};
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_spl::{dex, mint};

/// CFO is the program representing the Serum chief financial officer. It is
/// the program responsible for collecting and distributing fees from the Serum
/// DEX.
#[program]
pub mod cfo {
    use super::*;

    /// Creates a financial officer account associated with a DEX program ID.
    #[access_control(is_distribution_valid(&d))]
    pub fn create_officer(ctx: Context<CreateOfficer>, d: Distribution) -> Result<()> {
        let officer = &mut ctx.accounts.officer;
        officer.authority = *ctx.accounts.authority.key;
        officer.swap_program = *ctx.accounts.swap_program.key;
        officer.dex_program = *ctx.accounts.dex_program.key;
        officer.distribution = d;
        emit!(OfficerDidCreate {
            pubkey: *officer.to_account_info().key,
        });
        Ok(())
    }

    /// Creates a deterministic token account owned by the CFO.
    /// This should be used when a new mint is used for collecting fees.
    /// Can only be called once per token CFO and token mint.
    pub fn create_officer_token(_ctx: Context<CreateOfficerToken>) -> Result<()> {
        Ok(())
    }

    /// Updates the cfo's fee distribution.
    #[access_control(is_distribution_valid(&d))]
    pub fn set_distribution(ctx: Context<SetDistribution>, d: Distribution) -> Result<()> {
        ctx.accounts.officer.distribution = d.clone();
        emit!(DistributionDidChange { distribution: d });
        Ok(())
    }

    /// Transfers fees from the dex to the CFO.
    pub fn sweep_fees<'info>(ctx: Context<'_, '_, '_, 'info, SweepFees<'info>>) -> Result<()> {
        let seeds = associated_seeds!(ctx.accounts.officer);
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, dex::SweepFees<'info>> = (&*ctx.accounts).into();
        dex::sweep_fees(cpi_ctx.with_signer(&[seeds]))?;
        Ok(())
    }

    /// Convert the CFO's entire non-SRM token balance into USDC.
    /// Assumes USDC is the quote currency.
    #[access_control(is_not_trading())]
    pub fn swap_to_usdc<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToUsdc<'info>>,
        min_exchange_rate: ExchangeRate,
    ) -> Result<()> {
        let seeds = associated_seeds!(ctx.accounts.officer);
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> = (&*ctx.accounts).into();
        swap::cpi::swap(
            cpi_ctx.with_signer(&[seeds]),
            swap::Side::Bid,
            token::accessor::amount(&ctx.accounts.from_vault)?,
            min_exchange_rate.into(),
        )?;
        Ok(())
    }

    /// Convert the CFO's entire token balance into SRM.
    /// Assumes SRM is the base currency.
    #[access_control(is_not_trading())]
    pub fn swap_to_srm<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToSrm<'info>>,
        min_exchange_rate: ExchangeRate,
    ) -> Result<()> {
        let seeds = associated_seeds!(ctx.accounts.officer);
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> = (&*ctx.accounts).into();
        swap::cpi::swap(
            cpi_ctx.with_signer(&[seeds]),
            swap::Side::Bid,
            token::accessor::amount(&ctx.accounts.from_vault)?,
            min_exchange_rate.into(),
        )?;
        Ok(())
    }

    /// Distributes tokens.
    #[access_control(is_distribution_ready(&ctx.accounts))]
    pub fn distribute<'info>(ctx: Context<'_, '_, '_, 'info, Distribute<'info>>) -> Result<()> {
        // burn destroy
        //				token::burn
        // stake reward transfer
        // treasury transfer
        Ok(())
    }

    #[access_control(is_stake_reward_ready(&ctx.accounts))]
    pub fn drop_stake_reward<'info>(
        ctx: Context<'_, '_, '_, 'info, DropStakeReward<'info>>,
    ) -> Result<()> {
        // drop rewards onto stakers
        Ok(())
    }
}

// Context accounts.

#[derive(Accounts)]
pub struct CreateOfficer<'info> {
    #[account(init, associated = dex_program, payer = authority)]
    officer: ProgramAccount<'info, Officer>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    #[account(executable)]
    dex_program: AccountInfo<'info>,
    #[account(executable)]
    swap_program: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    system_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateOfficerToken<'info> {
    officer: ProgramAccount<'info, Officer>,
    #[account(
				init,
				token,
				associated = officer,
				with = mint,
				space = TokenAccount::LEN,
				payer = payer,
		)]
    token: CpiAccount<'info, TokenAccount>,
    #[account(owner = token_program)]
    mint: CpiAccount<'info, Mint>,
    #[account(mut, signer)]
    payer: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    system_program: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SetDistribution<'info> {
    #[account(has_one = authority)]
    officer: ProgramAccount<'info, Officer>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SweepFees<'info> {
    #[account(associated = dex.dex_program)]
    officer: ProgramAccount<'info, Officer>,
    #[account(owner = dex.token_program)]
    sweep_vault: AccountInfo<'info>,
    dex: Dex<'info>,
}

#[derive(Accounts)]
pub struct Dex<'info> {
    market: AccountInfo<'info>,
    pc_vault: AccountInfo<'info>,
    sweep_authority: AccountInfo<'info>,
    vault_signer: AccountInfo<'info>,
    dex_program: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapToUsdc<'info> {
    #[account(associated = dex_program)]
    officer: ProgramAccount<'info, Officer>,
    market: DexMarketAccounts<'info>,
    #[account(
				owner = token_program,
				constraint = &officer.treasury != from_vault.key,
				constraint = &officer.stake != from_vault.key,
		)]
    from_vault: AccountInfo<'info>,
    #[account(owner = token_program)]
    quote_vault: AccountInfo<'info>,
    #[account(
				associated = officer,
				with = mint::USDC,
		)]
    usdc_vault: AccountInfo<'info>,
    #[account(address = swap::ID)]
    swap_program: AccountInfo<'info>,
    #[account(address = dex::ID)]
    dex_program: AccountInfo<'info>,
    #[account(address = token::ID)]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapToSrm<'info> {
    #[account(associated = dex_program)]
    officer: ProgramAccount<'info, Officer>,
    market: DexMarketAccounts<'info>,
    #[account(
				owner = token_program,
				constraint = &officer.treasury != from_vault.key,
				constraint = &officer.stake != from_vault.key,
		)]
    from_vault: AccountInfo<'info>,
    #[account(owner = token_program)]
    quote_vault: AccountInfo<'info>,
    #[account(
				associated = officer,
				with = mint::SRM,
				constraint = &officer.treasury != from_vault.key,
				constraint = &officer.stake != from_vault.key,
		)]
    srm_vault: AccountInfo<'info>,
    #[account(address = swap::ID)]
    swap_program: AccountInfo<'info>,
    #[account(address = dex::ID)]
    dex_program: AccountInfo<'info>,
    #[account(address = token::ID)]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DexMarketAccounts<'info> {
    #[account(mut)]
    market: AccountInfo<'info>,
    #[account(mut)]
    open_orders: AccountInfo<'info>,
    #[account(mut)]
    request_queue: AccountInfo<'info>,
    #[account(mut)]
    event_queue: AccountInfo<'info>,
    #[account(mut)]
    bids: AccountInfo<'info>,
    #[account(mut)]
    asks: AccountInfo<'info>,
    // The `spl_token::Account` that funds will be taken from, i.e., transferred
    // from the user into the market's vault.
    //
    // For bids, this is the base currency. For asks, the quote.
    #[account(mut)]
    order_payer_token_account: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    #[account(mut)]
    coin_vault: AccountInfo<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    #[account(mut)]
    pc_vault: AccountInfo<'info>,
    // PDA owner of the DEX's token accounts for base + quote currencies.
    vault_signer: AccountInfo<'info>,
    // User wallets.
    #[account(mut)]
    coin_wallet: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    officer: ProgramAccount<'info, Officer>,
    #[account(
				owner = token_program,
				constraint = token::accessor::mint(&srm_vault)? == mint::SRM,
		)]
    srm_vault: AccountInfo<'info>,
    #[account(address = mint::SRM)]
    mint: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DropStakeReward<'info> {
    #[account(has_one = stake)]
    officer: ProgramAccount<'info, Officer>,
    #[account(owner = token_program)]
    stake: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    token_program: AccountInfo<'info>,
}

// Accounts.

#[associated]
#[derive(Default)]
pub struct Officer {
    // Priviledged account.
    pub authority: Pubkey,
    // Escrow vault where fees are swept into.
    pub sweep: Pubkey,
    // Escrow vault holding tokens which are dropped onto stakers.
    pub stake: Pubkey,
    // Token account to send treasury earned tokens to.
    pub treasury: Pubkey,
    // Defines the fee distribution, i.e., what percent each fee category gets.
    pub distribution: Distribution,
    // Swap frontend for the dex.
    pub swap_program: Pubkey,
    // Dex program the officer is associated with.
    pub dex_program: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct Distribution {
    bnb: u8,
    stake: u8,
    treasury: u8,
}

// CpiContext transformations.

impl<'info> From<&SweepFees<'info>> for CpiContext<'_, '_, '_, 'info, dex::SweepFees<'info>> {
    fn from(sweep: &SweepFees<'info>) -> Self {
        let program = sweep.dex.dex_program.to_account_info();
        let accounts = dex::SweepFees {
            market: sweep.dex.market.to_account_info(),
            pc_vault: sweep.dex.pc_vault.to_account_info(),
            sweep_authority: sweep.dex.sweep_authority.to_account_info(),
            sweep_receiver: sweep.sweep_vault.to_account_info(),
            vault_signer: sweep.dex.vault_signer.to_account_info(),
            token_program: sweep.dex.token_program.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

impl<'info> From<&SwapToSrm<'info>> for CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> {
    fn from(accs: &SwapToSrm<'info>) -> Self {
        let program = accs.swap_program.to_account_info();
        let accounts = swap::Swap {
            market: swap::MarketAccounts {
                market: accs.market.market.clone(),
                open_orders: accs.market.open_orders.clone(),
                request_queue: accs.market.request_queue.clone(),
                event_queue: accs.market.event_queue.clone(),
                bids: accs.market.bids.clone(),
                asks: accs.market.asks.clone(),
                order_payer_token_account: accs.market.order_payer_token_account.clone(),
                coin_vault: accs.market.coin_vault.clone(),
                pc_vault: accs.market.pc_vault.clone(),
                vault_signer: accs.market.vault_signer.clone(),
                coin_wallet: accs.srm_vault.clone(),
            },
            authority: accs.officer.to_account_info(),
            pc_wallet: accs.from_vault.to_account_info(),
            dex_program: accs.dex_program.to_account_info(),
            token_program: accs.token_program.to_account_info(),
            rent: accs.rent.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

impl<'info> From<&SwapToUsdc<'info>> for CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> {
    fn from(accs: &SwapToUsdc<'info>) -> Self {
        let program = accs.swap_program.to_account_info();
        let accounts = swap::Swap {
            market: swap::MarketAccounts {
                market: accs.market.market.clone(),
                open_orders: accs.market.open_orders.clone(),
                request_queue: accs.market.request_queue.clone(),
                event_queue: accs.market.event_queue.clone(),
                bids: accs.market.bids.clone(),
                asks: accs.market.asks.clone(),
                order_payer_token_account: accs.market.order_payer_token_account.clone(),
                coin_vault: accs.market.coin_vault.clone(),
                pc_vault: accs.market.pc_vault.clone(),
                vault_signer: accs.market.vault_signer.clone(),
                coin_wallet: accs.from_vault.to_account_info(),
            },
            authority: accs.officer.to_account_info(),
            pc_wallet: accs.usdc_vault.clone(),
            dex_program: accs.dex_program.to_account_info(),
            token_program: accs.token_program.to_account_info(),
            rent: accs.rent.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

impl<'info> From<&Distribute<'info>> for CpiContext<'_, '_, '_, 'info, token::Burn<'info>> {
    fn from(accs: &Distribute<'info>) -> Self {
        let program = accs.token_program.to_account_info();
        let accounts = token::Burn {
            mint: accs.mint.to_account_info(),
            to: accs.srm_vault.to_account_info(),
            authority: accs.officer.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

// Events.

#[event]
pub struct DistributionDidChange {
    distribution: Distribution,
}
#[event]
pub struct OfficerDidCreate {
    pubkey: Pubkey,
}

// Error.

#[error]
pub enum ErrorCode {
    #[msg("Distribution does not add to 100")]
    InvalidDistribution,
}

// Access control.

fn is_distribution_valid(d: &Distribution) -> Result<()> {
    if d.bnb + d.stake + d.treasury != 100 {
        return Err(ErrorCode::InvalidDistribution.into());
    }
    Ok(())
}

fn is_distribution_ready(accounts: &Distribute) -> Result<()> {
    // todo
    Ok(())
}

fn is_not_trading() -> Result<()> {
    // todo
    Ok(())
}

fn is_stake_reward_ready(accounts: &DropStakeReward) -> Result<()> {
    // todo
    Ok(())
}

// Redefintions.
//
// The following types are redefined so that they can be parsed into the IDL,
// since Anchor doesn't yet support idl parsing across multiple crates.

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ExchangeRate {
    rate: u64,
    from_decimals: u8,
    quote_decimals: u8,
    strict: bool,
}

impl From<ExchangeRate> for swap::ExchangeRate {
    fn from(e: ExchangeRate) -> Self {
        let ExchangeRate {
            rate,
            from_decimals,
            quote_decimals,
            strict,
        } = e;
        Self {
            rate,
            from_decimals,
            quote_decimals,
            strict,
        }
    }
}
