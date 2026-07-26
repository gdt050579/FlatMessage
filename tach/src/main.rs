use flat_message::FlatMessage;

fn main() {}

#[derive(FlatMessage)]
#[flat_message_options(store_name = false)]
struct MyDataV2<'a> {
    a: u8,
    #[flat_message_item(mandatory = false, default = "Hello")]
    b: &'a str,
}

