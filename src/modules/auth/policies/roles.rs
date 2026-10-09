use super::base_policy::AuthPolicyTrait;
use crate::modules::auth::token::AccessTokenClaims;

pub struct AdminRolePolicy;
pub struct DeveloperRolePolicy;
pub struct ViewerRolePolicy;

impl AuthPolicyTrait for AdminRolePolicy {
    fn check(claims: &AccessTokenClaims) -> bool {
        claims.roles.contains(&"admin".to_string())
    }
}

impl AuthPolicyTrait for DeveloperRolePolicy {
    fn check(claims: &AccessTokenClaims) -> bool {
        claims.roles.contains(&"developer".to_string())
    }
}

impl AuthPolicyTrait for ViewerRolePolicy {
    fn check(claims: &AccessTokenClaims) -> bool {
        claims.roles.contains(&"viewer".to_string())
    }
}
