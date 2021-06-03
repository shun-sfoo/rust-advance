use anyhow::Result;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

impl Encoder for u64 {
    fn encode(&self) -> Result<Vec<u8>> {
        //todo
        Ok(vec![1, 2, 3, 4, 5])
    }
}

impl Encoder for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}
pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data>
where
    Id: Encoder,
    Data: Encoder,
{
    pub fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let event = Event::new(1, "hello world".to_string());
        let _result = event.encode();
    }
}
