trait UI {
    fn render(&self) {
        println!("Rendering UI");
    }
}

struct Button {
    label: String,
}

impl UI for Button {}

// 作用3 讨论一个有未知大小的类型

struct Container {
    name: String,
    child: Box<Container>,
}

fn main() {
    let button_a = Button {
        label: String::from("Click me"),
    };
    let button_b = Box::new(Button {
        label: String::from("Click me"),
    });

    //作用1 避免在转移所有权时复制大量数据
    let button_c = button_a;
    let button_d = button_b;

    // button_a.render();

    //作用2 与trait对象一起使用
    let components: Vec<Box<dyn UI>> = vec![Box::new(button_c), button_d];
}
