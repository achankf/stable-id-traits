mod cast_usize;
mod maximum;
mod predecessor;
mod successor;

/**
Predecessor trait for numbers.
*/
pub trait Predecessor {
    /// Return `self` - 1. Panics when `self` is at 0.
    fn prev_value(self) -> Self;
}

/**
Successor trait for numbers.
*/
pub trait Successor {
    /// Return `self` + 1. Panics if `self` is at maximum value.
    fn next_value(self) -> Self;
}

/**
A trait that describes the max value of an unsigned integer. This trait is used to detect overflow.
Also, it's used like a NULL terminator for the free list in [`Tec`].
*/
pub trait Maximum {
    /// Generally this should be `X::MAX`, where `X` is an unsigned integer. The value is used to detect overflow.
    fn max_value() -> Self;
}

/**
Trait for casting between an unsigned integer to a usize, and vice versa.
Note: to() would panic if the value is greater or equal to the type's max.
*/
pub trait CastUsize {
    /** Turning an unsigned integer into a usize. */
    fn cast_to(self) -> usize;
    /** Turning a usize into an unsigned integer. */
    fn cast_from(val: usize) -> Self;
}

/**
Trait for projecting the inner value of the Id's tuple, i.e. returning u32 for Id(u32).
 */
pub trait Inner<T> {
    /** Return the inner value of the Id, i.e. returning u32 for Id(u32) */
    fn project(self) -> T;
}
