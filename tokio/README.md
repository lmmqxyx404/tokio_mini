# tokio_mini
learn tokio code

# basic mod
1. loom
  - std
   - `atomic_usize::AtomicUsize`(Used for `State`)
   - `loom::std::UnsafeCell`(Used for `Header`)

# about basic struct.
## 1.AtomicUsize
independent

## 2.UnsafeCell
independent

## 3.Vtable
independent

## 4.Header
includes 1 2 3 and other vital memeber

## 5.RawTask
main is 4

## 6.JoinHandle
used for `spawn_blocking<F, R>(func: F) -> JoinHandle<R>`
in runtime::blocking::poll