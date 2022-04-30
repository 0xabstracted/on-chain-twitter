import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OnChainTwitter } from "../target/types/on_chain_twitter";

describe("on-chain-twitter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OnChainTwitter as Program<OnChainTwitter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
