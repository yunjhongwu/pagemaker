use pagemaker::chart::{Data, Pair, ScatterPlot};
use pagemaker::{color, Columns, Field, Page, Row, Table, Text, TextObject};

fn main() {
    let page = Page::default().set_title("Demo");

    let text = Text::new("Hello World!")
        .set_text_color(color::RED)
        .set_font_size(36)
        .clone();
    let page = page.append_text(text);

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

    let data1 = Data::from_vec(vec![
        Pair::new(0.1, 0.2),
        Pair::new(0.2, 0.3),
        Pair::new(0.3, 0.4),
        Pair::new(0.4, 0.5),
        Pair::new(0.5, 0.6),
        Pair::new(0.6, 0.7),
        Pair::new(0.7, 0.8),
        Pair::new(0.8, 0.9),
        Pair::new(0.9, 1.0),
    ])
    .set_label("data2");
    let data2 = Data::from_vec(vec![
        Pair::new(0.1, 0.1),
        Pair::new(0.2, 0.2),
        Pair::new(0.3, 0.3),
        Pair::new(0.4, 0.4),
        Pair::new(0.5, 0.5),
        Pair::new(0.6, 0.6),
        Pair::new(0.7, 0.7),
        Pair::new(0.8, 0.8),
        Pair::new(0.9, 0.9),
    ])
    .set_label("data2");

    let chart = ScatterPlot::default()
        .set_title("Demo Chart")
        .set_x_label("x")
        .set_y_label("y")
        .add_data(data1)
        .add_data(data2);
    page.append_text(columns)
        .append_chart(chart)
        .save_to_html(std::path::PathBuf::from("examples/demo.html"))
        .unwrap();
}
