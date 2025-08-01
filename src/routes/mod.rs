/*
 * Module declaration for the routes.
 *
 * Contains the download route, which handles the download request.
 * Contains the files route, which handles file serving.
 * Contains the health route, which handles health checks.
 * Contains the page route, which serves the HTML download page.
 */
pub mod download;
pub mod files;
pub mod health;
pub mod page;
