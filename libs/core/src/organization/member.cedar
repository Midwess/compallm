// Default members discover models and invoke enabled logical models only.
permit(principal in Compallm::Role::"__ORG__:3", action == Compallm::Action::"model.list",
 resource == Compallm::Organization::"__ORG__");
permit(principal in Compallm::Role::"__ORG__:3", action == Compallm::Action::"model.metadata.read",
 resource is Compallm::Model in Compallm::Organization::"__ORG__");
permit(principal in Compallm::Role::"__ORG__:3", action == Compallm::Action::"model.use",
 resource is Compallm::Model in Compallm::Organization::"__ORG__") when { resource.enabled };
