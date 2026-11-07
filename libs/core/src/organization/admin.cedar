// Application-owned initial Admin policy; organization/role IDs are trusted numeric values.
permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"organization.settings.read", Compallm::Action::"organization.settings.update", Compallm::Action::"member.list", Compallm::Action::"member.invite", Compallm::Action::"member.metadata.read", Compallm::Action::"member.remove", Compallm::Action::"provider.list", Compallm::Action::"provider.create", Compallm::Action::"provider.connection.test", Compallm::Action::"provider.configuration.read", Compallm::Action::"provider.configuration.update", Compallm::Action::"provider.delete", Compallm::Action::"provider.credential.replace", Compallm::Action::"provider.model.list", Compallm::Action::"provider.model.create", Compallm::Action::"provider.model.configuration.read", Compallm::Action::"provider.model.configuration.update", Compallm::Action::"provider.model.pricing.update", Compallm::Action::"provider.model.delete", Compallm::Action::"model.list", Compallm::Action::"model.create", Compallm::Action::"model.metadata.read", Compallm::Action::"model.configuration.update", Compallm::Action::"model.delete", Compallm::Action::"model.use", Compallm::Action::"model.pricing.update", Compallm::Action::"role.list", Compallm::Action::"role.create", Compallm::Action::"role.metadata.read", Compallm::Action::"role.metadata.update", Compallm::Action::"role.policy.read", Compallm::Action::"role.policy.update", Compallm::Action::"role.delete", Compallm::Action::"group.list", Compallm::Action::"group.create", Compallm::Action::"group.metadata.read", Compallm::Action::"group.metadata.update", Compallm::Action::"group.delete"],
    resource in Compallm::Organization::"__ORG__"
);

permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"member.role.assign", Compallm::Action::"member.role.unassign", Compallm::Action::"group.role.assign", Compallm::Action::"group.role.unassign"],
    resource in Compallm::Organization::"__ORG__"
) when { context.role.organization == Compallm::Organization::"__ORG__" && context.role != Compallm::Role::"__ORG__:1" };

permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"group.member.add", Compallm::Action::"group.member.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.member.organization == Compallm::Organization::"__ORG__" }
unless { resource in Compallm::Role::"__ORG__:1" };

permit (
    principal in Compallm::Role::"__ORG__:2",
    action in [Compallm::Action::"model.route.add", Compallm::Action::"model.route.update", Compallm::Action::"model.route.remove"],
    resource in Compallm::Organization::"__ORG__"
) when { context.providerModel.organization == Compallm::Organization::"__ORG__" };

permit (
 principal in Compallm::Role::"__ORG__:2",
 action in [Compallm::Action::"role.assign", Compallm::Action::"role.unassign"],
 resource is Compallm::Role in Compallm::Organization::"__ORG__"
) unless { resource == Compallm::Role::"__ORG__:1" };

forbid(principal in Compallm::Role::"__ORG__:2", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") unless { resource.enabled };
