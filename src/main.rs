extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_sns::Sns;

fn main() {
    let _ = env_logger::try_init();
    let sns = rusoto_sns::SnsClient::new(rusoto_core::Region::default());
    let mut sns_input = rusoto_sns::PublishInput::default();

    sns_input.message = "test".to_string();
    sns_input.topic_arn = Some(std::env::var("TOPIC_ARN").unwrap());
    let mut attrs = std::collections::HashMap::<String, rusoto_sns::MessageAttributeValue>::new();
    attrs.insert(
        "Foo".to_string(),
        rusoto_sns::MessageAttributeValue {
            data_type: "String".to_string(),
            string_value: Some("Bar".to_string()),
            binary_value: None,
        },
    );
    sns_input.message_attributes = Some(attrs);

    match sns.publish(sns_input).sync() {
        Ok(_) => println!("ok"),
        Err(e) => eprintln!("{:?}", e),
    }
}
