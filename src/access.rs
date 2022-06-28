use std::net::IpAddr;

use combine::{Parser};
use combine::{choice};
use combine::error::StreamError;
use combine::easy::Error;

use ast::{Item, Source};
use helpers::{semi, ident, string};
use tokenizer::{TokenStream, Token};


fn parse_source<'a>(val: Token<'a>)
    -> Result<Source, Error<Token<'a>, Token<'a>>>
{
    let value = val.value;
    if value == "all" {
        return Ok(Source::All);
    } else if value == "unix:" {
        return Ok(Source::Unix);
    }
    let mut pair = value.splitn(2, '/');
    let addr = pair.next().unwrap().parse::<IpAddr>()?;
    if let Some(net) = pair.next() {
        let subnet = net.parse::<u8>()
            .map_err(|e| Error::unexpected_format(
                format!("invalid subnet: {}", e)))?;
        return Ok(Source::Network(addr, subnet));
    } else {
        return Ok(Source::Ip(addr));
    }
}

fn allow<'a>()
    -> impl Parser<TokenStream<'a>, Output=Item>
{
    ident("allow")
    .with(string())
    .and_then(parse_source)
    .skip(semi())
    .map(Item::Allow)
}

fn deny<'a>()
    -> impl Parser<TokenStream<'a>, Output=Item>
{
    ident("deny")
    .with(string())
    .and_then(parse_source)
    .skip(semi())
    .map(Item::Deny)
}

pub fn directives<'a>()
    -> impl Parser<TokenStream<'a>, Output=Item>
{
    choice((
        allow(),
        deny(),
    ))
}
