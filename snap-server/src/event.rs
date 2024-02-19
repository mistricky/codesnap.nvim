use serde::Serialize;

#[derive(Serialize)]
pub struct Event<T>
where
    T: Serialize,
{
    name: String,
    data: T,
}

impl<T> Event<T>
where
    T: Serialize,
{
    pub fn new(name: &str, data: T) -> Event<T> {
        Event {
            name: name.to_string(),
            data,
        }
    }
}

impl<T> Into<String> for Event<T>
where
    T: Serialize,
{
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
