use serde::{Deserialize, Serialize};
use session_types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StockInfo {
    twap: u64,
    name: String,
    is_trading: bool,
}
type stockServer = Choose<Rec<stockInner>, Eps>;
type stockInner = Offer<CashDeposit, Offer<ViewStock, Offer<CashBalance, Eps>>>;

type CashDeposit = Recv<u64, Send<u64, Var<Z>>>;
type ViewStock = Recv<String, Send<StockInfo, Var<Z>>>;
type CashBalance = Send<u64, Var<Z>>;

type Client = <stockServer as HasDual>::Dual;

fn stockServer(_c: Chan<(), stockServer>) {
    unimplemented!()
}

fn cash_deposit(c: Chan<(), Client>) {
    unimplemented!()
}

fn view_stock(c: Chan<(), Client>) {
    unimplemented!()
}

fn cash_balance(c: Chan<(), Client>) {
    unimplemented!()
}
