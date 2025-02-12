# Traits

- A trait defines the functionality a particular type has and can share with other types.
- We can use traits to defines shared behavior in an abstract way.
- We can use trait bounds to specify that a generic type can be any type that has certain behavior.
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish
  some purpose.
- **NOTE**: Traits are similar to a feature often called *interfaces* in other languages, although with some
  differences.

## Defining a Trait

- We declare a trait using the `trait` keyword and the trait's name.
- We declare the method signatures that describe the behaviors of the types that implement this trait.
- After the method signature, instead of providing an implementation which curly brackets, we use a semicolon.
- Each type implementing this trait must provide its own custom behavior for the body of the method.
- A trait can have multiple methods in its body: the method signatures are listed one per line, and each line ends in a
  semicolon.

## Implementing a Trait on a Type

- Implementing a trait on a type is similar to implementing regular methods.
- The difference is that after `impl`, we put the trait name we want to implement, the use the `for` keyword, and then
  specify the name of the type we want to implement the trait for.
- Within the `impl` block, we put the method signatures that the trait definition has defined and fill in the method
  body with the specific behavior that we want the methods of the trait to have for the particular type.
- When call the trait methods on instances, the user **MUST** bring the trait into scope as well as the types.
- **ONE RESTRICTION TO NOTE**: we can implement a trait on a type only if either the traitor the type, or both, are
  local to our crate.

## Default Implementation

- When you implement the trait on a particular type, you can **KEEP** or **OVERRIDE** each method’s default behavior.
- Default implementations can call other methods in the same trait, even if those other methods don’t have a default
  implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to
  specify a small part of it.
- It isn’t possible to call the default implementation from an overriding implementation of that same method.

## Traits as Parameters

- You can use traits to define functions that accept many different types.
- We specify the `impl` keyword and the trait name. (`pub fn notify(item: &impl Summary) {}`)
- This parameter accepts any type that implements the specified trait.
- พูดง่ายๆสร้างฟังก์ชั่นขึ้นมาที่รับ parameter เป็น type อะไรก็ได้ โดยมีเงื่อนไขว่า type นั้นจะต้อง implement trait
  นั้นๆตามที่ parameter กำหนด
- เราเรียกมันว่า The `impl Trait` syntax

## Trait Bound Syntax

- The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a
  *trait bound*. (`pub fn notify<T: Summary>(item: &T) {}` equivalent to `pub fn notify(item: &impl Summary) {}` from
  previous topic.
- Considering this `pub fn notify(item1: &impl Summary, item2: &impl Summary) {}`, the `item1` and `item2` can be
  different types (as long as both types implement Summary). If we want to force both parameters to have the same type?
- We can keep the `item1` and `item2` are the same types with the trait bound syntax.
  `pub fn notify<T: Summary>(item1: &T, item2: &T) {}`

### Specifying Multiple Trait Bounds with the `+` Syntax

- We can also specify more than one trait bound.
    - The `impl Trait` syntax: `pub fn notify(item: &(impl Summary + Display)) {}`
    - The trait bound with `+` syntax: `pub fn notify<T: Summary + Display>(item: &T) {}`

### Clearer Trait Bounds with where Clauses

- Using too many trait bounds has its downsides.
- Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait
  bound information between the function’s name and its parameter list, making the function signature hard to read.
- Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature.

From this

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

To this

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

## Returning Types That Implement Traits

- We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait.
- For example: `fn returns_summarizable() -> impl Summary {}`
- we specify the function that returns some type that implements the trait without naming the concrete type.
- The ability to specify a return type only by the trait it implements is especially useful in the context of closures
  and iterators.
- Closures and iterators create types that only the compiler knows or types that are very long to specify. The `impl
  Trait` syntax lets you concisely specify that a function returns some type that implements the `Iterator` trait
  without needing to write out a very long type.
- ข้อควรระวังเพื่อป้องกันการเข้าใจผิดสำหรับ ณ บทนี้ สมมุติ เรามี struct A กับ B ได้ทำการ implement trait C ทั้งคู่
  แล้วสร้างฟังก์ชั่น x ขึ้นมา โดย x จะ return type อะไรก็ได้ที่ implement C แต่ ในฟังก์ชั่น x เราไม่สามารถเขียน logic
  ให้ return A หรือ B ทั้งคู่ได้ ฟังก์ชั่น x ควร return ได้เพียง type เดียวที่ implement C

## Using Trait Bounds to Conditionally Implement Methods

- By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally
  for types that implement the specified traits.
- We can also conditionally implement a trait for any type that implements another trait.
- Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are used
  extensively in the Rust standard library.

```rust
// For example, the standard library implements the `ToString` trait on any type that implements the Display trait. 
// The impl block in the standard library looks similar to this code:
impl<T: Display> ToString for T {
    // --snip--
}
```

หัวข้อใช้ AI ช่วยสร้างตัวอย่างแต่ก็ยังไม่ค่อยเข้าใจนัก ดูจาก ไฟล์ `order`, `product` และ `service` ได้ จะเห็นว่า
`product` ไม่ได้ implement trait `order` แต่สามารถเรียก method ของ `order` ได้เฉยเลย ค่อนข้าง งงๆ กับวิธีการนี้พอสมควร

อันนี้นั่ง assume เอาเองนะ จาก syntax นี้ `impl<T: Display> Order for T` ถ้า T คือ type ใดๆก็ตามที่ implement Display,
เพราะฉะนั้น type T เราจะสามารถใช้ method ของ trait Order ได้เลยโดยอัตโนมัติโดยไม่ได้ต้องมาเขียนว่า `impl Order for T`
อีก