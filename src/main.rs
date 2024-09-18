use cypher_codegen::parse::node::parse_node;
use cypher_codegen::parse::relationship::parse_relationship;
use cypher_codegen::parse::core::{parse_identifier, parse_property, parse_properties};

fn main() {
    println!("Testing parse_identifier:");
    println!("{:?}", parse_identifier("xx xx"));
    println!("{:?}", parse_identifier("valid_identifier123 rest"));
    println!("{:?}", parse_identifier("123invalid"));

    println!("\nTesting parse_property:");
    println!("{:?}", parse_property("name: string"));
    println!("{:?}", parse_property("age : integer"));
    println!("{:?}", parse_property("invalid"));

    println!("\nTesting parse_properties:");
    println!("{:?}", parse_properties("{ name: string, age: integer }"));
    println!("{:?}", parse_properties("{ }"));
    println!("{:?}", parse_properties("{ invalid }"));

    println!("\nTesting parse_node:");
    println!(
        "{:?}",
        parse_node(
            "NODE
           Person
           { name: string, age: integer }"
        )
    );
    println!("{:?}", parse_node("NODE InvalidNode"));

    println!(
        "{:?}",
        parse_relationship(
            "RELATIONSHIP (Person)-ACTED_IN->(Movie) {
                role: string,
                salary: float
            }"
        )
    );
}

