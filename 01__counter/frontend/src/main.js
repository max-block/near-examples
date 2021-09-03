import "regenerator-runtime/runtime"
import * as nearAPI from "near-api-js"


async function connect() {
    console.log("connect...", process.env.CONTRACT_NAME)
    const contractName = process.env.CONTRACT_NAME;
    const nearConfig = {
        networkId: 'testnet',
        nodeUrl: 'https://rpc.testnet.near.org',
        contractName,
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org'
    }

    const near = await nearAPI.connect({
        deps: {
            keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
        },
        ...nearConfig
    })

    const walletConnection = new nearAPI.WalletConnection(near)

    // window.walletConnection.account()
    const contract = await new nearAPI.Contract(walletConnection.account(), contractName, {
        viewMethods: ["get_value", "get_history"],
        changeMethods: ["action", "reset"],
        sender: walletConnection.getAccountId()
    })


    const res = await contract.get_history()
    console.log(res)


}


connect().catch(err => console.error(err))

