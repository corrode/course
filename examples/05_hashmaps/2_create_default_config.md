# Creating a config map

The simplest way to use a `HashMap` is to create one and insert a few
key-value pairs. `HashMap::new()` gives you an empty map; the type is
usually inferred from the first `insert`.

Here you'll build a small configuration map with two defaults: a
`"host"` and a `"port"`.
