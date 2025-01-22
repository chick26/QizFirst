from typing import List, Dict, Any, Optional
from datetime import datetime
import asyncio
import aiohttp
from .db_utils import ClickHouseClient

class MarketDataFetcher:
    def __init__(self, api_base_url: str, db_client: ClickHouseClient):
        self.api_base_url = api_base_url
        self.db_client = db_client
        self.session: Optional[aiohttp.ClientSession] = None
    
    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
            
    async def fetch_klines(
        self,
        symbol: str,
        interval: str = '1m',
        start_time: Optional[datetime] = None,
        end_time: Optional[datetime] = None,
        limit: int = 1000
    ) -> List[Dict[str, Any]]:
        """获取K线数据"""
        if not self.session:
            raise RuntimeError("Please use 'async with' to initialize the session")
            
        params = {
            'symbol': symbol,
            'interval': interval,
            'limit': limit
        }
        
        if start_time:
            params['startTime'] = int(start_time.timestamp() * 1000)
        if end_time:
            params['endTime'] = int(end_time.timestamp() * 1000)
            
        async with self.session.get(f"{self.api_base_url}/klines", params=params) as response:
            response.raise_for_status()
            data = await response.json()
            
            return [
                {
                    'symbol': symbol,
                    'timestamp': datetime.fromtimestamp(item[0] / 1000),
                    'open': float(item[1]),
                    'high': float(item[2]),
                    'low': float(item[3]),
                    'close': float(item[4]),
                    'volume': float(item[5]),
                    'turnover': float(item[7])
                }
                for item in data
            ]
    
    async def fetch_and_store_klines(
        self,
        symbols: List[str],
        interval: str = '1m',
        start_time: Optional[datetime] = None,
        end_time: Optional[datetime] = None,
        batch_size: int = 1000
    ) -> None:
        """批量获取K线数据并存储到数据库"""
        for symbol in symbols:
            try:
                klines = await self.fetch_klines(
                    symbol=symbol,
                    interval=interval,
                    start_time=start_time,
                    end_time=end_time,
                    limit=batch_size
                )
                
                if klines:
                    self.db_client.insert_market_data(klines)
                    
            except Exception as e:
                print(f"Error fetching data for {symbol}: {str(e)}")
                continue