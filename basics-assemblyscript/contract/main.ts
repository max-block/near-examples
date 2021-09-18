import {context, logging, PersistentVector, storage, u128} from "near-sdk-as"

const minResetPay = u128.from("500000000000000000000000")


@nearBindgen
class Action {
    constructor(public user: string, public operation: string, public param: i32, public timestamp: u64) {
    }
}


export function value(): i32 {
    return storage.getPrimitive<i32>("value", 0);
}

export function history(): Action[] {
    const result = new Array<Action>();
    for (let i = 0; i < _history.length; i++) {
        result[i] = _history[i]
    }
    return result
}

export function action(operation: string, param: i32): i32 {
    const newAction = new Action(context.sender, operation, param, context.blockTimestamp)
    if (operation == "add") {
        storage.set<i32>("value", value() + param)
    } else if (operation == "sub") {
        storage.set<i32>("value", value() - param)
    } else {
        // TODO: how to panic here???
        return -1
    }
    _history.push(newAction)
    return value()
}

// You can reset only if you pay 0.5 NEAR
export function reset(): void {
    if (context.attachedDeposit < minResetPay) {
        logging.log("Pay at least 0.5 NEAR for this action.")
        return
    }
    storage.set<i32>("value", 0)
    // _history.clear() TODO: how???
}


const _history = new PersistentVector<Action>("history")