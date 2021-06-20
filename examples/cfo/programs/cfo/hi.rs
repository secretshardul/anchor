#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::associated_seeds;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, system_program};
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_spl::{dex, mint};
use registry::Registrar;
use cfo::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
#[cfg(not(feature = "no-entrypoint"))]
fn entry(program_id: &Pubkey, accounts: &[AccountInfo], ix_data: &[u8]) -> ProgramResult {
    if ix_data.len() < 8 {
        return Err(anchor_lang::__private::ErrorCode::InstructionMissing.into());
    }
    let mut ix_data: &[u8] = ix_data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    dispatch(program_id, accounts, sighash, ix_data).map_err(|e| {
        ::solana_program::log::sol_log(&e.to_string());
        e
    })
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    sighash: [u8; 8],
    ix_data: &[u8],
) -> ProgramResult {
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [26, 147, 121, 196, 232, 153, 199, 151] => {
            __private::__global::create_officer(program_id, accounts, ix_data)
        }
        [48, 213, 57, 212, 236, 191, 213, 24] => {
            __private::__global::create_officer_token(program_id, accounts, ix_data)
        }
        [236, 132, 205, 219, 156, 248, 162, 249] => {
            __private::__global::set_distribution(program_id, accounts, ix_data)
        }
        [175, 225, 98, 71, 118, 66, 34, 148] => {
            __private::__global::sweep_fees(program_id, accounts, ix_data)
        }
        [13, 146, 142, 174, 170, 132, 194, 49] => {
            __private::__global::swap_to_usdc(program_id, accounts, ix_data)
        }
        [190, 148, 108, 227, 114, 113, 5, 126] => {
            __private::__global::swap_to_srm(program_id, accounts, ix_data)
        }
        [191, 44, 223, 207, 164, 236, 126, 61] => {
            __private::__global::distribute(program_id, accounts, ix_data)
        }
        [255, 234, 236, 154, 46, 236, 103, 225] => {
            __private::__global::drop_stake_reward(program_id, accounts, ix_data)
        }
        _ => {
            :: solana_program :: log :: sol_log ( "Fallback functions are not supported. If you have a use case, please file an issue." ) ;
            Err(anchor_lang::__private::ErrorCode::InstructionFallbackNotFound.into())
        }
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> ProgramResult {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> ProgramResult {
            if program_id != accounts.program.key {
                return Err(anchor_lang::__private::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let lamports = accounts.rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> ProgramResult {
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> ProgramResult {
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> ProgramResult {
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> ProgramResult {
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create_officer(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::CreateOfficer::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateOfficer {
                d,
                registrar,
                msrm_registrar,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CreateOfficer::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::create_officer(
                Context::new(program_id, &mut accounts, remaining_accounts),
                d,
                registrar,
                msrm_registrar,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn create_officer_token(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::CreateOfficerToken::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateOfficerToken = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CreateOfficerToken::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::create_officer_token(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn set_distribution(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::SetDistribution::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SetDistribution { d } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SetDistribution::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::set_distribution(
                Context::new(program_id, &mut accounts, remaining_accounts),
                d,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn sweep_fees(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::SweepFees::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SweepFees = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SweepFees::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::sweep_fees(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn swap_to_usdc(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::SwapToUsdc::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SwapToUsdc { min_exchange_rate } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SwapToUsdc::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::swap_to_usdc(
                Context::new(program_id, &mut accounts, remaining_accounts),
                min_exchange_rate,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn swap_to_srm(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::SwapToSrm::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SwapToSrm { min_exchange_rate } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SwapToSrm::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::swap_to_srm(
                Context::new(program_id, &mut accounts, remaining_accounts),
                min_exchange_rate,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn distribute(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Distribute::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Distribute = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Distribute::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::distribute(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn drop_stake_reward(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::DropStakeReward::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DropStakeReward = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                DropStakeReward::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            cfo::drop_stake_reward(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
    }
}
/// CFO is the program representing the Serum chief financial officer. It is
/// the program responsible for collecting and distributing fees from the Serum
/// DEX.
pub mod cfo {
    use super::*;
    pub fn create_officer(
        ctx: Context<CreateOfficer>,
        d: Distribution,
        registrar: Pubkey,
        msrm_registrar: Pubkey,
    ) -> Result<()> {
        is_distribution_valid(&d)?;
        let officer = &mut ctx.accounts.officer;
        officer.authority = *ctx.accounts.authority.key;
        officer.swap_program = *ctx.accounts.swap_program.key;
        officer.dex_program = *ctx.accounts.dex_program.key;
        officer.distribution = d;
        officer.registrar = registrar;
        officer.msrm_registrar = msrm_registrar;
        {
            let data = anchor_lang::Event::data(&OfficerDidCreate {
                pubkey: *officer.to_account_info().key,
            });
            let msg_str = &anchor_lang::__private::base64::encode(data);
            ::solana_program::log::sol_log(msg_str);
        };
        Ok(())
    }
    /// Creates a deterministic token account owned by the CFO.
    /// This should be used when a new mint is used for collecting fees.
    /// Can only be called once per token CFO and token mint.
    pub fn create_officer_token(_ctx: Context<CreateOfficerToken>) -> Result<()> {
        Ok(())
    }
    pub fn set_distribution(ctx: Context<SetDistribution>, d: Distribution) -> Result<()> {
        is_distribution_valid(&d)?;
        ctx.accounts.officer.distribution = d.clone();
        {
            let data = anchor_lang::Event::data(&DistributionDidChange { distribution: d });
            let msg_str = &anchor_lang::__private::base64::encode(data);
            ::solana_program::log::sol_log(msg_str);
        };
        Ok(())
    }
    /// Transfers fees from the dex to the CFO.
    pub fn sweep_fees<'info>(ctx: Context<'_, '_, '_, 'info, SweepFees<'info>>) -> Result<()> {
        let seeds = &[
            b"anchor".as_ref(),
            ctx.accounts.dex.dex_program.to_account_info().key.as_ref(),
            &[anchor_lang::Bump::seed(&*ctx.accounts.officer)],
        ];
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, dex::SweepFees<'info>> = (&*ctx.accounts).into();
        dex::sweep_fees(cpi_ctx.with_signer(&[seeds]))?;
        Ok(())
    }
    pub fn swap_to_usdc<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToUsdc<'info>>,
        min_exchange_rate: ExchangeRate,
    ) -> Result<()> {
        is_not_trading()?;
        let seeds = &[
            b"anchor".as_ref(),
            ctx.accounts.dex_program.to_account_info().key.as_ref(),
            &[anchor_lang::Bump::seed(&*ctx.accounts.officer)],
        ];
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> = (&*ctx.accounts).into();
        swap::cpi::swap(
            cpi_ctx.with_signer(&[seeds]),
            swap::Side::Bid,
            token::accessor::amount(&ctx.accounts.from_vault)?,
            min_exchange_rate.into(),
        )?;
        Ok(())
    }
    pub fn swap_to_srm<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToSrm<'info>>,
        min_exchange_rate: ExchangeRate,
    ) -> Result<()> {
        is_not_trading()?;
        let seeds = &[
            b"anchor".as_ref(),
            ctx.accounts.dex_program.to_account_info().key.as_ref(),
            &[anchor_lang::Bump::seed(&*ctx.accounts.officer)],
        ];
        let cpi_ctx: CpiContext<'_, '_, '_, 'info, swap::Swap<'info>> = (&*ctx.accounts).into();
        swap::cpi::swap(
            cpi_ctx.with_signer(&[seeds]),
            swap::Side::Bid,
            token::accessor::amount(&ctx.accounts.from_vault)?,
            min_exchange_rate.into(),
        )?;
        Ok(())
    }
    pub fn distribute<'info>(ctx: Context<'_, '_, '_, 'info, Distribute<'info>>) -> Result<()> {
        is_distribution_ready(&ctx.accounts)?;
        Ok(())
    }
    pub fn drop_stake_reward<'info>(
        ctx: Context<'_, '_, '_, 'info, DropStakeReward<'info>>,
    ) -> Result<()> {
        is_stake_reward_ready(&ctx.accounts)?;
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct CreateOfficer {
        pub d: Distribution,
        pub registrar: Pubkey,
        pub msrm_registrar: Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateOfficer
    where
        Distribution: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.d, writer)?;
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.msrm_registrar, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateOfficer
    where
        Distribution: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                d: borsh::BorshDeserialize::deserialize(buf)?,
                registrar: borsh::BorshDeserialize::deserialize(buf)?,
                msrm_registrar: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CreateOfficer {
        fn data(&self) -> Vec<u8> {
            let mut d = [26, 147, 121, 196, 232, 153, 199, 151].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CreateOfficerToken;
    impl borsh::ser::BorshSerialize for CreateOfficerToken {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateOfficerToken {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for CreateOfficerToken {
        fn data(&self) -> Vec<u8> {
            let mut d = [48, 213, 57, 212, 236, 191, 213, 24].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SetDistribution {
        pub d: Distribution,
    }
    impl borsh::ser::BorshSerialize for SetDistribution
    where
        Distribution: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.d, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetDistribution
    where
        Distribution: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                d: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SetDistribution {
        fn data(&self) -> Vec<u8> {
            let mut d = [236, 132, 205, 219, 156, 248, 162, 249].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SweepFees;
    impl borsh::ser::BorshSerialize for SweepFees {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SweepFees {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SweepFees {
        fn data(&self) -> Vec<u8> {
            let mut d = [175, 225, 98, 71, 118, 66, 34, 148].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SwapToUsdc {
        pub min_exchange_rate: ExchangeRate,
    }
    impl borsh::ser::BorshSerialize for SwapToUsdc
    where
        ExchangeRate: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.min_exchange_rate, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SwapToUsdc
    where
        ExchangeRate: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                min_exchange_rate: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SwapToUsdc {
        fn data(&self) -> Vec<u8> {
            let mut d = [13, 146, 142, 174, 170, 132, 194, 49].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SwapToSrm {
        pub min_exchange_rate: ExchangeRate,
    }
    impl borsh::ser::BorshSerialize for SwapToSrm
    where
        ExchangeRate: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.min_exchange_rate, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SwapToSrm
    where
        ExchangeRate: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                min_exchange_rate: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SwapToSrm {
        fn data(&self) -> Vec<u8> {
            let mut d = [190, 148, 108, 227, 114, 113, 5, 126].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Distribute;
    impl borsh::ser::BorshSerialize for Distribute {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Distribute {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for Distribute {
        fn data(&self) -> Vec<u8> {
            let mut d = [191, 44, 223, 207, 164, 236, 126, 61].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DropStakeReward;
    impl borsh::ser::BorshSerialize for DropStakeReward {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DropStakeReward {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for DropStakeReward {
        fn data(&self) -> Vec<u8> {
            let mut d = [255, 234, 236, 154, 46, 236, 103, 225].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_set_distribution::*;
    pub use crate::__client_accounts_distribute::*;
    pub use crate::__client_accounts_swap_to_srm::*;
    pub use crate::__client_accounts_swap_to_usdc::*;
    pub use crate::__client_accounts_drop_stake_reward::*;
    pub use crate::__client_accounts_create_officer::*;
    pub use crate::__client_accounts_create_officer_token::*;
    pub use crate::__client_accounts_sweep_fees::*;
}
pub struct CreateOfficer<'info> {
    # [ account ( init , associated = dex_program , payer = authority ) ]
    officer: ProgramAccount<'info, Officer>,
    # [ account ( init , token , associated = officer , with = b"stake" , with = mint , space = TokenAccount :: LEN , payer = authority , ) ]
    stake: CpiAccount<'info, TokenAccount>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    #[account(executable)]
    dex_program: AccountInfo<'info>,
    #[account(executable)]
    swap_program: AccountInfo<'info>,
    # [ account ( address = system_program :: ID ) ]
    system_program: AccountInfo<'info>,
    # [ account ( address = spl_token :: ID ) ]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}
impl<'info> anchor_lang::Accounts<'info> for CreateOfficer<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer = &accounts[0];
        *accounts = &accounts[1..];
        let stake = &accounts[0];
        *accounts = &accounts[1..];
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let mint: AccountInfo = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let dex_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let swap_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let officer: anchor_lang::ProgramAccount<Officer> = {
            let space = 8 + Officer::default().try_to_vec().unwrap().len();
            let payer = authority.to_account_info();
            let (__associated_field, nonce) = Pubkey::find_program_address(
                &[&b"anchor"[..], dex_program.to_account_info().key.as_ref()],
                program_id,
            );
            if &__associated_field != officer.to_account_info().key {
                return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
            }
            let lamports = rent.minimum_balance(space);
            let ix = anchor_lang::solana_program::system_instruction::create_account(
                payer.to_account_info().key,
                officer.to_account_info().key,
                lamports,
                space as u64,
                program_id,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    officer.to_account_info(),
                    payer.to_account_info(),
                    system_program.to_account_info(),
                ],
                &[&[
                    &b"anchor"[..],
                    dex_program.to_account_info().key.as_ref(),
                    &[nonce],
                ][..]],
            )
            .map_err(|e| {
                ::solana_program::log::sol_log("Unable to create associated account");
                e
            })?;
            let mut pa: anchor_lang::ProgramAccount<Officer> =
                anchor_lang::ProgramAccount::try_from_init(&officer.to_account_info())?;
            pa.__nonce = nonce;
            pa
        };
        if !officer.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !rent.is_exempt(
            officer.to_account_info().lamports(),
            officer.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        let stake: anchor_lang::CpiAccount<TokenAccount> = {
            let space = TokenAccount::LEN;
            let payer = authority.to_account_info();
            let (__associated_field, nonce) = Pubkey::find_program_address(
                &[
                    &b"anchor"[..],
                    officer.to_account_info().key.as_ref(),
                    b"stake",
                    anchor_lang::Key::key(&mint).as_ref(),
                ],
                program_id,
            );
            if &__associated_field != stake.to_account_info().key {
                return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
            }
            let required_lamports = rent
                .minimum_balance(anchor_spl::token::TokenAccount::LEN)
                .max(1)
                .saturating_sub(stake.to_account_info().lamports());
            if required_lamports > 0 {
                anchor_lang::solana_program::program::invoke(
                    &system_instruction::transfer(
                        payer.to_account_info().key,
                        stake.to_account_info().key,
                        required_lamports,
                    ),
                    &[
                        payer.to_account_info(),
                        stake.to_account_info(),
                        system_program.to_account_info().clone(),
                    ],
                )?;
            }
            anchor_lang::solana_program::program::invoke_signed(
                &anchor_lang::solana_program::system_instruction::allocate(
                    stake.to_account_info().key,
                    anchor_spl::token::TokenAccount::LEN as u64,
                ),
                &[stake.to_account_info(), system_program.clone()],
                &[&[
                    &b"anchor"[..],
                    officer.to_account_info().key.as_ref(),
                    b"stake",
                    anchor_lang::Key::key(&mint).as_ref(),
                    &[nonce],
                ][..]],
            )?;
            let __ix = system_instruction::assign(
                stake.to_account_info().key,
                token_program.to_account_info().key,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &__ix,
                &[stake.to_account_info(), system_program.to_account_info()],
                &[&[
                    &b"anchor"[..],
                    officer.to_account_info().key.as_ref(),
                    b"stake",
                    anchor_lang::Key::key(&mint).as_ref(),
                    &[nonce],
                ][..]],
            )?;
            let cpi_program = token_program.to_account_info();
            let accounts = anchor_spl::token::InitializeAccount {
                account: stake.to_account_info(),
                mint: mint.to_account_info(),
                authority: officer.to_account_info(),
                rent: rent.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, accounts);
            anchor_spl::token::initialize_account(cpi_ctx)?;
            anchor_lang::CpiAccount::try_from_init(&stake.to_account_info())?
        };
        if !stake.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !rent.is_exempt(
            stake.to_account_info().lamports(),
            stake.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if true {
            if !authority.to_account_info().is_signer {
                return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
            }
        }
        if !dex_program.to_account_info().executable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintExecutable.into());
        }
        if !swap_program.to_account_info().executable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintExecutable.into());
        }
        if system_program.to_account_info().key != &system_program::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if token_program.to_account_info().key != &spl_token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(CreateOfficer {
            officer,
            stake,
            authority,
            mint,
            dex_program,
            swap_program,
            system_program,
            token_program,
            rent,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateOfficer<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.stake.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.dex_program.to_account_infos());
        account_infos.extend(self.swap_program.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for CreateOfficer<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.stake.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.dex_program.to_account_metas(None));
        account_metas.extend(self.swap_program.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for CreateOfficer<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.officer, program_id)?;
        anchor_lang::AccountsExit::exit(&self.stake, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_create_officer {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct CreateOfficer {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub stake: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub swap_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateOfficer
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.stake, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
            borsh::BorshSerialize::serialize(&self.swap_program, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for CreateOfficer {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.officer,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.stake, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.mint, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.dex_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.swap_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas
        }
    }
}
pub struct CreateOfficerToken<'info> {
    officer: ProgramAccount<'info, Officer>,
    # [ account ( init , token , associated = officer , with = mint , space = TokenAccount :: LEN , payer = payer , ) ]
    token: CpiAccount<'info, TokenAccount>,
    # [ account ( owner = token_program ) ]
    mint: CpiAccount<'info, Mint>,
    #[account(mut, signer)]
    payer: AccountInfo<'info>,
    # [ account ( address = system_program :: ID ) ]
    system_program: AccountInfo<'info>,
    # [ account ( address = spl_token :: ID ) ]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}
impl<'info> anchor_lang::Accounts<'info> for CreateOfficerToken<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token = &accounts[0];
        *accounts = &accounts[1..];
        let mint: CpiAccount<Mint> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let payer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token: anchor_lang::CpiAccount<TokenAccount> = {
            let space = TokenAccount::LEN;
            let payer = payer.to_account_info();
            let (__associated_field, nonce) = Pubkey::find_program_address(
                &[
                    &b"anchor"[..],
                    officer.to_account_info().key.as_ref(),
                    anchor_lang::Key::key(&mint).as_ref(),
                ],
                program_id,
            );
            if &__associated_field != token.to_account_info().key {
                return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
            }
            let required_lamports = rent
                .minimum_balance(anchor_spl::token::TokenAccount::LEN)
                .max(1)
                .saturating_sub(token.to_account_info().lamports());
            if required_lamports > 0 {
                anchor_lang::solana_program::program::invoke(
                    &system_instruction::transfer(
                        payer.to_account_info().key,
                        token.to_account_info().key,
                        required_lamports,
                    ),
                    &[
                        payer.to_account_info(),
                        token.to_account_info(),
                        system_program.to_account_info().clone(),
                    ],
                )?;
            }
            anchor_lang::solana_program::program::invoke_signed(
                &anchor_lang::solana_program::system_instruction::allocate(
                    token.to_account_info().key,
                    anchor_spl::token::TokenAccount::LEN as u64,
                ),
                &[token.to_account_info(), system_program.clone()],
                &[&[
                    &b"anchor"[..],
                    officer.to_account_info().key.as_ref(),
                    anchor_lang::Key::key(&mint).as_ref(),
                    &[nonce],
                ][..]],
            )?;
            let __ix = system_instruction::assign(
                token.to_account_info().key,
                token_program.to_account_info().key,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &__ix,
                &[token.to_account_info(), system_program.to_account_info()],
                &[&[
                    &b"anchor"[..],
                    officer.to_account_info().key.as_ref(),
                    anchor_lang::Key::key(&mint).as_ref(),
                    &[nonce],
                ][..]],
            )?;
            let cpi_program = token_program.to_account_info();
            let accounts = anchor_spl::token::InitializeAccount {
                account: token.to_account_info(),
                mint: mint.to_account_info(),
                authority: officer.to_account_info(),
                rent: rent.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, accounts);
            anchor_spl::token::initialize_account(cpi_ctx)?;
            anchor_lang::CpiAccount::try_from_init(&token.to_account_info())?
        };
        if !token.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !rent.is_exempt(
            token.to_account_info().lamports(),
            token.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if mint.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        if !payer.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if true {
            if !payer.to_account_info().is_signer {
                return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
            }
        }
        if system_program.to_account_info().key != &system_program::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if token_program.to_account_info().key != &spl_token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(CreateOfficerToken {
            officer,
            token,
            mint,
            payer,
            system_program,
            token_program,
            rent,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateOfficerToken<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.token.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.payer.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for CreateOfficerToken<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.token.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.payer.to_account_metas(Some(true)));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for CreateOfficerToken<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.token, program_id)?;
        anchor_lang::AccountsExit::exit(&self.payer, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_create_officer_token {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct CreateOfficerToken {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token: anchor_lang::solana_program::pubkey::Pubkey,
        pub mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub payer: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateOfficerToken
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.token, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.payer, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for CreateOfficerToken {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.token, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.mint, false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer, true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas
        }
    }
}
pub struct SetDistribution<'info> {
    # [ account ( has_one = authority ) ]
    officer: ProgramAccount<'info, Officer>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for SetDistribution<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &officer.authority != authority.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintBelongsTo.into());
        }
        if true {
            if !authority.to_account_info().is_signer {
                return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
            }
        }
        Ok(SetDistribution { officer, authority })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for SetDistribution<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for SetDistribution<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for SetDistribution<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_set_distribution {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct SetDistribution {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetDistribution
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for SetDistribution {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
                ),
            );
            account_metas
        }
    }
}
pub struct SweepFees<'info> {
    # [ account ( associated = dex . dex_program ) ]
    officer: ProgramAccount<'info, Officer>,
    # [ account ( mut , owner = dex . token_program , associated = officer , with = mint , ) ]
    sweep_vault: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    dex: Dex<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for SweepFees<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let sweep_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let mint: AccountInfo = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let dex: Dex<'info> = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let (__associated_field, nonce) = Pubkey::find_program_address(
            &[
                &b"anchor"[..],
                dex.dex_program.to_account_info().key.as_ref(),
            ],
            program_id,
        );
        if &__associated_field != officer.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
        }
        let (__associated_field, nonce) = Pubkey::find_program_address(
            &[
                &b"anchor"[..],
                officer.to_account_info().key.as_ref(),
                anchor_lang::Key::key(&mint).as_ref(),
            ],
            program_id,
        );
        if &__associated_field != sweep_vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
        }
        if !sweep_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if sweep_vault.to_account_info().owner != dex.token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        Ok(SweepFees {
            officer,
            sweep_vault,
            mint,
            dex,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for SweepFees<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.sweep_vault.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.dex.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for SweepFees<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.sweep_vault.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.dex.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for SweepFees<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.sweep_vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.dex, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_sweep_fees {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_dex::Dex;
    pub struct SweepFees {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub sweep_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub dex: __client_accounts_dex::Dex,
    }
    impl borsh::ser::BorshSerialize for SweepFees
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_dex::Dex: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.sweep_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.dex, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for SweepFees {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.sweep_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.mint, false,
                ),
            );
            account_metas.extend(self.dex.to_account_metas(None));
            account_metas
        }
    }
}
pub struct Dex<'info> {
    #[account(mut)]
    market: AccountInfo<'info>,
    #[account(mut)]
    pc_vault: AccountInfo<'info>,
    sweep_authority: AccountInfo<'info>,
    vault_signer: AccountInfo<'info>,
    dex_program: AccountInfo<'info>,
    # [ account ( address = spl_token :: ID ) ]
    token_program: AccountInfo<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for Dex<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let market: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pc_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let sweep_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let dex_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !market.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !pc_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if token_program.to_account_info().key != &spl_token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(Dex {
            market,
            pc_vault,
            sweep_authority,
            vault_signer,
            dex_program,
            token_program,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for Dex<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.market.to_account_infos());
        account_infos.extend(self.pc_vault.to_account_infos());
        account_infos.extend(self.sweep_authority.to_account_infos());
        account_infos.extend(self.vault_signer.to_account_infos());
        account_infos.extend(self.dex_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for Dex<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.market.to_account_metas(None));
        account_metas.extend(self.pc_vault.to_account_metas(None));
        account_metas.extend(self.sweep_authority.to_account_metas(None));
        account_metas.extend(self.vault_signer.to_account_metas(None));
        account_metas.extend(self.dex_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for Dex<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.market, program_id)?;
        anchor_lang::AccountsExit::exit(&self.pc_vault, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_dex {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Dex {
        pub market: anchor_lang::solana_program::pubkey::Pubkey,
        pub pc_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub sweep_authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Dex
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.market, writer)?;
            borsh::BorshSerialize::serialize(&self.pc_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.sweep_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.vault_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for Dex {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.market,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pc_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.sweep_authority,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vault_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.dex_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
pub struct SwapToUsdc<'info> {
    # [ account ( associated = dex_program ) ]
    officer: ProgramAccount<'info, Officer>,
    market: DexMarketAccounts<'info>,
    # [ account ( owner = token_program , constraint = & officer . treasury != from_vault . key , constraint = & officer . stake != from_vault . key , ) ]
    from_vault: AccountInfo<'info>,
    # [ account ( owner = token_program ) ]
    quote_vault: AccountInfo<'info>,
    # [ account ( associated = officer , with = mint :: USDC , ) ]
    usdc_vault: AccountInfo<'info>,
    # [ account ( address = swap :: ID ) ]
    swap_program: AccountInfo<'info>,
    # [ account ( address = dex :: ID ) ]
    dex_program: AccountInfo<'info>,
    # [ account ( address = token :: ID ) ]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}
impl<'info> anchor_lang::Accounts<'info> for SwapToUsdc<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let market: DexMarketAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let from_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let quote_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let usdc_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let swap_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let dex_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let (__associated_field, nonce) = Pubkey::find_program_address(
            &[&b"anchor"[..], dex_program.to_account_info().key.as_ref()],
            program_id,
        );
        if &__associated_field != officer.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
        }
        if !(&officer.treasury != from_vault.key) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if !(&officer.stake != from_vault.key) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if from_vault.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        if quote_vault.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        let (__associated_field, nonce) = Pubkey::find_program_address(
            &[
                &b"anchor"[..],
                officer.to_account_info().key.as_ref(),
                anchor_lang::Key::key(&mint::USDC).as_ref(),
            ],
            program_id,
        );
        if &__associated_field != usdc_vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
        }
        if swap_program.to_account_info().key != &swap::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if dex_program.to_account_info().key != &dex::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if token_program.to_account_info().key != &token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(SwapToUsdc {
            officer,
            market,
            from_vault,
            quote_vault,
            usdc_vault,
            swap_program,
            dex_program,
            token_program,
            rent,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for SwapToUsdc<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.market.to_account_infos());
        account_infos.extend(self.from_vault.to_account_infos());
        account_infos.extend(self.quote_vault.to_account_infos());
        account_infos.extend(self.usdc_vault.to_account_infos());
        account_infos.extend(self.swap_program.to_account_infos());
        account_infos.extend(self.dex_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for SwapToUsdc<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.market.to_account_metas(None));
        account_metas.extend(self.from_vault.to_account_metas(None));
        account_metas.extend(self.quote_vault.to_account_metas(None));
        account_metas.extend(self.usdc_vault.to_account_metas(None));
        account_metas.extend(self.swap_program.to_account_metas(None));
        account_metas.extend(self.dex_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for SwapToUsdc<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.market, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_swap_to_usdc {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_dex_market_accounts::DexMarketAccounts;
    pub struct SwapToUsdc {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub market: __client_accounts_dex_market_accounts::DexMarketAccounts,
        pub from_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub quote_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub usdc_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub swap_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SwapToUsdc
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_dex_market_accounts::DexMarketAccounts: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.market, writer)?;
            borsh::BorshSerialize::serialize(&self.from_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.usdc_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.swap_program, writer)?;
            borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for SwapToUsdc {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.extend(self.market.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.from_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.quote_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.usdc_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.swap_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.dex_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas
        }
    }
}
pub struct SwapToSrm<'info> {
    # [ account ( associated = dex_program ) ]
    officer: ProgramAccount<'info, Officer>,
    market: DexMarketAccounts<'info>,
    # [ account ( owner = token_program , constraint = & officer . treasury != from_vault . key , constraint = & officer . stake != from_vault . key , ) ]
    from_vault: AccountInfo<'info>,
    # [ account ( owner = token_program ) ]
    quote_vault: AccountInfo<'info>,
    # [ account ( associated = officer , with = mint :: SRM , constraint = & officer . treasury != from_vault . key , constraint = & officer . stake != from_vault . key , ) ]
    srm_vault: AccountInfo<'info>,
    # [ account ( address = swap :: ID ) ]
    swap_program: AccountInfo<'info>,
    # [ account ( address = dex :: ID ) ]
    dex_program: AccountInfo<'info>,
    # [ account ( address = token :: ID ) ]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}
impl<'info> anchor_lang::Accounts<'info> for SwapToSrm<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let market: DexMarketAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let from_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let quote_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let srm_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let swap_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let dex_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let (__associated_field, nonce) = Pubkey::find_program_address(
            &[&b"anchor"[..], dex_program.to_account_info().key.as_ref()],
            program_id,
        );
        if &__associated_field != officer.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
        }
        if !(&officer.treasury != from_vault.key) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if !(&officer.stake != from_vault.key) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if from_vault.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        if quote_vault.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        let (__associated_field, nonce) = Pubkey::find_program_address(
            &[
                &b"anchor"[..],
                officer.to_account_info().key.as_ref(),
                anchor_lang::Key::key(&mint::SRM).as_ref(),
            ],
            program_id,
        );
        if &__associated_field != srm_vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAssociatedInit.into());
        }
        if !(&officer.treasury != from_vault.key) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if !(&officer.stake != from_vault.key) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if swap_program.to_account_info().key != &swap::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if dex_program.to_account_info().key != &dex::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if token_program.to_account_info().key != &token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(SwapToSrm {
            officer,
            market,
            from_vault,
            quote_vault,
            srm_vault,
            swap_program,
            dex_program,
            token_program,
            rent,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for SwapToSrm<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.market.to_account_infos());
        account_infos.extend(self.from_vault.to_account_infos());
        account_infos.extend(self.quote_vault.to_account_infos());
        account_infos.extend(self.srm_vault.to_account_infos());
        account_infos.extend(self.swap_program.to_account_infos());
        account_infos.extend(self.dex_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for SwapToSrm<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.market.to_account_metas(None));
        account_metas.extend(self.from_vault.to_account_metas(None));
        account_metas.extend(self.quote_vault.to_account_metas(None));
        account_metas.extend(self.srm_vault.to_account_metas(None));
        account_metas.extend(self.swap_program.to_account_metas(None));
        account_metas.extend(self.dex_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for SwapToSrm<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.market, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_swap_to_srm {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_dex_market_accounts::DexMarketAccounts;
    pub struct SwapToSrm {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub market: __client_accounts_dex_market_accounts::DexMarketAccounts,
        pub from_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub quote_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub srm_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub swap_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SwapToSrm
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_dex_market_accounts::DexMarketAccounts: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.market, writer)?;
            borsh::BorshSerialize::serialize(&self.from_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.srm_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.swap_program, writer)?;
            borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for SwapToSrm {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.extend(self.market.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.from_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.quote_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.srm_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.swap_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.dex_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas
        }
    }
}
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
    #[account(mut)]
    order_payer_token_account: AccountInfo<'info>,
    #[account(mut)]
    coin_vault: AccountInfo<'info>,
    #[account(mut)]
    pc_vault: AccountInfo<'info>,
    vault_signer: AccountInfo<'info>,
    #[account(mut)]
    coin_wallet: AccountInfo<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for DexMarketAccounts<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let market: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let open_orders: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let request_queue: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let event_queue: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let bids: AccountInfo = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let asks: AccountInfo = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let order_payer_token_account: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let coin_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pc_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let coin_wallet: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !market.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !open_orders.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !request_queue.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !event_queue.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !bids.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !asks.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !order_payer_token_account.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !coin_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !pc_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !coin_wallet.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(DexMarketAccounts {
            market,
            open_orders,
            request_queue,
            event_queue,
            bids,
            asks,
            order_payer_token_account,
            coin_vault,
            pc_vault,
            vault_signer,
            coin_wallet,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for DexMarketAccounts<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.market.to_account_infos());
        account_infos.extend(self.open_orders.to_account_infos());
        account_infos.extend(self.request_queue.to_account_infos());
        account_infos.extend(self.event_queue.to_account_infos());
        account_infos.extend(self.bids.to_account_infos());
        account_infos.extend(self.asks.to_account_infos());
        account_infos.extend(self.order_payer_token_account.to_account_infos());
        account_infos.extend(self.coin_vault.to_account_infos());
        account_infos.extend(self.pc_vault.to_account_infos());
        account_infos.extend(self.vault_signer.to_account_infos());
        account_infos.extend(self.coin_wallet.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for DexMarketAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.market.to_account_metas(None));
        account_metas.extend(self.open_orders.to_account_metas(None));
        account_metas.extend(self.request_queue.to_account_metas(None));
        account_metas.extend(self.event_queue.to_account_metas(None));
        account_metas.extend(self.bids.to_account_metas(None));
        account_metas.extend(self.asks.to_account_metas(None));
        account_metas.extend(self.order_payer_token_account.to_account_metas(None));
        account_metas.extend(self.coin_vault.to_account_metas(None));
        account_metas.extend(self.pc_vault.to_account_metas(None));
        account_metas.extend(self.vault_signer.to_account_metas(None));
        account_metas.extend(self.coin_wallet.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for DexMarketAccounts<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.market, program_id)?;
        anchor_lang::AccountsExit::exit(&self.open_orders, program_id)?;
        anchor_lang::AccountsExit::exit(&self.request_queue, program_id)?;
        anchor_lang::AccountsExit::exit(&self.event_queue, program_id)?;
        anchor_lang::AccountsExit::exit(&self.bids, program_id)?;
        anchor_lang::AccountsExit::exit(&self.asks, program_id)?;
        anchor_lang::AccountsExit::exit(&self.order_payer_token_account, program_id)?;
        anchor_lang::AccountsExit::exit(&self.coin_vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.pc_vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.coin_wallet, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_dex_market_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct DexMarketAccounts {
        pub market: anchor_lang::solana_program::pubkey::Pubkey,
        pub open_orders: anchor_lang::solana_program::pubkey::Pubkey,
        pub request_queue: anchor_lang::solana_program::pubkey::Pubkey,
        pub event_queue: anchor_lang::solana_program::pubkey::Pubkey,
        pub bids: anchor_lang::solana_program::pubkey::Pubkey,
        pub asks: anchor_lang::solana_program::pubkey::Pubkey,
        pub order_payer_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub coin_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub pc_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub coin_wallet: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for DexMarketAccounts
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.market, writer)?;
            borsh::BorshSerialize::serialize(&self.open_orders, writer)?;
            borsh::BorshSerialize::serialize(&self.request_queue, writer)?;
            borsh::BorshSerialize::serialize(&self.event_queue, writer)?;
            borsh::BorshSerialize::serialize(&self.bids, writer)?;
            borsh::BorshSerialize::serialize(&self.asks, writer)?;
            borsh::BorshSerialize::serialize(&self.order_payer_token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.coin_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.pc_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vault_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.coin_wallet, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for DexMarketAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.market,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.open_orders,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.request_queue,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.event_queue,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.bids, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.asks, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.order_payer_token_account,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.coin_vault,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pc_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vault_signer,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.coin_wallet,
                false,
            ));
            account_metas
        }
    }
}
pub struct Distribute<'info> {
    officer: ProgramAccount<'info, Officer>,
    # [ account ( owner = token_program , constraint = token :: accessor :: mint ( & srm_vault ) ? == mint :: SRM , ) ]
    srm_vault: AccountInfo<'info>,
    # [ account ( address = mint :: SRM ) ]
    mint: AccountInfo<'info>,
    # [ account ( address = spl_token :: ID ) ]
    token_program: AccountInfo<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for Distribute<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let srm_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let mint: AccountInfo = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !(token::accessor::mint(&srm_vault)? == mint::SRM) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if srm_vault.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        if mint.to_account_info().key != &mint::SRM {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if token_program.to_account_info().key != &spl_token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(Distribute {
            officer,
            srm_vault,
            mint,
            token_program,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for Distribute<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.srm_vault.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for Distribute<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.srm_vault.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for Distribute<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_distribute {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Distribute {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub srm_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Distribute
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.srm_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for Distribute {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.srm_vault,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.mint, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
pub struct DropStakeReward<'info> {
    # [ account ( has_one = stake , constraint = srm . registrar . key == & officer . registrar , constraint = msrm . registrar . key == & officer . msrm_registrar , ) ]
    officer: ProgramAccount<'info, Officer>,
    # [ account ( owner = token_program , ) ]
    stake: CpiAccount<'info, TokenAccount>,
    srm: DropStakeRewardPool<'info>,
    msrm: DropStakeRewardPool<'info>,
    # [ account ( owner = registry_program ) ]
    msrm_registrar: CpiAccount<'info, Registrar>,
    # [ account ( address = token :: ID ) ]
    token_program: AccountInfo<'info>,
    # [ account ( address = registry :: ID ) ]
    registry_program: AccountInfo<'info>,
    # [ account ( address = lockup :: ID ) ]
    lockup_program: AccountInfo<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for DropStakeReward<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let officer: ProgramAccount<Officer> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let stake: CpiAccount<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let srm: DropStakeRewardPool<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let msrm: DropStakeRewardPool<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let msrm_registrar: CpiAccount<Registrar> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registry_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let lockup_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &officer.stake != stake.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintBelongsTo.into());
        }
        if !(srm.registrar.key == &officer.registrar) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if !(msrm.registrar.key == &officer.msrm_registrar) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        if stake.to_account_info().owner != token_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        if msrm_registrar.to_account_info().owner != registry_program.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
        }
        if token_program.to_account_info().key != &token::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if registry_program.to_account_info().key != &registry::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        if lockup_program.to_account_info().key != &lockup::ID {
            return Err(anchor_lang::__private::ErrorCode::ConstraintAddress.into());
        }
        Ok(DropStakeReward {
            officer,
            stake,
            srm,
            msrm,
            msrm_registrar,
            token_program,
            registry_program,
            lockup_program,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for DropStakeReward<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.officer.to_account_infos());
        account_infos.extend(self.stake.to_account_infos());
        account_infos.extend(self.srm.to_account_infos());
        account_infos.extend(self.msrm.to_account_infos());
        account_infos.extend(self.msrm_registrar.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.registry_program.to_account_infos());
        account_infos.extend(self.lockup_program.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for DropStakeReward<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.officer.to_account_metas(None));
        account_metas.extend(self.stake.to_account_metas(None));
        account_metas.extend(self.srm.to_account_metas(None));
        account_metas.extend(self.msrm.to_account_metas(None));
        account_metas.extend(self.msrm_registrar.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.registry_program.to_account_metas(None));
        account_metas.extend(self.lockup_program.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for DropStakeReward<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.srm, program_id)?;
        anchor_lang::AccountsExit::exit(&self.msrm, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_drop_stake_reward {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_drop_stake_reward_pool::DropStakeRewardPool;
    pub struct DropStakeReward {
        pub officer: anchor_lang::solana_program::pubkey::Pubkey,
        pub stake: anchor_lang::solana_program::pubkey::Pubkey,
        pub srm: __client_accounts_drop_stake_reward_pool::DropStakeRewardPool,
        pub msrm: __client_accounts_drop_stake_reward_pool::DropStakeRewardPool,
        pub msrm_registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub registry_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub lockup_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for DropStakeReward
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_drop_stake_reward_pool::DropStakeRewardPool: borsh::ser::BorshSerialize,
        __client_accounts_drop_stake_reward_pool::DropStakeRewardPool: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.officer, writer)?;
            borsh::BorshSerialize::serialize(&self.stake, writer)?;
            borsh::BorshSerialize::serialize(&self.srm, writer)?;
            borsh::BorshSerialize::serialize(&self.msrm, writer)?;
            borsh::BorshSerialize::serialize(&self.msrm_registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.registry_program, writer)?;
            borsh::BorshSerialize::serialize(&self.lockup_program, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for DropStakeReward {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.officer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.stake, false,
                ),
            );
            account_metas.extend(self.srm.to_account_metas(None));
            account_metas.extend(self.msrm.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.msrm_registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registry_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.lockup_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
pub struct DropStakeRewardPool<'info> {
    registrar: AccountInfo<'info>,
    reward_event_q: AccountInfo<'info>,
    pool_mint: AccountInfo<'info>,
    vendor: AccountInfo<'info>,
    vendor_vault: AccountInfo<'info>,
}
impl<'info> anchor_lang::Accounts<'info> for DropStakeRewardPool<'info> {
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let reward_event_q: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pool_mint: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        Ok(DropStakeRewardPool {
            registrar,
            reward_event_q,
            pool_mint,
            vendor,
            vendor_vault,
        })
    }
}
impl<'info> anchor_lang::ToAccountInfos<'info> for DropStakeRewardPool<'info> {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.reward_event_q.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.vendor.to_account_infos());
        account_infos.extend(self.vendor_vault.to_account_infos());
        account_infos
    }
}
impl<'info> anchor_lang::ToAccountMetas for DropStakeRewardPool<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.reward_event_q.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.vendor.to_account_metas(None));
        account_metas.extend(self.vendor_vault.to_account_metas(None));
        account_metas
    }
}
impl<'info> anchor_lang::AccountsExit<'info> for DropStakeRewardPool<'info> {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
mod __client_accounts_drop_stake_reward_pool {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct DropStakeRewardPool {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub reward_event_q: anchor_lang::solana_program::pubkey::Pubkey,
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor_vault: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for DropStakeRewardPool
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_event_q, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor_vault, writer)?;
            Ok(())
        }
    }
    impl anchor_lang::ToAccountMetas for DropStakeRewardPool {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.reward_event_q,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.pool_mint,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vendor,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vendor_vault,
                    false,
                ),
            );
            account_metas
        }
    }
}
pub struct Officer {
    pub authority: Pubkey,
    pub stake: Pubkey,
    pub treasury: Pubkey,
    pub distribution: Distribution,
    pub swap_program: Pubkey,
    pub dex_program: Pubkey,
    pub registrar: Pubkey,
    pub msrm_registrar: Pubkey,
    __nonce: u8,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Officer {
    #[inline]
    fn default() -> Officer {
        Officer {
            authority: ::core::default::Default::default(),
            stake: ::core::default::Default::default(),
            treasury: ::core::default::Default::default(),
            distribution: ::core::default::Default::default(),
            swap_program: ::core::default::Default::default(),
            dex_program: ::core::default::Default::default(),
            registrar: ::core::default::Default::default(),
            msrm_registrar: ::core::default::Default::default(),
            __nonce: ::core::default::Default::default(),
        }
    }
}
impl borsh::ser::BorshSerialize for Officer
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Distribution: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.authority, writer)?;
        borsh::BorshSerialize::serialize(&self.stake, writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, writer)?;
        borsh::BorshSerialize::serialize(&self.distribution, writer)?;
        borsh::BorshSerialize::serialize(&self.swap_program, writer)?;
        borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
        borsh::BorshSerialize::serialize(&self.registrar, writer)?;
        borsh::BorshSerialize::serialize(&self.msrm_registrar, writer)?;
        borsh::BorshSerialize::serialize(&self.__nonce, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Officer
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Distribution: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            authority: borsh::BorshDeserialize::deserialize(buf)?,
            stake: borsh::BorshDeserialize::deserialize(buf)?,
            treasury: borsh::BorshDeserialize::deserialize(buf)?,
            distribution: borsh::BorshDeserialize::deserialize(buf)?,
            swap_program: borsh::BorshDeserialize::deserialize(buf)?,
            dex_program: borsh::BorshDeserialize::deserialize(buf)?,
            registrar: borsh::BorshDeserialize::deserialize(buf)?,
            msrm_registrar: borsh::BorshDeserialize::deserialize(buf)?,
            __nonce: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Officer {
    #[inline]
    fn clone(&self) -> Officer {
        match *self {
            Officer {
                authority: ref __self_0_0,
                stake: ref __self_0_1,
                treasury: ref __self_0_2,
                distribution: ref __self_0_3,
                swap_program: ref __self_0_4,
                dex_program: ref __self_0_5,
                registrar: ref __self_0_6,
                msrm_registrar: ref __self_0_7,
                __nonce: ref __self_0_8,
            } => Officer {
                authority: ::core::clone::Clone::clone(&(*__self_0_0)),
                stake: ::core::clone::Clone::clone(&(*__self_0_1)),
                treasury: ::core::clone::Clone::clone(&(*__self_0_2)),
                distribution: ::core::clone::Clone::clone(&(*__self_0_3)),
                swap_program: ::core::clone::Clone::clone(&(*__self_0_4)),
                dex_program: ::core::clone::Clone::clone(&(*__self_0_5)),
                registrar: ::core::clone::Clone::clone(&(*__self_0_6)),
                msrm_registrar: ::core::clone::Clone::clone(&(*__self_0_7)),
                __nonce: ::core::clone::Clone::clone(&(*__self_0_8)),
            },
        }
    }
}
impl anchor_lang::AccountSerialize for Officer {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[89, 91, 240, 249, 102, 50, 177, 88])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
impl anchor_lang::AccountDeserialize for Officer {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [89, 91, 240, 249, 102, 50, 177, 88].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[89, 91, 240, 249, 102, 50, 177, 88] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
impl anchor_lang::Discriminator for Officer {
    fn discriminator() -> [u8; 8] {
        [89, 91, 240, 249, 102, 50, 177, 88]
    }
}
impl anchor_lang::Bump for Officer {
    fn seed(&self) -> u8 {
        self.__nonce
    }
}
pub struct Distribution {
    bnb: u8,
    stake: u8,
    treasury: u8,
}
impl borsh::ser::BorshSerialize for Distribution
where
    u8: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.bnb, writer)?;
        borsh::BorshSerialize::serialize(&self.stake, writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Distribution
where
    u8: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            bnb: borsh::BorshDeserialize::deserialize(buf)?,
            stake: borsh::BorshDeserialize::deserialize(buf)?,
            treasury: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Distribution {
    #[inline]
    fn default() -> Distribution {
        Distribution {
            bnb: ::core::default::Default::default(),
            stake: ::core::default::Default::default(),
            treasury: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Distribution {
    #[inline]
    fn clone(&self) -> Distribution {
        match *self {
            Distribution {
                bnb: ref __self_0_0,
                stake: ref __self_0_1,
                treasury: ref __self_0_2,
            } => Distribution {
                bnb: ::core::clone::Clone::clone(&(*__self_0_0)),
                stake: ::core::clone::Clone::clone(&(*__self_0_1)),
                treasury: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
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
pub struct DistributionDidChange {
    distribution: Distribution,
}
impl borsh::ser::BorshSerialize for DistributionDidChange
where
    Distribution: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.distribution, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for DistributionDidChange
where
    Distribution: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            distribution: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl anchor_lang::Event for DistributionDidChange {
    fn data(&self) -> Vec<u8> {
        let mut d = [168, 52, 123, 98, 233, 152, 8, 18].to_vec();
        d.append(&mut self.try_to_vec().unwrap());
        d
    }
}
impl anchor_lang::Discriminator for DistributionDidChange {
    fn discriminator() -> [u8; 8] {
        [168, 52, 123, 98, 233, 152, 8, 18]
    }
}
pub struct OfficerDidCreate {
    pubkey: Pubkey,
}
impl borsh::ser::BorshSerialize for OfficerDidCreate
where
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.pubkey, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for OfficerDidCreate
where
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            pubkey: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl anchor_lang::Event for OfficerDidCreate {
    fn data(&self) -> Vec<u8> {
        let mut d = [33, 61, 118, 147, 62, 25, 254, 131].to_vec();
        d.append(&mut self.try_to_vec().unwrap());
        d
    }
}
impl anchor_lang::Discriminator for OfficerDidCreate {
    fn discriminator() -> [u8; 8] {
        [33, 61, 118, 147, 62, 25, 254, 131]
    }
}
/// Anchor generated Result to be used as the return type for the
/// program.
pub type Result<T> = std::result::Result<T, Error>;
/// Anchor generated error allowing one to easily return a
/// `ProgramError` or a custom, user defined error code by utilizing
/// its `From` implementation.
#[doc(hidden)]
pub enum Error {
    #[error(transparent)]
    ProgramError(#[from] anchor_lang::solana_program::program_error::ProgramError),
    #[error(transparent)]
    ErrorCode(#[from] ErrorCode),
}
#[allow(unused_qualifications)]
impl std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        use thiserror::private::AsDynError;
        #[allow(deprecated)]
        match self {
            Error::ProgramError { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
            Error::ErrorCode { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::fmt::Display for Error {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            Error::ProgramError(_0) => std::fmt::Display::fmt(_0, __formatter),
            Error::ErrorCode(_0) => std::fmt::Display::fmt(_0, __formatter),
        }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<anchor_lang::solana_program::program_error::ProgramError> for Error {
    #[allow(deprecated)]
    fn from(source: anchor_lang::solana_program::program_error::ProgramError) -> Self {
        Error::ProgramError { 0: source }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<ErrorCode> for Error {
    #[allow(deprecated)]
    fn from(source: ErrorCode) -> Self {
        Error::ErrorCode { 0: source }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Error::ProgramError(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "ProgramError");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Error::ErrorCode(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "ErrorCode");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[repr(u32)]
pub enum ErrorCode {
    InvalidDistribution,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ErrorCode::InvalidDistribution,) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "InvalidDistribution");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ErrorCode {
    #[inline]
    fn clone(&self) -> ErrorCode {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for ErrorCode {}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::InvalidDistribution => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Distribution does not add to 100"],
                &match () {
                    () => [],
                },
            )),
        }
    }
}
impl std::error::Error for ErrorCode {}
impl std::convert::From<Error> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: Error) -> anchor_lang::solana_program::program_error::ProgramError {
        match e {
            Error::ProgramError(e) => e,
            Error::ErrorCode(c) => {
                anchor_lang::solana_program::program_error::ProgramError::Custom(
                    c as u32 + anchor_lang::__private::ERROR_CODE_OFFSET,
                )
            }
        }
    }
}
impl std::convert::From<ErrorCode> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: ErrorCode) -> anchor_lang::solana_program::program_error::ProgramError {
        let err: Error = e.into();
        err.into()
    }
}
fn is_distribution_valid(d: &Distribution) -> Result<()> {
    if d.bnb + d.stake + d.treasury != 100 {
        return Err(ErrorCode::InvalidDistribution.into());
    }
    Ok(())
}
fn is_distribution_ready(accounts: &Distribute) -> Result<()> {
    Ok(())
}
fn is_not_trading() -> Result<()> {
    Ok(())
}
fn is_stake_reward_ready(accounts: &DropStakeReward) -> Result<()> {
    Ok(())
}
pub struct ExchangeRate {
    rate: u64,
    from_decimals: u8,
    quote_decimals: u8,
    strict: bool,
}
impl borsh::ser::BorshSerialize for ExchangeRate
where
    u64: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    bool: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.rate, writer)?;
        borsh::BorshSerialize::serialize(&self.from_decimals, writer)?;
        borsh::BorshSerialize::serialize(&self.quote_decimals, writer)?;
        borsh::BorshSerialize::serialize(&self.strict, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for ExchangeRate
where
    u64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    bool: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            rate: borsh::BorshDeserialize::deserialize(buf)?,
            from_decimals: borsh::BorshDeserialize::deserialize(buf)?,
            quote_decimals: borsh::BorshDeserialize::deserialize(buf)?,
            strict: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
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
