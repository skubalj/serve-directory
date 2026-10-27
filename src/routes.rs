//! Definitions for routes served by the program

use crate::arguments;
use build_html::*;
use log::debug;
use std::fs::{DirEntry, read_dir};
use std::path::Path;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::filters::log::Info;
use warp::path::FullPath;
use warp::reject::{Rejection, not_found};
use warp::reply::{Reply, html};

const STYLES: &str = include_str!("assets/styles.css");
const FOLDER_ICON: &str = include_str!("assets/folder_icon.svg");
const FILE_ICON: &str = include_str!("assets/file_icon.svg");

/// The set of routes used by the program
pub fn routes() -> BoxedFilter<(impl Reply,)> {
    let handle_files = warp::fs::dir(&arguments().folder);
    let handle_directories = warp::get()
        .and(warp::path::full())
        .and_then(path_to_html)
        .map(html);

    handle_files
        .or(handle_directories)
        .with(warp::log::custom(log_request))
        .boxed()
}

/// Logs a handled request with who made it, what was asked for, the outcome and how long it took
fn log_request(info: Info) {
    let remote = info
        .remote_addr()
        .map_or_else(|| "unknown".to_owned(), |addr| addr.ip().to_string());

    debug!(
        "{remote} {} '{}' => {} ({:?})",
        info.method(),
        info.path(),
        info.status(),
        info.elapsed()
    );
}

/// Renders the listing of the folder at the requested route
async fn path_to_html(route: FullPath) -> Result<String, Rejection> {
    let route = route.as_str();
    let dir = arguments().folder.join(route.trim_start_matches('/'));
    let entries = read_entries(&dir).ok_or_else(not_found)?;

    Ok(HtmlPage::new()
        .with_style(STYLES)
        .with_container(main_container(route, entries))
        .to_html_string())
}

/// The page content: the route as the title, above the links to its entries
fn main_container(route: &str, entries: Vec<(String, &'static str)>) -> Container {
    Container::new(ContainerType::Main)
        .with_header(1, route)
        .with_container(links_container(route, entries))
}

/// The links to the parent folder and to every entry of the folder at the route
fn links_container(route: &str, entries: Vec<(String, &'static str)>) -> Container {
    let mut links = Container::new(ContainerType::Nav);
    if let Some(parent) = parent_route(route) {
        links.add_link(parent, "..");
    }
    for (name, icon) in entries {
        let href = format!("{}/{name}", route.trim_end_matches('/'));
        links.add_link(href, format!("{icon}<span>{name}</span>"));
    }
    links
}

/// The route of the folder containing the given one, or `None` at the root
fn parent_route(route: &str) -> Option<String> {
    let (parent, _) = route.trim_end_matches('/').rsplit_once('/')?;
    Some(if parent.is_empty() { "/" } else { parent }.to_owned())
}

/// The names of the entries in a folder, sorted, with the icon to show for each
fn read_entries(dir: &Path) -> Option<Vec<(String, &'static str)>> {
    let to_entry = |entry: std::io::Result<DirEntry>| {
        let entry = entry.ok()?;
        let icon = if entry.path().is_dir() {
            FOLDER_ICON
        } else {
            FILE_ICON
        };
        Some((entry.file_name().into_string().ok()?, icon))
    };

    let mut entries: Vec<_> = read_dir(dir).ok()?.filter_map(to_entry).collect();
    entries.sort();
    Some(entries)
}
