# 5MB 取代 Electron，轻松快速构建多端应用

我是前端开发工程师，我的电脑配置：macbook air m3 8G 256G  
我在公司从0到1开发了crm系统(react)，小程序(uniapp)，还有游戏控制台(vue)等项目  
可是某一天老大让我开发 windows 软件？OK，我装了 Parallels Desktop + windows11，学了 flutter，配置了 dart + vs studio + c++环境，安装了 Android Studio 开发工具，硬盘-100G。  
过了一个月，老大让我开发 iphone + iwatch 程序？OK，安装了 xcode，配置了 ios + iwatch 模拟器，学习了 swiftui，硬盘-128G。  
一个月后，老大又让我将我们的 crm 系统做成跨平台应用？OK，我开始学习调研 Electron+tauri+Pake 等。  
一个月后，老大又让我用 Cocos 开发抖音直播小游戏？OK，我装了抖音开发者工具，安装了 CocosDashBoard + 直播伴侣 + vs studio 环境 + python + Pycharm，然而使用 Cocos 打包 windows 一直报错，无奈只能打包为 html+js，然后使用 tauri 包裹一下。  
可是突然某一天我意识到了哪里不对劲，我应聘的是前端开发工程师啊！而且为啥我的微信提示：硬盘空间不足？啊啊啊啊啊，硬盘只剩 300M 不到......

