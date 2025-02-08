# Storing Keys with Associated Values in Hash Maps

- The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a *hashing function*, which determines how it places these keys and values into memory.
- Hash maps are useful when you want to look up data not by using an index, but by using a key that can be any type.
- All of the keys **MUST** have the same type.
- All of the values **MUST** have the same type.

## Creating a New Hash Map

- To create an empty hash map is to use `new` and to add elements with `insert`.

## Accessing Values in a Hash Map

- We can get a value out of the hash map by providing its key to the `get` method.
- The `get` method returns an `Option<&V>`.

## Hash Maps and Ownership

- For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map.
- For owned values like `String`, the values will be moved and the hash map will be the owner of those values.
- If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

## Updating a Hash Map

### Overwrite a Value

- If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

### Adding a Key and Value Only If a Key is NOT Present

- It's common to check whether a particular key already exists in the hash map with a value and then to take the following actions:
    1. If the key does NOT exist in the hash map, the existing value should remain the way it is.
    2. If the key does NOT exist in the hash map, insert it and a value for it.
- Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter.
- The return value of the `entry` method is an enum called `Entry` that represents a value that might or might **NOT** exist.
- The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if **NOT**, it insert the parameter as the new value for this key and returns a mutable reference to the new value.

### Updating a Value Based on the Old Value

- Another common use case for hash maps is to look up a key's value and then update it based on the old value.
