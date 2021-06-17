网络连接

数据库 

最后成功连接时间 ip 端口

ip 端口 最后成功连接时间 









请求缓存
  命令 参数


请求 列目录 上一个文件

sled 
  目录 
    文件编号 
      文件名 
      修改日期 
      文件大小
    -
      文件名 
      文件编号



延时 * 带宽 = 窗口大小 



// use libc::{stat as stat64, fstat as fstat64, fstatat as fstatat64, lstat as lstat64, lseek64,
// let n = cvt(unsafe { lseek64(self.0.raw(), pos, whence) })?;
// use sys::{cvt, cvt_r};
// 该软件包提供了cvt广泛使用的函数，该函数用于libstd将特定于平台的syscall错误代码转换为std::io::Result。


每个发送的包都有一个序号
每秒响应一次最新收到的序号

本地记录最新的序号
上一次确认的序号 上一次确认的时间

如果当前序号!=上一次确认的序号
延时 = (响应的序号-上一次确认的序号)/(最新的序号-上一次确认的序号) * (当前时间-上一次确认的时间) / 2
第一次收到包立刻发出ack，之后每秒发一次ack, ack会包含这一秒收到的包的总数量

序号每发送一个包会+1





指令队列
响应



请求 路径 路径编号 从什么offset开始发送文件数据（如果没有就不发送数据）
文件存在响应 路径编号(4个字节) 文件大小 最后更新时间
文件不存在响应 路径编号
响应 文件编号 offset

接收
接收
发送速度


丢失的包应答

如果接收增加，发送速度翻倍
如果接收没增加，发送速度=(发送速度+接收速度)/2+1



接收方 

  路径 缺失的offset

重发

## 参考文献

1. [OkFileTransfer](https://github.com/jinhang/OkFileTransfer)

