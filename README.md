Usage: 

1. run on a file: `nix-reduce /etc/nixos/mods/panglolin.nix`
2. run on a folder: `nix-reduce /etc/nixos/mods/`

```ebnf
topLevel ::= (attrSet :)? attrSet

attrSet  :: {  attr*  };
attr 	 :: ID = value;
value    :: ??

ANY      	:: .|\n
ID       	:: [a-zA-Z\_][a-zA-Z0-9\_\'\-]*
INT      	:: [0-9]+
FLOAT    	:: (([1-9][0-9]*\.[0-9]*)|(0?\.[0-9]+))([Ee][+-]?[0-9]+)?
PATH_CHAR	:: [a-zA-Z0-9\.\_\-\+]
PATH     	:: {PATH_CHAR}*(\/{PATH_CHAR}+)+\/?
PATH_SEG 	:: {PATH_CHAR}*\/
HPATH    	:: \~(\/{PATH_CHAR}+)+\/?
HPATH_START :: \~\/
SPATH     	:: \<{PATH_CHAR}+(\/{PATH_CHAR}+)*\>
URI      	::  [a-zA-Z][a-zA-Z0-9\+\-\.]*\:[a-zA-Z0-9\%\/\?\:\@\&\=\+\$\,\-\_\.\!\~\*\']+
```

Ideas: 

Eta reduce 
e.g.: 
`f = a: map a` === `f = map`

Map/fold compose
e.g.: 
`map a (map b xs)` === `map (a . b) xs`


TODO: 

- [] DOCUMENT EACH FUNCTION, input output example
- [] -find rust parser for nix language- write my own (for funzies)
- [] check if nix is compiled to an intermediate lang
- [] mkEnableOption, and meta.maintainers
- [] look up common patterns on hoogle/github

Resources used:

https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/

https://utrechtuniversity.github.io/infob3tc/downloads/MAIN.pdf

~~https://github.com/NixOS/nix/blob/master/src/libexpr/lexer.l~~

https://git.lix.systems/lix-project/lix/src/branch/main/lix/libexpr/parser/grammar.hh

https://nix.dev/manual/nix/2.28/language/index.html

https://ericlippert.com/2012/06/08/red-green-trees/

https://rust-analyzer.github.io/book/contributing/syntax.html

https://m.youtube.com/watch?v=n5LDjWIAByM&pp=ygUUcmVkIGdyZWVuIHBhcnNlIHRyZWU%3D
