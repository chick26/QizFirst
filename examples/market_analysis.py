import qiz_first
import matplotlib.pyplot as plt
import numpy as np
from datetime import datetime, timedelta

# 创建市场分析器实例
analyzer = qiz_first.MarketAnalyzer()

# 模拟一些K线数据
def generate_sample_data(days=30):
    now = datetime.now()
    data = []
    price = 100.0
    for i in range(days):
        timestamp = int((now - timedelta(days=days-i)).timestamp())
        price = price * (1 + np.random.normal(0, 0.02))
        high = price * (1 + abs(np.random.normal(0, 0.01)))
        low = price * (1 - abs(np.random.normal(0, 0.01)))
        volume = np.random.normal(1000, 200)
        data.append((timestamp, price, high, low, price, volume))
    return data

# 保存示例数据
symbol = "BTC/USDT"
klines = generate_sample_data()
analyzer.save_klines(symbol, klines)

# 查询数据并计算技术指标
start_ts = int((datetime.now() - timedelta(days=30)).timestamp())
end_ts = int(datetime.now().timestamp())

# 获取K线数据
historical_data = analyzer.query_klines(symbol, start_ts, end_ts)
timestamps = [datetime.fromtimestamp(ts) for ts, *_ in historical_data]
closes = [close for _, _, _, _, close, _ in historical_data]

# 计算SMA
sma_period = 5
sma_data = analyzer.calculate_sma(symbol, sma_period, start_ts, end_ts)
sma_timestamps = [datetime.fromtimestamp(ts) for ts, _ in sma_data]
sma_values = [val for _, val in sma_data]

# 绘制图表
plt.figure(figsize=(12, 6))
plt.plot(timestamps, closes, label='Price')
plt.plot(sma_timestamps, sma_values, label=f'SMA-{sma_period}', linestyle='--')
plt.title(f'{symbol} Price and SMA')
plt.xlabel('Date')
plt.ylabel('Price')
plt.legend()
plt.grid(True)
plt.xticks(rotation=45)
plt.tight_layout()
plt.show()