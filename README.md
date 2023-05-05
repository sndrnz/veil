# Veil

Veil is a command line tool built with Rust for encrypting and decrypting files. It provides a simple and secure way to protect your sensitive data from prying eyes.

## Installation

To install Veil, you will need to have Rust installed on your machine. Once you have Rust installed, you can install Veil by running the following command:

```shell
git clone https://github.com/sndrnz/veil.git
cd veil
cargo install --path .
```

## Usage

Veil is a command line tool that allows you to encrypt and decrypt files. Here is a summary of the available options:

```shell
Usage: veil [OPTIONS] --input <FILE>

Options:
  -i, --input <FILE>
          Input file

  -o, --output <FILE>
          Output file

  -d, --decrypt
          Decrypt input

  -r, --remove
          Remove input file

  -f, --input-format <FORMAT>
          Input format

          [default: bytes]

          Possible values:
          - bytes:  Output as bytes
          - base64: Output as Base64 encoded string

  -F, --output-format <FORMAT>
          Output format

          [default: bytes]

          Possible values:
          - bytes:  Output as bytes
          - base64: Output as Base64 encoded string

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

To encrypt a file, run the following command:

```
veil -i input_file.txt -o output_file.txt
```

To decrypt a file, add the `-d` option:

```
veil -i input_file.txt -o output_file.txt -d
```

By default, Veil outputs the encrypted or decrypted file as bytes. You can change this using the `-f` or `-F` options. For example, to output the file as a Base64 encoded string, use the following command:

```
veil -i input_file.txt -o output_file.txt -f bytes -F base64
```

You can also use the `-r` option to remove the input file after it has been encrypted or decrypted:

```
veil -i input_file.txt -o output_file.txt -r
```

## Contributing

Contributions to Veil are welcome and encouraged! If you find a bug, have a feature request, or want to contribute code, please submit an issue or pull request on the [GitHub repository](https://github.com/sndrnz/veil).

## License

Veil is licensed under the [MIT License](https://github.com/your_username/veil/blob/main/LICENSE).
