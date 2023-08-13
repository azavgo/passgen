# Utility to generate passwords
## Features: 
1. Generates pseudo-random passwords; 
1. Password length is between 1 and 256 symbols, default is 8 symbols; 
1. ASCI characters selected for the password generation: 33 = "!", 35 = "#" to 38 = "&", 40 = "(" to 93 = "]", 97 = "a" to 125 = "}". All other ASCI characters are excluded, e.g. quotation marks, grave accent, caret etc. 

## How to use passgen utility
1. Type passgen in the terminal and press Enter to generate 8-symbol password: 
```
$ passgen
$ Password 8 symbols length: iSPW}y\1
```
1. In the terminal type passgen and add a length of the password: 
```
$ passgen 12
$ Password 12 symbols length: e>C:kl|2&xt7
```