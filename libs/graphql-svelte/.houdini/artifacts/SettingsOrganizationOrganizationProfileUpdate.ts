import type { OrganizationProfileUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsOrganizationOrganizationProfileUpdate",
    "kind": "HoudiniMutation",
    "hash": "ca5b1885cf91885cd64c7399d5408eb18498d67fcd2e68812322d066c86f9031",
    "raw": `mutation SettingsOrganizationOrganizationProfileUpdate($organizationProfileUpdateInput: OrganizationProfileUpdateInput!) {
    organizationProfileUpdate(input: $organizationProfileUpdateInput) {
        node {
            id
            name
            avatarUrl
            canUpdateProfile
            __typename
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
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
                                        "visible": true,
                                    },

                                    "avatarUrl": {
                                        "type": "String",
                                        "keyRaw": "avatarUrl",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "canUpdateProfile": {
                                        "type": "Boolean",
                                        "keyRaw": "canUpdateProfile",
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

export type SettingsOrganizationOrganizationProfileUpdate = {
	readonly "input": SettingsOrganizationOrganizationProfileUpdate$input;
	readonly "result": SettingsOrganizationOrganizationProfileUpdate$result;
};

export type SettingsOrganizationOrganizationProfileUpdate$result = {
	readonly organizationProfileUpdate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly avatarUrl: string | null;
			readonly canUpdateProfile: boolean;
		};
	};
};

export type SettingsOrganizationOrganizationProfileUpdate$input = {
	organizationProfileUpdateInput: OrganizationProfileUpdateInput;
};

export type SettingsOrganizationOrganizationProfileUpdate$optimistic = {
	readonly organizationProfileUpdate?: {
		readonly node?: {
			readonly id?: string;
			readonly name?: string;
			readonly avatarUrl?: string | null;
			readonly canUpdateProfile?: boolean;
		};
	};
};

export type SettingsOrganizationOrganizationProfileUpdate$unmasked = {
	readonly organizationProfileUpdate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly avatarUrl: string | null;
			readonly canUpdateProfile: boolean;
			readonly __typename: "Organization";
		};
		readonly __typename: "OrganizationPayload";
	};
};

export type SettingsOrganizationOrganizationProfileUpdate$artifact = typeof artifact

"HoudiniHash=ca5b1885cf91885cd64c7399d5408eb18498d67fcd2e68812322d066c86f9031"
