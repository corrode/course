# Adding items

Now we modify the list in place. Where the previous step only read
from the `Vec`, this one needs to change it. The `&mut Vec<String>`
says "I need exclusive access for a moment", and that's what lets us
push a new item onto the end.
