---
name: openviking-context
description: 使用 OpenViking 构建 Agent 上下文数据库，支持资源导入、目录树检索、语义查找与长期记忆沉淀。
---

# OpenViking Context

这个技能用于把 `volcengine/OpenViking` 的核心能力接入你的日常开发流：
- 持久化资源（文档、仓库、网页）
- 结构化检索（tree / ls / grep）
- 语义检索（find）
- 为后续任务提供可复用上下文

## 何时使用

- 项目知识分散，想要统一管理到一个“上下文文件系统”
- 需要跨会话复用背景知识（规范、架构、接口文档）
- 想减少重复解释和 prompt token 消耗

## 快速命令

```bash
# 1) 启动服务
openviking-server

# 2) 健康检查
ov status

# 3) 导入资源（仓库 / URL / 文件）
ov add-resource https://github.com/volcengine/OpenViking --wait
ov add-resource https://raw.githubusercontent.com/volcengine/OpenViking/main/README.md --wait

# 4) 查看结构
ov ls viking://resources/
ov tree viking://resources/ -L 2

# 5) 检索
ov find "how does OpenViking organize memory"
ov grep "filesystem paradigm" --uri viking://resources/
```

## 推荐流程

1. **先导入**：把项目 README、设计文档、API 文档导入 OpenViking。
2. **再建索引**：导入时优先使用 `--wait`，确保后续查询命中率。
3. **分层检索**：先 `tree/ls` 定位，再 `find/grep` 精确查找。
4. **任务沉淀**：关键结论回写到 `viking://resources/<project>/docs` 形成长期记忆。

## 与 CC-Panes 协作建议

- 在开始一个新任务前，先执行一次 `ov find` 获取历史上下文。
- 在任务完成后，把关键文档/复盘结果 `ov add-resource` 入库。
- 通过固定目录规范（如 `viking://resources/<project>/`）提高多智能体协作一致性。

