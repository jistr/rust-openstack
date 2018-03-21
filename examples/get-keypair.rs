// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
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

extern crate env_logger;
extern crate openstack;

use std::env;


#[cfg(feature = "compute")]
fn main() {
    env_logger::init();

    let os = openstack::Cloud::from_env()
        .expect("Failed to create an identity provider from the environment");

    let id = env::args().nth(1).expect("Provide a key pair name");
    let keypair = os.get_keypair(id).expect("Cannot get a key pair");

    println!("Fingerprint = {}, type = {:?}",
             keypair.fingerprint(), keypair.key_type());
}

#[cfg(not(feature = "compute"))]
fn main() {
    panic!("This example cannot run with 'compute' feature disabled");
}

