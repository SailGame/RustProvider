use prost::Message;
use prost_types::Any;

pub fn unpack<T: Message + Default>(any: Any) -> T {
    T::decode(any.value.as_slice()).unwrap()
}

pub fn pack<T: Message>(msg: T, type_url: String) -> Any {
    let mut buf = vec![];
    msg.encode(&mut buf).unwrap();
    Any {
        // type_url seemingly doesn't matter
        type_url,
        value: buf
    }
}