#[macro_export]
macro_rules! define_policy {
    ($struct_name:ident, $perm_value:literal, $desc:literal) => {
        #[doc = $desc]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $struct_name;

        impl $crate::modules::auth::policies::base_policy::AuthPolicyTrait for $struct_name {
            fn check(claims: &$crate::modules::auth::token::JwtClaims) -> bool {
                claims.permissions.iter().any(|p| p == $perm_value)
            }
        }
    };
}

pub use define_policy;

#[cfg(test)]
mod tests {
    use crate::modules::auth::{policies::base_policy::AuthPolicyTrait, token::JwtClaims};
    use uuid::Uuid;

    define_policy!(
        SampleTestPolicy,
        "sample:test:permission",
        "Test policy for macro verification"
    );

    fn make_claims(permissions: Vec<&str>) -> JwtClaims {
        JwtClaims {
            sub: Uuid::new_v4(),
            email: "macro_test@example.com".to_string(),
            roles: vec![],
            permissions: permissions.into_iter().map(String::from).collect(),
            iat: 0,
            exp: 0,
        }
    }

    #[test]
    fn test_define_policy_macro() {
        let claims_allowed = make_claims(vec!["sample:test:permission"]);
        let claims_denied = make_claims(vec!["other:permission"]);

        assert!(SampleTestPolicy::check(&claims_allowed));
        assert!(!SampleTestPolicy::check(&claims_denied));
    }
}
