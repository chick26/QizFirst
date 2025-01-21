import os
import sys
from pathlib import Path

# 获取动态库路径
lib_path = str(Path(__file__).parent.parent / 'target' / 'release')

# 将动态库路径添加到系统路径
if sys.platform == 'darwin':
    lib_name = 'libqiz_first.dylib'
elif sys.platform == 'linux':
    lib_name = 'libqiz_first.so'
elif sys.platform == 'win32':
    lib_name = 'qiz_first.dll'
else:
    raise RuntimeError(f'Unsupported platform: {sys.platform}')

# 导入动态库
lib_file = os.path.join(lib_path, lib_name)
if not os.path.exists(lib_file):
    raise RuntimeError(f'Dynamic library not found: {lib_file}')

# 从动态库导出类和函数
from .market_analyzer import MarketAnalyzer

__all__ = ['MarketAnalyzer']