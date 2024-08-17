use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

/*
param:
    value:要被序列化的区块头结构体
return：
    返回字节数组
function:
    pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
*/
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

/*
param:
    bytes:要被反序列化的字节数组
return：
    返回原结构体
function:
    pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
*/

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

/*
function:
    传入序列化以后的字节数组，返回哈希
*/
pub fn get_hash(value: &[u8], mut out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(&mut out);
}



#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
//test
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn coder_test() {
        let point = Point { x: 1, y: 2 };
        let serialized = my_serialize(&point);
        let deserialized: Point = my_deserialize(&serialized);
        assert_eq!(point, deserialized);
    }
}