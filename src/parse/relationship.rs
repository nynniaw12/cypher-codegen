use nom::{bytes::complete::tag, character::complete::multispace0, sequence::{delimited, preceded}, IResult};
use super::core::{parse_identifier, parse_properties};

#[derive(Debug, PartialEq, Eq)]
pub struct RelationshipDefn {
    tag: String,
    properties: Vec<(String, String)>,
    from: String,
    to: String,
}

pub fn parse_relationship(input: &str) -> IResult<&str, RelationshipDefn> {
    let (input, _) = preceded(multispace0, tag("RELATIONSHIP"))(input)?;

    let (input, from) = delimited(
        preceded(multispace0, tag("(")),
        parse_identifier,
        preceded(multispace0, tag(")")),
    )(input)?;

    let (input, _) = preceded(multispace0, tag("-"))(input)?;
    let (input, tag_name) = parse_identifier(input)?;
    let (input, _) = preceded(multispace0, tag("-"))(input)?;

    let (input, to) = delimited(
        preceded(multispace0, tag(">(")),
        parse_identifier,
        preceded(multispace0, tag(")")),
    )(input)?;

    let (input, properties) = preceded(multispace0, parse_properties)(input)?;
    Ok((
        input,
        RelationshipDefn {
            from: from.to_string(),
            to: to.to_string(),
            tag: tag_name.to_string(),
            properties,
        },
    ))
}
