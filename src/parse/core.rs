use nom::{
    bytes::complete::take_while1,
    character::complete::{alpha1, char, multispace0},
    combinator::{opt, recognize},
    multi::separated_list0,
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    preceded(
        multispace0,
        recognize(pair(
            alpha1,
            opt(take_while1(|c: char| c.is_alphanumeric() || c == '_')),
        )),
    )(input)
}

pub fn parse_property(input: &str) -> IResult<&str, (String, String)> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = preceded(multispace0, char(':'))(input)?;
    let (input, property_type) = preceded(multispace0, parse_identifier)(input)?;
    Ok((input, (name.to_string(), property_type.to_string())))
}

pub fn parse_properties(input: &str) -> IResult<&str, Vec<(String, String)>> {
    delimited(
        char('{'),
        separated_list0(
            preceded(multispace0, char(',')),
            preceded(multispace0, parse_property),
        ),
        preceded(multispace0, char('}')),
    )(input)
}
