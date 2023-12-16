use pagemaker::{color, Columns, Field, Object, Page, Row, Table, Text};

fn main() {
    let page = Page::default().set_title("Demo");

    let text = Text::new("Hello World!")
        .set_text_color(color::RED)
        .set_font_size(36)
        .clone();
    let page = page.append(text);

    let header = Row::new()
        .add_field(Field::new("Name"))
        .add_field(Field::new("Age"))
        .add_field(Field::new("Height"));
    let table1 = Table::new(header).set_title("Demo Table");

    let table1 = table1
        .add_row(
            Row::new()
                .add_field(Field::new("A"))
                .add_field(
                    Field::new("20")
                        .set_background_color(color::YELLOW)
                        .set_text_color(color::CYAN)
                        .clone(),
                )
                .add_field(Field::new("1.80"))
                .clone(),
        )
        .set_text_color(color::GREEN)
        .clone()
        .add_row(
            Row::new()
                .add_field(Field::new("B"))
                .add_field(Field::new("19").set_background_color(color::RED).clone())
                .add_field(Field::new("-1.70"))
                .set_background_color(color::BLUE)
                .clone(),
        );

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
            .set_text_color(color::GREEN)
            .clone(),
    );

    let columns = Columns::new().add_column(table1).add_column(table2);

    page.append(columns)
        .save_to_html(std::path::PathBuf::from("examples/demo.html"))
        .unwrap();
}
