<img src="/public/assets/logo.png" width="400" align="right">

# putater
a virtual computer, for the funsies!

## what is this?
putater is a virtual computer featuring a 256x256 16-bit color display,
a 16-bit CPU, and a simple instruction set. all written in typescript!

## how do i use it?
documentation is ~coming soon~ [in the works](DOCUMENTATION.md)! for now, you can load stuff in the
`examples` folder by entering `load examples/01` in the console.

## how do i build it?
```bash
git clone https://github.com/undrscre/pot80.git
npm install
npm run build
```
    
## system specs?
despite being a virtual computer, putater has some pretty beefy specs! such as
- 256x256, 256 color display
- 16-bit CPU
- 2kb of memory
- 16kb of program data


## roadmap
here's a list of features we plan to implement:
- [ ] putacode (a simple compiled language for potasm)
    - [ ] variables
    - [ ] functions
    - [ ] conditionals
    - [ ] loops
- [x] putASM
- [ ] putater
    - [x] basic CPU
    - [x] basic memory
    - [x] basic terminal (for I/O)
    - extensions
        - [ ] more colors
        - [ ] pixel display
        - [ ] sound card
    
