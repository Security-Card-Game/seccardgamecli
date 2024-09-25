# GUI

The gui is implemented using [egui](https://github.com/emilk/egui). In September 2024 a switch to [ICED](https://iced.rs)
was evaluated and not performed. 

## The Command concept

### Why

In the first implementations of the UI each UI action was handled in place. This lead to duplicated code, super special
evaluations in some cases and thanks to its entanglement with UI components a lot of barely testable code. While researching
ICED and its ELM inspired mode, an idea began to shape how to make it better.

### What

Each user action triggers a command. The command is handled at a central place inside the `actions` module. Sound simple,
right? As this is rust it had some interesting side effects.

### How

In a first implementation the UI component called the command handler directly. However, this had some issues when
putting a callback into the card windows. Rust complained a lot about borrowing and lifetimes. In the end another
approach was implemented.

To understand this approach, lets look first at egui and the drawing of the UI. With egui, every time the UI need to be redrawn the `update` method if its App trait is called. Therefore, a button
press triggers a redraw and then the `update` method. Taken together with the fact, that **every** action triggers
a redraw, we either have to handle exactly **one** command or **none** at redraw. This is the reason each update cycle
first calls the `self.process_command()` method, before drawing the UI.

Inside the `self.process_command()` function the command is read from `self.command`, `self` is updated and the `self.command`
is set to `None`. Also, the game action result is evaluated and the correct message to display is set.

But how does the user action write the command into `self`? In rust, we cannot put simply a reference to `self` into the card and mutate the
`command` field in the `SecCardGameApp` struct. But, if we put the `Option<Command>` into a `Rc<RefCell<Option<Command>>>`. Rc allows
us to copy a pointer to a card window and RefCell in turn designates it as a mutable memory location with dynamically checked borrow rules.
Every time we now need to set the command field we can do this with 

```rust
let mut cmd = self.command.borrow_mut();
*cmd = ...
```

and also rust stops complaining about multiple borrows and lifetimes.


## Why not ICED

While ICED looks really promising it had two drawbacks when looking into it in September 2024. First of all,
not much documentation. Secondly, much more sever: No easy support for an application with windows in windows (e.g. the cards)
and also no simple drag & drop support. After spending around a workday I've decided not to move to ICED. //maschmi