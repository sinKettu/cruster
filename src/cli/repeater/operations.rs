//! Shared operations extracted from the CLI repeater commands.
use super::{parsing, read_repeaters, update_repeaters};
use crate::{
    cli::CrusterCLIError,
    cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper},
    siv_ui::repeater::{RepeaterParameters, RepeaterStateSerializable},
};
use std::path::Path;

pub(crate) fn select_repeater(
    records: &[RepeaterStateSerializable],
    id: Option<usize>,
    name: Option<&str>,
) -> Result<usize, CrusterCLIError> {
    match (id, name) {
        (Some(id), None) => id
            .checked_sub(1)
            .filter(|&i| i < records.len())
            .ok_or_else(|| {
                format!(
                    "Repeater {} was not found; use repeater_list for current IDs",
                    id
                )
                .into()
            }),
        (None, Some(name)) => records
            .iter()
            .position(|r| r.name == name)
            .ok_or_else(|| format!("Repeater named '{}' was not found", name).into()),
        _ => Err("Specify exactly one of id or name".into()),
    }
}

pub(crate) fn from_history(
    request: &HyperRequestWrapper,
    response: Option<&HyperResponseWrapper>,
    name: String,
    max_redirects: usize,
) -> Result<RepeaterStateSerializable, CrusterCLIError> {
    Ok(RepeaterStateSerializable {
        name,
        request: base64::encode(parsing::render_raw(request)?),
        response: response.map(|r| base64::encode(r.to_string())),
        parameters: RepeaterParameters {
            address: request.get_hostname(),
            https: request.get_scheme() == "https://",
            redirects: true,
            max_redirects,
        },
    })
}

impl RepeaterStateSerializable {
    pub(crate) fn raw(&self) -> Result<String, CrusterCLIError> {
        Ok(String::from_utf8(base64::decode(&self.request)?)?)
    }

    pub(crate) fn parsed_request(&self) -> Result<HyperRequestWrapper, CrusterCLIError> {
        Ok(parsing::parse_raw(
            &self.raw()?,
            &format!(
                "{}://{}/",
                if self.parameters.https {
                    "https"
                } else {
                    "http"
                },
                self.parameters.address
            ),
        )?)
    }
}

/// Save prepared edits and clear stale responses BEFORE any network work; save the
/// response BEFORE returning it to either presentation layer. A timeout is MCP policy.
pub(crate) async fn execute_repeater(
    path: &Path,
    record: &mut RepeaterStateSerializable,
    index: usize,
    request: &HyperRequestWrapper,
    timeout: Option<std::time::Duration>,
) -> Result<(HyperResponseWrapper, bool), CrusterCLIError> {
    record.response = None;
    update_repeaters(path, record, index)?;
    let parameters = &record.parameters;
    let max = if parameters.redirects {
        parameters.max_redirects
    } else {
        0
    };
    let result = send_request(request, max, timeout).await?;
    record.response = Some(base64::encode(result.0.to_string()));
    update_repeaters(path, record, index)?;
    Ok(result)
}

/// Shared transport policy for saved and one-shot repeater requests.
pub(crate) async fn send_request(
    request: &HyperRequestWrapper,
    max_redirects: usize,
    timeout: Option<std::time::Duration>,
) -> Result<(HyperResponseWrapper, bool), CrusterCLIError> {
    let send = crate::http_sender::send_request_from_wrapper(request, max_redirects);
    if let Some(timeout) = timeout {
        Ok(tokio::time::timeout(timeout, send).await.map_err(|_| {
            CrusterCLIError::from("HTTP request timed out; the target may already have received it")
        })??)
    } else {
        Ok(send.await?)
    }
}

pub(crate) fn load_selected(
    path: &str,
    id: Option<usize>,
    name: Option<&str>,
) -> Result<(usize, Vec<RepeaterStateSerializable>), CrusterCLIError> {
    let records = read_repeaters(Path::new(path))?;
    let index = select_repeater(&records, id, name)?;
    Ok((index, records))
}
