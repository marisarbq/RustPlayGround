use serde_json::json;

//尝试实现一个宏来处理 AVG 演出脚本
macro_rules! avg {
    ($(@$command:ident $target:ident $($keys:ident=$vals:ident)*)*) => {
        {
            let group = json!([
            $(
                {
                    "name": stringify!($command),
                    "target": stringify!($target),
                    "attr": {
                        $(
                            stringify!($keys):stringify!($vals),
                        )*
                    }
                },
            )*
            ]);
            group
        }
    };
}

fn main() {
    println!("Hello, world!");
    let srt = avg!(
        @command BG1 key1=v1 key2=v2
        @in BG2 x=s100 y=s100
    );

    println!("{:#?}", srt);
}
