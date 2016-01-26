// Copyright (c) 2016, Mikkel Kroman <mk@uplink.io>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use mio;
use mio::{EventLoop,Token,EventSet};
use mio::tcp::TcpStream;
use mio::util::Slab;
use openssl::ssl::{SslContext, SslMethod};

use error::Error;
use server::Server;
use connection::Connection;

#[derive(Debug)]
pub struct Client {
    pub servers: Vec<Server>,
    pub ssl_context: SslContext,
    pub connections: Slab<Connection>
}

impl Client {
    pub fn new() -> Result<Client, Error> {
        let ctx = try!(SslContext::new(SslMethod::Sslv23));

        Ok(Client {
            servers: vec![],
            connections: Slab::new(1024),
            ssl_context: ctx
        })
    }

    /// Connect to a given server.
    ///
    /// Returns the connection on success, an error otherwise.
    pub fn connect(&mut self, server: &mut Server) -> Result<(), Error> {

    }

    pub fn run(&mut self) {
        let mut event_loop = EventLoop::new().unwrap();

        println!("event_loop={:?}", event_loop);

        event_loop.run(&mut *self).unwrap();
    }
}

impl mio::Handler for Client {
    type Message = ();
    type Timeout = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Client>, token: Token, events: EventSet) {
        println!("socket is ready; token={:?}; events={:?}", token, events);

        self.connections[token].ready(event_loop, events);
    }
}


#[cfg(test)]
mod tests {
    use mio;
    use super::*;

    #[test]
    fn test_constructor() {
        let _ = Client::new().unwrap();
    }
}
