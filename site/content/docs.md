# MeowScript reference

MeowScript is a small, dynamically typed scripting language. Its keywords are cat puns, its values are the usual ones under different names, and its interpreter is written in Rust. This page is the whole language.

If you just want to poke at it, the [playground](/#playground) runs everything on this page. Every listing has a *Try it* button.

## A first program

```meow
scratch cat = { name: "Whiskers", lives: 9 };

pawction describe(c) {
    tail c's name + " has " + c's lives + " lives";
}

meow(describe(cat));
```

Statements end with a semicolon. You may leave it off before a closing `}` and at the very end of a file, and after anything that already ends in a `}` such as a `purrhaps` or a `pawction` declaration.

Comments are `// to the end of the line` or `/* anywhere in between */`.

## Printing

`meow` prints its arguments separated by spaces, announced with `Meow!`. `purr` prints them without the announcement. `log` is the same as `purr`, for people who miss JavaScript.

```meow
meow("hello", 42, purrfect);   // Meow! hello 42 purrfect
purr("hello", 42, purrfect);   //       hello 42 purrfect
```

## Values

| Type | Called | Looks like |
| --- | --- | --- |
| number | `number` | `42`, `3.5`, `1_000`, `2e3` |
| string | `whiskers` | `"meow"` |
| boolean | `boolean` | `purrfect`, `clawful` |
| null | `mew` | `mew` |
| array | `furrball` | `[1, 2, 3]` |
| object | `object` | `{ name: "Tom", age: 3 }` |
| function | `pawction` | `pawction(a, b) { tail a + b; }` |

`furreal` tells you which one you have: `furreal []` is `"furrball"`.

### Numbers

All numbers are 64-bit floats. Whole numbers print without a decimal point. Dividing by zero is an error rather than infinity.

### Whiskers

Strings use double quotes and understand `\n`, `\t`, `\"`, `\\`, and `\u{1F431}`. They may span lines.

`+` joins whiskers with anything, converting the other side the way `meow` would print it. `*` repeats.

```meow
purr("cats: " + 3);        // cats: 3
purr("=^.^= " * 3);        // =^.^= =^.^= =^.^=
purr("meow"[0]);           // m
purr("meow"[-1]);          // w
```

### Furrballs

Furrballs are arrays. Indexes start at 0 and negative indexes count from the end. Reading past either end is an error, not `mew`.

```meow
scratch toys = ["yarn", "box"];
purr(toys[0], toys[-1]);     // yarn box
amew toys[1] = "sunbeam";
purr(toys);                  // ["yarn", "sunbeam"]
```

Furrballs and objects are shared, not copied. Two variables pointing at the same furrball see the same changes, like in JavaScript or Python. `==` compares contents, though, so `[1, 2] == [1, 2]` is `purrfect`.

### Objects

Objects are ordered maps from whiskers to values. Keys can be written bare or quoted; `{ name }` is short for `{ name: name }`; `[expr]` computes a key.

Read a key with `obj's key` or `obj["key"]`. A key that isn't there gives `mew`. Write one with `amew`.

```meow
scratch cat = { name: "Tom", "fur color": "grey" };
purr(cat's name, cat["fur color"]);
amew cat's age = 4;
purr("age" ~ cat);          // purrfect
```

## Variables

`scratch` declares a variable in the current scope. `amew` changes one that already exists, searching outward through enclosing scopes. Using `amew` on a name that was never scratched is an error, and so is reading one, with a *did you mean* hint when there's a close match.

```meow
scratch lives = 9;
purrhaps purrfect {
    amew lives = lives - 1;    // changes the outer `lives`
    scratch lives = 100;       // a new, inner `lives`
}
purr(lives);                   // 8
```

Blocks, loops, and pawction calls each open a new scope.

## Operators

From loosest to tightest binding. Operators on the same row are applied left to right.

| Operators | Meaning |
| --- | --- |
| `\|\|` | or (short-circuits, returns the deciding value) |
| `&&` | and (short-circuits, returns the deciding value) |
| `==` `!=` | equal, not equal (by contents) |
| `<` `>` `<=` `>=` `~` | compare; `~` is "is in" |
| `\|` | bitwise or |
| `^` | bitwise xor |
| `&` | bitwise and |
| `<<` `>>` | shift |
| `+` `-` | add, subtract (and join whiskers or furrballs) |
| `*` `/` `%` | multiply, divide, remainder (and repeat whiskers) |
| `-x` `!x` `furreal x` | negate, not, type of |
| `f(x)` `a[i]` `a's b` | call, index, property |

This is the ordering Python and Rust use, so `6 & 3 == 2` means `(6 & 3) == 2`.

`~` works on all three containers: `2 ~ [1, 2]`, `"age" ~ cat`, and `"ow" ~ "meow"`.

Only `mew` and `clawful` are falsy. `0`, `""`, and `[]` are all truthy, so `purrhaps count { ... }` doesn't skip zero.

## Conditions

`purrhaps` is if, `meowtually` is else. Parentheses around the condition are optional. Chains work as you'd expect.

```meow
purrhaps hour < 6 {
    meow("too early");
} meowtually purrhaps hour < 12 {
    meow("breakfast time");
} meowtually {
    meow("second breakfast time");
}
```

`purrhaps` is an expression. A block's value is the value of its last statement, so you can use it on the right of a `scratch` or as the last line of a pawction.

```meow
scratch mood = purrhaps fed { "purring" } meowtually { "plotting" };
```

## Loops

`furrever` loops forever, or while a condition holds if you give one. `hiss` breaks out; `continue` skips to the next round.

```meow
scratch n = 0;
furrever {
    amew n = n + 1;
    purrhaps n % 2 == 0 { continue; }
    purrhaps n > 7 { hiss; }
    purr(n);
}

furrever n > 0 {
    amew n = n - 3;
}
```

`fur` walks over something: the items of a furrball, the characters of whiskers, the keys of an object, or the numbers from 0 up to (not including) a number.

```meow
fur toy ~ ["yarn", "box"] { purr(toy); }
fur letter ~ "cat"        { purr(letter); }
fur key ~ { a: 1, b: 2 }  { purr(key); }
fur i ~ 3                 { purr(i); }       // 0 1 2
```

## Pawctions

Declare a named pawction, or make an anonymous one and keep it in a variable. Either way they are values: pass them around, return them, put them in furrballs.

```meow
pawction add(a, b) {
    tail a + b;
}

scratch double = pawction(x) { x * 2 };

purr(add(2, 3), double(4));
```

`tail` returns. If a pawction runs off the end, it returns the value of its last statement, which is why `double` above works without one. A pawction with no useful last statement returns `mew`.

Pawctions close over the scope they were made in:

```meow
pawction counter() {
    scratch n = 0;
    tail pawction() {
        amew n = n + 1;
        tail n;
    };
}

scratch next = counter();
next(); next();
purr(next());    // 3
```

Calling with the wrong number of arguments is an error. So is nesting calls more than a thousand deep, which usually means a recursion with no way out.

## Pawckages

`pawckage` pulls in a library. The built-in ones start with `nya:` and are listed under [Standard library](#standard-library).

```meow
pawckage "nya:furrball";
pawckage "nya:catculator";

purr(map(range(5), pawction(n) { round(sqrt(n)) }));
```

Any `.meow` file is also a pawckage. Everything `scratch`ed or declared at its top level becomes available to whoever imports it. Paths are relative to the importing file, and the `.meow` can be left off.

```meow
pawckage "./lib/kitty_math";
purr(lives_left(2));
```

File pawckages need a filesystem, so they work from the command line but not in the browser playground.

## Errors

Every error says what went wrong and points at where:

```
Furbidden! type error: `-` doesn't work between whiskers and a number
  --> toys.meow:4:12
   |
 4 | scratch n = "nine" - 1;
   |             ^^^^^^^^^^
```

The exclamation tells you the kind: *Meowch!* for syntax, *Meow-sterious!* for an unknown name, *Furbidden!* for a type mismatch, *Paws off!* for the wrong number of arguments, *Lost kitten!* for a pawckage that can't be found, *Scratched!* for file trouble, and *Hiss!* for everything else at run time.

## The command line

Prebuilt binaries are on the [download page](/download). With a Rust toolchain you can also build it yourself:

```
cargo install --git https://github.com/AlenVelocity/MeowScript meowscript-cli

meowscript                   # start the REPL
meowscript hello.meow        # run a file
meowscript eval "1 + 2"      # evaluate a snippet
meowscript packages          # list the nya: pawckages
```

The REPL keeps reading lines while a block is open, prints the value of each expression, and leaves when you type `scram`.
