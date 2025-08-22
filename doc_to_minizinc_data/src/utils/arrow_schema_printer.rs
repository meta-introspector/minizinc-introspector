use arrow::datatypes::{DataType, Schema};

pub fn print_arrow_schema_fields(schema: &Schema, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    for field in schema.fields() {
        println!("{}  - Name: {}, Data Type: {:?}", indent, field.name(), field.data_type());

        match field.data_type() {
            DataType::List(field_arc) => {
                println!("{indent}    List Element:");
                print_arrow_schema_fields(&Schema::new(vec![field_arc.as_ref().clone()]), indent_level + 1);
            },
            DataType::Struct(fields_vec) => {
                println!("{indent}    Struct Fields:");
                print_arrow_schema_fields(&Schema::new(fields_vec.clone()), indent_level + 1);
            },
            _ => {
                // No children for other data types
            }
        }
    }
}
