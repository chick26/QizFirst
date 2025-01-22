from typing import List, Dict, Any, Optional
import clickhouse_connect
from datetime import datetime

class ClickHouseClient:
    def __init__(self, host: str = 'localhost', port: int = 8123, username: str = 'default', password: str = ''):
        self.client = clickhouse_connect.get_client(
            host=host,
            port=port,
            username=username,
            password=password
        )
    
    def init_market_data_table(self) -> None:
        """初始化分钟级行情数据表"""
        query = """
        CREATE TABLE IF NOT EXISTS market_data_minute (
            symbol String,
            timestamp DateTime,
            open Float64,
            high Float64,
            low Float64,
            close Float64,
            volume UInt64,
            turnover Float64
        )
        ENGINE = MergeTree
        PARTITION BY toYYYYMMDD(timestamp)
        ORDER BY (symbol, timestamp)
        """
        self.client.command(query)
    
    def insert_market_data(self, data: List[Dict[str, Any]]) -> None:
        """批量插入行情数据"""
        if not data:
            return
            
        columns = ['symbol', 'timestamp', 'open', 'high', 'low', 'close', 'volume', 'turnover']
        values = [
            [row[col] for col in columns]
            for row in data
        ]
        
        self.client.insert('market_data_minute', values, column_names=columns)
    
    def query_market_data(
        self,
        symbols: List[str],
        start_time: datetime,
        end_time: datetime
    ) -> List[Dict[str, Any]]:
        """查询指定时间范围内的行情数据"""
        query = """
        SELECT *
        FROM market_data_minute
        WHERE symbol IN {symbols}
        AND timestamp BETWEEN {start_time} AND {end_time}
        ORDER BY symbol, timestamp
        """
        
        result = self.client.query(
            query,
            parameters={
                'symbols': tuple(symbols),
                'start_time': start_time,
                'end_time': end_time
            }
        )
        
        return result.named_results()
    
    def close(self) -> None:
        """关闭数据库连接"""
        self.client.close()