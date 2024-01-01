# `utils/gen/wgl`

this module implements a parser for `/usr/include/GLES3/gl3.h`. \
the steps for parsing:

- filter (only selecting the lines interesting for parsing)
- tokenizer (tokenizing a line)
- parser (parse the tokens)

all the modules are implemented as iterators, the data generated is then used for codegen

```rs
let api = gen_wgl::Api::default();
```
