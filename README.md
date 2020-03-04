# rust-datasize
Model a 'data size' (bits, bytes, kilobytes, etc.)

A simple library to play around with Rust

Allows modeling a data size via some convenient functions & macros.

## Examples

### Creation

```
// Using functions
DataSize::new_from_bits(4);
DataSize::new_from_bytes(2);

// Using macros
bits!(4);
bytes!(2);
datasize!(2 bytes);
datasize!(4 megabytes);
```

### Comparison
```
bits!(8) == bytes!(1);
bits!(10) > bytes!(1);
datasize!(2 bytes) > bytes!(1)
```

### Addition/Subtraction
```
bits!(4) + bits!(4) == bytes!(1)
bytes!(1) - bits!(4) == bits!(4)
```

