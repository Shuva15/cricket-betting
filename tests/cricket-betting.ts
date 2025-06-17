import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CricketBetting } from "../target/types/cricket_betting";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("cricket-betting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.cricketBetting as Program<CricketBetting>;

  const gameId = new anchor.BN(1);
  let gameAccountPDA: PublicKey

  it("create a game", async () => {

    [gameAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("game"), gameId.toArrayLike(Buffer, "le", 8)],
      program.programId
    )

    // Create game
    const tx = await program.methods.createGame(gameId).accounts({
      gameAccount: gameAccountPDA,
      signer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    }).rpc()

    console.log("Game created in tx:", tx);

    const account = await program.account.gameAccount.fetch(gameAccountPDA);
    console.log("Game account:", account);

    //console.log("Your transaction signature", tx);
  });

  it("Place a bet", async () => {

    const [bettorAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("bettor-account"), gameAccountPDA.toBuffer(), provider.wallet.publicKey.toBuffer()],
      program.programId
    )

    const tx = await program.methods.placeBet(150, 4).accounts({
      bettorAccount: bettorAccountPDA,
      gameAccount: gameAccountPDA,
      signer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    }).rpc();

    console.log("Bet placed in tx:", tx);

    const bet = await program.account.bettorAccount.fetch(bettorAccountPDA);
    console.log("Bettor account:", bet);
  });

  it("update betting stoped", async () => {
    const tx = await program.methods.updateBettingStop().accounts({
      gameAccount: gameAccountPDA,
      signer: provider.wallet.publicKey,
    }).rpc();

    console.log("Game account updated:", tx);

    const game = await program.account.gameAccount.fetch(gameAccountPDA);
    console.log("game account:", game)
  })

  it("update results", async () => {
    const tx = await program.methods.updateGameResult(200, 2).accounts({
      gameAccount: gameAccountPDA,
      signer: provider.wallet.publicKey,
    }).rpc();

    console.log("Game account result updated:", tx);

    const game = await program.account.gameAccount.fetch(gameAccountPDA);
    console.log("game account:", game)
  })
});
