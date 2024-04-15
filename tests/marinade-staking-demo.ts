import {
  Program,
  AnchorProvider,
  setProvider,
  workspace,
  BN,
} from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { MarinadeStakingDemo } from "../target/types/marinade_staking_demo";

describe("marinade-staking-demo", () => {
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  setProvider(provider);
  const payer = provider.wallet;
  const connection = provider.connection;

  console.log("Payer address", payer.publicKey.toBase58());

  const marinadeProgram = new PublicKey(
    "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"
  );

  const driftProgram = new PublicKey(
    "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH"
  );

  const program = workspace.MarinadeStakingDemo as Program<MarinadeStakingDemo>;

  console.log("Program ID", program.programId.toBase58());

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .deposit(new BN(1e9))
        .accounts({
          signer: payer.publicKey,
          marinadeProgram,
        })
        .rpc({ commitment: "confirmed" });
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error", error);
      throw error;
    }
  });
});
