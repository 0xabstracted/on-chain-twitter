import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {PublicKey} from "@solana/web3.js";
import { OnChainTwitter } from "../target/types/on_chain_twitter";
import {expect} from "chai";

describe("on-chain-twitter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OnChainTwitter as Program<OnChainTwitter>;
  const nextTweetAddress = anchor.web3.Keypair.generate();
  const nextTweetAddress1 = anchor.web3.Keypair.generate();
      
  it("creates Twitter account for user", async () => {
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const tx = await program.methods
                    .createTwitterAccount("abstracted")
                    .accounts({
        author: provider.wallet.publicKey,
        twitterUserAccount: twitterUserPDA,
    }).rpc();
    expect((await program.account.twitterUser.fetch(twitterUserPDA)).username).to.equal("abstracted");
    const twitterUsers = await program.account.twitterUser.all();
    console.log("createTwitterAccount");
    console.log(twitterUsers);
    const tweets = await program.account.tweet.all();
    console.log(tweets);
   
    console.log("1. Your transaction signature", tx);
  });
  /*
  it("tries creates Twitter account for user again with same username ", async () => {
    // Add your test here.
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const tx = await program.methods
                    .createTwitterAccount("abstracted")
                    .accounts({
        author: provider.wallet.publicKey,
        twitterUserAccount: twitterUserPDA,
    }).rpc();
    expect((await program.account.twitterUser.fetch(twitterUserPDA)).username).to.equal("abstracted");
  
    console.log("2. Your transaction signature", tx);
  });
  it("tries creates Twitter account for user again with different username ", async () => {
    // Add your test here.
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const tx = await program.methods
                    .createTwitterAccount("Bhags")
                    .accounts({
        author: provider.wallet.publicKey,
        twitterUserAccount: twitterUserPDA,
    }).rpc();
    expect((await program.account.twitterUser.fetch(twitterUserPDA)).username).to.equal("Bhags");
  
    console.log("3. Your transaction signature", tx);
  });
  */
  it("updates twitter account of user with username", async () => {
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const tx = await program.methods
                    .changeUserName("lazytrader")
                    .accounts({
      author: provider.wallet.publicKey,
      twitterUserAccount: twitterUserPDA,                  
    }).rpc();
    console.log("changeUserName");
    const twitterUsers = await program.account.twitterUser.all();
    console.log(twitterUsers);
    const tweets = await program.account.tweet.all();
    console.log(tweets);
    expect((await program.account.twitterUser.fetch(twitterUserPDA)).username).to.equal("lazytrader");
    console.log("4. Your transaction signature", tx);
  });
  /*
  it("deletes twitter account", async () => {
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    const tx = await program.methods
                    .deleteTwitterAccount()
                    .accounts({
      author: provider.wallet.publicKey,
      twitterUserAccount: twitterUserPDA,                  
    }).rpc();
    expect(await program.account.twitterUser.fetch(twitterUserPDA));
    console.log("5. Your transaction signature", tx);
  });
  */
  it("send a tweet from the user", async () => {
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    /*
      let tweetCountOfUser = (await program.account.twitterUser.fetch(twitterUserPDA)).tweetCount;
      console.log("tweetCountOfUser",tweetCountOfUser);
      console.log("provider.wallet.publicKey.toBuffer()",provider.wallet.publicKey.toBuffer());
      const buffer = new ArrayBuffer(1);
      //const buffer = new ArrayBuffer(4);
      //let tweetCountSeedu32 = new Uint32Array(buffer);
      //tweetCountSeedu32[0] = 123456;
      //console.log(tweetCountSeedu32.byteLength);
      //console.log(tweetCountSeedu32.length);
      console.log(Uint8Array.BYTES_PER_ELEMENT); 
      console.log(Uint16Array.BYTES_PER_ELEMENT); 
      console.log(Uint32Array.BYTES_PER_ELEMENT); 
      //let tweetCountSeed = new Uint8Array(tweetCountSeedu32);
      let tweetCountSeed = new Uint8Array(buffer);
      //let tweetCountSeed = tweetCountOfUser;
      tweetCountSeed[0] = tweetCountOfUser;
      //console.log("tweetCountSeed32",tweetCountSeedu32);
      //console.log("tweetCountSeed32[0]",tweetCountSeedu32[0]);
      
      console.log("tweetCountSeed",tweetCountSeed);
      console.log("tweetCountSeed[0]",tweetCountSeed[0]);
      */
      const una_tx = await program.methods
                          .updateNextAddress(nextTweetAddress.publicKey)
                          .accounts({
      author:provider.wallet.publicKey,
      twitterUserAccount: twitterUserPDA,  
                          }).rpc();
      const [tweetAccountPDA, tweetAccountBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("tweet-account"),
        provider.wallet.publicKey.toBuffer(),
        nextTweetAddress.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("tweetAccountPDA",tweetAccountPDA);
    const tx = await program.methods
                    .sendTweet("First Topic","First Tweet")
                    .accounts({
      twitterUserAccount: twitterUserPDA,
      tweetAccount: tweetAccountPDA,
      author: provider.wallet.publicKey,
    }).rpc();
    console.log("6. Your transaction signature", tx);
  
    let tweetCountOfUser1 = (await program.account.twitterUser.fetch(twitterUserPDA)).tweetCount;
      console.log(tweetCountOfUser1);
      /*
      const buffer1 = new ArrayBuffer(1);
      //const buffer1 = new ArrayBuffer(4);
      //let tweetCountSeedu32_1 = new Uint32Array(buffer1)
      //tweetCountSeedu32_1[0] = tweetCountOfUser1;
      //let tweetCountSeed1 = new Uint8Array(tweetCountSeedu32_1);
      let tweetCountSeed1 = new Uint8Array(buffer1);
      tweetCountSeed1[0] = tweetCountOfUser1;
      console.log(tweetCountSeed1);
      console.log(tweetCountSeed1[0]);
      */
      const una_tx1 = await program.methods
                          .updateNextAddress(nextTweetAddress1.publicKey)
                          .accounts({
      author:provider.wallet.publicKey,
      twitterUserAccount: twitterUserPDA,  
                          }).rpc();
      const [tweetAccountPDA1, tweetAccountBump1] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("tweet-account"),
        provider.wallet.publicKey.toBuffer(),
        nextTweetAddress1.publicKey.toBuffer(),
      ],
      program.programId
    );
      
    const tx1 = await program.methods
                    .sendTweet("First Topic","Second Tweet")
                    .accounts({
      twitterUserAccount: twitterUserPDA,
      tweetAccount: tweetAccountPDA1,
      author: provider.wallet.publicKey,
    }).rpc();
    console.log("sendTweet");
    const twitterUsers = await program.account.twitterUser.all();
    console.log(twitterUsers);
    const tweets = await program.account.tweet.all();
    console.log(tweets);
    expect((await program.account.tweet.fetch(tweetAccountPDA)).topic).to.equal("First Topic");
    expect((await program.account.tweet.fetch(tweetAccountPDA)).content).to.equal("First Tweet");
    expect((await program.account.tweet.fetch(tweetAccountPDA1)).topic).to.equal("First Topic");
    expect((await program.account.tweet.fetch(tweetAccountPDA1)).content).to.equal("Second Tweet");
  
    console.log("6. Your transaction signature", tx1);
  });
  it("update a tweet from the user", async () => {
    const [twitterUserPDA, twitterUserBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("twitter-user"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
      let twitter_accounts = await program.account.twitterUser.all();
      console.log("twitter accpunts",twitter_accounts);
      let tweet_accounts = await program.account.tweet.all();
      console.log("tweet accounts",tweet_accounts);
      console.log("provider.wallet.publicKey.toBuffer()",provider.wallet.publicKey.toBuffer());
      /*
      let tweetCountOfUser = 0;
      const buffer = new ArrayBuffer(1);
      //const buffer = new ArrayBuffer(4);
      //let tweetCountSeedu32 = new Uint32Array(buffer);
      //tweetCountSeedu32[0] = 123456;
      //console.log(tweetCountSeedu32.byteLength);
      //console.log(tweetCountSeedu32.length);
      //let tweetCountSeed = new Uint8Array(tweetCountSeedu32);
      let tweetCountSeed = new Uint8Array(buffer);
      //let tweetCountSeed = tweetCountOfUser;
      tweetCountSeed[0] = tweetCountOfUser;
      //console.log("tweetCountSeed32",tweetCountSeedu32);
      //console.log("tweetCountSeed32[0]",tweetCountSeedu32[0]);
      
      console.log("tweetCountSeed",tweetCountSeed);
      console.log("tweetCountSeed[0]",tweetCountSeed[0]);
      */
      const twitterUser = (await program.account.twitterUser.fetch(twitterUserPDA)).nextAddress;
      const una_tx1 = await program.methods
                          .updateNextAddress(nextTweetAddress.publicKey)
                          .accounts({
      author:provider.wallet.publicKey,
      twitterUserAccount: twitterUserPDA,  
                          }).rpc();
      
      const [tweetAccountPDA, tweetAccountBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("tweet-account"),
        provider.wallet.publicKey.toBuffer(),
        nextTweetAddress.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("tweetAccountPDA",tweetAccountPDA);
    const tx = await program.methods
                    .updateTweet("First Updated Topic","First Tweet")
                    .accounts({
      twitterUserAccount: twitterUserPDA,
      tweetAccount: tweetAccountPDA,
      author: provider.wallet.publicKey,
    }).rpc();
    console.log("7. Your transaction signature", tx);
   /*
      let tweetCountOfUser1 = 1;
      console.log(tweetCountOfUser1);
      const buffer1 = new ArrayBuffer(1);
      //const buffer1 = new ArrayBuffer(4);
      //let tweetCountSeedu32_1 = new Uint32Array(buffer1)
      //tweetCountSeedu32_1[0] = tweetCountOfUser1;
      //let tweetCountSeed1 = new Uint8Array(tweetCountSeedu32_1);
      let tweetCountSeed1 = new Uint8Array(buffer1);
      tweetCountSeed1[0] = tweetCountOfUser1;
      console.log(tweetCountSeed1);
      console.log(tweetCountSeed1[0]);
      */
      const una_tx2 = await program.methods
                      .updateNextAddress(nextTweetAddress1.publicKey)
                      .accounts({
        author:provider.wallet.publicKey,
        twitterUserAccount: twitterUserPDA,  
      }).rpc();

      const [tweetAccountPDA1, tweetAccountBump1] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("tweet-account"),
        provider.wallet.publicKey.toBuffer(),
        nextTweetAddress1.publicKey.toBuffer(),
      ],
      program.programId
    );
    const tx1 = await program.methods
                    .updateTweet("First Topic","Second Updated Tweet")
                    .accounts({
      twitterUserAccount: twitterUserPDA,
      tweetAccount: tweetAccountPDA1,
      author: provider.wallet.publicKey,
    }).rpc();
    console.log("updateTweet");
    const twitterUsers = await program.account.twitterUser.all();
    console.log(twitterUsers);
    const tweets = await program.account.tweet.all();
    console.log(tweets);
    expect((await program.account.tweet.fetch(tweetAccountPDA)).topic).to.equal("First Updated Topic");
    expect((await program.account.tweet.fetch(tweetAccountPDA)).content).to.equal("First Tweet");
    expect((await program.account.tweet.fetch(tweetAccountPDA1)).topic).to.equal("First Topic");
    expect((await program.account.tweet.fetch(tweetAccountPDA1)).content).to.equal("Second Updated Tweet");
  
    console.log("7. Your transaction signature", tx1);
  });
  
});
