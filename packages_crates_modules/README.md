// Packages : A crgo feature that ets your build, test and share crates
// Crates: A tree or module that produces a librry or executable
// Modules and use: Let you control the organisation, scope and privacy of paths
// Paths: A way of naming an item such as struct, function, or a module

// Packages and crates:

Crates -
Crates are the smallest amount of code the Rust compiler will consider at a time. Crates can contain modules and modules may be defined another files that get compiled into the crate.

Crates can either be binary crate or a library crate. Binary crates are can be compiled into executable files which hold the main function.

Library crates dont have the main function and dont compile into an executable - they instead define functionality to be shared with multiple projects. - The terms crate is often referring to a library crate within Rust

Packages -
A package is a bundle of one or more creates that provides a set functionality. A package contains a Cargo.toml file that describes how to build those crates.

Cargo is actually a package that contains the command line tools we use to build code.

A package can contains as many binary crates as you like, but at most can only have one library crate. A package must contain at least one crate, whether a library or binary.

src/main.rs is the crate root of a binary crate

if a package contains src/lib.rs the package contains a library crate with the same name as the package and src/lib.rs is the crate root of the package.

Cargo passes the crate root filed to rustc to buid the library or binary. If the package only has src/main.rs it only contains a binary crate. If a package has src/main.rs and src/lib.rs it has 2 crates. A package can have mmulitle binary crates by placing files in the src/bin directory. Each file will be a separte binary crate.

// Modules: Privacy and scope with modules:
Paths allow you to name items,
'use' keyword to bring a path into scope
'pub' keyword to make items public

How the compiler works:

- Starts from the crate root (usually src/lib.rs for a library crate an src/main.rs for a binary crate) for code to compile
- Declaring modules. In the crate root you can declare new modules(such as garden by delcaring 'mod garden'). The compiler will look for the module's code in these places: inline within curly braces insead of the semi colon; in the file src/garden.rs; in the file src/garden/mod.rs.
- Declaring submodules - In any files other than the crate root you can declare sub-modules
- For example you might declare 'mod vegetables' in src/garden.rs. The compiler will look fro the submodules code within the directory named for the parent module in the following place: Inline, directly following 'mod vegetables', within curly brackets instead of the semi colon. In the file src/garden/vegetables.rs. In the file src/garden/vegetables/mod.rs.
- Paths to code in modules. Once a module is declared in your crate you can access the code within that module from anywhere withing that crate, providing it has the correct privacy settings/rules allow. For example: the Asparagus type in the garden module would be found with the following syntax 'crate::garden::vegetables::Asparagus'
- Private vs Public: Code within a module is pravte from its parent module by default. To make it public, you must declare it with 'pud mod' instead of 'mod'. To make items within a module public, use 'pub' before their declarations too.
- 'use' keyword: Within a scope, the use keywords provides a shortcut to an item to remove long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can use the 'use' keyword 'use crate::garden::vegetables::Asparus' to just call 'Asparagus' in the future.

Example:'backyard' directory in this file.

- The crate root file in this directory is src/main.rs
- the 'pub mod garden' tells the tells the compiler to include the code it finds in src/garden.rs. This file contains the code 'pub mod vegetables', telling the compiler to include the code in src/garden/vegetables.rs too.
- vegetables contains the struct 'Asparagus'. #[derive(Debug)]
  pub struct Asparagus {}

Grouping related code in Modules:
Modules let us organise code within a create for readbility and easy reuse. Modules also et us control the privacy of items because modules and their items are set to private by default. Private items are internal implementations not available for outside use. We can choose to make modules and items within them public, which exposes them and allows external code to depend on them.

We will use the restaurant crate to create an example. Funtion signatures will be defined but the body will be empty to the example.

Created new librayr crate using the line 'cargo new restaurant --lib'

In src/lib.rs we defined a module using the 'mod' keyword followed by the name of the module (in this case 'front_of_house'). Inside this module we can place other modules ('hosting' and 'serving'). Modules can also hold definitions for othe ritems such as structs, enums, constants, traits, and functions.

By creating modules we can we can group related definitions together and named how they are related. Programmers using the code can navigate bades on modules instead of scanning the whole code base for a function, aking it easier to find the code base relevant to them. Programmers adding new functionality woulso also know where to add new code to keep the program organised.

src/main.rs and src/lib.rs are both crate roots. This is because the contents of either of these two files form the module named 'crate' at the root of the crate's module structure, known as the module tree.

The module tree for this example is:
crate
└── front_of_house
├── hosting
│ ├── add_to_waitlist
│ └── seat_at_table
└── serving
├── take_order
├── serve_order
└── take_payment

From this tree we can see how some modules nest inside other modules, such as 'hosting' nesting inside 'front_of_house'. The tree also shows some modules are siblings, meaning they're defined in the same module. 'hosting' and 'serving' are siblings hosted within the 'front_of_house' module. If module A is inside module B, we call module A the child and module B the parent. The entrie module tree is rooted under the implicit module 'crate'.

The module tree if very similar to the file system in a computer. Just like directories in a file system, modules are used to organise code.
