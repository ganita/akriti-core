/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static! {
    static ref INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct InstanceId {
    id: usize
}

impl InstanceId {
    pub fn new() -> InstanceId {
        InstanceId {
            id: INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_increments() {
        assert_eq!(InstanceId::new().id, 0);
        assert_eq!(InstanceId::new().id, 1);
        assert_eq!(InstanceId::new().id, 2);
        assert_eq!(InstanceId::new().id, 3);
    }
}

