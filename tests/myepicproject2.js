const anchor = require('@project-serum/anchor');

//System Program
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting Test...");
  
  //Create and set provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  //Create an account kepair for our program to use
  const baseAccount = anchor.web3.Keypair.generate();
  
  //Call start_stuff_off and pass it the required params
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  
  console.log("📝 Your transaction signature", tx);

  //Fetch data from the account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(`👀 GIF Count`, account.totalGifs.toString());

  //Calling add_gif!
  await program.rpc.addGif("https://media.giphy.com/media/26gN16cJ6gy4LzZSw/giphy.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  //Fetch account data again to view change
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  console.log('👀 GIF List', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();