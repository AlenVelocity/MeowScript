# Docs
`MeowScript` is a dynamically-typed and interpreted language inspired by cat-puns

## Keywords

- `scratch`: Used to declare a variable. For example, `scratch x = 10;` declares a variable `x` with the value `10`.

- `amew`: Used to reassign a new value to an existing variable. For example, `amew x = 5;` changes the value of the variable `x` to `5`.
- `pawction`: Used to define a function. For example,
	```
	scratch greet = pawction() { 
		meow("Meow!"); 
	};
	``` 
defines a function `greet` that prints "Meow!" to the console.
- `purrhaps`: Used to start an if statement. Example: 
	```
	purrhaps x > y { 
		meow(x is greater than y!"); 
	}
	```


- `meowtually`: Used to start the else block of an if statement. Example:

```
purrhaps x > y { 
	meow("x is greater than y!"); 
} meowtually { 
	meow("x is not greater than y!"); 
}
``` 


- `tail`: Used to return a value from a function. Example
```
scratch add = pawction(a, b) { 
	tail a + b; 
}
``` 

- `pawckage`: Used to include external code from other packages. For example, 
```
pawckage "file.meow";
```
> NOTE: SEMICOLON IS NOT OPTIONAL


## Types / Variables

All of the types are similar to other C-like languages, but we have specific names for certain types

`whiskers` - Strings

Eg: ```scratch say = "Meow!";```

Note: They must be enclosed in double quotes 

`furrball` - Arrays

Eg: ```scratch words = ["Meow", "Nya", "Nyo"];```

Boolean: 
- `purrfect`: Represents the boolean value `true`.
- `clawful`: Represents the boolean value `false`.

The `furreal` operator can be used to check the type of a variable. For example, `furreal x;` checks the type of variable `x`.

## Loops

There's only a single type of loop in MeowScript

The keyword `furrever` can be used to start a loop that runs indefinitely. For example, `furrever { scratch("Meow!"); }` runs the block of code inside the loop forever.

### Loop Control 
The following keywords can be used for manupilating the contorl flow of the loop

- `hiss`: Used to break out of a loop. For example, `furrever { hiss; }` will break the loop immediately.
- `continue`: Used to skip the current iteration and continue to the next one in a loop. For example, `pawction loop() { furrever { continue; } }` will run the loop indefinitely but will skip each iteration.

## Standard Libraries 

Meowscript comes with a lot of Builtin Functions.

### General 


meow()
- Prints the given values into the stdout
```
meow([...values])
```

The following will be documented soon. Refer the [src](https://github.com/AlenVelocity/MeowScript/tree/master/src/std_library) for now 
### Lib 1 nya:clawtility
### Lib 2 nya:catculator
### Lib 3 nya:whiskers 
### Lib 2 nya:furrball
