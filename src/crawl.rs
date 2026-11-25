use crate::{
    client::HttpClient,
    config::ILIAS_URL,
    error::{AppError, Result},
    html,
};
use std::path::Path;
use url::Url;

pub async fn run(client: &HttpClient) -> Result<()> {
    let base = Url::parse(ILIAS_URL)?;

    let overview_url = base.join("/ilias.php?baseClass=ilmembershipoverviewgui")?;
    let (_, overview) = client.get_html(&overview_url).await?;
    let course_url = html::find_anchor(&overview, &base, "Programmieren 1")?
        .ok_or_else(|| AppError::Ilias("course 'Programmieren 1' not found".into()))?;

    let (course_final, course_page) = client.get_html(&course_url).await?;
    let ub_url = html::find_anchor(&course_page, &course_final, "Übungsblätter")?
        .ok_or_else(|| AppError::Ilias("'Übungsblätter' not found".into()))?;

    let (folder_final, folder_page) = client.get_html(&ub_url).await?;
    let mut sheets = html::find_numbered_anchors(&folder_page, &folder_final, "Übungsblatt")?;
    if sheets.is_empty() {
        return Err(AppError::Ilias("no 'Übungsblatt N' anchors".into()));
    }
    sheets.sort_by(|a, b| b.0.cmp(&a.0));
    sheets.dedup_by(|a, b| a.1 == b.1);

    for (num, sheet_url) in &sheets {
        let (final_url, page) = client.get_html(sheet_url).await?;
        if let Some(cs) = html::find_anchor(&page, &final_url, "checkstyle")? {
            eprintln!("checkstyle in Übungsblatt {num}: {cs}");
            return download(client, &cs, "checkstyle-sheet").await;
        }
    }
    Err(AppError::Ilias("no checkstyle sheet found".into()))
}

async fn download(client: &HttpClient, url: &Url, fallback: &str) -> Result<()> {
    let (headers, bytes) = client.get_bytes(url).await?;
    let filename = headers
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|h| h.to_str().ok())
        .and_then(|cd| {
            cd.split(';')
                .map(str::trim)
                .find_map(|part| part.strip_prefix("filename="))
        })
        .map(|f| {
            f.trim_matches(|c: char| c == '"' || c == '\'' || c == ' ')
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fallback.to_string());

    let path = Path::new(&filename);
    if path.exists() {
        let existing = tokio::fs::read(path).await?;
        if existing == bytes {
            eprintln!("unchanged: {}", path.display());
            return Ok(());
        }
        eprintln!("changed: {}", path.display());
    } else {
        eprintln!("new: {}", path.display());
    }
    tokio::fs::write(path, &bytes).await?;
    eprintln!("wrote {} bytes to {}", bytes.len(), path.display());
    Ok(())
}
