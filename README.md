# is-odd-macro
A crate that exports macros that generates an if-else statement that tests either an identifier, or a number against ever
number in range from T::MIN to T::MAX, with a hard coded value of whether or not the number is odd. This was inspired
by the JavaScript community, and a meme that shall not be named.

For the love of Graydon, and all that is memory safe. DO NOT USE THIS CRATE. You probably can't even build it. If you want to test
that it works, you can clone the repository and change the start and end values in `is-odd-macro-cor/src/lib.rs`.

The below examples are of limited range, so that I could get some sort of output. 

# Identifiers
```rust
fn using_identifier(n: u32) {
    is_odd!(n);
}

fn using_identifier(n: u32) {
    if n as usize == 0usize {
        false
    } else if n as usize == 1usize {
        true
    } else if n as usize == 2usize {
        false
    } else if n as usize == 3usize {
        true
    } else if n as usize == 4usize {
        false
    } else if n as usize == 5usize {
        true
    } else if n as usize == 6usize {
        false
    } else if n as usize == 7usize {
        true
    } else if n as usize == 8usize {
        false
    } else if n as usize == 9usize {
        true
    } else if n as usize == 10usize {
        false
    } else if n as usize == 11usize {
        true
    } else if n as usize == 12usize {
        false
    } else if n as usize == 13usize {
        true
    } else if n as usize == 14usize {
        false
    } else if n as usize == 15usize {
        true
    } else if n as usize == 16usize {
        false
    } else if n as usize == 17usize {
        true
    } else if n as usize == 18usize {
        false
    } else if n as usize == 19usize {
        true
    } else if n as usize == 20usize {
        false
    } else if n as usize == 21usize {
        true
    } else if n as usize == 22usize {
        false
    } else if n as usize == 23usize {
        true
    } else if n as usize == 24usize {
        false
    } else if n as usize == 25usize {
        true
    } else if n as usize == 26usize {
        false
    } else if n as usize == 27usize {
        true
    } else if n as usize == 28usize {
        false
    } else if n as usize == 29usize {
        true
    } else if n as usize == 30usize {
        false
    } else if n as usize == 31usize {
        true
    } else if n as usize == 32usize {
        false
    } else if n as usize == 33usize {
        true
    } else if n as usize == 34usize {
        false
    } else if n as usize == 35usize {
        true
    } else if n as usize == 36usize {
        false
    } else if n as usize == 37usize {
        true
    } else if n as usize == 38usize {
        false
    } else if n as usize == 39usize {
        true
    } else {
        false
    };
}
```
#  Raw numbers
```rust
fn using_number() {
    is_odd!(10);
}

fn using_number() {
    if 10 == 0usize {
        false
    } else if 10 == 1usize {
        true
    } else if 10 == 2usize {
        false
    } else if 10 == 3usize {
        true
    } else if 10 == 4usize {
        false
    } else if 10 == 5usize {
        true
    } else if 10 == 6usize {
        false
    } else if 10 == 7usize {
        true
    } else if 10 == 8usize {
        false
    } else if 10 == 9usize {
        true
    } else if 10 == 10usize {
        false
    } else if 10 == 11usize {
        true
    } else if 10 == 12usize {
        false
    } else if 10 == 13usize {
        true
    } else if 10 == 14usize {
        false
    } else if 10 == 15usize {
        true
    } else if 10 == 16usize {
        false
    } else if 10 == 17usize {
        true
    } else if 10 == 18usize {
        false
    } else if 10 == 19usize {
        true
    } else if 10 == 20usize {
        false
    } else if 10 == 21usize {
        true
    } else if 10 == 22usize {
        false
    } else if 10 == 23usize {
        true
    } else if 10 == 24usize {
        false
    } else if 10 == 25usize {
        true
    } else if 10 == 26usize {
        false
    } else if 10 == 27usize {
        true
    } else if 10 == 28usize {
        false
    } else if 10 == 29usize {
        true
    } else if 10 == 30usize {
        false
    } else if 10 == 31usize {
        true
    } else if 10 == 32usize {
        false
    } else if 10 == 33usize {
        true
    } else if 10 == 34usize {
        false
    } else if 10 == 35usize {
        true
    } else if 10 == 36usize {
        false
    } else if 10 == 37usize {
        true
    } else if 10 == 38usize {
        false
    } else if 10 == 39usize {
        true
    } else {
        false
    };
}
```

