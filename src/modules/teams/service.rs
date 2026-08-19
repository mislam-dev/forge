pub use super::members::service::TeamMembersService;
pub use super::teams::service::TeamsService;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teams_service_reexports() {
        let _ = TeamsService;
        let _ = TeamMembersService;
    }
}
