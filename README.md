# Ruse

This is an experimental project to create an R7RS small-specification-compliant
Scheme as an embedded language in Rust, similar to the relationship between Lua
and C.

## Specification

The goal is for Ruse to be compliant with the R7RS small specification, along
with whatever additional features are deemed necessary or desirable.

The specification may be found here: http://www.scheme-reports.org/

The 7th edition of the Scheme standard was chosen for several reasons. First,
that it is the most recent edition of the standard. Second, that it retains
the smallness and simplicity of R5RS, which many felt was lost in the
language's 6th version. Finally, the most recent Scheme report is the easiest
to find.

It is likely that additional features will be implemented that are not
described in the R7RS small specification. This is fine, and should be no
cause for concern so long as the specified semantics are maintained.

## Project Goals

The main goal of Ruse is to be a learning project and learning experience for
those who make and use it. It may be that it becomes a more serious effort at
some point in the future, but for now it is little more than a hobbyist's
project. It allows the contributors to experiment and stretch themselves a bit
with implementing a programming language, without the pressure of concern that
anyone will actually use it. That said, if you do use it, let us know how it
goes.

## Contributing

All discussions and contributions will be handled via GitHub Issues and Pull
Requests. If you have an idea or change you'd like to have made, open an Issue,
and possibly make a Pull Request. If you have any questions about the process,
open an Issue asking for clarification.

## Structure

Ruse is structured currently into 5 separate Rust crates.

- `ruse` is the Ruse interpreter, and is a binary.
- `libruse` is the library interface to Ruse. The CLI just uses `libruse`.
- `libruse-read` is the reading portion of the interpreter.
- `libruse-eval` is the evaluation portion of the interpreter.
- `libruse-print` is the printing portion of the interpreter.

