use derive_screaming_snake_case::Display;

#[derive(Display)]
enum MyEnum {
    HelloWorld,
    XmlHttpRequest,
    Test,
}

#[test]
fn derive_screaming_snake_case() {
    assert_eq!(MyEnum::HelloWorld.to_string(), "HELLO_WORLD");
    assert_eq!(MyEnum::XmlHttpRequest.to_string(), "XML_HTTP_REQUEST");
    assert_eq!(MyEnum::Test.to_string(), "TEST");
}