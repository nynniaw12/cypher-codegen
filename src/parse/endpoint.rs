use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum Opts {
    MATCH
}

#[derive(Debug, PartialEq, Eq)]
pub enum Methods {
   CREATE,
   READ,
   UPDATE,
   DELETE
}

#[derive(Debug, PartialEq, Eq)]
pub struct OptDefn {
    opt: Opts,
    params: Vec<(String, String)>
}

#[derive(Debug, PartialEq, Eq)]
pub struct EndpointDefn {
    tag: String,
    path: String,
    opts: Vec<OptDefn>,
    methods: Vec<Methods>
}

fn parse_endpoint(input: &str) -> IResult<&str, EndpointDefn> {
    todo!()
}
