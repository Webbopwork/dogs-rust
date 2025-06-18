use crate::*;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

pub trait CheckableDog {
    fn simple_peek_data(&self, size: usize) -> io::Result<(Vec<u8>, usize)>;
    fn simple_peek_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize)>;
    fn simple_peek_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)>;
    fn simple_get_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)>;
    fn peek_checker_mono(&self) -> io::Result<(Vec<u8>, u64, u64)>;
    fn get_checker_mono(&self) -> io::Result<(Vec<u8>, u64, u64)>;
    fn peek_checker_duo(&self) -> io::Result<(Vec<u8>, u64, u64, u64)>;
    fn get_checker_duo(&self) -> io::Result<(Vec<u8>, u64, u64, u64)>;
}

impl CheckableDog for Dog {
    fn simple_peek_data(&self, size: usize) -> io::Result<(Vec<u8>, usize)> {
        let (data, buf_size, _addr) = self.peek_data(size)?;
        Ok((data, buf_size))
    }
    fn simple_peek_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let (data, buf_size, _addr) = self.peek_data_skip(size, skip)?;
        Ok((data, buf_size))
    }
    fn simple_peek_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let (data, buf_size, _addr) = self.peek_all_data_skip(skip)?;
        Ok((data, buf_size))
    }
    fn simple_get_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let (data, buf_size, _addr) = self.get_all_data_skip(skip)?;
        Ok((data, buf_size))
    }

    fn peek_checker_mono(&self) -> io::Result<(Vec<u8>, u64, u64)> {
        DataCheck::peek_checker_mono(self)
    }
    fn get_checker_mono(&self) -> io::Result<(Vec<u8>, u64, u64)> {
        DataCheck::get_checker_mono(self)
    }
    fn peek_checker_duo(&self) -> io::Result<(Vec<u8>, u64, u64, u64)> {
        DataCheck::peek_checker_duo(self)
    }
    fn get_checker_duo(&self) -> io::Result<(Vec<u8>, u64, u64, u64)> {
        DataCheck::get_checker_duo(self)
    }
}

impl CheckableDog for ConnectedDog {
    fn simple_peek_data(&self, size: usize) -> io::Result<(Vec<u8>, usize)> {
        self.peek_data(size)
    }
    fn simple_peek_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        self.peek_data_skip(size, skip)
    }
    fn simple_peek_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        self.peek_all_data_skip(skip)
    }
    fn simple_get_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        self.get_all_data_skip(skip)
    }

    fn peek_checker_mono(&self) -> io::Result<(Vec<u8>, u64, u64)> {
        DataCheck::peek_checker_mono(self)
    }
    fn get_checker_mono(&self) -> io::Result<(Vec<u8>, u64, u64)> {
        DataCheck::get_checker_mono(self)
    }
    fn peek_checker_duo(&self) -> io::Result<(Vec<u8>, u64, u64, u64)> {
        DataCheck::peek_checker_duo(self)
    }
    fn get_checker_duo(&self) -> io::Result<(Vec<u8>, u64, u64, u64)> {
        DataCheck::get_checker_duo(self)
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct DataCheck {

}

impl DataCheck {
    pub fn as_le_bytes<T: Hash>(t: &T) -> [u8; 8] {
        calculate_hash(t).to_le_bytes()
    }

    pub fn add_checker_mono(data: &[u8]) -> Vec<u8> {
        [&Self::as_le_bytes(&data)[..], data].concat().to_owned()
    }

    pub fn add_checker_duo(data: &[u8]) -> Vec<u8> {
        let hash_bytes = &Self::as_le_bytes(&data)[..];
        [hash_bytes, hash_bytes, data].concat().to_owned()
    }

    fn peek_checker_mono<T: CheckableDog>(t: &T) -> io::Result<(Vec<u8>, u64, u64)> {
        let (hash_vec, _size) = t.simple_peek_data(8)?;
        if let Some(hash_array) = hash_vec.first_chunk::<8>() {
            let hash = u64::from_le_bytes(*hash_array);
            let (data, _size) = t.simple_peek_all_data_skip(8)?;
            let data_hash = calculate_hash(&data);
            Ok((data, hash, data_hash))
        } else {
            Err(error::throw_error("Less data recieved than the 8 bytes required"))
        }
    }

    fn get_checker_mono<T: CheckableDog>(t: &T) -> io::Result<(Vec<u8>, u64, u64)> {
        let (hash_vec, _size) = t.simple_peek_data(8)?;
        if let Some(hash_array) = hash_vec.first_chunk::<8>() {
            let hash = u64::from_le_bytes(*hash_array);
            let (data, _size) = t.simple_get_all_data_skip(8)?;
            let data_hash = calculate_hash(&data);
            Ok((data, hash, data_hash))
        } else {
            Err(error::throw_error("Less data recieved than the 8 bytes required"))
        }
    }

    fn peek_checker_duo<T: CheckableDog>(t: &T) -> io::Result<(Vec<u8>, u64, u64, u64)> {
        let (hash_vec, _size) = t.simple_peek_data(8)?;
        if let Some(hash_array) = hash_vec.first_chunk::<8>() {
            let hash = u64::from_le_bytes(*hash_array);
            let (hash_vec, _size) = t.simple_peek_data_skip(16, 8)?;
            if let Some(hash_array) = hash_vec.first_chunk::<8>() {
                let hash2 = u64::from_le_bytes(*hash_array);
                let (data, _size) = t.simple_peek_all_data_skip(16)?;
                let data_hash = calculate_hash(&data);
                Ok((data, hash, hash2, data_hash))
            } else {
                Err(error::throw_error("Less data recieved than the 16 total bytes required for the first and second checker"))
            }
        } else {
            Err(error::throw_error("Less data recieved than the 8 bytes required for the first checker"))
        }
    }

    fn get_checker_duo<T: CheckableDog>(t: &T) -> io::Result<(Vec<u8>, u64, u64, u64)> {
        let (hash_vec, _size) = t.simple_peek_data(8)?;
        if let Some(hash_array) = hash_vec.first_chunk::<8>() {
            let hash = u64::from_le_bytes(*hash_array);
            let (hash_vec, _size) = t.simple_peek_data_skip(16, 8)?;
            if let Some(hash_array) = hash_vec.first_chunk::<8>() {
                let hash2 = u64::from_le_bytes(*hash_array);
                let (data, _size) = t.simple_get_all_data_skip(16)?;
                let data_hash = calculate_hash(&data);
                Ok((data, hash, hash2, data_hash))
            } else {
                Err(error::throw_error(format!("Less data recieved than the 16 total bytes required for the first and second checker. Recieved bytes are: {}/16", _size).as_str()))
            }
        } else {
            Err(error::throw_error("Less data recieved than the 8 bytes required for the first checker"))
        }
    }
}