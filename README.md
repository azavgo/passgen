# Utility to generate passwords
## Features: 
1. Generates pseudo-random passwords; 
1. Password length is between 1 and 256 ASCI characters, default is 8 characters; 
1. ASCI characters selected for the password generation: 33 = "!", 35 = "#" to 38 = "&", 40 = "(" to 93 = "]", 95 = "_", 97 = "a" to 125 = "}". All other ASCI characters are excluded, e.g. quotation marks, grave accent, caret etc. 

## How to use passgen utility
1. Type ./passgen in a terminal and press Enter to generate a default 8-character password: 
```
$ ./passgen
$ Password 8 characters length (default): iSPW}y\1
```
1. In a terminal type ./passgen and add a length of the password, e.g a 12-character password: 
```
$ ./passgen 12
$ Password 12 characters length: e>C:kl|2&xt7
```

1. For help, type ./passgen ?, alternatively type ./passgen --help in a terminal: 
```
$ ./passgen ?
$ usage: passgen [? | --help] [n=<number between 0 and 255, length of the password>]
```
