
# ToDo

A simple ToDo list cli app using Rust.


## Installation

Install the project with cargo

```bash
    git clone https://github.com/SavvyHex/todo.git
    cd todo
    cargo build
```

The binary will be created in target/debug/ and will be called todo.

## Usage/Examples

### Show

The show command will show the todo list.

```bash
    > todo show
    +----+------+-----------+
    | ID | NAME | COMPLETED |
    +----+------+-----------+
```

### Add

This command is used to add items to the todo list.

```bash
    > todo add one
    Added element one to the todo list

    > todo show
    +----+------+-----------+
    | ID | NAME | COMPLETED |
    +----+------+-----------+
    | 1  | one  | O         |
    +----+------+-----------+
```

### Delete

The delete (or del) command can be used to delete an item from the list, based on the ID of the element.

```bash
    > todo del 1
    Removed element 1 from the todo list

    > todo show
    +----+-------+-----------+
    | ID | NAME  | COMPLETED |
    +----+-------+-----------+
    | 2  | two   | O         |
    +----+-------+-----------+
    | 3  | three | O         |
    +----+-------+-----------+
    | 4  | four  | O         |
    +----+-------+-----------+
```

### Done

This command can mark an item as completed, based on the ID of the element.

```bash
    > todo done 3
    Marked element 3 as done from the todo list

    > todo show
    +----+-------+-----------+
    | ID | NAME  | COMPLETED |
    +----+-------+-----------+
    | 2  | two   | O         |
    +----+-------+-----------+
    | 3  | three | X         |
    +----+-------+-----------+
    | 4  | four  | O         |
    +----+-------+-----------+
```

### Reset

This will reset the todo list, in other words, it will clear all elements from it and the ID will start from 1.

```bash
    > todo reset
    Reset the todo list

    > todo show
    +----+------+-----------+
    | ID | NAME | COMPLETED |
    +----+------+-----------+

    > todo add another-one
    Added element another-one to the todo list

    > todo show
    +----+-------------+-----------+
    | ID | NAME        | COMPLETED |
    +----+-------------+-----------+
    | 1  | another-one | O         |
    +----+-------------+-----------+
```

### Multiple Arguments

The commands add, del and done can be used with multiple arguments, separated by space

```bash
    > todo add two three four
    Added element two to the todo list
    Added element three to the todo list
    Added element four to the todo list

    > todo show
    +----+-------+-----------+
    | ID | NAME  | COMPLETED |
    +----+-------+-----------+
    | 1  | one   | O         |
    +----+-------+-----------+
    | 2  | two   | O         |
    +----+-------+-----------+
    | 3  | three | O         |
    +----+-------+-----------+
    | 4  | four  | O         |
    +----+-------+-----------+
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

