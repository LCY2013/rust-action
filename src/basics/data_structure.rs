// 《Rust 数据结构》
// 数据结构是程序的核心组成部分，在对复杂的问题进行建模时，我们就要自定义数据结构。
// Rust 非常强大，可以用 struct 定义结构体，用 enum 定义标签联合体（tagged union），还可以像 Python 一样随手定义元组（tuple）类型。

// 定义一个聊天服务的数据结构

#[derive(Debug)]
pub(crate) enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct UserId(u64);

#[derive(Debug, Copy, Clone)]
pub(crate) struct TopicId(u64);

#[derive(Debug)]
pub(crate) struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
pub(crate) struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 定义聊天室发生的事件
#[derive(Debug)]
pub(crate) enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello, World!".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );
}

// Gender：一个枚举类型，在 Rust 下，使用 enum 可以定义类似 C 的枚举类型
// UserId/TopicId ：struct 的特殊形式，称为元组结构体。它的域都是匿名的，可以用索引访问，适用于简单的结构体。
// User/Topic：标准的结构体，可以把任何类型组合在结构体里使用。
// Event：标准的标签联合体，它定义了三种事件：Join、Leave、Message。每种事件都有自己的数据结构。

// 在定义数据结构的时候，我们一般会加入修饰，为数据结构引入一些额外的行为。在 Rust 里，数据的行为通过 trait 来定义，后续我们会详细介绍 trait，你现在可以暂时认为 trait 定义了数据结构可以实现的接口，类似 Java 中的 interface。
// 一般我们用 impl  关键字为数据结构实现 trait，但 Rust 贴心地提供了派生宏（derive macro），可以大大简化一些标准接口的定义，比如  #[derive(Debug)] 为数据结构实现了 Debug trait，提供了 debug 能力，这样可以通过  {:?}，用  println! 打印出来。
// 在定义 UserId / TopicId 时我们还用到了 Copy / Clone 两个派生宏，Clone 让数据结构可以被复制，而 Copy 则让数据结构可以在参数传递的时候自动按字节拷贝。
