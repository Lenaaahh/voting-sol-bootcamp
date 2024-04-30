import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import { VotingProgram } from '../target/types/voting_program'
import {
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js'
import {Keypair} from '@solana/web3.js'


describe("voting-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider()

  const program = anchor.workspace.VotingProgram as Program<VotingProgram>;


  const connection = provider.connection


  const [admin, electionPDA, user] = Array.from({length: 3}, () => Keypair.generate())
  

  const choicesPDA = PublicKey.findProgramAddressSync([
    Buffer.from("choices"),
    electionPDA.publicKey.toBuffer(),
  ], program.programId)[0]
  

  const voterPDA = PublicKey.findProgramAddressSync([
    Buffer.from("voter"),
    user.publicKey.toBuffer(),
    electionPDA.publicKey.toBuffer(),
  ], program.programId)[0]

  before('init accounts', async ()=>{
    let tx = new Transaction()
    tx.instructions = [
      ...[admin,user].map((k) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: k.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      ),
    ]
  
    await provider.sendAndConfirm(tx)
  })


  it("create election", async () => {

   await program.methods.createElection(4).signers([admin,electionPDA]).accounts({
    election: electionPDA.publicKey,
    signer: admin.publicKey
   }).rpc()

   let election = await program.account.election.fetch(electionPDA.publicKey);

   console.log(election.choicesLen == 4)

    
  });


  it ("add choices", async()=>{
    let choices = ["choice1","choice2","choice3","choice4"]
    await program.methods.addChoices(choices).signers([admin]).accounts({
      election: electionPDA.publicKey,
      choices: choicesPDA,
      signer:admin.publicKey,
    }).rpc()


    let choicesAccount = await program.account.choices.fetch(choicesPDA)

    
   
  })

  it('Update election state', async()=>{
    await program.methods.updateElectionState(1).signers([admin]).accounts({
      election: electionPDA.publicKey,
      signer: admin.publicKey
    }).rpc()

    let election = await program.account.election.fetch(electionPDA.publicKey)

    console.log(election.state == 1)
  })


  it('Vote for a choice', async()=>{

    await program.methods.vote(new anchor.BN(1)).signers([user]).accounts({
      election: electionPDA.publicKey,
      choices: choicesPDA,
      voter: voterPDA,
      signer: user.publicKey
    }).rpc()


    const election  = await program.account.election.fetch(electionPDA.publicKey)
    console.log(election)

    const choices = await program.account.choices.fetch(choicesPDA)

    console.log(choices)


    })
  });

