use chrono::NaiveDate;
use std::str::FromStr;
use three_d::{vec3, Matrix3, Vector3};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Color {
    value: u32,
}

// make traits and extend traits
#[derive(Debug, Clone)]
pub enum LDrawCommand {
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

#[derive(Debug, PartialEq, Clone)]
pub enum BFCDirection {
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

#[derive(Debug, PartialEq, Clone)]
pub enum LDrawType {
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

pub async fn tokenize_file(lines: Vec<String>) -> Result<Vec<Option<LDrawCommand>>, JsValue> {
    let parsed_lines: Vec<Option<LDrawCommand>> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| tokenize_line(line.to_string(), i))
        .filter(|p| p.is_some())
        .collect();

    Ok(parsed_lines)
}

fn tokenize_line(line: String, index: usize) -> Option<LDrawCommand> {
    if line.trim().len() <= 1 {
        return None;
    }

    let tokens: Vec<&str> = line.split_whitespace().collect();
    let tail = tokens.split_first().unwrap().1;
    match tokens[0] {
        "0" => Some(tokenize_meta(tail.to_vec(), index)),
        "1" => Some(tokenize_subfile_reference(tail.to_vec())),
        "2" => Some(tokenize_contour(tail.to_vec())),
        "3" => Some(tokenize_triangle(tail.to_vec())),
        "4" => Some(tokenize_quadrilateral(tail.to_vec())),
        "5" => Some(tokenize_optional_contour(tail.to_vec())),
        _ => None,
    }
}

fn tokenize_optional_contour(tokens: Vec<&str>) -> LDrawCommand {
    let color = tokenize_color(tokens[0]);
    let x = tokenize_vec3(tokens[1..4].to_vec());
    let y = tokenize_vec3(tokens[4..7].to_vec());
    let z = tokenize_vec3(tokens[7..10].to_vec());
    let w = tokenize_vec3(tokens[10..13].to_vec());

    LDrawCommand::OptionalContour(color, x, y, z, w)
}

fn tokenize_quadrilateral(tokens: Vec<&str>) -> LDrawCommand {
    let color = tokenize_color(tokens[0]);
    let x = tokenize_vec3(tokens[1..4].to_vec());
    let y = tokenize_vec3(tokens[4..7].to_vec());
    let z = tokenize_vec3(tokens[7..10].to_vec());
    let w = tokenize_vec3(tokens[10..13].to_vec());

    LDrawCommand::Quadrilateral(color, x, y, z, w)
}

fn tokenize_triangle(tokens: Vec<&str>) -> LDrawCommand {
    let color = tokenize_color(tokens[0]);
    let x = tokenize_vec3(tokens[1..4].to_vec());
    let y = tokenize_vec3(tokens[4..7].to_vec());
    let z = tokenize_vec3(tokens[7..10].to_vec());

    LDrawCommand::Triangle(color, x, y, z)
}

fn tokenize_contour(tokens: Vec<&str>) -> LDrawCommand {
    let color = tokenize_color(tokens[0]);
    let x = tokenize_vec3(tokens[1..4].to_vec());
    let y = tokenize_vec3(tokens[4..7].to_vec());

    LDrawCommand::Contour(color, x, y)
}

fn tokenize_subfile_reference(tokens: Vec<&str>) -> LDrawCommand {
    let color = tokenize_color(tokens[0]);
    let file = sanitize_file_name(tokens[13]);

    let translation = tokenize_vec3(tokens[1..4].to_vec());
    let transformation = tokenize_mat3(tokens[4..13].to_vec());

    LDrawCommand::SubfileReference(color, translation, transformation, file)
}

fn tokenize_vec3(tokens: Vec<&str>) -> Vector3<f32> {
    let tokens: Vec<f32> = tokens.iter().map(|token| token.parse().unwrap()).collect();
    vec3(tokens[0], tokens[1], tokens[2])
}

fn tokenize_mat3(tokens: Vec<&str>) -> Matrix3<f32> {
    let tokens: Vec<f32> = tokens.iter().map(|token| token.parse().unwrap()).collect();
    Matrix3::new(
        tokens[0], tokens[1], tokens[2], tokens[3], tokens[4], tokens[5], tokens[6], tokens[7],
        tokens[8],
    )
}

fn tokenize_color(token: &str) -> Color {
    Color {
        value: token.parse().unwrap(),
    }
}

fn sanitize_file_name(token: &str) -> String {
    token.replace("\\", "/")
}

fn tokenize_license(tokens: Vec<&str>) -> LDrawCommand {
    LDrawCommand::License(
        tokens[0..tokens.len() - 3].join(" "),
        tokens.last().unwrap().to_string(),
    )
}

fn tokenize_user_name(token: &str) -> Option<String> {
    if token.starts_with("[") && token.ends_with("]") {
        Some(token[1..token.len() - 1].to_string())
    } else {
        None
    }
}

fn tokenize_author(tokens: Vec<&str>) -> LDrawCommand {
    let user_name_option = tokens.split_last().unwrap().0;
    let user_name = tokenize_user_name(user_name_option);

    let real_name = if user_name.is_none() {
        tokens.join(" ")
    } else {
        tokens.split_last().unwrap().1.join(" ")
    };
    LDrawCommand::Author(real_name, user_name)
}

fn tokenize_history(tokens: Vec<&str>) -> LDrawCommand {
    let date = NaiveDate::parse_from_str(tokens[0], "%Y-%m-%d");
    let user_name = tokenize_user_name(tokens[1]);
    let text = tokens.split_at(2).1.join(" ");
    LDrawCommand::History(date.unwrap(), user_name, text)
}

// 0 BFC ( CERTIFY ( CCW | CW ) | NOCERTIFY )
fn tokenize_bfc_certification(tokens: Vec<&str>) -> LDrawCommand {
    match tokens[0] {
        "CERTIFY" => LDrawCommand::BFCCertification(BFCDirection::from_str(tokens[1]).ok()),
        "NOCERTIFY" => LDrawCommand::BFCCertification(None),
        _ => LDrawCommand::Comment,
    }
}

fn tokenize_meta(tokens: Vec<&str>, line_index: usize) -> LDrawCommand {
    if line_index == 0 {
        LDrawCommand::Title(tokens.join(" "))
    } else {
        let tail = tokens.split_first().unwrap().1;
        match tokens[0] {
            "Name:" => LDrawCommand::Name(sanitize_file_name(&tail.join(" "))),
            "Author:" => tokenize_author(tail.to_vec()),
            "!LICENSE" => tokenize_license(tail.to_vec()),
            "!LDRAW_ORG" => LDrawCommand::LDrawOrg(LDrawType::from_str(tail[0]).unwrap()),
            "!CATEGORY" => LDrawCommand::Category(tail[0].to_string()),
            "!KEYWORDS" => LDrawCommand::Keywords(
                tail.iter()
                    .map(|s| s.to_string().replace(",", ""))
                    .collect(),
            ),
            "!HISTORY" => tokenize_history(tail.to_vec()),
            "BFC" => tokenize_bfc_certification(tail.to_vec()),
            _ => LDrawCommand::Comment,
        }
    }
}
