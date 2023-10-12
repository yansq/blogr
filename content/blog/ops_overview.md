## 故障应急处理方法

## 处理问题

优先切应用/重启服务：

● 银河应用：F5下应用，注意目前鄞州办公前置区与办公主机区合并，江北未合并。切鄞州应用去办公主机区F5，切江北应用去办公前置区F5。操作见科技知识库 F5 相关条件。账号/密码都是 operator_user。
● 水星应用：在星云空间站选择应用实例，点击隔离。

修改配置文件：

● Apollo 修改，如之前未使用过 Apollo 需要先赋权；
● vim 直接进入 jar 包修改。

## 外部问题

###  网络异常

留意近期网络部变更

### 数据库连接异常

联系部门 DBA 或系统管理员

### 外围系统异常

联系对应系统系统负责人，或ESB、数据总线、对象存储等相关人员

## 自身问题

### 磁盘空间不足

系统管理员会对系统磁盘空间状况进行日常巡检，如磁盘使用率超过75%，会在科管上对系统负责人发起异常巡检流程。

如磁盘使用率达到85% - 90%（检测不是实时），触发8级故障。

#### 查看硬盘

查看服务器硬盘使用情况：

df -h

查看服务器硬盘使用分布：

du -h --max-depth=1

查看目录下有多少文件：

ll | wc -l

#### 删除文件

删除10GB内文件使用 nbrm 命令，选择远程备份选项。

由于nbrm命令受网络限制，远程备份速度较慢，加上备份盘容量有限，如果删除超过10GB文件，建议在确定文件可被清除后，使用rm命令清除。

由于合规要求，不能直接在服务器执行rm命令，有以下几个替代方案：

● 将rm命令写在脚本里，然后执行脚本。注意在写操作流程时，要加上给脚本赋权的语句，默认权限无法执行脚本。
● 使用SFTP的可视化界面删除，之前邮件提过不允许，但是目前应该没有相关监测机制。

按命名匹配删除：rm -rf /folder/2019*
按最后一次修改日期删除：find /path/file* -mtime +5 -exec rm {} \;

#### 删除海量文件
如果要删除文件夹内的部分文件，且文件夹中存在海量文件（数十万），执行上面的命令会报参数过长错误，此时有几种做法：

● find /path/dir/2019* -delete；
● 创建空文件夹deletetmp，然后rsync -a --delete deletetmp /path/dir；
● 在脚本里面写循环删除，一次删除几条。

#### 删除后空间不释放
rm命令只是把链接解除，进程仍然可以读取已删除的文件，所以并不释放磁盘空间。使用lsof | grep deleted命令查看被删除但其实未真正释放的文件，找到进程号以后杀掉进程。（对 nas 盘无效，如 nas 盘长时间不释放，联系系统管理员）

后续记得上线定期清理策略，有些中间件自带清理配置，优先改这些配置，然后再考虑写 Cron 脚本。

### CPU过高

需了解服务器的核心数量，对于几十核CPU的服务器来说，CPU占用百分之几百可能是正常现象。/proc/cpuinfo

● top命令，然后按P，查看CPU使用率高的进程；
● top -p pid -H发现问题线程 pid；
● printf '%x\n' pid将 pid 转换成16进制得到 nid；
● 在 jstack 中找到相应的堆栈信息jstack pid | grep 'nid' -C5 –color，主要关注 WAITING 、 TIMED_WAITING状态。

使用  jstat -gcutil pid 命令或可视化监控查看 GC 情况。

### 内存不足

使用free -m查看内存使用情况，注意关注 available 这列，而不是 free。


查看是否有heapdump文件生产，如没有，使用./jmap -dump:live,format=b,file=heap.hprof <pid>生产 heapdump 文件
，使用 mat(Eclipse Memory Analysis Tools)导入 dump 文件进行分析。

在 Eclipse/MemoryAnalyzer.ini 修改启动参数，在info.plist修改 java 路径。

### 应用报错

尝试在前端复现错误，使用 F12 查看报错接口信息。

特殊场景：点击打开一个新的页面，报错出现在新的页面，来不及点 F12 查看
打开开发者工具，进设置，在 Global 栏选上 Auto-open DevTools for popups。

然后在对应的大数据运维平台或服务器上查询报错日志。
大数据运维平台搜索内容尽量加引号，避免触发分词。