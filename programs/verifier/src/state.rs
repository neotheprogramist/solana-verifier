use utils::{AccountCast, BidirectionalStack};

use crate::error::VerifierError;

const CAPACITY: usize = 65536;
const LENGTH_SIZE: usize = 2;

/// Define the type of state stored in accounts
#[derive(Debug)]
pub struct BidirectionalStackAccount {
    pub front_index: usize,
    pub back_index: usize,
    pub buffer: [u8; CAPACITY],
}
impl Default for BidirectionalStackAccount {
    fn default() -> Self {
        Self {
            front_index: 0,
            back_index: CAPACITY,
            buffer: [0; CAPACITY],
        }
    }
}

impl AccountCast for BidirectionalStackAccount {}

impl BidirectionalStack for BidirectionalStackAccount {
    type Error = VerifierError;

    fn push_front(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        for byte in data {
            self.buffer[self.front_index] = *byte;
            self.front_index = self.front_index.saturating_add(1);
        }

        let data_length = data.len();
        for i in 0..LENGTH_SIZE {
            self.buffer[self.front_index] = ((data_length >> (i * 8)) & 0xFF).try_into()?;
            self.front_index = self.front_index.saturating_add(1);
        }

        Ok(())
    }

    fn push_back(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        for byte in data.iter().rev() {
            self.back_index = self.back_index.saturating_sub(1);
            self.buffer[self.back_index] = *byte;
        }

        let data_length = data.len();
        for i in 0..LENGTH_SIZE {
            self.back_index = self.back_index.saturating_sub(1);
            self.buffer[self.back_index] = ((data_length >> (i * 8)) & 0xFF).try_into()?;
        }

        Ok(())
    }

    fn pop_front(&mut self) {
        let mut data_length = 0_usize;
        for _ in 0..LENGTH_SIZE {
            self.front_index = self.front_index.saturating_sub(1);
            let x: usize = self.buffer[self.front_index].into();
            data_length = (data_length << 8) | x;
        }

        self.front_index = self.front_index.saturating_sub(data_length);
    }

    fn pop_back(&mut self) {
        let mut data_length = 0_usize;
        for _ in 0..LENGTH_SIZE {
            let x: usize = self.buffer[self.back_index].into();
            data_length = (data_length << 8) | x;
            self.back_index = self.back_index.saturating_add(1);
        }

        self.back_index = self.back_index.saturating_add(data_length);
    }

    fn borrow_front(&self) -> &[u8] {
        let mut data_length = 0_usize;
        for i in 1..=LENGTH_SIZE {
            let x: usize = self.buffer[self.front_index.saturating_sub(i)].into();
            data_length = (data_length << 8) | x;
        }

        &self.buffer[self.front_index.saturating_sub(data_length + LENGTH_SIZE)
            ..self.front_index.saturating_sub(LENGTH_SIZE)]
    }

    fn borrow_back(&self) -> &[u8] {
        let mut data_length = 0_usize;
        for i in 0..LENGTH_SIZE {
            let x: usize = self.buffer[self.back_index.saturating_add(i)].into();
            data_length = (data_length << 8) | x;
        }

        &self.buffer[self.back_index.saturating_add(LENGTH_SIZE)
            ..self.back_index.saturating_add(LENGTH_SIZE + data_length)]
    }

    fn borrow_mut_front(&mut self) -> &mut [u8] {
        let mut data_length = 0_usize;
        for i in 1..=LENGTH_SIZE {
            let x: usize = self.buffer[self.front_index.saturating_sub(i)].into();
            data_length = (data_length << 8) | x;
        }

        &mut self.buffer[self.front_index.saturating_sub(data_length + LENGTH_SIZE)
            ..self.front_index.saturating_sub(LENGTH_SIZE)]
    }

    fn borrow_mut_back(&mut self) -> &mut [u8] {
        let mut data_length = 0_usize;
        for i in 0..LENGTH_SIZE {
            let x: usize = self.buffer[self.back_index.saturating_add(i)].into();
            data_length = (data_length << 8) | x;
        }

        &mut self.buffer[self.back_index.saturating_add(LENGTH_SIZE)
            ..self.back_index.saturating_add(LENGTH_SIZE + data_length)]
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{BidirectionalStackAccount, CAPACITY};
    use utils::BidirectionalStack;

    #[test]
    fn test_default() {
        let stack = BidirectionalStackAccount::default();
        assert_eq!(stack.front_index, 0);
        assert_eq!(stack.back_index, CAPACITY);
        assert_eq!(stack.buffer, [0; CAPACITY]);
    }

    #[test]
    fn test_push_front_and_borrow_front() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data to front
        let data = [1, 2, 3, 4];
        stack.push_front(&data).unwrap();

        // Borrow and verify
        let borrowed = stack.borrow_front();
        assert_eq!(borrowed, &[1, 2, 3, 4]); // Data is stored in reverse
    }

    #[test]
    fn test_push_back_and_borrow_back() {
        let mut stack = BidirectionalStackAccount::default();

        // Push data to back
        let data = [1, 2, 3, 4];
        stack.push_back(&data).unwrap();

        // Borrow and verify
        let borrowed = stack.borrow_back();
        assert_eq!(borrowed, &[1, 2, 3, 4]); // Data is stored in reverse
    }
}
