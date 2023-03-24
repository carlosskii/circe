<div align="center" style="margin-bottom: 4rem">
  <img
    src="https://fs.cski.dev/assets/circelang.svg"
    alt="Circe Logo"
    height="128"
  />

  <h1 style="margin-top: 0.2rem; border: none">
    The Circe Project
  </h1>
</div>

<div align="center">
  <hr />

  Circe is currently in development. Even though this README writes like a finished product, it's not. The language is not ready for any kind of use, development or otherwise.

  <hr />
</div>

Circe is a programming language designed for extreme simplicity and extreme performance. It uses an English-based syntax, so the learning curve is very low. It compiles directly to machine code, which (along with its intense optimizations) results in very fast execution times.

Under the hood, Circe is made up of a few different components:

- The Circe parser, which generates an AST from the source code.
- The Circe Inference Engine (CIE), which uses a massive collection of descriptions and default configurations to convert an AST to Circe IR.
- Circe IR, which is a low-level intermediate representation of the code.
- The Circe Compiler, which converts Circe IR to machine code.

## Why?

Every popular programming language tells the machine how to do a task, but fails to communicate the actual task in question. This leads to the primary job of the modern developer: plan how a computer should do a task. This is inefficient and time-consuming. If the language is compiled, optimizations won't be as effective as they could be. Some special cases may not be necessary or shortcuts possible, but the compiler won't know without the extra context.

Circe is built to flip this standard on its head. Instead of giving the machine precise instructions, developers give the machine a complex task to perform and let the machine figure out the rest. Extra information can be provided to the compiler, but that's usually not necessary. This results in a smoother learning curve for developers, and more efficient code for the machine.

## Example Code

The syntax of Circe is not yet finalized, so this example may change in the future.

### Hello, world! (Basic)

```circe
print 'Hello, world!' to the console.
```

### Hello, world! (Behind the scenes)

```circe
howto print a string to the console
- write the string to stdout
- add a newline.

whatis stdout
- the output stream of the console.

whatis a stream
- a readable or writable sequence of bytes
| these are represented by integer IDs on Linux.

howto write a string to a stream
- run a 'WRITE' system call
| the first argument is the ID of the stream
| the second argument is the address of the string
| the third argument is the length of the string.

...
```

The Circe Inference Engine (CIE) will use the descriptions above to compile the hello world program. MANY more descriptions are required for the CIE to work, but this is abstracted away from the average developer.

## Progress

Circe is just a hobby project, so it's not going to be finished any time soon. However, I am open to contributions, so if you want to help, feel free to do so.