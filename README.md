# clipboard-recorder
[简体中文](README.md) | [English](README-en.md)

    
在Mac上通过自定义快捷键暂存你的剪贴板文本并且唤起弹窗快速使用剪贴板历史。

![demo](https://github.com/Dramalf/clipboard-recorder/assets/43701793/f82e67b6-71a4-4e35-b3da-cabc04ad01b5)
## 先告诉你

这个脚本通过rust实现，需要配合系统快捷键配置使用。理论上是跨平台的，但windows自带了这个功能，所以没必要

因为不支持重复的快捷键以及中断回调，这个工具使用起来有点繁琐，并不好用😅 


## 下载
从  [Releases page](https://github.com/Dramalf/clipboard-recorder/releases) 下载`clipboard-recorder`的可执行文件

    
## 创建剪贴板暂存能力快捷键

打开 `自动操作` 创建一个`快速操作`.
![Quick Action](https://github.com/Dramalf/clipboard-recorder/assets/43701793/86ef2b6f-3201-49cd-add6-e8106217135b)

选择 `运行AppleScript脚本` or `运行shell脚本`。
输入指令 `<your-path-to>/clipboard-recorder --save` 并保存。
![12](https://github.com/Dramalf/clipboard-recorder/assets/43701793/32218180-e194-4d23-94c2-3a2f7eae066b)

打开 `系统偏好 > 键盘 > 快捷键 > 服务`.
找到你创建的`快速操作`并绑定快捷键，例如 `command+]`

## 创建唤起剪贴板历史弹窗快捷键

与`创建剪贴板暂存能力快捷键`相似，创建`快速操作`，选择 `运行AppleScript脚本` or `运行shell脚本`。
输入指令 `<your-path-to>/clipboard-recorder` 

绑定快捷键，例如 `command+]`.


