// comments are written with two '/' characters
// multi-line comments will be for doc comments and will be written like jsdoc, but testable like rust

// variables can be declared constant, making them unmodifiable
const x = 5
// or mutatable with let
let y = 2

// enumerable types are declared as follows, similar to unions in other languages, they are called tags because they are both a union and enumerable
const Day = 
  | monday
  | tuesday
  | wednesday
  | thursday
  | friday
  | saturday
  | sunday


// functions must be const
const next_weekday = fn(d:Day) Day {
 // tags can be matched on
 // return must be specified
 return match d {
  .monday => tuesday
  .tuesday => wednesday
  .wednesday => thursday
  .thursday => friday
  // rest operator specifies any remaining match arms
  _ => monday
 }
}
// functions are defined in the previous example, to give the lambda syntax for free
let x = [0,1,2,3].map(fn(x) usize { return x + 2 })
print(x) // [2,3,4,5]

// errors are first class citizens
const InvalidWeekday = error {}

const assert_weekday = fn(d:Day) InvalidWeekday!Day {
 return match d {
  .saturday => InvalidWeekday
  .sunday => InvalidWeekday
  _ => d
 }
}
// InvalidWeekday!Day can be thought of as a tag of `InvalidWeekday | Day` the syntax gives errors more visiblity
// errors must be handled, or bubbled

// struct syntax
const Car = struct {
  wheels: f64
  make: [char]
  model: [char]
  direction: u8
}

// you can attach functions to structs
const drive = fn(self: Car, direction: u8) void {
  self.direction = direction
}

// arrays can be sized at compile time, or unsized (growable, shrinkable)
// here x is immutable, so the compiler can infer it to be size 11
// the elements inside x can be mutated
const x: [char] = "Hello There"
// here x is immutable, and the elements inside the array are immutable
const x: [const char] = "Hello There"

// here y is mutable, meaning the array can be lengthend or shortened. this will not compile, as explicitly sized arrays, can't be resized
let y: [char; 11] = "Hello There" //! we can't give this array a compile time known size, and still be mutable!

// here y is mutable, and its elements are mutable. can be lengthened, shortened, or elements mutated
let y: [char] = "Hello There"

// values can be copied, moved, borrowed mutably, or borrowed immutably
// x is set to 5
const x = 5
// x is copied. All primative scalar types are copied
const y = x
// m is moved to z, m is no longer reachable after z has been assigned
let m = "Hello There"
let z = m
print(m)

// count_spaces takes a read-only borrowed slice of an array of known or unknown length. & is a read-only borrow, and * is a mutable borrowx
// to_check is read only slice with read only chars
const count_spaces = fn(to_check: &[&char]) f64 {
  let count = 0
  for (to_check) |x| {
    if (x == ' ') {
      count += 1
    }
  }
  return count
}
// note: do not think of these as pointers, as &[&char] in tradional pointer terminolgy would mean this is a reference to an array of char references. this is clearly an array of chars, they are just read-only

// you can pass a value by ownership, (moved)
const count_spaces = fn(to_check: [char]) f64 { }
let x = "Hello";
const spaces = count_spaces(x)
print(x) // ! count_spaces took ownership of x. won't compile

// reference self for 1 to 1 aliasing
type c_chars: const [const char] = self
const x: c_chars = "Hello" // can only be assigned to const vars, and members are immutable

// some types have a special way to subtype values
type months: u8 = 0..12
const x: months = 7
const y: months = 13 // ! can't do that. out of bounds

// prepare a type to be used as an implementation
type to_draw: fn(self) void = trait

const rgba = struct {
  r: u8
  g: u8
  b: u8
  a: u8
}

impl rgba: to_draw = fn(self) void {
  // do some cool drawing
}

// subtyping revisited
type months: u8 = 0..12
// specifying any, allows any type to implement months: objects, functions, arrays, errors, and scalar primatives
type to_months: fn(self: any) !months = trait
// be more specific if that was not the intent
type to_months: fn(self: scalar) !months = trait

impl u8: to_months = fn(self) !months {
  if (self > 12) { return InvalidCastException }
  return self as months
}

// future versions of type-lang will have formal methods to check bounded ranges during compile time when casting, it is best to handle all outcomes for now as seen above.
