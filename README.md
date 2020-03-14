# creepypasta::nosleep

This library provides one function: nosleep. This function attempts to resolve a future
immediately: if the future is not ready, the function panics. You can think of this as like
"unwrap" for a future.

This is very unlikely to be useful in production, where you should use a real executor system. But
during prototyping maybe it would be useful.
