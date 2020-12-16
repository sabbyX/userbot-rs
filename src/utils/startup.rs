/* 
 * This file is part of the userbot-rs (github.com/sabbyX/userbot-rs).
 * Copyright (c) 2020 Sabby.
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
 
use std::{error::Error, io::{stdin, stdout, Write}};

pub fn prompt(prompt: &'a str, string: &'a mut String, flush: bool) -> Result<&'a mut String, Box<dyn Error>> {
    // ask the prompt
    println!("{}", prompt);

    // read
    stdin()
        .read_line(string)
        .expect("Failed to read the input..!");

    // flush the output if needed
    if flush {
        stdout().flush()?;
    }
    Ok(string)
}
