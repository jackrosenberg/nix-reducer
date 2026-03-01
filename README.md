Usage: 

1. run on a file: `nix-reduce /etc/nixos/mods/panglolin.nix`
2. run on a folder: `nix-reduce /etc/nixos/mods/`


Ideas: 

Eta reduce 
e.g.: 
f = a: map a === f = map

Map/fold compose
e.g.: 
map ()


TODO: 

- [] DOCUMENT EACH FUNCTION, LAWRENCE STYLE
- [] -find rust parser for nix language- write my own (for funzies)
- [] check if nix is compiled to an intermediate lang
- [] mkEnableOption, and meta.maintainers
- [] look up common patterns on hoogle/github

Resources used:
https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/
https://utrechtuniversity.github.io/infob3tc/downloads/MAIN.pdf
