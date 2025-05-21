const updateWebUrl = async (webUrl) => {
    try {
        let content = 'WebView(url: URL(string: "https://juejin.cn/")!)'
        content = content.replace(
            /WebView\(url: URL\(string: ".*?"\)!\)/,
            `WebView(url: URL(string: "${webUrl}")!)`
        )
        console.log(`✅ Updated web URL to: ${webUrl}`)
        console.log('content', content)
    } catch (error) {
        console.error('❌ Error updating web URL:', error)
    }
}

updateWebUrl('https://www.baidu.com')