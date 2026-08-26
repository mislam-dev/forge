use super::base_policy::AuthPolicyTrait;
use crate::modules::auth::token::JwtClaims;

pub struct AdminRolePolicy;
pub struct DeveloperRolePolicy;
pub struct ViewerRolePolicy;

impl AuthPolicyTrait for AdminRolePolicy {
    fn check(claims: &JwtClaims) -> bool {
        claims.roles.contains(&"admin".to_string())
    }
}

impl AuthPolicyTrait for DeveloperRolePolicy {
    fn check(claims: &JwtClaims) -> bool {
        claims.roles.contains(&"developer".to_string())
    }
}

impl AuthPolicyTrait for ViewerRolePolicy {
    fn check(claims: &JwtClaims) -> bool {
        claims.roles.contains(&"viewer".to_string())
    }
}
