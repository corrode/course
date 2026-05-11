# Borrow in, own out

This step is the canonical "borrowed in, owned out" shape. The
caller hands you a cheap `&str` view, and you give back a brand new
`String` that they get to keep. You'll see this pattern over and
over in real Rust code, so it's worth getting comfortable with the
signature now.
