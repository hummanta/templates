// Copyright (c) The Hummanta Authors. All rights reserved.
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

use std::process::Command;

#[test]
fn test_detector_binary() {
    let path = "samples";
    let output = Command::new("cargo")
        .args(["run", "--", "--path", path])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).expect("stdout is not valid UTF-8");

    #[rustfmt::skip]
    assert_eq!(stdout, "{\"pass\":true,\"language\":\"{{language}}\",\"extension\":\"{{extension}}\"}\n");
}
