use crate::{
    config::AuditConfig,
    cli::CrusterCLIError,
    audit::{load_rule::compose_files_list_by_config, Rule}
};

pub(crate) fn exec(audit_conf: &AuditConfig) -> Result<(), CrusterCLIError> {
    let rules_files = compose_files_list_by_config(audit_conf)?;

    let mut rules: Vec<Rule> = Vec::new();
    for fln in rules_files.iter() {
        println!("{}", fln);
        let mut rule = Rule::from_file(fln)?;

        match rule.check_up() {
            Ok(()) => {
                println!("{:#?}", &rule);
                println!();

                rules.push(rule);
                
            },
            Err(err) => { return Err(err.into()); }
        }
        
    }

    Ok(())
}