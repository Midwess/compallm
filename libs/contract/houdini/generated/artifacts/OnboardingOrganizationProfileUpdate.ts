import type { OrganizationProfileUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "OnboardingOrganizationProfileUpdate",
    "kind": "HoudiniMutation",
    "hash": "239585bf8eba211e5e408e6e0af3792bdb45801d968ee43688c09a8a8730ead4",
    "raw": `mutation OnboardingOrganizationProfileUpdate($organizationProfileUpdateInput: OrganizationProfileUpdateInput!) {
    organizationProfileUpdate(input: $organizationProfileUpdateInput) {
        node {
            name
            __typename
            id
        }
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "organizationProfileUpdate": {
                "type": "OrganizationPayload",
                "keyRaw": "organizationProfileUpdate(input: $organizationProfileUpdateInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Organization",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },

                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                    },
                                },
                            },

                            "visible": true,
                        },

                        "__typename": {
                            "type": "String",
                            "keyRaw": "__typename",
                        },
                    },
                },

                "visible": true,
            },
        },
    },

    "pluginData": {},

    "input": {
        "fields": {
            "organizationProfileUpdateInput": "OrganizationProfileUpdateInput",
        },

        "types": {
            "OrganizationProfileUpdateInput": {
                "avatarUploaded": "Boolean",
                "name": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type OnboardingOrganizationProfileUpdate = {
	readonly "input": OnboardingOrganizationProfileUpdate$input;
	readonly "result": OnboardingOrganizationProfileUpdate$result;
};

export type OnboardingOrganizationProfileUpdate$result = {
	readonly organizationProfileUpdate: {
		readonly node: {
			readonly name: string;
		};
	};
};

export type OnboardingOrganizationProfileUpdate$input = {
	organizationProfileUpdateInput: OrganizationProfileUpdateInput;
};

export type OnboardingOrganizationProfileUpdate$optimistic = {
	readonly organizationProfileUpdate?: {
		readonly node?: {
			readonly name?: string;
		};
	};
};

export type OnboardingOrganizationProfileUpdate$unmasked = {
	readonly organizationProfileUpdate: {
		readonly node: {
			readonly name: string;
			readonly __typename: "Organization";
			readonly id: string;
		};
		readonly __typename: "OrganizationPayload";
	};
};

export type OnboardingOrganizationProfileUpdate$artifact = typeof artifact

"HoudiniHash=239585bf8eba211e5e408e6e0af3792bdb45801d968ee43688c09a8a8730ead4"
