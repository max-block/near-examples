import "regenerator-runtime/runtime"
import * as nearAPI from "near-api-js"
import Big from 'big.js'

const contractName = process.env.CONTRACT_NAME
const nearConfig = {
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    contractName,
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org'
}
let near
let walletConnection
let contract

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();


async function connect() {

    near = await nearAPI.connect({
        deps: {
            keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
        },
        ...nearConfig
    })

    walletConnection = new nearAPI.WalletConnection(near)

    contract = await new nearAPI.Contract(walletConnection.account(), contractName, {
        viewMethods: ["value", "history"],
        changeMethods: ["action", "reset"],
        sender: walletConnection.getAccountId()
    })

    console.log(walletConnection.getAccountId())
}

async function printHistory() {
    const res = await contract.history()
    document.getElementById("history").innerText = JSON.stringify(res, null, 2)
}

async function printValue() {
    const res = await contract.value()
    document.getElementById("value").innerText = res+""
}

function printWelcome() {
    if (walletConnection.getAccountId()) {
        document.getElementById("account").innerText = walletConnection.getAccountId()
        document.getElementById("login").style.display = "none"
    } else {
        document.getElementById("logout").style.display = "none"
    }
}


async function load() {
    await connect()
    printWelcome()
    await printValue()
    await printHistory()
}

export function login() {
    walletConnection.requestSignIn(
      contractName,
      'NEAR Counter / Rust'
    )
}

export function logout() {
    walletConnection.signOut()
    window.location.replace(window.location.origin + window.location.pathname)
}

export async function sendAction() {
    const operation = document.getElementById("operation").value
    const param = parseInt(document.getElementById("param").value)
    const res = await contract.action({operation, param})
    await printValue()
    await printHistory()
}

export async function sendReset() {
    const value = Big(document.getElementById("resetPayValue").value).times(10 ** 24).toFixed()
    console.log(value)
    const res = await contract.reset({}, BOATLOAD_OF_GAS, value)
}

load().catch(err => console.error(err))

