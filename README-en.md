# clipboard-recorder
[简体中文](README.md) | [English](README-en.md)

    
Store text from clipboard temporarily and call a history management popup with just a combination of custom keyboard shortcuts.

![demo](https://github.com/Dramalf/clipboard-recorder/assets/43701793/f82e67b6-71a4-4e35-b3da-cabc04ad01b5)
## You should know

This tool is implemented in Rust and requires configuration with system keyboard shortcuts for use.

But it is a little bit hard to use now😅 


## Download
Download the latest release version of the clipboard-recorder executable file from the [Releases page](https://github.com/Dramalf/clipboard-recorder/releases) on GitHub.

    
## Create Text Storage Shortcut

Open `Automator` on your Mac and then create a new `Quick Action`.
![Quick Action](https://github.com/Dramalf/clipboard-recorder/assets/43701793/86ef2b6f-3201-49cd-add6-e8106217135b)

Choose `Run AppleScript` or `Run Shell Script` as the action.
Enter the command `<your-path-to>/clipboard-recorder --save` to save the clipboard content.Save your Quick Action.
![12](https://github.com/Dramalf/clipboard-recorder/assets/43701793/32218180-e194-4d23-94c2-3a2f7eae066b)

Open `System Preferences > Keyboard > Shortcuts > Services`.
Find your newly created service and bind it to a new shortcut, such as `command+]`.

## Create History Management Popup Shortcut

Again, open Automator and create a new Quick Action.Choose to Run AppleScript or Run Shell Script.

Enter the command  `<your-path-to>/clipboard-recorder` to activate the history management popup.

Save and go to `System Preferences > Keyboard > Shortcuts > Services`.
Assign a shortcut to this action, for example, `command+shift+v`.


