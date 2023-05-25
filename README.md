# RandomnessTestingOfKeys

The program implements a practical task  "Testing keys for randomness according to the FIPS-140 standard."

Here is an example of the program output:

```
Monobit test result: 9970
Monobit test passed?: true
Maximum series length test result: 16 and 12
Maximum series length test is passed?: true
Poker test result: 25.100800000000163
Poker test is passed?: true
Serial test result for one: [2580, 1240, 596, 299, 145, 164]
Serial test result for zero [2516, 1269, 595, 327, 169, 148]
Serial test is passed?: true

```

## Code Explanation

- ` monobit_test `: counts the number of ones in the sequence and checks if it falls within a specified range.

- `maximum_series_length_test`: checks the length of consecutive series of ones and zeros in the sequence and verifies if they are below a certain threshold.

- `poker_test`: splits the sequence into blocks of four characters and counts the occurrences of each block. Calculates a test statistic using the Pokers formula and checks if it falls within a specific range.

- `serial_test`: counts the lengths of consecutive series of ones and zeros in the sequence and compares them against predefined intervals to determine if they are within the expected range.

The code generates a random sequence, performs these tests on the sequence, and outputs the test results.

## Running the Program on the site play.rust-lang.org.

To run your code on play.rust-lang.org, you can follow these steps:

Go to the website [play.rust-lang.org](https://play.rust-lang.org/) .

Clear the existing code on the page.

Paste your code into the empty field.

Click the "Run" button.

Your code will be compiled and executed in the output window on the right. The execution result will be displayed in that area.


## Running the Program on the computer

To run the program, execute the following steps:

1. Ensure you have the Rust programming language installed.

2. Open a terminal or command prompt.

3. Navigate to the directory containing the source code file.

4. Compile the program by running the command: `cargo build`.

5. Run the program by executing the command: `cargo run`.



## License

This program is released under the [MIT License](LICENSE).

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

