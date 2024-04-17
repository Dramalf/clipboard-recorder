use clipboard::{ClipboardContext, ClipboardProvider};
use iced::widget::{button, container, scrollable, text, Column};
use iced::{ window, Application, Command, Element, Length, Settings, Size, Theme};

#[derive(Debug, Clone)]
enum Message {
    ItemClicked(String),
}

struct ListComponent {
    items: Vec<String>, // 存储列表项
}
fn copy_to_clipboard(text_to_copy: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text_to_copy.to_owned()).unwrap();
}
impl Application for ListComponent {
    // 创建带有初始值的组件
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Vec<String>;
    type Theme = Theme;
    fn title(&self) -> String {
        String::from("Text List Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // 匹配修改后的消息
            Message::ItemClicked(clicked_item) => {
                // 在这里，你现在可以知道哪个项目被点击，并可以据此作出相应的逻辑处理
                println!("Clicked item: {}", clicked_item);
                // 例如，此处可以直接退出程序或执行其他逻辑
                copy_to_clipboard(clicked_item.as_str());
                std::process::exit(0);
            }
        }
    }

    fn new(initial_items: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                items: initial_items,
            },
            Command::none(),
        )
    }

    // 生成组件的视图
    fn view(&self) -> Element<Message> {
        let mut column = Column::new().spacing(2);

        for item in &self.items {
            let btn = button(text(item.clone().replace("\t", "    ")))
                .width(Length::Fill)
                .on_press(Message::ItemClicked(item.clone()));
            let wrapper = container(btn).width(400).max_height(100);

            column = column.push(wrapper);
        }

        scrollable(column).into()
    }
}

pub fn show(history: Vec<String>) {
    // 初始值作为启动参数传递给 App
    let mut s = Settings::with_flags(history);
    s.window = window::Settings {
        size: Size::new(400.0, 300.0), // 定义窗口的大小
        resizable: true,               // 允许窗口大小调整

        ..window::Settings::default() // 使用其它默认设置
    };
    let _ = ListComponent::run(s);
}
