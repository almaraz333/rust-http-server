use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        
        let mut data = HashMap::new();

        for sub_str in s.split("&") {
            let mut key = sub_str;
            let mut value = "";

           if let Some( i) = sub_str.find("=") {
                let (left,right) = sub_str.split_at(i);

                key = left;
                value = &right[1..];
           }

           data.entry(key).and_modify(|val| match val{
                Value::Single(prev_val) => {
                    *val = Value::Multiple(vec![value, prev_val]);
                },
                Value::Multiple(vec) => vec.push(value)
           }).or_insert(Value::Single(value));

           
        }

        QueryString { data }
    }
}