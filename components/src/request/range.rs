use std::ops::RangeInclusive;
use crate::{response::{Response, Status, Body}, result::ElseResponse, header::HeaderKey};
use super::buffer::Buffer;

#[derive(Clone, Copy)]
pub struct BufRange(
    usize, usize
); impl BufRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self(start, end)
    }
    pub(crate) fn as_range(&self) -> RangeInclusive<usize> {
        self.0..=self.1
    }
}

pub(crate) const RANGE_MAP_SIZE: usize = 4;
pub struct RangeMap(
    [Option<(BufRange, BufRange)>; RANGE_MAP_SIZE]
); impl RangeMap {
    pub(crate) fn new() -> Self {
        Self([None, None, None, None])
    }
    pub(crate) fn insert(&mut self, index: usize, key: BufRange, value: BufRange) {
        self.0[index] = Some((key, value))
    }
    pub(crate) fn read_match_part_of_buffer<'map, 'key, 'buf>(
        &'map self,
        key:    &'key str,
        buffer: &'buf Buffer,
    ) -> Option<&'buf str> {
        let target_key = key.as_bytes();
        for key_value in &self.0 {
            if key_value.is_none() {return None}
            let (key, value) = key_value.as_ref().unwrap();
            if &buffer[*key] == target_key {
                return Some(buffer.read_str(value))
            }
        }
        None
    }

    pub(crate) fn debug_fmt_with(&self, buffer: &Buffer) -> String {
        let mut fmt = String::from("[");
        for pair in &self.0 {
            let Some((key_range, value_range)) = pair.as_ref() else {break};
            fmt += "`";
            fmt += buffer.read_str(key_range);
            fmt += "`: `";
            fmt += buffer.read_str(value_range);
            fmt += "`, ";
        }
        fmt + "]"
    }
}

pub const RANGE_LIST_SIZE: usize = 2;
pub struct RangeList {
    count: usize,
    list:  [Option<BufRange>; RANGE_LIST_SIZE],
} impl RangeList {
    pub fn new() -> Self {
        Self {
            count: 0,
            list:  [None, None],
        }
    }
    pub fn push(&mut self, range: BufRange) -> Result<(), Response> {
        (self.count < RANGE_LIST_SIZE)
            ._else(|| Response {
                status: Status::NotImplemented,
                additional_headers: String::new(),
                body: Some(Body::text("Current aoi can't handle more than 4 path params")),
            })?;
        self.list[self.count] = Some(range);
        self.count += 1;
        Ok(())
    }
    pub(crate) fn get1(&self) -> Option<BufRange> {
        self.list.as_ref()[0]
    }
    pub(crate) fn get2(&self) -> Option<(BufRange, BufRange)> {
        let list = self.list.as_ref();
        Some((list[0]?, list[1]?))
    }
}

pub struct HeaderRangeMap(
    Vec<(BufRange, BufRange)>
); impl HeaderRangeMap {
    pub(crate) fn get<'buf, K: HeaderKey>(&self, key: K, buffer: &'buf Buffer) -> Option<&'buf str> {
        let key = key.as_key_str();
        for (key_range, value_range) in &self.0 {
            if buffer.read_str(key_range) == key {
                return Some(buffer.read_str(value_range))
            }
        }
        None
    }

    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
    pub(crate) fn push(&mut self, key_range: BufRange, value_range: BufRange) {
        self.0.push((key_range, value_range))
    }

    pub(crate) fn debug_fmt_with(&self, buffer: &Buffer) -> String {
        self.0.iter().fold(
            String::new(),
            |it, (key_range, value_range)| {
                it + "\n" +
                buffer.read_str(key_range) +
                ": " +
                buffer.read_str(value_range)
            }
        )
    }
}

