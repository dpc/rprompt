// Copyright 2014-2017 The Rprompt Developers
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


use std::io::Write;

pub fn read_reply() -> std::io::Result<String> {
    let mut reply = String::new();

    std::io::stdin().read_line(&mut reply)?;

    // We should have a newline at the end. This helps prevent things such as:
    // > printf "no-newline" | program-using-rprompt
    // If we didn't have the \n check, we'd be removing the last "e" by mistake.
    if reply.chars().last() != Some('\n') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "unexpected end of file",
        ));
    }

    // Remove the \n from the line.
    reply.pop().ok_or(std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "unexpected end of file",
    ))?;

    Ok(reply)
}

pub fn prompt_reply_stdout(prompt: &str) -> std::io::Result<String> {
    let mut stdout = std::io::stdout();

    write!(stdout, "{}", prompt)?;
    stdout.flush()?;
    read_reply()
}

pub fn prompt_reply_stderr(prompt: &str) -> std::io::Result<String> {
    let mut stderr = std::io::stderr();

    write!(stderr, "{}", prompt)?;
    stderr.flush()?;
    read_reply()
}