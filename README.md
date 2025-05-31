# derive_screaming_snake_case

A lightweight Rust proc-macro crate that implements the Display trait for enums with unit variants by converting each variant's name into SCREAMING_SNAKE_CASE.

## What It Does

This macro implements `std::fmt::Display` for enums where each variant is a unit variant (i.e., no data). It turns the variant name into SCREAMING_SNAKE_CASE at compile time.

## Example

```rust
use derive_screaming_snake_case::Display;

#[derive(Display)]
enum Status {
    Ok,
    NotFound,
    InternalServerError,
}

fn main() {
    assert_eq!(Status::Ok.to_string(), "OK");
    assert_eq!(Status::NotFound.to_string(), "NOT_FOUND");
    assert_eq!(Status::InternalServerError.to_string(), "INTERNAL_SERVER_ERROR");
}
```

## This Will Fail to Compile

```rust
use derive_screaming_snake_case::Display;

#[derive(Display)]
enum Message {
    // ❌ This is NOT a unit variant
    Text(String),

    // ❌ This is a struct variant
    Error { code: u32, message: String },

    // ✅ This is fine
    Ping,
}
```

#### License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

</br>

<sub>
Copyright (c) 2025 Abhinandh S</sub>

</br>

<sub>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</sub>
