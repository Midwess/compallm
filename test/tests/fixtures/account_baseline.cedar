// Accepted baseline contract fixture; not loaded by production authorization.
// Public user and organization profiles have no Cedar read action.
// Customer organization policies must not govern these account decisions.
permit (
    principal is Compallm::User,
    action in [Compallm::Action::"user.profile.update", Compallm::Action::"session.list"],
    resource is Compallm::User
) when { principal == resource };

permit (
    principal is Compallm::User,
    action == Compallm::Action::"user.authentication.read",
    resource is Compallm::UserAuthentication
) when { resource.user == principal };

// Authentication changes need a separately specified verified workflow;
// this ownership-only baseline does not authorize them.
permit (
    principal is Compallm::User,
    action in [Compallm::Action::"session.metadata.read", Compallm::Action::"session.revoke"],
    resource is Compallm::Session
) when { resource.user == principal };
