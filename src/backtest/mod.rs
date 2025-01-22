use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: Decimal,
    pub average_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct BacktestEngine {
    initial_capital: Decimal,
    commission_rate: Decimal,
    positions: HashMap<String, Position>,
    cash: Decimal,
    trades: Vec<Trade>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub commission: Decimal,
    pub side: OrderSide,
}

impl BacktestEngine {
    pub fn new(initial_capital: Decimal, commission_rate: Decimal) -> Self {
        Self {
            initial_capital,
            commission_rate,
            positions: HashMap::new(),
            cash: initial_capital,
            trades: Vec::new(),
        }
    }

    pub fn execute_order(&mut self, order: Order) -> anyhow::Result<Option<Trade>> {
        // 验证订单数量和价格
        if order.quantity <= Decimal::ZERO || order.price <= Decimal::ZERO {
            return Err(anyhow::anyhow!("Invalid order quantity or price"));
        }

        // 使用高精度计算手续费和总成本
        let commission = (order.price * order.quantity * self.commission_rate).round_dp(8);
        let total_cost = (order.price * order.quantity + commission).round_dp(8);

        // 检查资金是否足够
        if order.side == OrderSide::Buy {
            if total_cost > self.cash {
                return Ok(None);
            }
        }

        // 更新持仓
        let position = self.positions.entry(order.symbol.clone())
            .or_insert(Position {
                symbol: order.symbol.clone(),
                quantity: Decimal::ZERO,
                average_price: Decimal::ZERO,
            });

        match order.side {
            OrderSide::Buy => {
                // 使用高精度计算新的持仓成本和数量
                let new_total = (position.quantity * position.average_price + order.quantity * order.price).round_dp(8);
                position.quantity += order.quantity;
                if position.quantity > Decimal::ZERO {
                    position.average_price = (new_total / position.quantity).round_dp(8);
                }
                self.cash = (self.cash - total_cost).round_dp(8);
            },
            OrderSide::Sell => {
                if position.quantity < order.quantity {
                    return Ok(None);
                }
                position.quantity = (position.quantity - order.quantity).round_dp(8);
                self.cash = (self.cash + order.price * order.quantity - commission).round_dp(8);
            }
        }

        // 处理浮点数精度问题，当持仓接近于0时直接设为0
        if position.quantity.abs() < Decimal::new(1, 8) {
            self.positions.remove(&order.symbol);
        }

        let trade = Trade {
            timestamp: order.timestamp,
            symbol: order.symbol,
            price: order.price,
            quantity: order.quantity,
            commission,
            side: order.side,
        };

        self.trades.push(trade.clone());
        Ok(Some(trade))
    }

    pub fn get_position(&self, symbol: &str) -> Option<&Position> {
        self.positions.get(symbol)
    }

    pub fn get_cash(&self) -> Decimal {
        self.cash
    }

    pub fn get_trades(&self) -> &[Trade] {
        &self.trades
    }
}