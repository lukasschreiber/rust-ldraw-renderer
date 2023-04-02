use chrono::NaiveDate;
use std::str::FromStr;
use three_d::{vec3, Matrix3, Vector3};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug)]
struct Color {
    value: u32,
}

// make traits and extend traits
#[derive(Debug)]
enum LDrawCommand {
    Comment,
    Title(String),
    Name(String),
    Author(String, Option<String>),
    License(String, String),
    LDrawOrg(LDrawType),
    Category(String),
    Keywords(Vec<String>),
    History(NaiveDate, Option<String>, String),
    BFCCertification(Option<BFCDirection>),
    SubfileReference(Color, Vector3<f32>, Matrix3<f32>, String),
    Contour(Color, Vector3<f32>, Vector3<f32>),
    Triangle(Color, Vector3<f32>, Vector3<f32>, Vector3<f32>),
    Quadrilateral(
        Color,
        Vector3<f32>,
        Vector3<f32>,
        Vector3<f32>,
        Vector3<f32>,
    ),
    OptionalContour(
        Color,
        Vector3<f32>,
        Vector3<f32>,
        Vector3<f32>,
        Vector3<f32>,
    ),
}

#[derive(Debug, PartialEq)]
enum BFCDirection {
    CW,
    CCW,
}

impl FromStr for BFCDirection {
    type Err = ();
    fn from_str(input: &str) -> Result<BFCDirection, Self::Err> {
        match input {
            "CW" => Ok(BFCDirection::CW),
            "CCW" => Ok(BFCDirection::CCW),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum LDrawType {
    Part,
    Subpart,
    Primitive,
    Primitive8,
    Primitive48,
    Shortcut,
    UnofficialPart,
    UnofficialSubpart,
    UnofficialPrimitive,
    UnofficialPrimitive8,
    UnofficialPrimitive48,
    UnofficialShortcut,
}

impl FromStr for LDrawType {
    type Err = ();
    fn from_str(input: &str) -> Result<LDrawType, Self::Err> {
        match input {
            "Part" => Ok(LDrawType::Part),
            "Subpart" => Ok(LDrawType::Subpart),
            "Primitive" => Ok(LDrawType::Primitive),
            "8_Primitive" => Ok(LDrawType::Primitive8),
            "48_Primitive" => Ok(LDrawType::Primitive48),
            "Shortcut" => Ok(LDrawType::Shortcut),
            "Unofficial_Part" => Ok(LDrawType::UnofficialPart),
            "Unofficial_Subpart" => Ok(LDrawType::UnofficialSubpart),
            "Unofficial_Primitive" => Ok(LDrawType::UnofficialPrimitive),
            "Unofficial_8_Primitive" => Ok(LDrawType::UnofficialPrimitive8),
            "Unofficial_48_Primitive" => Ok(LDrawType::UnofficialPrimitive48),
            "Unofficial_Shortcut" => Ok(LDrawType::UnofficialShortcut),
            _ => Err(()),
        }
    }
}

pub async fn test(id: &str) -> Result<(), JsValue> {
    let lines = get_subfile(&format!("{}.dat", id)).await?;
    let parsed_lines: Vec<Option<LDrawCommand>> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.trim().len() >= 1)
        .map(|(i, line)| parse_line(line.to_string(), i))
        .collect();

    let filtered_lines: Vec<&LDrawCommand> = parsed_lines
        .iter()
        .filter(|p| p.is_some())
        .map(|p| p.as_ref().unwrap())
        .collect();

    log::info!("lines: {:?}", filtered_lines);
    Ok(())
}

fn parse_line(line: String, index: usize) -> Option<LDrawCommand> {
    if line.trim().len() <= 1 {
        return None;
    }

    let tokens: Vec<&str> = line.split_whitespace().collect();
    let tail = tokens.split_first().unwrap().1;
    match tokens[0] {
        "0" => Some(parse_meta(tail.to_vec(), index)),
        "1" => Some(parse_subfile_reference(tail.to_vec())),
        "2" => Some(parse_contour(tail.to_vec())),
        "3" => Some(parse_triangle(tail.to_vec())),
        "4" => Some(parse_quadrilateral(tail.to_vec())),
        "5" => Some(parse_optional_contour(tail.to_vec())),
        _ => None,
    }
}

fn parse_optional_contour(tokens: Vec<&str>) -> LDrawCommand {
    let color = parse_color(tokens[0]);
    let x = parse_vec3(tokens[1..4].to_vec());
    let y = parse_vec3(tokens[4..7].to_vec());
    let z = parse_vec3(tokens[7..10].to_vec());
    let w = parse_vec3(tokens[10..13].to_vec());

    LDrawCommand::OptionalContour(color, x, y, z, w)
}

fn parse_quadrilateral(tokens: Vec<&str>) -> LDrawCommand {
    let color = parse_color(tokens[0]);
    let x = parse_vec3(tokens[1..4].to_vec());
    let y = parse_vec3(tokens[4..7].to_vec());
    let z = parse_vec3(tokens[7..10].to_vec());
    let w = parse_vec3(tokens[10..13].to_vec());

    LDrawCommand::Quadrilateral(color, x, y, z, w)
}

fn parse_triangle(tokens: Vec<&str>) -> LDrawCommand {
    let color = parse_color(tokens[0]);
    let x = parse_vec3(tokens[1..4].to_vec());
    let y = parse_vec3(tokens[4..7].to_vec());
    let z = parse_vec3(tokens[7..10].to_vec());

    LDrawCommand::Triangle(color, x, y, z)
}

fn parse_contour(tokens: Vec<&str>) -> LDrawCommand {
    let color = parse_color(tokens[0]);
    let x = parse_vec3(tokens[1..4].to_vec());
    let y = parse_vec3(tokens[4..7].to_vec());

    LDrawCommand::Contour(color, x, y)
}

fn parse_subfile_reference(tokens: Vec<&str>) -> LDrawCommand {
    let color = parse_color(tokens[0]);
    let file = parse_file_name(tokens[13]);

    let translation = parse_vec3(tokens[1..4].to_vec());
    let transformation = parse_mat3(tokens[4..13].to_vec());

    LDrawCommand::SubfileReference(color, translation, transformation, file)
}

fn parse_vec3(tokens: Vec<&str>) -> Vector3<f32> {
    let tokens: Vec<f32> = tokens.iter().map(|token| token.parse().unwrap()).collect();
    vec3(tokens[0], tokens[1], tokens[2])
}

fn parse_mat3(tokens: Vec<&str>) -> Matrix3<f32> {
    let tokens: Vec<f32> = tokens.iter().map(|token| token.parse().unwrap()).collect();
    Matrix3::new(
        tokens[0], tokens[1], tokens[2], tokens[3], tokens[4], tokens[5], tokens[6], tokens[7],
        tokens[8],
    )
}

fn parse_color(token: &str) -> Color {
    Color {
        value: token.parse().unwrap(),
    }
}

fn parse_file_name(token: &str) -> String {
    token.replace("\\", "/")
}

fn parse_license(tokens: Vec<&str>) -> LDrawCommand {
    LDrawCommand::License(
        tokens[0..tokens.len() - 3].join(" "),
        tokens.last().unwrap().to_string(),
    )
}

fn parse_user_name(token: &str) -> Option<String> {
    if token.starts_with("[") && token.ends_with("]") {
        Some(token[1..token.len() - 1].to_string())
    } else {
        None
    }
}

fn parse_author(tokens: Vec<&str>) -> LDrawCommand {
    let user_name_option = tokens.split_last().unwrap().0;
    let user_name = parse_user_name(user_name_option);

    let real_name = if user_name.is_none() {
        tokens.join(" ")
    } else {
        tokens.split_last().unwrap().1.join(" ")
    }
    .to_string();
    LDrawCommand::Author(real_name, user_name)
}

fn parse_history(tokens: Vec<&str>) -> LDrawCommand {
    let date = NaiveDate::parse_from_str(tokens[0], "%Y-%m-%d");
    let user_name = parse_user_name(tokens[1]);
    let text = tokens.split_at(2).1.join(" ");
    LDrawCommand::History(date.unwrap(), user_name, text)
}

// 0 BFC ( CERTIFY ( CCW | CW ) | NOCERTIFY )
fn parse_bfc_certification(tokens: Vec<&str>) -> LDrawCommand {
    match tokens[0] {
        "CERTIFY" => LDrawCommand::BFCCertification(BFCDirection::from_str(tokens[1]).ok()),
        "NOCERTIFY" => LDrawCommand::BFCCertification(None),
        _ => LDrawCommand::Comment,
    }
}

fn parse_meta(tokens: Vec<&str>, line_index: usize) -> LDrawCommand {
    if line_index == 0 {
        LDrawCommand::Title(tokens.join(" "))
    } else {
        let tail = tokens.split_first().unwrap().1;
        match tokens[0] {
            "Name:" => LDrawCommand::Name(tail.join(" ")),
            "Author:" => parse_author(tail.to_vec()),
            "!LICENSE" => parse_license(tail.to_vec()),
            "!LDRAW_ORG" => LDrawCommand::LDrawOrg(LDrawType::from_str(tail[0]).unwrap()),
            "!CATEGORY" => LDrawCommand::Category(tail[0].to_string()),
            "!KEYWORDS" => LDrawCommand::Keywords(
                tail.iter()
                    .map(|s| s.to_string().replace(",", ""))
                    .collect(),
            ),
            "!HISTORY" => parse_history(tail.to_vec()),
            "BFC" => parse_bfc_certification(tail.to_vec()),
            _ => LDrawCommand::Comment,
        }
    }
}

async fn get_subfile(name: &str) -> Result<Vec<String>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:3000/ldraw/data/parts/{}", name);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();

    let text = JsFuture::from(response.text()?).await?.as_string().unwrap();
    let lines = text.lines().map(|line| line.to_string()).collect();

    Ok(lines)
}
