从 `FEATURE_LIST.toon` 中选择合适一个功能点来实现

1. 首先，你需要通过 `git log` 和 `AGENT_LOGGING.toon` 获取项目的基础状况和工作进展
2. 你需要根据 `FEATURE_LIST.toon` 中的功能点，选择一个合适的功能点来实现
3. 你需要将你的工作进展记录到 `AGENT_LOGGING.toon` 中
4. 你需要编写完善的单元测试
5. 不允许在 main 分支上提交；你需要提交你的代码变更到 Git 仓库，提交信息和分支名遵循 [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) 规范

## AGENT_LOGGING.toon 的格式

```toon
Log[0]
```

每次记录一条日志，你都需要增加 Log[x] 中的 x，变成以下情况：

```toon
Log[1]
  - time: 2024-12-28T00:00:00Z
    content: "some log"
    file-related[2]: a.md,b.md
```
