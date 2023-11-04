# 计划安排
从2023.10.9正式开始，使用[Pomodoro technique](https://www.bilibili.com/video/BV1eT4y157M8/?vd_source=d3e4fc18b1b113df78590c8a2bffc597)记录，参考了这个[模板](https://paste.ubuntu.com/p/xtzrbJhfZP/)。

## Activity Inventory（活动目录）

| id  | Item | DDL | Estimating | Important | Emergency |
| --- | ---- | --- | ---------- | --------- | --------- |
|  1  | rustlings |  10.15   |   16    |    normal  | 有两天比较忙，抽不出空 |
|  2  | ch3编程题 |  10.24   |   4    |    normal  |  |
|  3  | ch4编程题 |  10.29   |   12    |    normal  |  |
|  4  | ch5编程题 |  11.3   |   12    |    normal  |  |
|  5  | 完成前三章问答题，写总结 |  11.3   |   6    |    normal  | 每每太注意晋级要求，又补了问答题和总结 |

## Records （记录表）

| TaskId | Start time | TO DO | Status | Estimating | Actual | Comment | Category | Date |
| ------ | ---------- | ----- | ------ | ---------- | ------ | ------- | -------- | ---- |
|  1  |  23.00  |  准备rustlings环境   |   D     |      1      |     1   |     rustlings安装和README有一点不同。    |     DEV     |    10.9  |
| 1 | 16:00 | rustlings1-11题 | D | 1 | 0.5 | 比较容易 | DEV | 10.10 |
| 1 | 16:30 | rustlings12-40题 | D | 3 | 4 |  | DEV | 10.10 |
| 1 | ----- | rustlings41-80题 | D | 10 | 13 |  | DEV | 10.10、10.11、10.12 |
| 1 | ----- | rustlings41-80题 | D | 10 | 9 |  | DEV | 10.17 |
| 2 | 14.38 | ch3编程题 | D | 4 | XXXXX |  | DEV | 10.24 |
| 3.1 | 19:13 | ch4编程题，重写get_time | D | 4 | XXXXX | 参考了printf函数的地址转化 | DEV | 10.24 |
| 3.2 | 12:23 | ch4编程题，实现mmap和munmap | D | 6 | X(9) | 使用iset包的IntervalMap | DEV | 10.27 |
| 3.3 | 16:00 | 通过ch4测试 | D | 6 | X(9) | 想的太简单了，维护MemorySet中的map_areas需要小心 | DEV | 10.28 |
| 3.4 | 16:00 | 通过ch4测试 | D | 6 | X(9) | 想的太简单了，维护MemorySet中的map_areas需要小心 | DEV | 10.28 |
| 3.5 | 15:00 | 学习ch5 | D | 1 | IXXX | ch5的内容比预想的多 | LR | 10.29 |
| 4.1 | 18:10 | 合并之前的分支，解决第一题 | W | 6 | X(6) |  | DEV | 10.31 |
| 4.1 | 10:05 | 解决第一题 | D | 4 | XX |  | DEV | 11.1 |
| 4.2 | 11:30 | 解决第二题 | D | 4 | X(12) | 适配了优先队列花了很多时间，队列取元素用了peek导致错误 | DEV | 11.1 |
| 5.1 | 14:30 | 完成ch4和ch5的问答题 | D | 2 | X(4) | ch4的问答题不太好回答 | DEV | 11.3 |
| 5.2 | 21:11 | 完成ch3问答题和总结报告 | D | 2 | X(4) |  | DEV | 11.3 |
| 5.3 | 0:15 | 提交pr | D | 1 | X(2) | 一开始不知道怎么提交pr | DEV | 11.4 |
| 5.4 | 11:00 | 更新学习记录 | D | 1 | X(2) |  | DEV | 11.4 |

## Tech details

- Pomodoro: 25 mins
- Short break: 3-5 mins
- Long break for 4P: 15-30 mins
- Tracking:
  - interruption cause 0 pomodoro
  - done within 5 mins as 0 pomodoro
  - done over 5 mins then reviewing
  - X for 1 clean pomodoro
  - I for an internal interruption
  - O for the outside interruption
- Interruption
  - new issue to Activity Inventory/TODO/Emergency issue
  - add a task and ask for waiting util one pomodoro done
  - handle it first then add to Emergency issue with DONE and input Tracking
- Estimating
  - need more: if a task need more time, estimating again with "+n"
  - less: if a tast done within estimation, noted with "-n"
- Status
  - TD: Todo
  - W: Working
  - D: Done
  - C: Cancel
- Category for records
  - DEV: developing
  - LR: learning
  - MT: meeting
  - CM: communication
  - OT: others
  - RD: reading sth about 3C
  - WT: writing sth about 3C