# Image to ASCII generator

Written in rust, my main goal with this project was to learn [Rust](https://www.rust-lang.org/).

**The algorithm works as following:** 
1. accepts image path with extension(for example: image.jpg) as a command line argument.
2. convert the image to grayscale.
3. split the grayscale image into r(rows) X c{columns}
4. calculate the average brightness of each tile and assign the suitable ASCII character.
5. assemble the ASCII characters and print the final image.
