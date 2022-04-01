## Genenv - A simple cli to generate .env.example files

![image](https://user-images.githubusercontent.com/68228472/161290781-318ab89c-0418-4418-88e9-ad79ffd5e791.png)

## Installation
```
cargo install genenv
```

## Usage examples:

Generate .env.example in current directory
```
genenv
```

Generate .env.example in specified directory
```
genenv path/to/directory
```

Generate .env.example with custom value placeholder
```
genenv --value="placeholder"
```

Generate .env.examples recursively from current directory
```
genenv -r
```
