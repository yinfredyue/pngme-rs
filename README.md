# pngme-rs

pngme-rs: a CLI program that allows you to hide secret messages in PNG files. 

You can

- Encode a message into a PNG file

    ```
    cargo run -- encode ./dice.png ruSt 'A secret message!'
    ```

- Decode a message stored in a PNG file

    ```
    cargo run -- decode ./dice.png ruSt
    ```

- Remove a message from a PNG file

    ```
    cargo run -- remove ./dice.png ruSt
    ```

- Print a list of PNG chunks that can be searched for messages

    ```
    cargo run -- print ./dice.png
    ```

## Reference

https://picklenerd.github.io/pngme_book/introduction.html