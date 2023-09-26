// classical enum
const DIRECTION = enum {
  NORTH,
  EAST,
  SOUTH,
  WEST,
};

// classical union
const x = union {
  a: char[50],
  n: f64,
  z: i64,
};

// in classical unions, the union takes up as much space
// as the largest member. so 'x' is as large as char[50] with some padding, 
// if it were a struct the size is the size of char[50] but also an f64, i64, and possible padding

// enums allow storing a value directly.

const HTTP = enum {
  OK = 200,
  CREATED = 201,
  NO_CONTENT = 204,
  INTERNAL_SERVER = 500,
};

// traditional enums and tradional unions have no overlap, this is why zig has an enum, union, and a tagged union.
// rust is in a hotspot with unions because they are unsafe, and enums can have values, and be non homogoneous

// this puts the range of the tag values as u16
const HTTP = enum(u16) {
  OK = 200,
  CREATED = 201,
  NO_CONTENT = 204,
  INTERNAL_SERVER = 500,
};

const response = HTTP(200); // might be able to do this instead of a cast 
const response_cast = 200 as HTTP;
response == HTTP.OK
response != HTTP.CREATED

// could even have a range
const HTTP = enum(0..599) { 
  //...
}; 

// zig-like enum and tagged union.

const HTTP = enum {
  OK,
  CREATED,
  NO_CONTENT,
};
const Response = union(HTTP) {
  OK: Json,
  CREATED: Json,
  NO_CONTENT: undefined,
  INTERNAL_SERVER: Error,
};

// from this we can pattern match on the members of a union
const x = Response.OK("'x': 'hello'".to_json_obj());

switch x {
  Response.OK => |val| print(val.to_string()),
  Response.CREATED => |val| print(val.to_string()),
  Response.NO_CONTENT => || print("val undefined"),
  Response.INTERNAL_SERVER => |val| print("error"),
}

// how do we get the value of the status code out of the response union?

const HTTP = enum {
  OK = 200,
  CREATED = 201,
  NO_CONTENT = 204,
};

const http = x as HTTP as u16; // gross, but works

// this is because you can't have both the value of the enum and the value of the union. because you need to pattern match. this is not allowed because there is no pattern match.
//

x.OK.HTTP







