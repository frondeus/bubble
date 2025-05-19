# Bubble

A derive macro for bubbling up error variants up in the nested enums.

> ![WARNING]
>
> If you have an idea for a better name, or a verb than "bubbling up" then please leave a comment in an issue tracker

> ![WARNING-2]
> 
> So far this is an experiment. I did not polish every possible scenario but made a proof of concept that it is possible
> to achieve with current type system.


# Why should I care about this crate?

The idea is simple. Assume you have a complex crate that does some kind of network requests.

Maybe, you use `thiserror` to create an enum that should cover all error handling.

Basically you have two options:

* You can write a bigass enum that handles everything always:

```rust

#[derive(Debug, thiserror::Error)]
enum MyCrateError {
    #[error("Thing A broke!")]
    ThingABroke,
    ...
    ...
    ...
    #[error("Network error happened")]
    NetworkError(#[from] reqwest::Error)
}

```

but the problem is, as your application grows, it becomes unmaintanable and does not help your signature functions.
If every function in your code returns

```rust
fn foo() -> Result<Something, MyCrateError>
```

then looking at the signature of this function we cannot say what kind of errors can happen, because maybe `ThingABroke` is triggered only in function `bar()`.

Alternatively:

* You can have multiple enums and combine them in a tree

```rust
#[derive(Debug, thiserror::Error)]
enum OperationAError {
    #[error(transparent)]
    SubOperationABroke (#[from] SubOperationError),
    #[error("Oh no! My thing in the parent operation broke!")]
    ThingDirectlyInParentBroke
}
```

That's better. 

This crate tries to solve one big hindrance of the second approach.

Let's say that our `OperationA` invokes `SubOperationA` which invokes network call.
At the same time, `OperationA` invokes another network call at the same time.
Both can fail.

Now the caller of `OperationA` would like to handle network errors, because let's face it - a good program probably
should retry the operation or return a nice error "device is in an offline mode" or whatever.

Currently what we have to do is:

```rust
match operation_a() {
   Ok(_) => whatever!(),
   Err(error) => match error{
    OperationAError::NetworkFailed(net) | 
       OperationAError::SubOperationABroke(SubOperationAError::NetworkFailed(net)) => /* handle network error */
   }
}
```

That... Works, but requires you to know every suboperation error and handle it in a big*** match statement.
If you, like me have dosens of variants in dosens of error enums, that becomes a:

* Impossible to track
* Or cumbersome because now you need to explicitly handle every f*** case.

Both are not fun.

Especially if you replace "network error" with diesel::Error and your whole application is based on diesel :)

# Why not anyhow?

Yeah, that should work. But hear me out, that is my alternative:

# Okay, so what is the bubbling and how is it helpful.

What i'm doing in this crate, is im changing how `#[from]` attribute works (from `thiserror` macro).

Let's define such an error:

```rust

#[derive(Debug, thiserror::Error)]
enum OperationError {
    #[error("Network call has failed")]
    NetworkFailed(#[from] NetworkError),

    #[error("The sub operation has failed")]
    SubOperation(#[from] SubOperationError),

    #[error("Some io error has happened")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
enum SubOperationError {    
    #[error("Network call has failed")]
    NetworkFailed(#[from] NetworkError),

    #[error("Custom validation inside of this suboperation failed")]
    CustomValidationFailed
}

```

As you can see, `OperationError` and `SubOperationError` have a common error `NetworkError`.

So if you have an exceution of the operations like this:

```rust
fn operation () -> Result<(), OperationError> {
    sub_operation()?;
    Ok(())
}

fn sub_operation() -> Result<(), SubOperationError> {
    Err(NetworkFailed(...))
}
```

Then you would expect to get a result:
```rust
OperationError::SubOperation(SubOperationError::NetworkFailed(...))
```

right? Because `#[from] SubOperationError` generates trivial `From<SubOperationError> for OperationError`.

Okay. Now let's use my `Bubble` macro and see what will be the difference:


```rust

#[derive(Debug, thiserror::Error, Bubble)]
enum OperationError {
    #[error("Network call has failed")]
    NetworkFailed(#[bubble(bubble)] Bubble<NetworkError>),
    //                              ^^^^^^ note an extra wrapper type  
    //             ^^^^ - also we need to tag it

    #[error("The sub operation has failed")]
    SubOperation(
        #[bubble(from)] 
    //    ^^^^^^ note, this attribute has changed!
        #[source] 
        //^^^^^ thiserror #[from] was implying #[source]. So now it's explicit
        SubOperationError
    ),

    #[error("Some io error has happened")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error, Bubble)]
enum SubOperationError {    
    #[error("Network call has failed")]
    NetworkFailed(#[from] NetworkError),

    #[error("Custom validation inside of this suboperation failed")]
    CustomValidationFailed
}

```

What is the outcome?
```rust
OperationError::NetworkFailed(...)
```

!!!

a new, custom implementation of `From<SubOperationError> for OperationError` has detected, that
`NetworkError` is also a variant of `SubOperationError` and therefore can be instantiated instead

> ![WARNING]
> The ordering of variants if NOT important! We first check every variant that has `#[bubble(bubble)]` attribute and only after that the rest

We also keep the information about the context.
Even though we returned `OperationError::NetworkFailed(error)` the error inside has a type
`Bubble<NetworkError>`... That structure contains `Box<dyn Error>` to the original error stack.

How can we use it?

Either downcasting to the `NetworkError` or accessing that dyn error:

```rust
let bubble = Bubble::<FooError>::build(BarError).unwrap(); // It fails, if BarError does not contain FooError in its source chain.
let bubble = bubble.downcast_ref(); // Note, lack <T> and lack of unwrap. The former is kept in the `Bubble<T>` signature thanks to PhantomData. The latter does not need unwrap since we ensured the right type during `::build()` method.
```

# Does it work with deeply nested enums?

Yep. Thanks to the (dyn Error).source() implementation.

# Okay so how it works internally?

First of all this crate provides `Bubble<T>` type that uses `dyn Error` source chain to get the cause.


Second important trait is `BuildFrom`. That's where magic happens!

for every enum we generate couple of trait implementations.
Each per variant.

So, if your enum contains three variants :

```rust
enum Top {
    A(A),
    B(B),
    C(C)
}
```

Then we want to have:

```rust
impl BuildFrom<A> for Top {}
impl BuildFrom<B> for Top {}
and 
impl BuildFrom<C> for Top {}
```

the idea is, that the "BuildFrom" is different from normal "From" because:
1. It's fallible:
```rust
fn build_from(from: From) -> Result<Self, From>
```
2. Implementation is using `.or_else(|from| ...)` to try every variant that
has `Bubble<>` (marked by `#[bubble(bubble)]`).
3. If it fails, it goes back to the original `From::from` implementation.

Okay, so what about intermediates?

Lets say you have this tree:
```rust
Top::Middle(Middle::Bottom(Bottom::A(A)));
```

That still works, as long as `#[source]` attributes in the chain are provided.