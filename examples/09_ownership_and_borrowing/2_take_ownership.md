# Taking ownership by value

When a function parameter has an owned type like `String` (no `&` in
front), calling the function *moves* the argument in. The caller's
binding is no longer usable afterwards. The value lives at the
callee now, and will be dropped when the callee finishes (unless it
hands ownership back via the return value, which is exactly what
happens here).

Note the signature: `String` in, `String` out. Implement the body by
mutating the parameter (`s.push_str(...)`) and then returning `s`.
Because you own `s`, you're free to mutate it without any `&mut`
dance: ownership implies the right to modify.
