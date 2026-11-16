// Application-owned initial Admin template.
permit (
    principal in ?principal,
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"member.read", Compallm::Action::"member.invite", Compallm::Action::"member.remove", Compallm::Action::"provider.read", Compallm::Action::"provider.create", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.read", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.read", Compallm::Action::"model.create", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.read", Compallm::Action::"role.create", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"group.read", Compallm::Action::"group.create", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in ?resource
);

permit (
    principal in ?principal,
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in ?resource
) when { context.role.organization == Compallm::Organization::"__ORG__" && context.role != Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in ?resource
) when { context.member.organization == Compallm::Organization::"__ORG__" }
unless { resource in Compallm::Role::"__ORG__:__OWNER__" };

permit (
    principal in ?principal,
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in ?resource
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

permit (
 principal in ?principal,
 action in [Compallm::Action::"role.assign", Compallm::Action::"role.unassign"],
 resource is Compallm::Role in ?resource
) unless { resource == Compallm::Role::"__ORG__:__OWNER__" };

forbid(principal in ?principal, action == Compallm::Action::"model.use",
 resource is Compallm::Model in ?resource) unless { resource.enabled };
