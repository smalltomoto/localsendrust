# Tauri 安装流程

## 1.安装 Rust（如果已安装，执行 rustup update 更新）

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup update 更新到最新的版本

## 2.安装node.js

https://nodejs.org/zh-cn安装LTS 版本

## 3.安装 Tauri

参考Tauri 官方文档

import { invoke } from "@tauri-apps/api/tauri";
const result = await invoke("greet", { name: "Vue + Tauri" });
console.log(result);

## 4.选择前端的构建类型

构建前端类型

**npm create vite@latest . --template vue-ts**

安装vite插件

**npm install @vitejs/plugin-vue --save-dev**

**npm install vue@next --save**

## git服务代码

## 1.切换到主分支，创建副本分支dev

```sh
git init  初始化一个空仓库
git switch main
git checkout -b dev

git push -u origin dev //将本地的dev分支的所有内容推送到远程仓库

git add .  //add . 跟踪所有文件，随后执行 提交内容
git commit -am "test" // -am 上传到本地git仓库 ，冒号是上传描述信息

git pull //从远程仓库拉取下来，先同步修改内容，push的习惯前置操作
git push //推送到远程仓库

## 手动添加忽略列表

# 忽略所有编译文件
*.o
*.a
*.so
*.dll
*.exe

# 忽略 IDE 生成的文件（例如 VSCode 和 JetBrains）
.vscode/
.idea/
*.iml

# 忽略日志和临时文件
*.log
*.tmp
*.bak

# 忽略 node_modules 目录（如果是前端项目）
node_modules/

# 忽略 Rust 相关的 target 目录
target/

# 忽略 Qt 相关的 build 目录
build*/

# 忽略 Tauri 生成的 dist 目录
src-tauri/target/
```

| 界面     | 主要功能                     | 关键控件               |
| -------- | ---------------------------- | ---------------------- |
| 主界面   | 设备列表、连接设备、传输记录 | 列表、卡片、按钮       |
| 发送文件 | 选择设备、选文件、文件传输   | 拖拽上传、表格、进度条 |
| 接收文件 | 显示文件进度、更改存储路径   | 列表、进度条、按钮     |
| 传输记录 | 查看已发送 & 已接收文件      | 表格、搜索、筛选       |
| 设置     | 更改设备名称、存储路径       | 输入框、文件选择、开关 |

## 开放局域网的端口：

```ts
export default defineConfig({
  plugins: [vue()],
  server: {
    host: "0.0.0.0", // 监听所有 IP
    port: 5173,      // 自定义端口（可选）
  }
```

## 前端业务逻辑：

首先是： components 下的页面vue文件独立分割开来，每一个页面都有一个vue文件，统一集中汇总在主页面App.vue ，在将主页面挂载到同一目录的显示界面 index.html

```html
<!DOCTYPE html>
<html lang="zh">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Tauri 文件传输</title>
</head>
<body>
  <div id="app"></div>  <!-- Vue 组件会挂载到这里 -->
  <script type="module" src="/src/main.ts"></script>
</body>
</html>

Demo 学习实例
//父子组件数据更新和数据流向
学习如何引入 ：Tailcss 
页面需要做的： 下载svg icon 引入高清小图
数据获取加上loading动图
```

### vue 的学习笔记

本质上还是写HTML.

```vue
<template>  
#其中可以放置你所需要的控件 大小标题，按钮这类的东西
<h1 class="text-2xl font-bold mb-4">局域网文件传输</h1>
其中的class是代表 这个标题的属性效果
</template>

生命周期函数
onBeforeMount：与 beforeMount 类似，组件挂载之前调用。
onMounted：与 mounted 类似，组件挂载后调用。
onBeforeUpdate：与 beforeUpdate 类似，组件数据更新之前调用。
onUpdated：与 updated 类似，组件数据更新后调用。
onBeforeUnmount：与 beforeUnmount 类似，组件销毁前调用。
onUnmounted：与 unmounted 类似，组件销毁后调用。

前端样式不能过度相信ai
windows下面需要频繁起服务
```

## 后端rust业务逻辑

**ip 逻辑需要更改，不是设备序号 而是数据**

tokio::spawn() : 异步并发环境下如果想实现数据自增而避免数据竞争->多个线程同时修改一个变量需要使用：

usestd::sync::atomic::{AtomicUsize, Ordering};

static SORTING:AtomicUsize=AtomicUsize::new(1);

letid= SORTING.fetch_add(1, Ordering::SeqCst);

    .invoke_handler(tauri::generate_handler![

    commands::network::scan_devices

    ])

后端的rust函数注册方式

**学习自己写一个rust链表** **rust入门检测**

```rust
async fn check_online(addr: SocketAddr) ->bool {
    TcpStream::connect(addr).await.is_ok()
}
async 异步函数，返回值是一个future， 使用返回值必须加上await ：
✅ 等待异步操作完成
✅ 不阻塞线程（不同于 std::thread::sleep）
✅ 提高并发效率


方案	是否线程安全	Rust 是否允许	是否推荐	适用场景
let mut sorting = 1; sorting += 1;	❌ 不是	❌ 不允许（多任务数据竞争）	🚫 不推荐	只适用于单线程
static SORTING: AtomicUsize	✅ 是	✅ 允许，但需要手动 store(1, Ordering::SeqCst)	🔶 适中	适用于全局计数
let sorting = AtomicUsize::new(1); &sorting	✅ 是	❌ 不允许（生命周期错误）	🚫 不推荐	变量生命周期短
let sorting = Arc::new(AtomicUsize::new(1)); sorting.clone()	✅ 是	✅ 允许（最优解）	✅ 推荐	适用于多任务并发
✅ 推荐使用 Arc<AtomicUsize>，可以安全地在 tokio::spawn 任务中共享递增 id！ 🚀

### invoke 命令使用方式
1.

## 开发反向：

1.先遍历局域网下同一个网段的所有设备，然后再发现安装了本软件的所有设备

```
