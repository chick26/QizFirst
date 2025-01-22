from typing import List, Dict, Any, Optional, Callable
from datetime import datetime
import pandas as pd
from .db_utils import ClickHouseClient

class BacktestEngine:
    def __init__(
        self,
        db_client: ClickHouseClient,
        initial_capital: float = 1000000.0,
        commission_rate: float = 0.0003
    ):
        self.db_client = db_client
        self.initial_capital = initial_capital
        self.commission_rate = commission_rate
        self.positions: Dict[str, float] = {}
        self.cash: float = initial_capital
        self.trades: List[Dict[str, Any]] = []
        
    def load_market_data(
        self,
        symbols: List[str],
        start_time: datetime,
        end_time: datetime
    ) -> pd.DataFrame:
        """从数据库加载历史行情数据"""
        data = self.db_client.query_market_data(symbols, start_time, end_time)
        df = pd.DataFrame(data)
        df.set_index(['timestamp', 'symbol'], inplace=True)
        return df
    
    def run_backtest(
        self,
        strategy_func: Callable[[pd.DataFrame, Dict[str, Any]], List[Dict[str, Any]]],
        symbols: List[str],
        start_time: datetime,
        end_time: datetime,
        strategy_params: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """执行回测"""
        # 加载市场数据
        market_data = self.load_market_data(symbols, start_time, end_time)
        
        # 执行策略逻辑
        signals = strategy_func(market_data, strategy_params or {})
        
        # 模拟交易执行
        for signal in signals:
            self._execute_trade(signal)
        
        # 计算回测结果
        return self._calculate_results()
    
    def _execute_trade(self, signal: Dict[str, Any]) -> None:
        """执行交易信号"""
        symbol = signal['symbol']
        price = signal['price']
        volume = signal['volume']
        timestamp = signal['timestamp']
        
        # 计算交易成本
        cost = price * abs(volume) * (1 + self.commission_rate)
        
        # 检查资金是否足够
        if cost > self.cash and volume > 0:
            return
        
        # 更新持仓
        current_position = self.positions.get(symbol, 0.0)
        new_position = current_position + volume
        
        if abs(new_position) < 1e-6:  # 如果持仓接近于0，直接平仓
            self.positions.pop(symbol, None)
        else:
            self.positions[symbol] = new_position
        
        # 更新现金
        self.cash -= (volume * price + cost)
        
        # 记录交易
        self.trades.append({
            'timestamp': timestamp,
            'symbol': symbol,
            'price': price,
            'volume': volume,
            'cost': cost,
            'cash': self.cash
        })
    
    def _calculate_results(self) -> Dict[str, Any]:
        """计算回测结果"""
        if not self.trades:
            return {
                'initial_capital': self.initial_capital,
                'final_capital': self.initial_capital,
                'total_return': 0.0,
                'total_trades': 0,
                'positions': {},
                'trades': []
            }
        
        trades_df = pd.DataFrame(self.trades)
        
        return {
            'initial_capital': self.initial_capital,
            'final_capital': self.cash,
            'total_return': (self.cash - self.initial_capital) / self.initial_capital,
            'total_trades': len(self.trades),
            'positions': self.positions,
            'trades': self.trades
        }