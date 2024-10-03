# tokio_mini
learn tokio code

# basic mod
1. loom
  - std
   - `atomic_usize::AtomicUsize`(Used for `State`)
   - `loom::std::UnsafeCell`(Used for `Header`)

# About basic struct and enum.
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

## 7.Handle
### 7.1 runtime::Handle(struct)
### 7.2 runtime::scheduler::Handle(enum)

## 8.EnterGuard

## 9.SyncNotSend

## code skills
### `use std::hash::{BuildHasher, Hash, Hasher};`
The above traits are all useful