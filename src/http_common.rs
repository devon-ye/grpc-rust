use futures::stream::Stream;

use solicit::http::StaticHeader;
use solicit::http::HttpError;

#[derive(Debug)]
pub enum HttpStreamPart {
    Headers(Vec<StaticHeader>),
    Data(Vec<u8>),
}

pub type HttpStreamStream = Box<Stream<Item=HttpStreamPart, Error=HttpError>>;
pub type HttpStreamStreamSend = Box<Stream<Item=HttpStreamPart, Error=HttpError> + Send>;