use std::collections::HashMap;

use crate::parser::tokenizer::*;
use three_d::{Matrix3, Vector3};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone)]
pub struct LDrawAuthor {
    name: String,
    username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LDrawContour {
    color: Color,
    x: Vector3<f32>,
    y: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct LDrawOptionalContour {
    color: Color,
    x: Vector3<f32>,
    y: Vector3<f32>,
    ox: Vector3<f32>,
    oy: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct LDrawTriangle {
    color: Color,
    x: Vector3<f32>,
    y: Vector3<f32>,
    z: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct LDrawSubfile {
    color: Color,
    bfc_direction: BFCDirection,
    transformation: Matrix3<f32>,
    translation: Vector3<f32>,
    filename: String,
}

#[derive(Debug, Clone)]
pub struct LDrawFile {
    name: String,
    title: String,
    author: LDrawAuthor,
    bfc_direction: BFCDirection,
    lines: Vec<LDrawContour>,
    optional_lines: Vec<LDrawOptionalContour>,
    triangles: Vec<LDrawTriangle>,
    subfiles: Vec<LDrawSubfile>,
}

#[derive(Debug, Clone)]
pub struct LDrawBrick {
    entry: String,
    files: HashMap<String, LDrawFile>,
}

pub async fn parse_part(id: &str) -> Result<LDrawBrick, JsValue> {
    let files = get_bundle_lst(&format!("{}", id)).await?;
    let mut file_map = HashMap::new();

    for file in files {
        let lines = get_subfile(&file).await?;
        let tokens = tokenize_file(lines).await?;

        let mut file = LDrawFile {
            name: String::new(),
            title: String::new(),
            author: LDrawAuthor {
                name: String::new(),
                username: None,
            },
            bfc_direction: BFCDirection::CW,
            lines: Vec::new(),
            optional_lines: Vec::new(),
            triangles: Vec::new(),
            subfiles: Vec::new(),
        };

        for token in &tokens {
            match token {
                Some(LDrawCommand::Name(name)) => file.name = name.to_string(),
                Some(LDrawCommand::Title(title)) => file.title = title.to_string(),
                Some(LDrawCommand::Author(name, username)) => {
                    file.author.name = name.to_string();
                    file.author.username = username.clone()
                }
                Some(LDrawCommand::BFCCertification(direction)) => {
                    if direction.is_some() {
                        file.bfc_direction = direction.clone().unwrap();
                    }
                }
                Some(LDrawCommand::Contour(color, x, y)) => file.lines.push(LDrawContour {
                    color: color.clone(),
                    x: x.clone(),
                    y: y.clone(),
                }),
                Some(LDrawCommand::OptionalContour(color, x, y, ox, oy)) => {
                    file.optional_lines.push(LDrawOptionalContour {
                        color: color.clone(),
                        x: x.clone(),
                        y: y.clone(),
                        ox: ox.clone(),
                        oy: oy.clone(),
                    })
                }
                Some(LDrawCommand::Triangle(color, x, y, z)) => {
                    file.triangles.push(LDrawTriangle {
                        color: color.clone(),
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    })
                }
                Some(LDrawCommand::Quadrilateral(color, x, y, z, w)) => {
                    file.triangles.push(LDrawTriangle {
                        color: color.clone(),
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    });
                    file.triangles.push(LDrawTriangle {
                        color: color.clone(),
                        x: z.clone(),
                        y: w.clone(),
                        z: x.clone(),
                    })
                }
                Some(LDrawCommand::SubfileReference(
                    color,
                    translation,
                    transformation,
                    filename,
                )) => file.subfiles.push(LDrawSubfile {
                    color: color.clone(),
                    bfc_direction: file.bfc_direction.clone(),
                    translation: translation.clone(),
                    transformation: transformation.clone(),
                    filename: filename.to_string(),
                }),
                _ => {}
            }
        }

        file_map.insert(file.name.to_string(), file.clone());
    }

    Ok(LDrawBrick {
        entry: format!("{}.dat", id),
        files: file_map,
    })
}

async fn get_bundle_lst(id: &str) -> Result<Vec<String>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:3000/ldraw/bundle/{}.lst", id);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();

    let text = JsFuture::from(response.text()?).await?.as_string().unwrap();
    let files = text.lines().map(|line| line.to_string()).collect();

    Ok(files)
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
