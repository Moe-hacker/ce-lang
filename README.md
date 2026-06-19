# WIP:
Nothoing implemented yet, just some design ideas.
# About CE-lang:
Just a cute error handling extension for C, with no syntax breaking, and the tail will never wag the cat.
We will just have a new happy face `:>` for default handling, and a sad face `:<` for error handling, and `#[[ce_foo()]]` for code generation.     
These syntax will be translated to C code, you can use CE-lang for error handling, CE-generator transform it to C, and you compile/run/debug the generated C code.      
# Why CE-lang:
In ruri:      
```c
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept4), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(access), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
```
Too ugly you see.     
seccomp_rule_add() uses va_args, so if you don't use these complex code, you can only use a macro. But in cross-arch ci, it will bomb to TLE, as the performance of macro is not good, and qemu is slow.      
So, I want a:     
```ce
#[[ce_reg(seccomp_rule_add, int, _<0)]]
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0) :<;
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept4), 0) :<;
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(access), 0) :<;
```
It's better, right?    
And it will be auto expanded to code like this:    
```c
if(seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0) != 0) {
    warning("seccomp_rule_add", __FILE__, __LINE__, res, errno);
}
```
So that's CE lang, C with better Error handling/Cute Error handling.    
The tail will never wag the cat.    
So ce-lang will never break c syntax, except the old `:>` as `]` design. But as ce will translate .ce to c, and if you only use `:>` as happy face in .ce, that's fine.    
In one word, CE-lang makes a zipped error handling in C, and it's kawaii.      