// Copyright 2015 Corey Farwell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::{Write, Read};
use std::process::{Command, Stdio};


pub struct DotBuilder {
    buf: String,
}

impl DotBuilder {
    pub fn new_digraph(name: &str) -> Self {
        DotBuilder{buf: format!("digraph \"{}\" {}", name, "{")}
    }

    pub fn set_ratio(&mut self, ratio: &str) {
        self.buf.push_str(&format!("ratio={};", ratio))
    }

    pub fn set_node_attrs(&mut self, node: &str, attrs: &str) {
        self.buf.push_str(&format!("\"{}\" [{}];", node, attrs));
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.buf.push_str(&format!("\"{}\" -> \"{}\";", from, to));
    }

    pub fn finish(&mut self) {
        self.buf.push_str("}");
    }

    pub fn png_bytes(&self) -> Vec<u8> {
        let child = Command::new("dot").arg("-Tpng")
                                       .stdin(Stdio::piped()).stdout(Stdio::piped())
                                       .spawn().unwrap();
        child.stdin.unwrap().write_all(self.buf.as_bytes()).unwrap();
        let mut ret = vec![];
        child.stdout.unwrap().read_to_end(&mut ret).unwrap();
        ret
    }
}
