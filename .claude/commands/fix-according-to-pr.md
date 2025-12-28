你需要从 GitHub pull request 中获取评论并根据评论意见进行修复

1. 在 `AGENT_LOGGING.toon` 中记录和获得你已经回应的的 comment ID
2. 使用 `gh pr view 123 --comments --json comments | jq '[.comments | map(select(.id | IN("COMMENT_ID_1", "COMMENT_ID_2")) ) | .[].body ]'` 过滤已回应的 comment ID 并获得 comment 内容
