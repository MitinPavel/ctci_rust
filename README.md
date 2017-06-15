# Cracking the Coding Interview in Rust

Rust solutions for "Cracking the Coding Interview" (6th edition).

Disclaimer: this repository does not contain an idiomatic Rust code. Assuming coding interview goals:
* it often doesn't use standard library functions and third party traits
* it prefers more imperative code style
* it prefers mutability

## Auxiliary aspects of interview questions

Tags:
* BASE - the main assumption
* BRANCH - a mutually exclusive alternative
* UNSPECIFIED - the book doesn't mentioned the aspect

### Chapter 1 (Arrays and Strings)

#### 1.1 Is Unique

* How to handle an empty string
  * UNSPECIFIED
* Are we allowed to modify the input string
  * BRANCH
* The character set
  * BASE Assuming ASCII symbols only
  * BRANCH Unicode strings

#### 1.2 Check Permutations

* Are whitespace chars significant
  * BASE YES
* Is algorithm case sensitive
  * BASE YES
