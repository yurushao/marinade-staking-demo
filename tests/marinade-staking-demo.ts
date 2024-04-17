import {
  Program,
  AnchorProvider,
  setProvider,
  workspace,
  BN,
} from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { MarinadeStakingDemo } from "../target/types/marinade_staking_demo";
import { Marinade, MarinadeConfig } from "@marinade.finance/marinade-ts-sdk";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("marinade-staking-demo", () => {
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  setProvider(provider);
  const wallet = provider.wallet;
  const connection = provider.connection;

  const config = new MarinadeConfig({
    connection: connection,
    publicKey: wallet.publicKey,
  });
  const marinade = new Marinade(config);
  let marinadeState;

  before(async () => {
    /*
    try {
      const { transaction: liqTx } = await marinade.addLiquidity(
        MarinadeUtils.solToLamports(100)
      );
      await provider.sendAndConfirm(liqTx);
    } catch (err) {
      console.error("Failure on beforeAll addLiquidity transaction", err);
      throw err;
    }
    */
    marinadeState = await marinade.getMarinadeState();

    console.log(
      "marinadaState reservePda",
      await marinadeState.reserveAddress()
    );
    console.log("marinadaState state", marinadeState.marinadeStateAddress);
    console.log("marinadaState msolMint", marinadeState.mSolMintAddress);
    console.log(
      "marinadaState msolMintAuthority",
      await marinadeState.mSolMintAuthority()
    );
    console.log(
      "marinadaState liqPoolMsolLegAuthority",
      await marinadeState.mSolLegAuthority()
    );
    console.log("marinadaState liqPoolMsolLeg", marinadeState.mSolLeg);
    console.log("marinadaState liqPoolSolLegPda", await marinadeState.solLeg());
  });

  const marinadeProgram = new PublicKey(
    "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"
  );

  const program = workspace.MarinadeStakingDemo as Program<MarinadeStakingDemo>;

  it("Is initialized!", async () => {
    const { associatedMSolTokenAccountAddress, transaction } =
      await marinade.deposit(new BN(1));
    const signature = await provider.sendAndConfirm(transaction);
    console.log(
      "associatedMSolTokenAccountAddress",
      associatedMSolTokenAccountAddress
    );

    // create treasury account
    // create treasury ata for msol
    // getAssociatedTokenAddressSync(marinadeState.mSolMintAddress);

    // Add your test here.
    try {
      const tx = await program.methods
        .deposit(new BN(1e10))
        .accounts({
          signer: wallet.publicKey,
          reservePda: await marinadeState.reserveAddress(),
          marinadeState: marinadeState.marinadeStateAddress,
          msolMint: marinadeState.mSolMintAddress,
          msolMintAuthority: await marinadeState.mSolMintAuthority(),
          liqPoolMsolLeg: marinadeState.mSolLeg,
          liqPoolMsolLegAuthority: await marinadeState.mSolLegAuthority(),
          liqPoolSolLegPda: await marinadeState.solLeg(),
          mintTo: associatedMSolTokenAccountAddress,
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
