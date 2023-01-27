use yahoo_finance_api as yahoo;

use clap::Parser;
use serde::{Deserialize, Serialize};
use session_types::*;

//makes a json request with this
#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    index: String,
}

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

fn stockServer(c: Chan<(), stockServer>) {
    let mut c = c.sel1().enter();
    let mut balance = 0;
    loop {
        c = offer! {
            c,
            CashDeposit => {
                todo!()
            },
            ViewStock => {
                let args = Args::parse();
                let (c, stock) = c.recv();
                //do some json parsing of reply from finance api and send it back as stock info???
                //c.send(StockInfo);
                todo!()
            },
            CashBalance => {
                todo!()
            }
        }
    }
}

fn cash_deposit(c: Chan<(), Client>) {
    unimplemented!()
}

fn view_stock(c: Chan<(), Client>) {
    let idx = Args::parse();
    todo!()
    //    let c = match c.send()
}

fn cash_balance(c: Chan<(), Client>) {
    unimplemented!()
}
