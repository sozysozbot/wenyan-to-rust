While the [spec.html](https://wy-lang.org/spec.html) on the official website as of 11:07 July 27, 2020 (UTC) gives you a good enough approximation on how wenyan-lang works, there nevertheless seems to be some discrepancies between the spec.html and the current official implementation. Here I will list what I encountered while writing this transpiler.

## keywords lacking in spec.html

* 或若
* 若其然者
* 若其不然者
* 乃止是遍

## treatment of name_single_statement following statements
While treated as part of a statement in many syntactic rules, such as

```
reference_statement         : '夫' data ('之' (STRING_LITERAL|INT_NUM|'其餘'|IDENTIFIER|'長'))? name_single_statement? ;
```

there actually is no need to handle `name_single_statement` to be part of a `reference_statement`.

```
加二以四。夫「丙」。名之曰「戊」曰「己」。
```

compiles to

```
const _ans1 = 2 + 4;
const _ans2 = BING3;
var WU4 = _ans1;
var JI3 = _ans2;
```

, and thus it must be that the optional `name_single_statement` can just as validly treated as a separate entity that can be replaced by a `name_single_statement`.

However, note that

```
加二以三。有數九。名之曰「酒數」曰「亜」。
```

compiles to

```
const _ans1 = 2 + 3;
var JIU3SHU4 = 9;
```

and that `有數九。` does not compile. Thus, while the spec says

```
init_define_statement       : '有' TYPE data (name_single_statement)? ;
```

The `name_single_statement` is not optional; and if a `name_multi_statement` comes, it is cropped to a `name_single_statement` and the remaining identifiers are silently ignored.

## other differences

### mod_math_statement

spec:
```
mod_math_statement          : '除' (INT_NUM|FLOAT_NUM|IDENTIFIER|'其') preposition (INT_NUM|FLOAT_NUM|IDENTIFIER) POST_MOD_MATH_OP? ;
```

Actual behavior: seems to allow `INT_NUM|FLOAT_NUM|IDENTIFIER|'其'` in the second argument.

### assign_statement

spec:
```
assign_statement            : '昔之' IDENTIFIER ('之' (INT_NUM|STRING_LITERAL|IDENTIFIER))? '者' (('今' ((data ('之' INT_NUM)?)|'其') '是矣')|'今不復存矣') ;
```

Actual behavior: seems to allow `'之' (INT_NUM|STRING_LITERAL|IDENTIFIER|'長')` in the second argument.

### for_enum_statement

spec:
```
for_enum_statement          : FOR_START_ENUM  (INT_NUM|IDENTIFIER)  FOR_MID_ENUM statement* FOR_IF_END ;
```

Actual behavior: seems to allow `INT_NUM|IDENTIFIER|'其'` in the second argument.

### unary_if_expression

spec:
```
unary_if_expression         : data|(IDENTIFIER '之'('長'|STRING_LITERAL|IDENTIFIER))|'其' ;
```

Actual behavior: seems to allow `'長'|STRING_LITERAL|IDENTIFIER|INT_NUM`.
