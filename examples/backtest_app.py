import streamlit as st
import qiz_first
import matplotlib.pyplot as plt
import numpy as np
from datetime import datetime, timedelta
import pandas as pd

# 页面配置
st.set_page_config(page_title="量化交易回测系统", layout="wide")
st.title("量化交易回测系统")

# 创建市场分析器实例
analyzer = qiz_first.MarketAnalyzer()

# 侧边栏 - 参数设置
with st.sidebar:
    st.header("参数设置")
    symbol = st.text_input("交易对", value="BTC/USDT")
    days = st.slider("回测天数", min_value=7, max_value=90, value=30)
    sma_period = st.slider("SMA周期", min_value=3, max_value=30, value=5)

# 生成模拟数据
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

# 主要内容区域
st.header("市场数据分析")

# 生成并保存数据
klines = generate_sample_data(days)
analyzer.save_klines(symbol, klines)

# 设置时间范围
start_ts = int((datetime.now() - timedelta(days=days)).timestamp())
end_ts = int(datetime.now().timestamp())

# 获取K线数据
historical_data = analyzer.query_klines(symbol, start_ts, end_ts)
timestamps = [datetime.fromtimestamp(ts) for ts, *_ in historical_data]
closes = [close for _, _, _, _, close, _ in historical_data]

# 计算SMA
sma_data = analyzer.calculate_sma(symbol, sma_period, start_ts, end_ts)
sma_timestamps = [datetime.fromtimestamp(ts) for ts, _ in sma_data]
sma_values = [val for _, val in sma_data]

# 创建图表
fig, ax = plt.subplots(figsize=(12, 6))
ax.plot(timestamps, closes, label='Price')
ax.plot(sma_timestamps, sma_values, label=f'SMA-{sma_period}', linestyle='--')
ax.set_title(f'{symbol} Price and SMA')
ax.set_xlabel('Date')
ax.set_ylabel('Price')
ax.legend()
ax.grid(True)
plt.xticks(rotation=45)
plt.tight_layout()

# 显示图表
st.pyplot(fig)

# 显示数据统计
st.header("数据统计")
col1, col2, col3 = st.columns(3)

with col1:
    st.metric("当前价格", f"{closes[-1]:.2f}")

with col2:
    price_change = ((closes[-1] - closes[0]) / closes[0]) * 100
    st.metric("价格变化", f"{price_change:.2f}%")

with col3:
    volatility = np.std(closes) / np.mean(closes) * 100
    st.metric("波动率", f"{volatility:.2f}%")

# 显示原始数据
st.header("历史数据")
df = pd.DataFrame(historical_data, 
                 columns=['Timestamp', 'Open', 'High', 'Low', 'Close', 'Volume'])
df['Timestamp'] = pd.to_datetime(df['Timestamp'], unit='s')
st.dataframe(df)