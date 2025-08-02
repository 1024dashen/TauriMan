console.log('custom.js')

function hideError() {
    // 隐藏logo
    if (document.querySelector('.ydLogo')) {
        document.querySelector('.ydLogo').style.display = 'none'
    }
    // 隐藏barEntrance
    if (document.querySelector('.barEntrance')) {
        document.querySelectorAll('.barEntrance').forEach((element) => {
            if (
                element.innerHTML.includes('新手指导') ||
                element.innerHTML.includes('在线客服')
            ) {
                element.style.display = 'none'
            }
        })
    }
    // 隐藏查看按钮
    if (document.querySelector('tr > td > a')) {
        console.log('has actionBtns')
        const actionBtns = document.querySelectorAll('tr > td > a')
        console.log('actionBtns', actionBtns)
        actionBtns.forEach((element) => {
            if (element.innerText.includes('查看')) {
                element.style.display = 'none'
            }
        })
    } else {
        console.log('no actionBtns')
    }
    // 隐藏打印按钮
    if (document.querySelector('.hollowThemeButton')) {
        const printBtn = document.querySelectorAll('.hollowThemeButton')
        printBtn.forEach((element) => {
            if (element.innerText.includes('打印')) {
                element.style.display = 'none'
            }
        })
    }
}

window.addEventListener('DOMContentLoaded', () => {
    console.log('DOMContentLoaded')
    hideError()
    const targetNode = document.body
    // 配置观察选项
    const config = {
        childList: true,
        subtree: true,
    }
    const observer = new MutationObserver((mutationsList, observer) => {
        for (const mutation of mutationsList) {
            if (mutation.type === 'childList') {
                hideError()
            }
        }
    })
    observer.observe(targetNode, config)
})

// 监听点击事件
const hookClick = (e) => {
    console.log('click a')
    const origin = e.target.closest('a')
    if (origin && origin.href && origin.target === '_blank') {
        e.preventDefault()
        console.log('handle origin', origin)
        location.href = origin.href
    } else {
        console.log('not handle origin', origin)
    }
}

document.addEventListener('click', hookClick, { capture: true })
