# 如何写个Parser

依赖于pest、nom做文件解析

## 流程

input(&str) -> pest或nom预法(AST)

## 支持文件类型

- [x] csv
- [x] dotEnv
- [x] ini
- [x] json

...