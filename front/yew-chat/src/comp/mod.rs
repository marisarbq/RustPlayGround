use yew::prelude::*;

//Hook他来了！
//https://yew.rs/zh-Hans/docs/concepts/function-components/attribute
//早就说了这玩意比魔法管用多了！

pub mod chat;

#[derive(Clone, PartialEq, Properties)]
pub struct TestClickProps {
    #[prop_or_default]
    pub count: i64,
}

#[function_component(TestClick)]
pub fn test_click(props: &TestClickProps) -> Html {
    let TestClickProps { count } = props;

    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div onclick={onclick}>
               {format!("
               父组件透传:{:#?}
               当前组件:{:#?}",
               count, *counter)}
        </div>
    }
}
