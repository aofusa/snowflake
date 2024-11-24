64 bit ID Generator
=====

64 bit id generator like Snowflake, ULID, UUIDv7.  
upper 32bits are timestamp and lower 32bits are random part.  

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           unix_ts_ms                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           unix_ts_ms                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                            rand                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                            rand                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

unix_ts_ms:
32 bit big-endian unsigned number of Unix epoch timestamp.

rand:
The final 32 bits of pseudo-random data to provide uniqueness.

Usage
-----

```rust
// Create instance
let mut snowflake = Snowflake::new();

// Get some ids
let a = snowflake.next();
let b = snowflake.next();

// Print id like 45232ef285594ea, 4526da9190ca349, etc..
println!("{:x}", a);
println!("{:x}", b);
```

