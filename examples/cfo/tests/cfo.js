const assert = require("assert");
const { Token } = require("@solana/spl-token");
const utils = require("./utils");
const anchor = require("@project-serum/anchor");
const serumCmn = require("@project-serum/common");
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;

const DEX_PID = new PublicKey("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
const SWAP_PID = new PublicKey("22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD");
const TOKEN_PID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

describe("cfo", () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Cfo;
  let officer;
  let TOKEN_CLIENT;
  let officerAccount;
  const sweepAuthority = program.provider.wallet.publicKey;

  it("Creates a CFO!", async () => {
    let distribution = {
      bnb: 80,
      stake: 20,
      treasury: 0,
    };
    officer = await program.account.officer.associatedAddress(DEX_PID);
    await program.rpc.createOfficer(distribution, {
      accounts: {
        officer,
        authority: program.provider.wallet.publicKey,
        dexProgram: DEX_PID,
        swapProgram: SWAP_PID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
    });

    officerAccount = await program.account.officer.associated(DEX_PID);
    assert.ok(
      officerAccount.authority.equals(program.provider.wallet.publicKey)
    );
    assert.ok(
      JSON.stringify(officerAccount.distribution) ===
        JSON.stringify(distribution)
    );
  });

  it("Creates a token account for the CFO!", async () => {
    const tokenClient = await Token.createMint(
      program.provider.connection,
      program.provider.wallet.payer,
      program.provider.wallet.publicKey,
      null,
      6,
      TOKEN_PID
    );
    const mint = tokenClient.publicKey;
    const token = await anchor.utils.publicKey.associated(
      program.programId,
      officer,
      mint
    );
    await program.rpc.createOfficerToken({
      accounts: {
        officer,
        token,
        mint,
        payer: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PID,
        rent: SYSVAR_RENT_PUBKEY,
      },
    });

    const tokenAccount = await tokenClient.getAccountInfo(token);
    assert.ok(tokenAccount.state === 1);
    assert.ok(tokenAccount.isInitialized);
  });

  // Accounts used to setup the orderbook.
  let ORDERBOOK_ENV,
    // Accounts used for A -> USDC swap transactions.
    SWAP_A_USDC_ACCOUNTS,
    // Accounts used for  USDC -> A swap transactions.
    SWAP_USDC_A_ACCOUNTS,
    // Serum DEX vault PDA for market A/USDC.
    marketAVaultSigner,
    // Serum DEX vault PDA for market B/USDC.
    marketBVaultSigner;

  it("BOILERPLATE: Sets up a market with funded fees", async () => {
    ORDERBOOK_ENV = await utils.initMarket({
      provider: program.provider,
    });
    TOKEN_CLIENT = new Token(
      program.provider.connection,
      ORDERBOOK_ENV.usdc,
      TOKEN_PID,
      program.provider.wallet.payer
    );

    await TOKEN_CLIENT.transfer(
      ORDERBOOK_ENV.godUsdc,
      ORDERBOOK_ENV.marketA._decoded.quoteVault,
      program.provider.wallet.payer,
      [],
      10000000000000
    );

    const tokenAccount = await TOKEN_CLIENT.getAccountInfo(
      ORDERBOOK_ENV.marketA._decoded.quoteVault
    );
    assert.ok(tokenAccount.amount.toString() === "10000902263700");
  });

  it("BOILERPLATE: Crates a token account for the officer associated iwth the market", async () => {
    const token = await anchor.utils.publicKey.associated(
      program.programId,
      officer,
      ORDERBOOK_ENV.usdc
    );
    await program.rpc.createOfficerToken({
      accounts: {
        officer,
        token,
        mint: ORDERBOOK_ENV.usdc,
        payer: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PID,
        rent: SYSVAR_RENT_PUBKEY,
      },
    });
  });

  it("Sweeps fees", async () => {
    const sweepVault = await anchor.utils.publicKey.associated(
      program.programId,
      officer,
      ORDERBOOK_ENV.usdc
    );
    const beforeTokenAccount = await serumCmn.getTokenAccount(
      program.provider,
      sweepVault
    );
    await program.rpc.sweepFees({
      accounts: {
        officer,
        sweepVault,
        mint: ORDERBOOK_ENV.usdc,
        dex: {
          market: ORDERBOOK_ENV.marketA._decoded.ownAddress,
          pcVault: ORDERBOOK_ENV.marketA._decoded.quoteVault,
          sweepAuthority,
          vaultSigner: ORDERBOOK_ENV.vaultSigner,
          dexProgram: DEX_PID,
          tokenProgram: TOKEN_PID,
        },
      },
    });
    const afterTokenAccount = await serumCmn.getTokenAccount(
      program.provider,
      sweepVault
    );
    assert.ok(
      afterTokenAccount.amount.sub(beforeTokenAccount.amount).toString() ===
        "10000000000"
    );
  });
});
