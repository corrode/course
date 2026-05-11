# Validate email

Now the `Ok` value is a borrow of the input. The `&str` in the return
type implicitly borrows from `email`, so the compiler infers a
lifetime linking input and output via lifetime elision. Chapter 9
makes this explicit; for now, just notice the function compiles even
though no lifetimes appear in the signature.
