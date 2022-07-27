// Rust 的模式匹配吸取了函数式编程语言的优点，强大优雅且效率很高。
// 它可以用于 struct / enum 中匹配部分或者全部内容，比如上文中我们设计的数据结构 Event，可以这样匹配。

fn process_event(event: &Event) {
   match event {
       Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
       Event::Leave((uid, tid)) => println!("user {:?} left {:?}",uid,tid),
       Event::Message((_,_,msg)) => println!("broadcast: {}", msg),
   }
}

// 从代码中可以看到，可以直接对 enum 内层的数据进行匹配并赋值，这比很多只支持简单模式匹配的语言，例如 JavaScript 、Python ，可以省出好几行代码。
// 除了使用 match 关键字做模式匹配外，我们还可以用 if let / while let 做简单的匹配，如果上面的代码我们只关心 Event::Message，可以这么写：
fn process_event_message(event: &Event) {
    if let Event::Message((_,_,msg)) = event {
        println!("broadcast: {}", msg)
    }
}

// Rust 的模式匹配是一个很重要的语言特性，被广泛应用在状态机处理、消息处理和错误处理中，如果你之前使用的语言是 C / Java / Python / JavaScript ，没有强大的模式匹配支持。

