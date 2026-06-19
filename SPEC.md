CE-lang Specification (Draft)

Version: 0.0 (WIP)

Status: Experimental

---

1. Overview

CE-lang (Cute Error / Compact Error Language) is a minimal, source-to-source extension for C.

CE-lang introduces a small amount of syntax for expressing error-handling intent while preserving the semantics and execution model of C.

A CE-lang compiler translates ".ce" files into ordinary ".c" files. The generated C source is the canonical compilation target.

CE-lang does not define a new runtime, ABI, object model, type system, or execution semantics.

Its sole purpose is to reduce boilerplate around non-core error handling paths.

---

2. Design Principles

2.1 The tail shall never wag the cat.

CE-lang exists to serve C.

C remains the source of truth.

CE-lang must not:

- redefine the C execution model;
- alter ABI behavior;
- introduce hidden runtimes;
- require compiler modifications;
- replace existing C debugging workflows.

Generated C code SHALL be readable and reviewable.

Users compile, debug, profile, and deploy the generated C code.

---

2.2 Minimal syntax surface

CE-lang only introduces:

- ":<"
- ":>"
- "#[[ce_xxx(... )]]"

No other syntax extensions are defined.

---

2.3 Explicit error knowledge

CE-lang SHALL NOT infer failure conditions.

Functions participating in CE error handling MUST be registered via "ce_reg".

Unregistered functions SHALL result in CE compilation errors.

---

3. Translation Model

Input:

.ce

↓

CE Generator

↓

.c

↓

Existing C toolchain

clang / gcc / tcc / ...

No modifications to existing C compilers are required.

---

4. Error Registration

CE-lang requires explicit declaration of:

- return type,
- failure condition.

Registration syntax:

#[[ce_reg(function, return_type, failure_expression)]]

Example:

#[[ce_reg(open, int, _ < 0)]]

Meaning:

For calls to "open(...)", the return value SHALL be treated as failure whenever:

(result < 0)

The identifier "_" refers to the evaluated return value.

---

5. Default Handlers

Optional default handlers may be registered.

5.1 Panic handler

Syntax:

#[[ce_pan(function, handler)]]

Example:

#[[ce_pan(open, panic)]]

Meaning:

When failure is detected, invoke:

panic(...)

unless overridden locally.

---

5.2 Default handler

Syntax:

#[[ce_dft(function, handler)]]

Example:

#[[ce_dft(open, log)]]

Meaning:

Invoke:

log(...)

after evaluation unless overridden locally.

Default handlers execute regardless of success or failure.

---

6. Error Operators

6.1 Panic operator (":<")

Syntax:

expression :<;

Meaning:

Evaluate expression.

If its registered failure condition evaluates true:

invoke panic logic.

Otherwise:

do nothing.

Example:

int fd = open("file", O_RDONLY) :<;

Equivalent C:

int fd = open("file", O_RDONLY);

if (fd < 0) {
    panic("open", __FILE__, __LINE__, fd, errno);
}

using registered handlers.

---

6.2 Default operator (":>")

Syntax:

expression :>;

Meaning:

Invoke default handling logic after expression evaluation.

Example:

int fd = open("file", O_RDONLY) :>;

Equivalent:

int fd = open("file", O_RDONLY);

log("open", __FILE__, __LINE__, fd, errno);

---

6.3 Combined operators

Syntax:

expression :<, :>;

Meaning:

1. Evaluate expression.
2. Invoke panic logic if failure occurs.
3. Invoke default logic.

Equivalent ordering:

evaluate
↓
panic check
↓
default handler

---

7. Local Handler Override

Users may replace registered handlers inline.

7.1 Local panic override

Syntax:

expression :<
{
    /* custom panic */
};

Example:

open(path, O_RDONLY) :<
{
    fprintf(stderr, "open failed\n");
    exit(1);
};

---

7.2 Local default override

Syntax:

expression :>
{
    puts("done");
};

---

7.3 Combined override

Syntax:

expression :<
{
    ...
}
:>
{
    ...
};

Local handlers SHALL take precedence over registered handlers.

---

8. Temporary Result Semantics

CE Generator SHALL guarantee that expressions are evaluated exactly once.

For example:

foo() :<;

MUST NOT expand to:

if (foo() < 0)

if that would evaluate "foo()" multiple times.

Instead, CE Generator SHALL introduce temporaries as necessary.

Example:

typeof(foo()) __ce_tmp = foo();

if (__ce_tmp < 0) {
    ...
}

or equivalent implementation.

---

9. Handler Context

Generated handlers MAY expose implementation-defined metadata.

Recommended context includes:

__FILE__
__LINE__
errno
function name
return value

Example:

warning(
    "open",
    __FILE__,
    __LINE__,
    ret,
    errno
);

The exact handler signature is implementation-defined.

---

10. .hce Files

".hce" ("happy c ending/handle c error") files provide CE registrations.

They are declarative metadata files.

They SHALL only contain:

#[[ce_reg(...)]]
#[[ce_pan(...)]]
#[[ce_dft(...)]]

No other CE directives are permitted.

Example:

#[[ce_reg(open, int, _ < 0)]]

#[[ce_pan(open, panic)]]

#[[ce_dft(open, log)]]

C declarations remain in ordinary headers.

Example:

typedef ...
#define ...
extern ...

---

11. Generated Code Visibility

Generated C source SHALL be inspectable.

Users are encouraged to review diffs between:

.ce

and

.c

before deployment.

CE-lang intentionally exposes its translation output.

---

12. Undefined and Implementation-Defined Behavior

CE-lang does not eliminate C undefined behavior.

The generator may introduce implementation-defined behavior in edge cases.

Users remain responsible for validating generated output.

CE-lang provides convenience, not safety guarantees.

---

13. Non-Goals

CE-lang SHALL NOT provide:

- exceptions;
- stack unwinding;
- garbage collection;
- RAII;
- coroutines;
- algebraic effects;
- type inference;
- ownership systems;
- runtime reflection;
- automatic error propagation;
- replacement of C compilers.

CE-lang is not a safer C.

CE-lang is not a new programming language.

CE-lang is merely a compact notation for expressing error-handling intent.

---

14. Philosophy

CE-lang makes unhappy paths smaller.

It does not make C disappear.

The generated C is the real program.

The tail never wags the cat.