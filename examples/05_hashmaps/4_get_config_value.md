# Getting a value with a fallback

Looking up a key in a `HashMap` returns an `Option<&V>`, because the
key might not be there — there's no null. From chapter 7 you've
already seen a few ways to collapse an `Option` into a concrete
value; the same techniques apply here.
