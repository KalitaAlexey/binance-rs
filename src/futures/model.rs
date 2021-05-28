use serde::{Deserialize, Serialize};
use crate::model::{string_or_float, string_or_float_opt};

pub use crate::model::{
    Asks, Bids, BookTickers, Filters, KlineSummaries, KlineSummary, RateLimit, ServerTime,
    SymbolPrice, Tickers,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<String>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub filters: Vec<Filters>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    // Undocumented
    #[serde(rename = "E")]
    pub event_time: u64,
    // Undocumented
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Trades {
    AllTrades(Vec<Trade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub is_buyer_maker: bool,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AggTrades {
    AllAggTrades(Vec<AggTrade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarkPrices {
    AllMarkPrices(Vec<MarkPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LiquidationOrders {
    AllLiquidationOrders(Vec<LiquidationOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(with = "string_or_float")]
    pub average_price: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub status: String,
    pub symbol: String,
    pub time: u64,
    pub time_in_force: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(with = "string_or_float")]
    pub open_interest: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Order {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub reduce_only: bool,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub orig_type: String,
    #[serde(with = "string_or_float", default = "default_activation_price")]
    pub activation_price: f64,
    #[serde(with = "string_or_float", default = "default_price_rate")]
    pub price_rate: f64,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub orig_type: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: u8,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction: u64,

    #[serde(rename = "a")]
    pub data: AccountUpdateData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountUpdateData {
    #[serde(rename = "m")]
    pub reason: AccountUpdateReason,

    #[serde(rename = "B")]
    pub balance_updates: Vec<AccountBalanceUpdate>,

    #[serde(rename = "P")]
    pub position: Vec<AccountPositionUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountUpdateReason {
    Deposit,
    Withdraw,
    Order,
    FundingFee,
    WithdrawReject,
    Adjustment,
    InsuranceClear,
    AdminDeposit,
    AdminWithdraw,
    MarginTransfer,
    MarginTypeChange,
    AssetTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
    AutoExchange,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountBalanceUpdate {
    #[serde(rename = "a")]
    pub asset: String,

    #[serde(rename = "wb")]
    pub balance: String,

    #[serde(rename = "cw")]
    pub cross_balance: String,

    #[serde(rename = "bc")]
    pub balance_change: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountPositionUpdate {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "pa")]
    pub position_amount: String,

    #[serde(rename = "ep")]
    pub entry_price: String,

    #[serde(rename = "cr")]
    pub accumulated_realized: String,

    #[serde(rename = "up")]
    pub unrealized: String,

    #[serde(rename = "mt")]
    pub margin_type: String,

    #[serde(rename = "iw")]
    pub isolated_wallet: String,

    #[serde(rename = "ps")]
    pub position_side: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction: u64,

    #[serde(rename = "o")]
    pub data: OrderUpdateData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderUpdateData {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub client_order_id: String,

    #[serde(rename = "S")]
    pub side: OrderUpdateSide,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub original_quantity: String,

    #[serde(rename = "p")]
    pub original_price: String,

    #[serde(rename = "ap")]
    pub average_price: String,

    #[serde(rename = "sp")]
    pub stop_price: String,

    #[serde(rename = "x")]
    pub execution_type: String,

    #[serde(rename = "X")]
    pub status: String,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l")]
    pub last_filled_quantity: String,

    #[serde(rename = "z")]
    pub filled_accumulated_quantity: String,

    #[serde(rename = "L")]
    pub last_filled_price: String,

    #[serde(rename = "N")]
    #[serde(default)]
    pub commission_asset: Option<String>,

    #[serde(rename = "n")]
    #[serde(default)]
    pub commission: Option<String>,

    #[serde(rename = "T")]
    pub order_trade_time: u64,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "b")]
    pub bids_notional: String,

    #[serde(rename = "a")]
    pub asks_notional: String,

    #[serde(rename = "m")]
    pub trade_maker_side: bool,

    #[serde(rename = "R")]
    pub reduce_only: bool,

    #[serde(rename = "wt")]
    pub stop_price_working_type: String,

    #[serde(rename = "ot")]
    pub original_order_type: String,

    #[serde(rename = "ps")]
    pub position_side: String,

    #[serde(rename = "cp")]
    pub close_all: bool,

    #[serde(rename = "AP")]
    #[serde(default)]
    pub activation_price: Option<String>,

    #[serde(rename = "cr")]
    #[serde(default)]
    pub callback_rate: Option<String>,

    #[serde(rename = "rp")]
    pub realized_profit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderUpdateSide {
    Buy,
    Sell,
}

#[test]
fn deserialize_order_update_event() {
    let value = serde_json::from_str::<OrderUpdateEvent>(
        r#"{ "e":"ORDER_TRADE_UPDATE", "E":1568879465651, "T":1568879465650,
        "o":{
          "s":"BTCUSDT", "c":"TEST", "S":"SELL", "o":"TRAILING_STOP_MARKET",
          "f":"GTC", "q":"0.001", "p":"0", "ap":"0", "sp":"7103.04", "x":"NEW", "X":"NEW", "i":8886774,
          "l":"0", "z":"0",
          "L":"0",
          "N":"USDT",
          "n":"0",
          "T":1568879465651,
          "t":0,
          "b":"0",
          "a":"9.91",
          "m":false,
          "R":false,
          "wt":"CONTRACT_PRICE",
          "ot":"TRAILING_STOP_MARKET",
          "ps":"LONG",
          "cp":false,
          "AP":"7476.89",
          "cr":"5.0",
          "rp":"0"
        }
      }
      "#,
    ).unwrap();
    assert_eq!(
        value,
        OrderUpdateEvent {
            event_type: "ORDER_TRADE_UPDATE".to_string(),
            event_time: 1568879465651,
            transaction: 1568879465650,
            data: OrderUpdateData {
                symbol: "BTCUSDT".to_string(),
                client_order_id: "TEST".to_string(),
                side: OrderUpdateSide::Sell,
                order_type: "TRAILING_STOP_MARKET".to_string(),
                time_in_force: "GTC".to_string(),
                original_quantity: "0.001".to_string(),
                original_price: "0".to_string(),
                average_price: "0".to_string(),
                stop_price: "7103.04".to_string(),
                execution_type: "NEW".to_string(),
                status: "NEW".to_string(),
                order_id: 8886774,
                last_filled_quantity: "0".to_string(),
                filled_accumulated_quantity: "0".to_string(),
                last_filled_price: "0".to_string(),
                commission_asset: Some("USDT".to_string()),
                commission: Some("0".to_string()),
                order_trade_time: 1568879465651,
                trade_id: 0,
                bids_notional: "0".to_string(),
                asks_notional: "9.91".to_string(),
                trade_maker_side: false,
                reduce_only: false,
                stop_price_working_type: "CONTRACT_PRICE".to_string(),
                original_order_type: "TRAILING_STOP_MARKET".to_string(),
                position_side: "LONG".to_string(),
                close_all: false,
                activation_price: Some("7476.89".to_string()),
                callback_rate: Some("5.0".to_string()),
                realized_profit: "0".to_string()
            }
        }
    );

    let value = serde_json::from_str::<OrderUpdateEvent>(
        r#"{"e":"ORDER_TRADE_UPDATE","T":1622142325683,"E":1622142325687,"o":{"s":"DOTUSDT","c":"web_2F1nPfgmy53m5J4Hh9of","S":"BUY","o":"LIMIT","f":"GTC","q":"198.6","p":"23","ap":"0","sp":"0","x":"CANCELED","X":"CANCELED","i":5595810084,"l":"0","z":"0","L":"0","T":1622142325683,"t":0,"b":"0","a":"0","m":false,"R":false,"wt":"CONTRACT_PRICE","ot":"LIMIT","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0}}"#,
    ).unwrap();
    assert_eq!(
        value,
        OrderUpdateEvent {
            event_type: "ORDER_TRADE_UPDATE".to_string(),
            event_time: 1622142325687,
            transaction: 1622142325683,
            data: OrderUpdateData {
                symbol: "DOTUSDT".to_string(),
                client_order_id: "web_2F1nPfgmy53m5J4Hh9of".to_string(),
                side: OrderUpdateSide::Buy,
                order_type: "LIMIT".to_string(),
                time_in_force: "GTC".to_string(),
                original_quantity: "198.6".to_string(),
                original_price: "23".to_string(),
                average_price: "0".to_string(),
                stop_price: "0".to_string(),
                execution_type: "CANCELED".to_string(),
                status: "CANCELED".to_string(),
                order_id: 5595810084,
                last_filled_quantity: "0".to_string(),
                filled_accumulated_quantity: "0".to_string(),
                last_filled_price: "0".to_string(),
                commission_asset: None,
                commission: None,
                order_trade_time: 1622142325683,
                trade_id: 0,
                bids_notional: "0".to_string(),
                asks_notional: "0".to_string(),
                trade_maker_side: false,
                reduce_only: false,
                stop_price_working_type: "CONTRACT_PRICE".to_string(),
                original_order_type: "LIMIT".to_string(),
                position_side: "BOTH".to_string(),
                close_all: false,
                activation_price: None,
                callback_rate: None,
                realized_profit: "0".to_string()
            }
        }
    );

    let value = serde_json::from_str::<OrderUpdateEvent>(
        r#"{"e":"ORDER_TRADE_UPDATE","T":1622142670086,"E":1622142670089,"o":{"s":"GRTUSDT","c":"web_zf9zndTx6Ih1tuU0paEB","S":"BUY","o":"LIMIT","f":"GTC","q":"12","p":"0.75919","ap":"0","sp":"0","x":"NEW","X":"NEW","i":1697404867,"l":"0","z":"0","L":"0","T":1622142670086,"t":0,"b":"9.11028","a":"0","m":false,"R":false,"wt":"CONTRACT_PRICE","ot":"LIMIT","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0}}"#,
    ).unwrap();
    assert_eq!(
        value,
        OrderUpdateEvent {
            event_type: "ORDER_TRADE_UPDATE".to_string(),
            event_time: 1622142670089,
            transaction: 1622142670086,
            data: OrderUpdateData {
                symbol: "GRTUSDT".to_string(),
                client_order_id: "web_zf9zndTx6Ih1tuU0paEB".to_string(),
                side: OrderUpdateSide::Buy,
                order_type: "LIMIT".to_string(),
                time_in_force: "GTC".to_string(),
                original_quantity: "12".to_string(),
                original_price: "0.75919".to_string(),
                average_price: "0".to_string(),
                stop_price: "0".to_string(),
                execution_type: "NEW".to_string(),
                status: "NEW".to_string(),
                order_id: 1697404867,
                last_filled_quantity: "0".to_string(),
                filled_accumulated_quantity: "0".to_string(),
                last_filled_price: "0".to_string(),
                commission_asset: None,
                commission: None,
                order_trade_time: 1622142670086,
                trade_id: 0,
                bids_notional: "9.11028".to_string(),
                asks_notional: "0".to_string(),
                trade_maker_side: false,
                reduce_only: false,
                stop_price_working_type: "CONTRACT_PRICE".to_string(),
                original_order_type: "LIMIT".to_string(),
                position_side: "BOTH".to_string(),
                close_all: false,
                activation_price: None,
                callback_rate: None,
                realized_profit: "0".to_string()
            }
        }
    );

    let value = serde_json::from_str::<OrderUpdateEvent>(r#"{"e":"ORDER_TRADE_UPDATE","T":1622143071600,"E":1622143071605,"o":{"s":"GRTUSDT","c":"web_TF7zjup9BmWWFluoSz6M","S":"BUY","o":"MARKET","f":"GTC","q":"10","p":"0","ap":"0.76867","sp":"0","x":"TRADE","X":"FILLED","i":1697440152,"l":"10","z":"10","L":"0.76867","n":"0.00307468","N":"USDT","T":1622143071600,"t":69281933,"b":"0","a":"0","m":false,"R":false,"wt":"CONTRACT_PRICE","ot":"MARKET","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0}}"#).unwrap();
    assert_eq!(
        value,
        OrderUpdateEvent {
            event_type: "ORDER_TRADE_UPDATE".to_string(),
            event_time: 1622143071605,
            transaction: 1622143071600,
            data: OrderUpdateData {
                symbol: "GRTUSDT".to_string(),
                client_order_id: "web_TF7zjup9BmWWFluoSz6M".to_string(),
                side: OrderUpdateSide::Buy,
                order_type: "MARKET".to_string(),
                time_in_force: "GTC".to_string(),
                original_quantity: "10".to_string(),
                original_price: "0".to_string(),
                average_price: "0.76867".to_string(),
                stop_price: "0".to_string(),
                execution_type: "TRADE".to_string(),
                status: "FILLED".to_string(),
                order_id: 1697440152,
                last_filled_quantity: "10".to_string(),
                filled_accumulated_quantity: "10".to_string(),
                last_filled_price: "0.76867".to_string(),
                commission_asset: Some("USDT".to_string()),
                commission: Some("0.00307468".to_string()),
                order_trade_time: 1622143071600,
                trade_id: 69281933,
                bids_notional: "0".to_string(),
                asks_notional: "0".to_string(),
                trade_maker_side: false,
                reduce_only: false,
                stop_price_working_type: "CONTRACT_PRICE".to_string(),
                original_order_type: "MARKET".to_string(),
                position_side: "BOTH".to_string(),
                close_all: false,
                activation_price: None,
                callback_rate: None,
                realized_profit: "0".to_string()
            }
        }
    );

    let value = serde_json::from_str::<OrderUpdateEvent>(r#"{"e":"ORDER_TRADE_UPDATE","T":1622143569768,"E":1622143569771,"o":{"s":"GRTUSDT","c":"web_055cCwQDrbfL4riYpzDT","S":"SELL","o":"MARKET","f":"GTC","q":"12","p":"0","ap":"0","sp":"0","x":"NEW","X":"NEW","i":1697483971,"l":"0","z":"0","L":"0","T":1622143569768,"t":0,"b":"0","a":"0","m":false,"R":true,"wt":"CONTRACT_PRICE","ot":"MARKET","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0}}"#).unwrap();

    let value = serde_json::from_str::<crate::websockets::Events>(r#"{"e":"ORDER_TRADE_UPDATE","T":1622143569768,"E":1622143569771,"o":{"s":"GRTUSDT","c":"web_055cCwQDrbfL4riYpzDT","S":"SELL","o":"MARKET","f":"GTC","q":"12","p":"0","ap":"0","sp":"0","x":"NEW","X":"NEW","i":1697483971,"l":"0","z":"0","L":"0","T":1622143569768,"t":0,"b":"0","a":"0","m":false,"R":true,"wt":"CONTRACT_PRICE","ot":"MARKET","ps":"BOTH","cp":false,"rp":"0","pP":false,"si":0,"ss":0}}"#).unwrap();
}

#[test]
fn deserialize_account_update_event() {
    let value = serde_json::from_str::<AccountUpdateEvent>(
        r#"{
            "e": "ACCOUNT_UPDATE",
            "E": 1564745798939,
            "T": 1564745798938 ,
            "a":
              {
                "m":"ORDER",
                "B":[
                  {
                    "a":"USDT",
                    "wb":"122624.12345678",
                    "cw":"100.12345678",
                    "bc":"50.12345678"
                  },
                  {
                    "a":"BUSD",
                    "wb":"1.00000000",
                    "cw":"0.00000000",
                    "bc":"-49.12345678"
                  }
                ],"P":[
                  {
                    "s":"BTCUSDT",
                    "pa":"0",
                    "ep":"0.00000",
                    "cr":"200",
                    "up":"0",
                    "mt":"isolated",
                    "iw":"0.00000000",
                    "ps":"BOTH"
                  },
                  {
                      "s":"BTCUSDT",
                      "pa":"20",
                      "ep":"6563.66500",
                      "cr":"0",
                      "up":"2850.21200",
                      "mt":"isolated",
                      "iw":"13200.70726908",
                      "ps":"LONG"
                   },
                  {
                      "s":"BTCUSDT",
                      "pa":"-10",
                      "ep":"6563.86000",
                      "cr":"-45.04000000",
                      "up":"-1423.15600",
                      "mt":"isolated",
                      "iw":"6570.42511771",
                      "ps":"SHORT"
                  }
                ]
              }
          }
          "#,
    )
    .unwrap();
    assert_eq!(
        value,
        AccountUpdateEvent {
            event_type: "ACCOUNT_UPDATE".to_string(),
            event_time: 1564745798939,
            transaction: 1564745798938,
            data: AccountUpdateData {
                reason: AccountUpdateReason::Order,
                balance_updates: vec![
                    AccountBalanceUpdate {
                        asset: "USDT".to_string(),
                        balance: "122624.12345678".to_string(),
                        cross_balance: "100.12345678".to_string(),
                        balance_change: "50.12345678".to_string()
                    },
                    AccountBalanceUpdate {
                        asset: "BUSD".to_string(),
                        balance: "1.00000000".to_string(),
                        cross_balance: "0.00000000".to_string(),
                        balance_change: "-49.12345678".to_string()
                    }
                ],
                position: vec![
                    AccountPositionUpdate {
                        symbol: "BTCUSDT".to_string(),
                        position_amount: "0".to_string(),
                        entry_price: "0.00000".to_string(),
                        accumulated_realized: "200".to_string(),
                        unrealized: "0".to_string(),
                        margin_type: "isolated".to_string(),
                        isolated_wallet: "0.00000000".to_string(),
                        position_side: "BOTH".to_string()
                    },
                    AccountPositionUpdate {
                        symbol: "BTCUSDT".to_string(),
                        position_amount: "20".to_string(),
                        entry_price: "6563.66500".to_string(),
                        accumulated_realized: "0".to_string(),
                        unrealized: "2850.21200".to_string(),
                        margin_type: "isolated".to_string(),
                        isolated_wallet: "13200.70726908".to_string(),
                        position_side: "LONG".to_string()
                    },
                    AccountPositionUpdate {
                        symbol: "BTCUSDT".to_string(),
                        position_amount: "-10".to_string(),
                        entry_price: "6563.86000".to_string(),
                        accumulated_realized: "-45.04000000".to_string(),
                        unrealized: "-1423.15600".to_string(),
                        margin_type: "isolated".to_string(),
                        isolated_wallet: "6570.42511771".to_string(),
                        position_side: "SHORT".to_string()
                    }
                ]
            }
        }
    );
}

fn default_stop_price() -> f64 {
    0.0
}
fn default_activation_price() -> f64 {
    0.0
}
fn default_price_rate() -> f64 {
    0.0
}
