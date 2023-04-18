# Turkish ID Check
This basic Rust script makes sure the input is an actual 11 digit Turkish ID Number (Türkiye Cumhuriyeti Kimlik Numarası/TCKN)

I got inspiration from https://github.com/ssg/TurkishId library, and for sake of learning Rust, I tried to remake it as much as I can.

I've tried to explain code with comments as much as I can, sorry if I couldn't explain the code as I should.

Output is written in Turkish and logging is written in English, also giving info about why the input is rejected.


# Usage
Compile the code with Cargo
`cargo build`

Or to use the code as release `cargo build --release`

Afterward, just input 11-digit ID to prompt "TC Kimlik No Giriniz:" and let the magic work.

To debug and learn why input is rejected pass `RUST_LOG=info` environment variable

# The Algorithm
Let's say we name each digit as _d(n)_ where the leftmost digit is called _d1_ and the rightmost is _d11_

If:

> _d1_ > 0

and

> _n_ = (_d1_ + _d3_ + _d5_ + _d7_ + _d9_) * 7 - (_d2_ + _d4_ + _d6_ + _d8_)
>
> if _n_ < 0 then _n_ = _n_ + 10
>
> _d10_ = _n_ mod 10

and

> _d11_ = sum(_d1_.._d10_) mod 10 

then the ID given is valid.
