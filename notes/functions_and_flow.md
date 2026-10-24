# Functions

- expressions return something and do not use a semicolon
- statements use a semicolon and do not return anything

functions that return something must have an arrow and their return
type and the last expression is what they return

the unit type `()` is what is returned for functions that do
not return anything. If you accidentally mark a return expression
as a statement with a semicolon, you'll get an error that the
() does not equal the return type

# Conditions

Rust does not auto cast conditions to booleans either and will throw an error

## Control Flow

standard if, else if, else blocks


## using if in a let statement

like python comprehensions

`let value = if true {5} else {10};`

## remember

blocks of code, anything wrapped in {} or a function, evaluates to the last expression inside of them

## Loops

rust has 3 loop types: `loop` `while` `for`

loops can return a value with an expression in the break statement

`break 10 * 2` returns 20 and breaks from the loop

## Return

return exists the current function while break just exists the current loop

## Loop labels

labels begin with a single quote. You can then write `break;` to break from the current
loop or break `'my_label;` to break from the current loop.
`'my_label: loop {...}`

## Looping through a collection

for item in items {
    
}