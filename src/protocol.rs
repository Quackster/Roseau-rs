use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NettyRequest {
    header: String,
    content: String,
}

impl NettyRequest {
    pub fn new(header: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            content: content.into(),
        }
    }

    pub fn decode_frame(frame: &[u8]) -> Result<Self, DecodeError> {
        if frame.len() < 4 {
            return Err(DecodeError::FrameTooShort);
        }

        let length_text =
            std::str::from_utf8(&frame[..4]).map_err(|_| DecodeError::InvalidLength)?;
        let length = length_text
            .trim()
            .parse::<usize>()
            .map_err(|_| DecodeError::InvalidLength)?;

        if frame.len() < 4 + length {
            return Err(DecodeError::IncompleteFrame {
                expected: length,
                actual: frame.len().saturating_sub(4),
            });
        }

        let content = latin1_to_string(&frame[4..4 + length]);
        Ok(Self::from_content(&content))
    }

    pub fn from_content(content: &str) -> Self {
        match content.split_once(' ') {
            Some((header, request)) => Self::new(header, request),
            None => Self::new(content, ""),
        }
    }

    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub trait ClientMessage {
    fn get_header(&self) -> &str;
    fn get_message_body(&self) -> String;
    fn get_argument_amount(&self) -> usize;
    fn get_argument_amount_with(&self, delimiter: &str) -> usize;
    fn get_argument(&self, index: usize) -> Option<&str>;
    fn get_argument_with(&self, index: usize, delimiter: &str) -> Option<&str>;
}

impl ClientMessage for NettyRequest {
    fn get_header(&self) -> &str {
        &self.header
    }

    fn get_message_body(&self) -> String {
        visible_controls(&self.content, 13)
    }

    fn get_argument_amount(&self) -> usize {
        self.get_argument_amount_with(" ")
    }

    fn get_argument_amount_with(&self, delimiter: &str) -> usize {
        if delimiter.is_empty() {
            1
        } else {
            self.content.split(delimiter).count()
        }
    }

    fn get_argument(&self, index: usize) -> Option<&str> {
        self.get_argument_with(index, " ")
    }

    fn get_argument_with(&self, index: usize, delimiter: &str) -> Option<&str> {
        if delimiter.is_empty() {
            (index == 0).then_some(self.content.as_str())
        } else {
            self.content.split(delimiter).nth(index)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    FrameTooShort,
    InvalidLength,
    IncompleteFrame { expected: usize, actual: usize },
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FrameTooShort => write!(f, "frame is shorter than the 4-byte length prefix"),
            Self::InvalidLength => write!(f, "frame length prefix is not a valid integer"),
            Self::IncompleteFrame { expected, actual } => {
                write!(
                    f,
                    "frame body is incomplete: expected {expected} bytes, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for DecodeError {}

pub trait SerializableObject {
    fn serialise(&self, response: &mut NettyResponse);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NettyResponse {
    header: String,
    finalised: bool,
    buffer: String,
}

impl NettyResponse {
    pub fn new() -> Self {
        Self {
            header: String::new(),
            finalised: false,
            buffer: String::new(),
        }
    }

    pub fn with_header(header: impl AsRef<str>) -> Self {
        let mut response = Self::new();
        response.init(header);
        response
    }

    pub fn init(&mut self, header: impl AsRef<str>) {
        self.finalised = false;
        self.header = header.as_ref().to_owned();
        self.buffer.clear();
        self.buffer.push('#');
        self.append(header.as_ref());
    }

    pub fn append(&mut self, value: impl Display) {
        let data = value.to_string().replace('#', "*");
        self.buffer.push_str(&data);
    }

    pub fn append_argument(&mut self, value: impl Display) {
        self.append_argument_with(value, ' ');
    }

    pub fn append_new_argument(&mut self, value: impl Display) {
        self.append_argument_with(value, '\r');
    }

    pub fn append_part_argument(&mut self, value: impl Display) {
        self.append_argument_with(value, '/');
    }

    pub fn append_tab_argument(&mut self, value: impl Display) {
        self.append_argument_with(value, '\t');
    }

    pub fn append_kv_argument(&mut self, key: impl Display, value: impl Display) {
        self.append('\r');
        self.append(key);
        self.append('=');
        self.append(value);
    }

    pub fn append_kv2_argument(&mut self, key: impl Display, value: impl Display) {
        self.append('\r');
        self.append(key);
        self.append(':');
        self.append(value);
    }

    pub fn append_argument_with(&mut self, value: impl Display, delimiter: char) {
        self.append(delimiter);
        self.append(value);
    }

    pub fn append_object(&mut self, object: &impl SerializableObject) {
        object.serialise(self);
    }

    pub fn get_body_string(&mut self) -> String {
        visible_controls(&self.get(), 14)
    }

    pub fn get(&mut self) -> String {
        if !self.finalised {
            self.buffer.push('#');
            self.buffer.push('#');
            self.finalised = true;
        }

        self.buffer.clone()
    }

    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn is_finalised(&self) -> bool {
        self.finalised
    }
}

impl Default for NettyResponse {
    fn default() -> Self {
        Self::new()
    }
}

fn visible_controls(input: &str, exclusive_upper: u32) -> String {
    (0..exclusive_upper).fold(input.to_owned(), |current, value| {
        let control = char::from_u32(value).unwrap_or_default().to_string();
        current.replace(&control, &format!("[{value}]"))
    })
}

fn latin1_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| char::from(*byte)).collect()
}
