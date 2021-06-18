const assert = require("assert");
const Token = require("@solana/spl-token").Token;
const anchor = require("@project-serum/anchor");
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;

const DEX_PID = new PublicKey("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
const SWAP_PID = new PublicKey("22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD");
const TOKEN_PID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

describe("cfo", () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Cfo;
  let officer;

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

    const officerAccount = await program.account.officer.associated(DEX_PID);
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
      anchor.utils.bytes.utf8.encode("my-seed"),
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
});