![](https://self.pakeplus.com/dev1.png)

我开始思考：
-   我的硬盘空间都被哪些程序占用了？各种开发环境...
-   为什么我前端工程师要学那么多语言？因为跨平台开发需求...
-   为什么老大拒绝给我配置高配置电脑？因为老大穷...呸，是公司穷...
-   有没有免费快速将网站变成跨平台应用还不用装环境的服务？没有...
-   有没有用 html+js 就可以开发跨平台应用技术？electron/tauri/Pake 等
-   看了 tauri，但是要 rust 环境和学习 rust？rust 依赖也太大了吧...
-   有没有什么办法不需要安装 electron/tauri 也可以开发跨平台应用？无...
-   我可不可以把老大开除了，或者换个老板？不可以，现在行情不好...

可是我现在非常迫切需要不依赖安装任何环境，就可以打包网站为跨平台应用的工具或网站，而且还要体积小，性能好，支持中文名称打包，而且还要支持使用 vue/react 等项目也可以开发跨平台应用的东西，我该怎么办？谁能救救我的电脑硬盘，无奈搜遍整个互联网，也没有人为我发声......

> 如果前方无路，那我们就走出一条路

## 一、PakePlus 诞生

没办法了，只能我自己出手了，我会 tauri + js + github + python + rust + kotlin + swiftui + flutter 等，并参考Pake项目打包流程，其实打包核心只是需要一个环境而已，我之前的 tauri 项目依托在 github 服务器上不是就可以打包？那我为什么非要在本地安装 tauri 依赖呢？我的 ios 项目也可以依赖 github 来打包吗？那 android 项目肯定也可以吧？整个过程可以实现自动化化吗？带着这些疑问，我做了一个最小化 demo 验证，完全可行！在这里还要非常感谢 github 提供的服务器，真的是太方便了！（美酒虽好，但是不可贪杯，小心闯祸，后文会讲）。  
PakePLus 只需要几分钟，不需要配置任何依赖环境，就可以将你的 web 项目打包成 Windows、Linux、MacOS 和 Android、iOS 等平台的应用程序，并且体积仅为 5MB。

![PakePlus](https://self.pakeplus.com/p1.jpg)

> PakePlus 官网：https://www.pakeplus.com  
> GitHub：https://github.com/Sjj1024/PakePlus

### 快速使用

下载安装 PakePlus 或使用 PakePLus 网页版，填入 Github token，创建项目，填写要打包的 web 地址或上传静态文件，点击发布，选择打包平台，等待打包完成，下载打包后的安装包，即可使用。并且有详细的使用指南：https://pakeplus.com/guide/

![](https://self.pakeplus.com/pp12.gif)

### 技术原理

其实 PakePlus 之所以能将 web 项目打包为跨平台应用，其内部的核心就是使用了强大的 webview，tauri 内部其实是使用了封装好的 wry 库，wry 库是一个基于 Rust 语言开发的 webview 库，它可以让我们在 Rust 中轻松地创建和管理 webview，并且支持多种平台，包括 Windows、Linux、macOS、Android 和 iOS 等。

```rust
unstable_struct!(
  #[doc = "A builder for a webview."]
  struct WebviewBuilder<R: Runtime> {
    pub(crate) label: String,
    pub(crate) webview_attributes: WebviewAttributes,
    pub(crate) web_resource_request_handler: Option<Box<WebResourceRequestHandler>>,
    pub(crate) navigation_handler: Option<Box<NavigationHandler>>,
    pub(crate) on_page_load_handler: Option<Box<OnPageLoad<R>>>,
    pub(crate) download_handler: Option<Arc<DownloadHandler<R>>>,
  }
);
```

对于桌面端默认很多操作系统已经默认安装了 webview，所以体积可以控制非常小，这也是我喜欢 tauri 的原因。但是使用 tauri 编译为移动端应用后，一个最简单的 demo 竟然要 39.5M，这是我体积洁癖不能接受的。所以我决定使用原生来支持移动端，其实内部还是使用了 webview。  
android 部分代码如下

```kotlin
<?xml version="1.0" encoding="utf-8"?>
<androidx.constraintlayout.widget.ConstraintLayout xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:app="http://schemas.android.com/apk/res-auto"
    xmlns:tools="http://schemas.android.com/tools"
    android:id="@+id/ConstraintLayout"
    android:layout_width="match_parent"
    android:layout_height="match_parent">

    <WebView
        android:id="@+id/webview"
        android:layout_width="match_parent"
        android:layout_height="match_parent" />
</androidx.constraintlayout.widget.ConstraintLayout>
```

ios 部分代码如下

```swift
import SwiftUI
import WebKit

struct WebView: UIViewRepresentable {
    let url: URL
    let debug = false

    func makeUIView(context: Context) -> WKWebView {
        let webView = WKWebView()
        // load url
        webView.load(URLRequest(url: url))
    }
    //此处省略1万行....
}
```

### 架构优化

![](https://self.pakeplus.com/pp13.png)

最初 PakePLus 的打包流程是依靠自身 main 分支来打包，新项目也基于此分支进行编辑修改，然后使用 github action 打包发布，但是这实在是太绕了，相当于无限套娃一样，我自己都被绕晕了...后来赶上 tauri2 发布，我也重新整理了架构流程，并且考虑了移动端支持的逻辑，现在的打包流程已经非常清晰，所有用户填入 github token 之后，都会默认 fork 一份 PakePLus 仓库，并包含所有分支，然后用户在自己的仓库中进行创建项目并编辑修改，并根据是 web/dist/中文名称等逻辑，使用自动化脚本自由完成项目配置，最后触发 github action 进行打包发布。

```js
const main = async () => {
    const { name, showName, version, webUrl, id, pubBody, debug } = ppconfig.ios
    // Update app name if provided
    await updateAppName(showName)
    // Update web URL if provided
    await updateWebUrl(webUrl)
    // update debug
    await updateDebug(debug)
    // update android applicationId
    await updateBundleId(id)
    // set github env
    setGithubEnv(name, version, pubBody)
    // success
    console.log('✅ Worker Success')
}

// run
;(async () => {
    try {
        console.log('🚀 worker start')
        await main()
        console.log('🚀 worker end')
    } catch (error) {
        console.error('❌ Worker Error:', error)
    }
})()
```

当然现在的 PakePLus 还有很多不足，还处于快速迭代升级阶段，后续会继续优化升级，并且会支持更多的功能，比如：本地打包，摆脱 github 环境，秒级打包，支持所有 tauri2 api 以及自定义 api 等。

### 技术对比

<!-- ![技术对比](https://pakeplus.com/assets/publish3.d0EgpzGe.webp) -->

| 特性           | PakePlus | Tauri  | Electron | Pake   |
| -------------- | -------- | ------ | -------- | ------ |
| **依赖环境**   | 无需     | 需要   | 需要     | 需要   |
| **开发复杂度** | 简单     | 复杂   | 复杂     | 中等   |
| **打包速度**   | 分钟级   | 小时级 | 小时级   | 分钟级 |
| **安装体积**   | 10MB     | 2GB+   | 2GB+     | 2GB+   |
| **支持移动端** | ✅       | 部分   | ❌       | ❌     |
| **支持中文名**   | ✅       | ❌     | ✅       | ❌     |
| **图形化界面** | ✅       | ❌     | ❌       | ❌     |

## 二、PakePlus 闯祸

就在 2025 年 4 月 30 号我好不容易加上了 HelloGithub 站长的微信，并让  PakePLus  成功被 HelloGithub 推荐，然后接的私活也交付了，而且也是 4 月的最后两天，我的股票收益也超上证指数 14.01%，并在 29 号尾盘全部抛出。非常清晰记得，那天我开心了一整天，上扬的嘴角比 AK47 还难压。

### 被封

可是第二天早上醒来，我的天塌了，整个世界都是灰色的，因为我的 Github 账号被封了...

![](https://self.pakeplus.com/github2.jpg)

### 追诉

作为一个开源爱好者并重度依赖 github 的程序猿来讲，github 比我上大学的女朋友还上瘾，但是现在它把我拉黑删除了 😭 我真的不知道该怎么面对接下来的工作。然后我开始紧急做舔狗，开始和 github 联系，但是......为什么在联系方式里面没有中国手机号啊？没事没事，它虐我千百遍，我依然对它像初恋。我找了很多方式，最后还是用了我日本同事的手机号，在此感谢我的常总！  
HelloGithub 站长也非常热心的帮我，我真的非常感动，说实在的，作为一个刚加上好友不到两天的陌生网友，HelloGithub 站长就像对待老朋友一样非常用心帮助我，还通过自己的频道单独和 github 联系！

![](https://self.pakeplus.com/github4.jpg)

但是那些天好像刚好是国外节日，并且我们也遇上了难得的五一假期，索性就把这些超过我能力范围的事情放下了，开始和女朋友享受美好的五一假期了

![](https://self.pakeplus.com/github5.jpg)

### 解封

能走到这一步，那是真不容易啊，比西天取经还难......
其实原本想着过了五一之后，就可以收到 github 的回复，然后对症下药了，结果五一后上班两天了，还没有动静，此时我已经注册了 github 小号，真不行就用小号苟且活着。但是好在 HelloGithub 站长有专属催单通道，在 5 月 12 号晚上 22 点 11 催单，10 分钟后就有回复说接单了，凌晨 00:34 终于收到了 github 谅解通知，我又回来了！究其原因：因为的 PakePlus 项目被太多人无限制使用，导致我的 github 接口被疯狂调用，结果就是我的 Github 断更了 8 天......

![](https://self.pakeplus.com/github6.jpg)

## 三、感恩

感恩我的 HelloGithub 朋友，虽然我们从未见过面，也不知道你家住何方，更没有利益关系，但是我知道，没有你我就不会这么快被 github 解封，没有你，就不会带给 PakePLus 这么大的流量，没有你，开源背后的热情与故事就不会被传递。

感谢我的日本同事常总，没有你，我将不会那么快通过 github 的手机验证。

感谢阮一峰技术周刊：感谢阮一峰的技术周刊推荐，虽然我没接住这波破天的流量(被推荐期间正被 github 封禁)

我的朋友，有你们在真好！