#  Negative numbers
```rust
fn using_negative_number() {
    is_odd!(-10);
}

fn using_negative_number() {
    if -10 == -9223372036854775808isize {
        false
    } else if -10 == -9223372036854775807isize {
        true
    } else if -10 == -9223372036854775806isize {
        false
    } else if -10 == -9223372036854775805isize {
        true
    } else if -10 == -9223372036854775804isize {
        false
    } else if -10 == -9223372036854775803isize {
        true
    } else if -10 == -9223372036854775802isize {
        false
    } else if -10 == -9223372036854775801isize {
        true
    } else if -10 == -9223372036854775800isize {
        false
    } else if -10 == -9223372036854775799isize {
        true
    } else if -10 == -9223372036854775798isize {
        false
    } else if -10 == -9223372036854775797isize {
        true
    } else if -10 == -9223372036854775796isize {
        false
    } else if -10 == -9223372036854775795isize {
        true
    } else if -10 == -9223372036854775794isize {
        false
    } else if -10 == -9223372036854775793isize {
        true
    } else if -10 == -9223372036854775792isize {
        false
    } else if -10 == -9223372036854775791isize {
        true
    } else if -10 == -9223372036854775790isize {
        false
    } else if -10 == -9223372036854775789isize {
        true
    } else if -10 == -9223372036854775788isize {
        false
    } else if -10 == -9223372036854775787isize {
        true
    } else if -10 == -9223372036854775786isize {
        false
    } else if -10 == -9223372036854775785isize {
        true
    } else if -10 == -9223372036854775784isize {
        false
    } else if -10 == -9223372036854775783isize {
        true
    } else if -10 == -9223372036854775782isize {
        false
    } else if -10 == -9223372036854775781isize {
        true
    } else if -10 == -9223372036854775780isize {
        false
    } else if -10 == -9223372036854775779isize {
        true
    } else if -10 == -9223372036854775778isize {
        false
    } else if -10 == -9223372036854775777isize {
        true
    } else if -10 == -9223372036854775776isize {
        false
    } else if -10 == -9223372036854775775isize {
        true
    } else if -10 == -9223372036854775774isize {
        false
    } else if -10 == -9223372036854775773isize {
        true
    } else if -10 == -9223372036854775772isize {
        false
    } else if -10 == -9223372036854775771isize {
        true
    } else if -10 == -9223372036854775770isize {
        false
    } else if -10 == -9223372036854775769isize {
        true
    } else {
        false
    };
}
```

# Numbers with format identifiers
```rust
fn hexidecimal_number() {
    is_odd!(0x10);
}

fn hexidecimal_number() {
    if 0x10 == 0usize {
        false
    } else if 0x10 == 1usize {
        true
    } else if 0x10 == 2usize {
        false
    } else if 0x10 == 3usize {
        true
    } else if 0x10 == 4usize {
        false
    } else if 0x10 == 5usize {
        true
    } else if 0x10 == 6usize {
        false
    } else if 0x10 == 7usize {
        true
    } else if 0x10 == 8usize {
        false
    } else if 0x10 == 9usize {
        true
    } else if 0x10 == 10usize {
        false
    } else if 0x10 == 11usize {
        true
    } else if 0x10 == 12usize {
        false
    } else if 0x10 == 13usize {
        true
    } else if 0x10 == 14usize {
        false
    } else if 0x10 == 15usize {
        true
    } else if 0x10 == 16usize {
        false
    } else if 0x10 == 17usize {
        true
    } else if 0x10 == 18usize {
        false
    } else if 0x10 == 19usize {
        true
    } else if 0x10 == 20usize {
        false
    } else if 0x10 == 21usize {
        true
    } else if 0x10 == 22usize {
        false
    } else if 0x10 == 23usize {
        true
    } else if 0x10 == 24usize {
        false
    } else if 0x10 == 25usize {
        true
    } else if 0x10 == 26usize {
        false
    } else if 0x10 == 27usize {
        true
    } else if 0x10 == 28usize {
        false
    } else if 0x10 == 29usize {
        true
    } else if 0x10 == 30usize {
        false
    } else if 0x10 == 31usize {
        true
    } else if 0x10 == 32usize {
        false
    } else if 0x10 == 33usize {
        true
    } else if 0x10 == 34usize {
        false
    } else if 0x10 == 35usize {
        true
    } else if 0x10 == 36usize {
        false
    } else if 0x10 == 37usize {
        true
    } else if 0x10 == 38usize {
        false
    } else if 0x10 == 39usize {
        true
    } else {
        false
    };
}
```

