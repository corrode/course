# Setting a value

Updating a `HashMap` is the same operation as adding to it: one method
covers both cases, and it doesn't care whether the key was already
there. If the key existed, the old value is replaced (and returned);
if not, it's inserted fresh.
