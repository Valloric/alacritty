// Copyright 2016 Joe Wilm, The Alacritty Project Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
/// Threading utilities
pub mod thread {
    /// Like `thread::spawn`, but with a `name` argument
    pub fn spawn_named<F, T, S>(name: S, f: F) -> ::std::thread::JoinHandle<T>
        where F: FnOnce() -> T,
              F: Send + 'static,
              T: Send + 'static,
              S: Into<String>
    {
        ::std::thread::Builder::new().name(name.into()).spawn(f).expect("thread spawn works")
    }

    pub use ::std::thread::*;
}

pub fn encode_char(c: char) -> Vec<u8> {
    let mut buf = Vec::with_capacity(4);
    unsafe {
        buf.set_len(4);
        let len = {
            let s = c.encode_utf8(&mut buf[..]);
            s.len()
        };
        buf.set_len(len);
    }

    buf
}

