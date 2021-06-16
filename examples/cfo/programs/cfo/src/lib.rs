use anchor_lang::associated_seeds;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::dex;
use anchor_spl::token::{self, Mint};

pub const SRM_MINT: &str = "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt";

/// CFO is the program representing the Serum chief financial officer. It is
/// the program responsible for collecting and distributing fees from the Serum
/// DEX.
#[program]
pub mod cfo {
    use super::*;

    /// Creates a financial officer account associated with a DEX program ID.
    pub fn create_officer(ctx: Context<CreateOfficer>, distribution: Distribution) -> Result<()> {
        let officer = &mut ctx.accounts.officer;
        officer.authority = *ctx.accounts.authority.key;
        officer.swap_program = *ctx.accounts.swap_program.key;
        officer.dex_program = *ctx.accounts.dex_program.key;
        officer.distribution = distribution;
        emit!(OfficerDidCreate {
            officer: officer.clone().into_inner(),
            pubkey: *officer.to_account_info().key,
        });
        Ok(())
    }

    /// Creates a deterministic token account for the program as a convenient
    /// alternative to the associated token program.
    pub fn create_officer_token(ctx: Context<CreateOfficerToken>) -> Result<()> {
        Ok(())
    }

    /// Updates the cfo's fee distribution.
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

    /// Convert the entire balance of a usd(x) token (owned by the CFO) into SRM
    /// by trading on the DEX.
    #[access_control(is_not_trading())]
    pub fn swap_to_srm<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToSrm<'info>>,
        min_exchange_rate: ExchangeRate,
    ) -> Result<()> {
        let seeds = associated_seeds!(ctx.accounts.officer);
        let side = swap::Side::Bid;
        let amount = token::accessor::amount(&ctx.accounts.from_vault)?;
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> = (&*ctx.accounts).into();
        swap::cpi::swap(cpi_ctx, side, amount, min_exchange_rate.into())?;
        Ok(())
    }

    /// A transitive version of `swap_usdx_to_srm` for arbitrary, non usdx
    /// tokens.
    #[access_control(is_not_trading())]
    pub fn swap_to_srm_transitive<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToSrmTransitive<'info>>,
    ) -> Result<()> {
        let seeds = associated_seeds!(ctx.accounts.officer);
        /*
        let side = ;
        let amount = ;
        let min_exchange_rate =;
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> = (&*ctx.accounts).into();
        swap::cpi::swap(cpi_ctx, side, amount, min_exchange_rate)?;
        */
        Ok(())
    }

    /// Distributes tokens.
    #[access_control(is_distribution_ready(&ctx.accounts))]
    pub fn distribute<'info>(ctx: Context<'_, '_, '_, 'info, Distribute<'info>>) -> Result<()> {
        Ok(())
    }
}

// Macros.

macro_rules! pk {
    ($str:ident) => {
        $str.parse().unwrap()
    };
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
    #[account(init, token, associated = officer, with = mint, with = token_program, space = 165)]
    token: AccountInfo<'info>,
    #[account(owner = token_program)]
    mint: CpiAccount<'info, Mint>,
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
pub struct SwapToSrm<'info> {
    #[account(associated = dex_program)]
    officer: ProgramAccount<'info, Officer>,
    #[account(owner = token_program)]
    from_vault: AccountInfo<'info>,
    #[account(constraint = token::accessor::mint(&srm_vault)? == pk!(SRM_MINT))]
    srm_vault: AccountInfo<'info>,
    dex_program: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapToSrmTransitive<'info> {
    #[account(associated = dex_program)]
    officer: ProgramAccount<'info, Officer>,
    #[account(owner = token_program)]
    from_vault: AccountInfo<'info>,
    #[account(constraint = token::accessor::mint(&srm_vault)? == pk!(SRM_MINT))]
    srm_vault: AccountInfo<'info>,
    dex_program: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(
				owner = token_program,
				constraint = token::accessor::mint(&srm_vault)? == pk!(SRM_MINT),
		)]
    srm_vault: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    token_program: AccountInfo<'info>,
}

// Accounts.

#[associated]
#[derive(Default)]
pub struct Officer {
    pub authority: Pubkey,
    pub distribution: Distribution,
    pub swap_program: Pubkey,
    pub dex_program: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct Distribution {
    pub bnb: u8,
    pub stake: u8,
    pub treasury: u8,
}

impl Distribution {
    pub fn assert_valid(&self) -> Result<()> {
        if self.bnb + self.stake + self.treasury != 100 {
            return Err(ErrorCode::InvalidDistribution.into());
        }
        Ok(())
    }
}

// CpiContext transformations.

impl<'info> From<&SweepFees<'info>> for CpiContext<'_, '_, '_, 'info, dex::SweepFees<'info>> {
    fn from(sweep: &SweepFees<'info>) -> Self {
        let program = sweep.dex.dex_program.to_account_info();
        let accounts = dex::SweepFees {
            market: sweep.dex.market.to_account_info(),
            pc_vault: sweep.dex.pc_vault.to_account_info(),
            sweep_authority: sweep.dex.sweep_authority.to_account_info(),
            sweep_receiver: sweep.dex.sweep_receiver.to_account_info(),
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
						// todo
				};
        CpiContext::new(program, accounts)
    }
}

/*
impl<'info> From<&SwapToSrmTransitive<'info>> for CpiContext<'_, '_, '_, 'info, swap::SwapTransitive<'info>> {
    fn from(accs: &SwapToSrmTransitive<'info>) -> Self {
        let program = accs.swap_program.to_account_info();
        let accounts = swap::SwapTransitive {
// todo
                };
CpiContext::new(program, accounts)
    }
}
 */

// Events.

#[event]
pub struct DistributionDidChange {
    distribution: Distribution,
}
#[event]
pub struct OfficerDidCreate {
    officer: Officer,
    pubkey: Pubkey,
}

// Error.

#[error]
pub enum ErrorCode {
    #[msg("Distribution does not add to 100")]
    InvalidDistribution,
}

// Access control.

fn is_distribution_ready(accounts: &Distribute) -> Result<()> {
    // todo
    Ok(())
}

fn is_not_trading() -> Result<()> {
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
        };
        Self {
            rate,
            from_decimals,
            quote_decimals,
            strict,
        }
    }
}