# Negative numbers with format identifiers
```rust
fn negative_hexidecimal_number() {
    is_odd!(-0x10);
}

fn negative_hexidecimal_number() {
    if -0x10 == -9223372036854775808isize {
        false
    } else if -0x10 == -9223372036854775807isize {
        true
    } else if -0x10 == -9223372036854775806isize {
        false
    } else if -0x10 == -9223372036854775805isize {
        true
    } else if -0x10 == -9223372036854775804isize {
        false
    } else if -0x10 == -9223372036854775803isize {
        true
    } else if -0x10 == -9223372036854775802isize {
        false
    } else if -0x10 == -9223372036854775801isize {
        true
    } else if -0x10 == -9223372036854775800isize {
        false
    } else if -0x10 == -9223372036854775799isize {
        true
    } else if -0x10 == -9223372036854775798isize {
        false
    } else if -0x10 == -9223372036854775797isize {
        true
    } else if -0x10 == -9223372036854775796isize {
        false
    } else if -0x10 == -9223372036854775795isize {
        true
    } else if -0x10 == -9223372036854775794isize {
        false
    } else if -0x10 == -9223372036854775793isize {
        true
    } else if -0x10 == -9223372036854775792isize {
        false
    } else if -0x10 == -9223372036854775791isize {
        true
    } else if -0x10 == -9223372036854775790isize {
        false
    } else if -0x10 == -9223372036854775789isize {
        true
    } else if -0x10 == -9223372036854775788isize {
        false
    } else if -0x10 == -9223372036854775787isize {
        true
    } else if -0x10 == -9223372036854775786isize {
        false
    } else if -0x10 == -9223372036854775785isize {
        true
    } else if -0x10 == -9223372036854775784isize {
        false
    } else if -0x10 == -9223372036854775783isize {
        true
    } else if -0x10 == -9223372036854775782isize {
        false
    } else if -0x10 == -9223372036854775781isize {
        true
    } else if -0x10 == -9223372036854775780isize {
        false
    } else if -0x10 == -9223372036854775779isize {
        true
    } else if -0x10 == -9223372036854775778isize {
        false
    } else if -0x10 == -9223372036854775777isize {
        true
    } else if -0x10 == -9223372036854775776isize {
        false
    } else if -0x10 == -9223372036854775775isize {
        true
    } else if -0x10 == -9223372036854775774isize {
        false
    } else if -0x10 == -9223372036854775773isize {
        true
    } else if -0x10 == -9223372036854775772isize {
        false
    } else if -0x10 == -9223372036854775771isize {
        true
    } else if -0x10 == -9223372036854775770isize {
        false
    } else if -0x10 == -9223372036854775769isize {
        true
    } else {
        false
    };
}
```

# Machine Code Generation

In this test, I used a min of 0 and max of 200, for the `is_odd!` macro.

The machine code generated in release mode optimizes the following code to a jump table
```rust
fn main() {
    for i in 0..100 {
        if is_odd(i) {
            println!("Hello, odd world!");
        } else {
            println!("Hello, even world!");
        }
    }
}
fn is_odd(n: u32) -> bool {
    is_odd!(n)
}
```
For loop with jump table to load the right string to print.
![image](https://github.com/user-attachments/assets/1e9e32a3-0af0-40b5-b668-20e8d041caf1)

The table generated.
![image](https://github.com/user-attachments/assets/965bd8d8-d51f-4fb6-94e1-9bd99fe375c6)

In Debug builds, it doesn't optimize the `is_odd(n: u32)` function at all. This generates a comparison all the way to 199.
![image](https://github.com/user-attachments/assets/34fc09ec-ac72-4c1e-96f5-4564f3d306b2)

It generates a very normal looking for loop.
![image](https://github.com/user-attachments/assets/8ea300bf-9223-47ae-8679-abb40d7f370f)

So, I added a few statements, after, and now the code looks like this.
```rust
fn main() {
    for i in 0..100 {
        if is_odd(i) {
            println!("Hello, odd world!");
        } else {
            println!("Hello, even world!");
        }
    }

    if is_odd!(-11) {
        println!("Hello, odd world -11!");
    }
    if is_odd!(101) {
        println!("Hello, odd world 101!");
    };

}
fn is_odd(n: u32) -> bool {
    is_odd!(n)
}
```

Nothing remarkable in release build. This shows the end of the for loop, and then prints the two statements.
![image](https://github.com/user-attachments/assets/3877ac37-3cb0-4631-9d78-c9ea9763d086)

But in debug build, it also generated a ton of these jumps in between the code that has the loop
![image](https://github.com/user-attachments/assets/b5203c7a-f8ad-43bb-888b-591d688dfd78)

The JNZ leads to this
![image](https://github.com/user-attachments/assets/1c4100c1-1421-4e88-8f0a-f3e7cc63a7a5)

and the JMP after leads to this.
![image](https://github.com/user-attachments/assets/d81b2cee-a748-4e9b-aab3-01f948016ef2)

And this long list of jumps after lead to 
![image](https://github.com/user-attachments/assets/419e2bf2-b48c-4d3c-8394-ea31ff84ac9f)

This, which prints out the -11 message, more weird jumping, and then
![image](https://github.com/user-attachments/assets/e8431da6-a909-4aa1-8603-ddec76b3ac14)

We print out the 101 message and return...
![image](https://github.com/user-attachments/assets/cbcce02d-01ef-4196-b622-570a126d48d8)




