use lazy_static::lazy_static;
use std::collections::HashMap;


macro_rules! hash {
    ($($key:expr, $val:expr)*) => {
        HashMap::from([$($key,$val),*]);
    };
    ($key:expr => $val:expr) => {
        HashMap::from([($key, $val)]);
    };
    ($($key:expr => $val:expr),* $(,)?) => {
        HashMap::from([$(($key,$val),)*])
    };
}



macro_rules! static_hash {
    ($ident:ident : ($key_type:ty => $val_type:ty) = {$($key:expr => $val:expr),* $(,)?}) => {
            lazy_static! {
            static ref $ident: HashMap<$key_type, $val_type>= HashMap::from([$(($key,$val),)*]);
            }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_map_macro() {
        let my_map2 = hash!(("a", 1), ("b", 2));
        let my_pair2 = hash! {"key" => 2};
        let my_dict2 = hash! {
            "a" => 1,
            "b" => 2,

        };

        println!("{:?}", my_map2);
        println!("{:?}", my_pair2);
        println!("{:?}", my_dict2);
    }
}
