const
    {
        Connection,
        PublicKey,
        clusterApiUrl,
        Keypair,
        LAMPORTS_PER_SOL
    } = require('@solana/web3.js');

const wallet = new Keypair()

const publicKey = new PublicKey(wallet._keypair.publicKey);
const secretKey = wallet._keypair.secretKey;

const getWalletBalance = async () =>
{
    try
    {
        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
        const walletBalance = await connection.getBalance(publicKey);
        console.log("Wallet balance: ", walletBalance);
    }
    catch (err)
    {
        console.log("Error getting wallet balance: ", err);
    }
}

const MAX_RETRIES = 10; // Set the maximum number of retries
let numRetries = 0;

const airdropSol = async () =>
{
    try
    {
        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
        const signature = await connection.requestAirdrop(publicKey, 2 * LAMPORTS_PER_SOL);
        const latestBlockHash = await connection.getLatestBlockhash();
        await connection.confirmTransaction({signature:signature, ...latestBlockHash});
    }
    catch (err)
    {
        if (err.message.includes('Too Many Requests') && numRetries < MAX_RETRIES) {
            numRetries++;
            const delay = 2 ** numRetries * 1000; // Exponential backoff delay
            console.log(`Server responded with 429 Too Many Requests. Retrying after ${delay}ms delay...`);
            setTimeout(airdropSol, delay);
          } else {
            console.log("Error airdropping sol:", err);
          }
    }
}

const main = async () =>
{
    await getWalletBalance();
    await airdropSol();
    await getWalletBalance();
}

main();