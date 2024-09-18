use nom::{bytes::complete::tag, character::complete::{multispace0, multispace1}, sequence::preceded, IResult};
use super::core::{parse_identifier, parse_properties};

#[derive(Debug, PartialEq, Eq)]
pub struct NodeDefn {
    tag: String,
    properties: Vec<(String, String)>,
}

pub fn parse_node(input: &str) -> IResult<&str, NodeDefn> {
    let (input, _) = tag("NODE")(input)?; // tagged by NODE
    let (input, tag) = preceded(multispace1, parse_identifier)(input)?; // pull in its main tag
    let (input, properties) = preceded(multispace0, parse_properties)(input)?; // parse properties

    Ok((
        input,
        NodeDefn {
            tag: tag.to_string(),
            properties,
        },
    ))
}
