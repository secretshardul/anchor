#!/usr/bin/env node

// Script to infinitely post orders that are immediately filled.

const process = require("process");
const fs = require("fs");
const anchor = require("@project-serum/anchor");
const { Market, OpenOrders } = require("@project-serum/serum");
const Account = anchor.web3.Account;
const Program = anchor.Program;
const provider = anchor.Provider.local();
const secret = JSON.parse(fs.readFileSync("./scripts/market-maker.json"));
const MARKET_MAKER = new Account(secret);
const PublicKey = anchor.web3.PublicKey;

const DEX_PID = new PublicKey("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");

async function main() {
  const market = new PublicKey(process.argv[2]);
  let marketClient = await Market.load(
    provider.connection,
    market,
    { commitment: "recent" },
    DEX_PID
  );
  const baseTokenUser1 = (
    await marketClient.getTokenAccountsByOwnerForMint(
      provider.connection,
      MARKET_MAKER.publicKey,
      marketClient.baseMintAddress
    )
  )[0].pubkey;
  const quoteTokenUser1 = (
    await marketClient.getTokenAccountsByOwnerForMint(
      provider.connection,
      MARKET_MAKER.publicKey,
      marketClient.quoteMintAddress
    )
  )[0].pubkey;

  const baseTokenUser2 = (
    await marketClient.getTokenAccountsByOwnerForMint(
      provider.connection,
      provider.wallet.publicKey,
      marketClient.baseMintAddress
    )
  )[0].pubkey;
  const quoteTokenUser2 = (
    await marketClient.getTokenAccountsByOwnerForMint(
      provider.connection,
      provider.wallet.publicKey,
      marketClient.quoteMintAddress
    )
  )[0].pubkey;

  const makerOpenOrdersUser1 = (
    await OpenOrders.findForMarketAndOwner(
      provider.connection,
      market,
      MARKET_MAKER.publicKey,
      DEX_PID
    )
  )[0];
  makerOpenOrdersUser2 = (
    await OpenOrders.findForMarketAndOwner(
      provider.connection,
      market,
      provider.wallet.publicKey,
      DEX_PID
    )
  )[0];

  const price = 6.041;
  const size = 700000.8;

  let maker = MARKET_MAKER;
  let taker = provider.wallet.payer;
  let baseToken = baseTokenUser1;
  let quoteToken = quoteTokenUser2;
  let makerOpenOrders = makerOpenOrdersUser1;

  let k = 1;

  while (true) {
    const clientId = new anchor.BN(k);
    if (k % 5 === 0) {
      if (maker.publicKey.equals(MARKET_MAKER.publicKey)) {
        maker = provider.wallet.payer;
        makerOpenOrders = makerOpenOrdersUser2;
        taker = MARKET_MAKER;
        baseToken = baseTokenUser2;
        quoteToken = quoteTokenUser1;
      } else {
        maker = MARKET_MAKER;
        makerOpenOrders = makerOpenOrdersUser1;
        taker = provider.wallet.payer;
        baseToken = baseTokenUser1;
        quoteToken = quoteTokenUser2;
      }
    }

    // Post ask.
    const { transaction: tx_ask, signers: sigs_ask } =
      await marketClient.makePlaceOrderTransaction(provider.connection, {
        owner: maker,
        payer: baseToken,
        side: "sell",
        price,
        size,
        orderType: "postOnly",
        clientId,
        openOrdersAddressKey: undefined,
        openOrdersAccount: undefined,
        feeDiscountPubkey: null,
        selfTradeBehavior: "abortTransaction",
      });
    let txSig = await provider.send(tx_ask, sigs_ask.concat(maker));
    console.log("Ask", txSig);

    // Take.
    const { transaction: tx_bid, signers: sigs_bid } =
      await marketClient.makePlaceOrderTransaction(provider.connection, {
        owner: taker,
        payer: quoteToken,
        side: "buy",
        price,
        size,
        orderType: "ioc",
        clientId: undefined,
        openOrdersAddressKey: undefined,
        openOrdersAccount: undefined,
        feeDiscountPubkey: null,
        selfTradeBehavior: "abortTransaction",
      });
    txSig = await provider.send(tx_bid, sigs_bid.concat(taker));
    console.log("Bid", txSig);

    await sleep(1000);

    // Cancel anything remaining.
    try {
      txSig = await marketClient.cancelOrderByClientId(
        provider.connection,
        maker,
        makerOpenOrders.address,
        clientId
      );
      console.log("Cancelled the rest", txSig);
      await sleep(1000);
    } catch (e) {
      console.log("Unable to cancel order", e);
    }
    k += 1;

    // If the open orders account wasn't previously initialized, it is now.
    if (makerOpenOrdersUser2 === undefined) {
      makerOpenOrdersUser2 = (
        await OpenOrders.findForMarketAndOwner(
          provider.connection,
          market,
          provider.wallet.publicKey,
          DEX_PID
        )
      )[0];
    }
  }
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

main();
