use cursive::{Cursive, views::TextView};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::utils::CrusterError;

pub(super) enum CopySubject {
    FullScreenRequest,
    FullScreenResponse,
    FullScreenRequestAndResponse,
    Help
}

pub(super) trait CopyToClipboard {
    fn copy_to_clipboard(&mut self, subject: CopySubject) -> Result<(), CrusterError>;
}

impl CopyToClipboard for Cursive {
    fn copy_to_clipboard(&mut self, subject: CopySubject) -> Result<(), CrusterError> {
        let mut ctx = match ClipboardContext::new() {
            Ok(ctx) => ctx,
            Err(e) => return Err(
                CrusterError::UndefinedError(
                    format!("Could not access clipboard: {}", e.to_string())
                )
            )
        };

        let result = match subject {
            CopySubject::FullScreenRequest => {
                self.call_on_name("request-fs-content", |req: &mut TextView| {
                    let content = req.get_content();
                    if let Err(err) = ctx.set_contents(content.source().to_string()) {
                        return Err(
                            CrusterError::UndefinedError(
                                format!("Could not set clipboard's content: {}", err.to_string())
                            )
                        );
                    }

                    Ok(())
                })
            },
            CopySubject::FullScreenResponse => {
                self.call_on_name("response-fs-content", |req: &mut TextView| {
                    let content = req.get_content();
                    if let Err(err) = ctx.set_contents(content.source().to_string()) {
                        return Err(
                            CrusterError::UndefinedError(
                                format!("Could not set clipboard's content: {}", err.to_string())
                            )
                        );
                    }

                    Ok(())
                })
            },
            CopySubject::FullScreenRequestAndResponse => {
                let mut content = String::default();
                content.push_str("-- REQUEST --\n");

                let req_res = self.call_on_name("request-fs-content", |req: &mut TextView| {
                    let content = req.get_content();
                    content.source().to_string()
                });

                match req_res {
                    Some(req_str) => content.push_str(&req_str),
                    None => {
                        return Err(
                            CrusterError::UndefinedError(
                                format!("Could set access to request content to copy")
                            )
                        )
                    }
                }

                content.push_str("\n-- REQUEST END --\n-- RESPONSE --\n");

                let res_res = self.call_on_name("response-fs-content", |req: &mut TextView| {
                    let content = req.get_content();
                    content.source().to_string()
                });

                match res_res {
                    Some(res_str) => content.push_str(&res_str),
                    None => {
                        return Err(
                            CrusterError::UndefinedError(
                                format!("Could set access to response content to copy")
                            )
                        )
                    }
                }

                content.push_str("\n-- RESPONSE END --");
                if let Err(err) = ctx.set_contents(content) {
                    Some(
                        Err(
                            CrusterError::UndefinedError(
                                format!("Could not set clipboard's content: {}", err.to_string())
                            )
                        )
                    )
                }
                else {
                    Some(Ok(()))
                }
            },
            CopySubject::Help => {
                self.call_on_name("help-popup", |req: &mut TextView| {
                    let content = req.get_content();
                    if let Err(err) = ctx.set_contents(content.source().to_string()) {
                        return Err(
                            CrusterError::UndefinedError(
                                format!("Could not set clipboard's content: {}", err.to_string())
                            )
                        );
                    }

                    Ok(())
                })
            }
        };

        if let None = result {
            return Err(CrusterError::UndefinedError("Could not access content to copy it".to_string()));
        }
        else {
            return result.unwrap();
        }
    }
}