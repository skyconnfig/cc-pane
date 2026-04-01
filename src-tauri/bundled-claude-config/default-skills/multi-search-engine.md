---
name: multi-search-engine
description: 使用 17 个搜索引擎（8 个中文 + 9 个国际）进行无 API Key 的信息检索，支持高级检索语法、时间过滤、站内搜索、隐私搜索、WolframAlpha 知识查询。
---

# Multi Search Engine

统一封装常见搜索引擎 URL 模板，按场景快速切换，减少单一来源偏差。

## 搜索引擎

### 中文（8）
- Baidu: `https://www.baidu.com/s?wd={keyword}`
- Bing CN: `https://cn.bing.com/search?q={keyword}&ensearch=0`
- Bing INT: `https://cn.bing.com/search?q={keyword}&ensearch=1`
- 360: `https://www.so.com/s?q={keyword}`
- Sogou: `https://sogou.com/web?query={keyword}`
- WeChat: `https://wx.sogou.com/weixin?type=2&query={keyword}`
- Toutiao: `https://so.toutiao.com/search?keyword={keyword}`
- Jisilu: `https://www.jisilu.cn/explore/?keyword={keyword}`

### 国际（9）
- Google: `https://www.google.com/search?q={keyword}`
- Google HK: `https://www.google.com.hk/search?q={keyword}`
- DuckDuckGo: `https://duckduckgo.com/html/?q={keyword}`
- Yahoo: `https://search.yahoo.com/search?p={keyword}`
- Startpage: `https://www.startpage.com/sp/search?query={keyword}`
- Brave: `https://search.brave.com/search?q={keyword}`
- Ecosia: `https://www.ecosia.org/search?q={keyword}`
- Qwant: `https://www.qwant.com/?q={keyword}`
- WolframAlpha: `https://www.wolframalpha.com/input?i={keyword}`

## 推荐检索策略

1. **先广后深**：先用 Google/Bing 获取候选，再用站内检索缩小范围。
2. **交叉验证**：中文和国际引擎各至少 1 个来源，避免单一偏差。
3. **时效优先**：新闻/政策/版本类查询必须加时间过滤（如 `tbs=qdr:w`）。
4. **隐私优先**：敏感主题优先使用 DuckDuckGo/Startpage/Brave。

## 高级语法

- 站内：`site:github.com rust async`
- 文件类型：`filetype:pdf architecture review`
- 精确匹配：`"multi agent pipeline"`
- 排除：`openai -chatgpt`
- 或逻辑：`k8s OR kubernetes`

## 时间过滤（Google `tbs`）

- 近 1 小时：`tbs=qdr:h`
- 近 1 天：`tbs=qdr:d`
- 近 1 周：`tbs=qdr:w`
- 近 1 月：`tbs=qdr:m`
- 近 1 年：`tbs=qdr:y`

## DuckDuckGo Bangs

- `!g` Google
- `!gh` GitHub
- `!so` Stack Overflow
- `!w` Wikipedia
- `!yt` YouTube

## WolframAlpha 示例

- 数学：`integrate x^2 dx`
- 汇率：`100 USD to CNY`
- 股票：`AAPL stock`
- 天气：`weather in Shanghai`

