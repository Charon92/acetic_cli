<h1 align="center">Acetic</h1>
<p align="center">A Rust image steganography and processing CLI</p>

### Why?
Initially this was supposed to be a Rust version of a c++ library I had created, that I could use in different 
projects but requirements soon dictated that more and more functionality be added, so it's becoming more of a "various 
image processes" library instead.

Currently, I am still fleshing it out. I could 100% use other libraries for lots of these things but
1. I wanted to learn some more Rust
2. I wanted to learn more about how certain image processes work (Gaussian blur, edge finding etc.)
3. Most libraries that provide these sorts of functionalities are _massive_ (Looking at you OpenCV)

### How to use?
Once built there will be an executable called `acetic` that can be called like so:

```shell
acetic edge ./files/image.jpg
```

```shell
acetic encode ./files/image.png "Some text that will be encoded into an image file"
```

#### Args
`filepath` - A relative path to the file you want to process.

`process` - What process you would like to do (encode, decode, edge, phash etc.)

`data` - For `encode` only. The text to be encoded

#### Currently supported processes (as of Nov 30 2022)
- edge
- phash
- encode (png)

Note: Images saved for edge detection use the filename format `{UTC NOW}_{ORIGINAL FILE NAME}` and will save in the 
same directory that the file is in.

### Development notes

The structure is subject to change. I am still learning Rust and how best to structure things.

Currently, the steganographic operations will be split by file type (PNG, JPEG) due to the fact that the operations will
be different for each file type in order to achieve the best results.
