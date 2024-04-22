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
import { getOrCreateAssociatedTokenAccount } from "@marinade.finance/marinade-ts-sdk/dist/src/util";

describe("marinade-staking-demo", () => {
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  setProvider(provider);
  const wallet = provider.wallet;
  const connection = provider.connection;
  const program = workspace.MarinadeStakingDemo as Program<MarinadeStakingDemo>;

  // marinade setup
  const marinadeProgram = new PublicKey(
    "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"
  );

  const config = new MarinadeConfig({
    connection: connection,
    publicKey: wallet.publicKey,
  });
  const marinade = new Marinade(config);
  let marinadeState; // will be initialized in beforeAll

  let treasurymSolAta; // will be initialized in beforeAll
  let treasuryPda; // will be initialized in beforeAll
  let treasuryBump; // will be initialized in beforeAll

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

    console.log("--------");
    console.log(
      "marinadaState reservePda",
      (await marinadeState.reserveAddress()).toBase58()
    );
    console.log(
      "marinadaState state",
      marinadeState.marinadeStateAddress.toBase58()
    );
    console.log(
      "marinadaState msolMint",
      marinadeState.mSolMintAddress.toBase58()
    );
    console.log(
      "marinadaState msolMintAuthority",
      (await marinadeState.mSolMintAuthority()).toBase58()
    );
    console.log(
      "marinadaState liqPoolMsolLegAuthority",
      (await marinadeState.mSolLegAuthority()).toBase58()
    );
    console.log(
      "marinadaState liqPoolMsolLeg",
      marinadeState.mSolLeg.toBase58()
    );
    console.log(
      "marinadaState liqPoolSolLegPda",
      (await marinadeState.solLeg()).toBase58()
    );

    // treasury setup
    const [pda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    );
    console.log("treasuryPda", pda.toBase58(), "bump", bump);
    treasuryPda = pda;
    treasuryBump = bump;
    treasurymSolAta = (
      await getOrCreateAssociatedTokenAccount(
        provider,
        marinadeState.mSolMintAddress,
        treasuryPda
      )
    ).associatedTokenAccountAddress;
    console.log("treasurymSolAta", treasurymSolAta.toBase58());
  });

  it("Init treasury", async () => {
    try {
      const tx = await program.methods
        .init(treasuryBump)
        .accounts({
          signer: wallet.publicKey,
          treasuryPda,
        })
        .rpc({ commitment: "confirmed" });
    } catch (error) {
      console.log("Error", error);
      throw error;
    }
    await connection.requestAirdrop(treasuryPda, 100_000_000_000);
    // delay 1s for the airdrop to be confirmed
    await new Promise((resolve) => setTimeout(resolve, 1000));
  });

  it("Stake", async () => {
    try {
      const tx = await program.methods
        .deposit(new BN(1e10), treasuryBump)
        .accounts({
          signer: wallet.publicKey,
          reservePda: await marinadeState.reserveAddress(),
          marinadeState: marinadeState.marinadeStateAddress,
          msolMint: marinadeState.mSolMintAddress,
          msolMintAuthority: await marinadeState.mSolMintAuthority(),
          liqPoolMsolLeg: marinadeState.mSolLeg,
          liqPoolMsolLegAuthority: await marinadeState.mSolLegAuthority(),
          liqPoolSolLegPda: await marinadeState.solLeg(),
          mintTo: treasurymSolAta,
          treasuryPda,
          marinadeProgram,
        })
        .rpc({ commitment: "confirmed" });
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error", error);
      throw error;
    }
  });

  it("Withdraw", async () => {
    try {
      const tx = await program.methods
        .unstake(new BN(1e9), treasuryBump)
        .accounts({
          marinadeState: marinadeState.marinadeStateAddress,
          msolMint: marinadeState.mSolMintAddress,
          liqPoolSolLegPda: await marinadeState.solLeg(),
          liqPoolMsolLeg: marinadeState.mSolLeg,
          getMsolFrom: treasurymSolAta,
          getMsolFromAuthority: treasuryPda,
          transferSolTo: treasuryPda,
          treasuryMsolAccount: new PublicKey(
            "B1aLzaNMeFVAyQ6f3XbbUyKcH2YPHu2fqiEagmiF23VR"
          ),
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
