use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use crate::backtest::{BacktestEngine, Order, OrderSide, Position, Trade};

#[pyclass(name = "RustBacktestEngine")]
struct PyBacktestEngine {
    engine: BacktestEngine,
}

#[pymethods]
impl PyBacktestEngine {
    #[new]
    fn new(initial_capital: f64, commission_rate: f64) -> Self {
        Self {
            engine: BacktestEngine::new(
                Decimal::from_f64(initial_capital).unwrap(),
                Decimal::from_f64(commission_rate).unwrap()
            )
        }
    }

    fn execute_order(&mut self, order_dict: &PyDict) -> PyResult<Option<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let symbol = order_dict.get_item("symbol")?.extract::<String>()?;
        let timestamp = order_dict.get_item("timestamp")?.extract::<DateTime<Utc>>()?;
        let price = order_dict.get_item("price")?.extract::<f64>()?;
        let quantity = order_dict.get_item("quantity")?.extract::<f64>()?;
        let side = match order_dict.get_item("side")?.extract::<String>()?.as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid order side"))
        };

        let order = Order {
            symbol,
            timestamp,
            price: Decimal::from_f64(price).unwrap(),
            quantity: Decimal::from_f64(quantity).unwrap(),
            side,
        };

        match self.engine.execute_order(order) {
            Ok(Some(trade)) => {
                let trade_dict = PyDict::new(py);
                trade_dict.set_item("timestamp", trade.timestamp)?;
                trade_dict.set_item("symbol", trade.symbol)?;
                trade_dict.set_item("price", trade.price.to_f64().unwrap())?;
                trade_dict.set_item("quantity", trade.quantity.to_f64().unwrap())?;
                trade_dict.set_item("commission", trade.commission.to_f64().unwrap())?;
                trade_dict.set_item("side", match trade.side {
                    OrderSide::Buy => "buy",
                    OrderSide::Sell => "sell",
                })?;
                Ok(Some(trade_dict.into()))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
        }
    }

    fn get_position(&self, symbol: &str) -> PyResult<Option<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        match self.engine.get_position(symbol) {
            Some(position) => {
                let pos_dict = PyDict::new(py);
                pos_dict.set_item("symbol", &position.symbol)?;
                pos_dict.set_item("quantity", position.quantity.to_f64().unwrap())?;
                pos_dict.set_item("average_price", position.average_price.to_f64().unwrap())?;
                Ok(Some(pos_dict.into()))
            },
            None => Ok(None)
        }
    }

    fn get_cash(&self) -> f64 {
        self.engine.get_cash().to_f64().unwrap()
    }

    fn get_trades(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let trades = self.engine.get_trades();
        let trade_list = PyList::empty(py);

        for trade in trades {
            let trade_dict = PyDict::new(py);
            trade_dict.set_item("timestamp", trade.timestamp)?;
            trade_dict.set_item("symbol", &trade.symbol)?;
            trade_dict.set_item("price", trade.price.to_f64().unwrap())?;
            trade_dict.set_item("quantity", trade.quantity.to_f64().unwrap())?;
            trade_dict.set_item("commission", trade.commission.to_f64().unwrap())?;
            trade_dict.set_item("side", match trade.side {
                OrderSide::Buy => "buy",
                OrderSide::Sell => "sell",
            })?;
            trade_list.append(trade_dict)?;
        }

        Ok(trade_list.into())
    }
}

#[pymodule]
fn qiz_first(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBacktestEngine>()?;
    Ok(())
}