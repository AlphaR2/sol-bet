import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solbet } from "../target/types/solbet";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";
import { assert } from "chai";
import fs from 'fs';
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
// enum Outcome {
//   Win = 0,
//   Lose = 1,
// };

const keypairJs = fs.readFileSync('id.json', 'utf8');
const keypairData = JSON.parse(keypairJs);

const admin = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(keypairData)
);

describe("solbet", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  
  const connection = provider.connection;

  const program = anchor.workspace.Solbet as Program<Solbet>;
  // bettors
 
  const bettor1 = anchor.web3.Keypair.generate();
  const bettor2  = anchor.web3.Keypair.generate();

  const matchId = new anchor.BN(1);
  const betResult =  { lose: {} };

  const matchAccount = anchor.web3.Keypair.generate();
      // Place a bet
      const amount = 0.2; 
      const amount2 = 0.3;// Example amount
      const outcome1  =  { win: {} };
      const outcome2  =  { lose: {} };
      const odds = 1; // Example odds
  
  const betAccount : any = anchor.web3.Keypair.generate();
  const betAccount2 : any = anchor.web3.Keypair.generate();


  // pdas handling 
  const escrow  = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('escrow'), Buffer.from(new BN(0).toArray('le', 8))],
    program.programId
  )[0];

  const betsData = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('bets_data'), Buffer.from(new BN(0).toArray('le', 8))],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('vault'), Buffer.from(new BN(0).toArray('le', 8))],
    program.programId
  )[0];


    const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

    // Function to airdrop SOL to a keypair
    const airdropSol = async (connection: Connection, publicKey: PublicKey, amountSol: number) => {
      const airdropSignature = await connection.requestAirdrop(publicKey, amountSol * LAMPORTS_PER_SOL);
      await confirm(airdropSignature);
    };
  
    it("Airdrop SOL to bettor1", async () => {
      const connection = provider.connection;
      
      // Airdrop 2 SOL to bettor1 and bettor2
      await airdropSol(connection, bettor1.publicKey, 1);
      await airdropSol(connection, bettor2.publicKey, 1);
  
      // Check the balance1
      const balance1 = await connection.getBalance(bettor1.publicKey);
      console.log("Bettor1 Balance:", balance1 / LAMPORTS_PER_SOL, "SOL");

        // Check the balance1
      const balance2 = await connection.getBalance(bettor2.publicKey);
      console.log("Bettor1 Balance:", balance2 / LAMPORTS_PER_SOL, "SOL");
      

  
      // Ensure the balance is correct
      if (balance1 < 1 * LAMPORTS_PER_SOL) {
        throw new Error("Airdrop failed to bettor 1. Insufficient balance.");

     
      }

      if (balance2 < 1 * LAMPORTS_PER_SOL) {
        throw new Error("Airdrop failed2. Insufficient balance.");

     
      }
    });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize()
    .accountsPartial({
      admin: admin.publicKey,
      escrow,
      betsData,
      vault,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

     console.log("\nYour transaction signature", tx);
     const escrowAccount = await program.account.escrow.fetch(escrow);
     assert.ok(escrowAccount.totalAmount.toNumber() === 0);

     const betData = await program.account.betsData.fetch(betsData);
    console.log(betData.matchId);
    console.log(betData.status);
    console.log(betData.totalAmountBet);
   

    console.log("Escrow total amount:", escrowAccount.totalAmount);
    // console.log("Escrow account details:", escrowAccount);
     console.log("Escrow Info:", (await provider.connection.getAccountInfo(escrow)));
     console.log("BetsData Info:", (await provider.connection.getAccountInfo(betsData)));
     console.log("Vault Info:", (await provider.connection.getAccountInfo(vault)));
     


  });


  it('Creates a match', async () => {

   const tx =  await program.methods.createMatch(matchId).accountsPartial({
    admin: admin.publicKey,
    matchAccount: matchAccount.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
   })
   .signers([admin, matchAccount])
   .rpc(); 

   console.log("\nYour transaction signature", tx);
    });



  it("Places a bet successfully", async () => {
    // Create a bet account for bettor1
   



    const txPlaceBet = await program.methods
    .placebet(new anchor.BN(amount * LAMPORTS_PER_SOL), outcome1, new anchor.BN(odds))
      .accountsPartial({
        bettor: bettor1.publicKey,
        bet: betAccount.publicKey,
        betsData,
        escrow,
        matchAccount: matchAccount.publicKey,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([bettor1, betAccount])
      .rpc();

      const trxplacebet2 = await program.methods
      .placebet(new anchor.BN(amount2 * LAMPORTS_PER_SOL), outcome2, new anchor.BN(odds))
      .accountsPartial({
        bettor: bettor2.publicKey,
        bet: betAccount2.publicKey,
        betsData,
        escrow,
        matchAccount: matchAccount.publicKey,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([bettor2, betAccount2])
      .rpc();

    console.log("Place Bettor 1 Transaction Signature:", txPlaceBet);
    console.log("Place Bettor 2 Transaction Signature:", trxplacebet2);

    // Fetch and log account info
    const betAccountInfo = await provider.connection.getAccountInfo(betAccount.publicKey);
    const betsDataInfo = await provider.connection.getAccountInfo(betsData);
    const escrowInfo = await provider.connection.getAccountInfo(escrow);
    const vaultInfo = await provider.connection.getAccountInfo(vault);

    console.log("Bet Account Info:", betAccountInfo);
    console.log("BetsData Info:", betsDataInfo);
    console.log("Escrow Info:", escrowInfo);
    console.log("Vault Info:", vaultInfo);

    const escrowInfoData = await program.account.escrow.fetch(escrow);

    // vault data 
    // const vaultInfoData = await program.account.
    const betData = await program.account.bet.fetch(betAccount.publicKey);
    const betData2 = await program.account.bet.fetch(betAccount2.publicKey);
    const betTotalData = await program.account.betsData.fetch(betsData);
console.log("Escrow total amount:", escrowInfoData.totalAmount);

  console.log('Bets Data:');
  console.log('Total Bets:', betTotalData.totalBets.toString());
  console.log('Total Amount Bet:', betTotalData.totalAmountBet.toString());
  console.log('Status:', betTotalData.status);
  console.log('Match ID:', betTotalData.matchId.toString());
  console.log('Bet Accounts:', betTotalData.betAccounts.map((pk: anchor.web3.PublicKey) => pk.toBase58()));

   // Log the bet data
   console.log('Bet Data for bettor One:');
   console.log('Bettor_One :', betData.bettor.toBase58());
   console.log('Amount_One:', betData.amount.toString());
   console.log('Outcome_One:', betData.outcome);
   console.log('Odds:', betData.odds.toString());
   console.log('Match ID:', betData.matchId.toString());



    // Log the bet data
    console.log('Bet Data for bettor Two:');
    console.log('Bettor:', betData2.bettor.toBase58());
    console.log('Amount:', betData2.amount.toString());
    console.log('Outcome:', betData2.outcome);
    console.log('Odds:', betData2.odds.toString());
    console.log('Match ID:', betData2.matchId.toString());
  


    // Perform assertions (optional, based on what you expect)
    if (betAccountInfo === null) throw new Error("Bet account was not created.");
    if (betsDataInfo === null) throw new Error("BetsData account was not found.");
    if (escrowInfo === null) throw new Error("Escrow account was not found.");
    if (vaultInfo === null) throw new Error("Vault account was not found.");
  });


  it("Update Match Data Successfully", async () => {

    const trxUpdate  = await  program.methods.update( matchId, betResult ).accountsPartial({
      admin: admin.publicKey,
      matchAccount: matchAccount.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([admin])
   .rpc(); 

   console.log("\nYour transaction signature", trxUpdate);

    
  });

});



