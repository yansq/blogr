# 运维提升题目

## 单选

S1

Class TestClass {
	static int a = 123;
}

以上代码中，变量 a 的内存被分配在（D）
A. 虚拟机栈
B. 本地方法栈
C. 堆
D. 方法区

S2
在主流 JVM 中，如何判断一个对象已经不再使用，可以被回收？(B)
A. 引用计数法
B. 可达性分析
C. 双亲委派
D. 标记-清除

S3
foo=bar
echo "$foo"
echo '$foo'

输出结果为 (C)
A bar bar
B foo $foo
C bar $foo
D $foo bar

S4
如需手动部署银河应用构件包，应于（B）部署
A Websphere
B Governor
C Workspace
D dmgr

S5
集群模式下，Redis 至少需要部署 (D) 个节点，才能保证完整高可用
A 2
B 3
C 4
D 6

S6
使用 (A) 命令查看某端口被哪个进程占用？
A lsof
B ps
C sed
D netstat

S7
VPN使用的是 (A)
A 正向代理
B 反向代理

S8
在 VIM / VI 中，键入（D）可进入搜索模式
A f
B :
C &
D /

S9
使用 (C) 命令可检查 Nginx 配置文件的语法是否正确？
A ./nginx -s test
B ./nginx test
C ./nginx -t
D ./testNginx

S10
输入以下命令，打印的结果是（B）？

echo -e 'a=111\nexport a' > test.sh
chmod 777 ./test.sh
./source test.sh
echo ${a}
./test.sh
echo $a

A
a=111
export a
# 此处代表空行
111

B
111
111

C
# 此处代表空行
111

D
111
# 此处代表空行

## 多选

M1
以下垃圾回收器中，可以并行回收的是（CD）：
a. ParNew
b. Parallel Scavenge
c. CMS (Concurrent Mark Sweep)
d. G1

M2
CMS垃圾回收器的4个阶段中，哪些阶段会主线程暂停（stop the world）？（AC）
A. 初始标记
B. 并发标记
C. 重新标记
D. 并发清除

M3
JVM内存模型中，线程隔离的区域有?（BC）
A. 方法区
B. 程序计数器
C. 本地方法栈
D. 堆

M4
以下属于 Redis 基础数据结构的有？（ACD）
A String
B Array
C List
D Hash

M5
针对 testFolder 文件夹，以下说法正确的有 (AC)

missing:~$ ls -l /home
drwx--x--x 1 root  root  4096 Jun 15  2019 testFolder

A 使用 was 用户可以进行进入 testFolder 文件夹
B 使用 was 用户可以在 testFolder 文件夹中创建文件
C 使用 was 用户不可以在 testFolder 文件夹中删除文件
D 使用 was 用户可以列出 testFolder 文件夹中包含哪些文件

M6
当遗忘 linux 的 ps 命令有哪些参数时，可以使用（AC）查看参数使用说明？
A man ps
B help ps
C ps --help
D claim ps

M7
以下属于传输层协议的有（BC）
A IP
B TCP
C UDP
D HTTP

M8
以下关于 Elasticsearch 分片的说法，正确的有 (ABCD)
A 主分片数在索引创建过后不能修改
B 副本分片可用于处理读请求
C 主分片和其对应的副本分片不应部署在同一节点上
D 执行写操作时，只有当副本分片也写入后，才会返回成功

## 判断

C1
JDK1.8中，永久代被替换为元空间，元空间使用堆外内存（T）

C2
可以使用 finalize() 方法主动通知 JVM 进行垃圾回收（F）

C3
Redis 使用引用计数法判断共享对象能否被销毁 (T)

C4
如 Nginx 日志过大，可能导致交易超时。因此需对 Nginx 进行日志分割，并配置定期清理策略 (T)

C5
Redis 的缓存雪崩，指的是用户不断查询缓存中和数据库中都没有的数据，导致数据库压力过大，甚至崩溃 (F)

C6
可以将 Elasticsearch 的数据存储在 NAS，以实现集群间数据共享 (F)

C7
Kafka 性能优异的原因之一是将数据保存在内存中 (F)

C8
在 Kafka 中，同一个消费组的两个消费者不会同时消费一个 partition (T)

C9
Kafka 中，如果一个消费者组中有 k 个消费者，该消息者组消费的topic 有 n 个分区，且 k > n，则会有 k - n 个消费者无法接受消息 (T)

## 简答

A1
请简述 JVM 垃圾回收流程

A2
请简述 redis 的持久化机制，并比较不同方案的优缺点

A3
请简述我部 ES 架构，及数据加载/查询流程