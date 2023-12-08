# pagemaker
A simple library to rend tables in HTML.

[Demo](examples/demo.html)


```rust
let page = Page::new().set_title("Demo");

let header = Row::new()
    .add_field(Field::new("Name"))
    .add_field(Field::new("Age"))
    .add_field(Field::new("Height"));
let table = Table::new(header).set_title("Demo Table");

let table = table
    .add_row(
        Row::new()
            .add_field(Field::new("A"))
            .add_field(
                Field::new("20")
                    .set_background_color(color::YELLOW)
                    .set_text_color(color::CYAN),
            )
            .add_field(Field::new("1.80")),
    )
    .set_text_color("#00ff00")
    .add_row(
        Row::new()
            .add_field(Field::new("B"))
            .add_field(Field::new("19").set_background_color(color::RED))
            .add_field(Field::new("1.70"))
            .set_background_color(color::BLUE),
    );
let page = page.append(table);

let table2 = Table::new(
    Row::new()
        .add_field(Field::new("Id"))
        .add_field(Field::new("Time"))
        .add_field(Field::new("Value")),
)
.set_title("Demo Table 2")
.add_row(
    Row::new()
        .add_field(Field::new("1"))
        .add_field(Field::new("2021-01-01 00:00:00"))
        .add_field(Field::new("1.0")),
)
.add_row(
    Row::new()
        .add_field(Field::new("2"))
        .add_field(Field::new("2021-01-01 00:00:01"))
        .add_field(Field::new("2.0"))
        .set_text_color(color::GREEN),
);

page.append(table2)
    .save_to_html(std::path::PathBuf::from("examples/demo.html"))
    .unwrap();
```