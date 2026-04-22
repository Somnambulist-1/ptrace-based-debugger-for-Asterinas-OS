# proj9 项目技术文档
2026 年全国大学生计算机系统能力大赛操作系统设计赛——功能赛道

赛题：面向Rust OS的进程调试能力构建

队名：OS Rookies

成员：颜锡宇，苏语麒

学校：北京大学

## 阶段实现目标
- [ ] 增添 `kernel/src/process/status.rs` 中对 `ptrace stop` 的支持 (line 168)
- [ ] 在 `kernel/src/thread/task.rs` 中增加因 `ptrace` 暂停的逻辑 (line 108)
- [ ] 在 `kernel/src/process/process/mod.rs` 中拓展现有 `process.stop()/resume()`
- [ ] 同步进行 `ptrace` 基础功能函数的框架搭建和完善