const artifact = {
    "name": "SettingsOrganizationOrganizationProfile",
    "kind": "HoudiniQuery",
    "hash": "d96cfde91e441968b32b302ee47b9f2fb1b22b687333ad5da91a2bc430f436c4",
    "raw": `query SettingsOrganizationOrganizationProfile($organizationProfileId: ID!) {
    organizationProfile(id: $organizationProfileId) {
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

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "organizationProfile": {
                "type": "OrganizationPayload",
                "keyRaw": "organizationProfile(id: $organizationProfileId)",
                "nullable": true,

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
            "organizationProfileId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsOrganizationOrganizationProfile = {
	readonly "input": SettingsOrganizationOrganizationProfile$input;
	readonly "result": SettingsOrganizationOrganizationProfile$result | undefined;
};

export type SettingsOrganizationOrganizationProfile$result = {
	readonly organizationProfile: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly avatarUrl: string | null;
			readonly canUpdateProfile: boolean;
		};
	} | null;
};

export type SettingsOrganizationOrganizationProfile$input = {
	organizationProfileId: string;
};

export type SettingsOrganizationOrganizationProfile$unmasked = {
	readonly organizationProfile: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly avatarUrl: string | null;
			readonly canUpdateProfile: boolean;
			readonly __typename: "Organization";
		};
		readonly __typename: "OrganizationPayload";
	} | null;
};

export type SettingsOrganizationOrganizationProfile$artifact = typeof artifact

"HoudiniHash=d96cfde91e441968b32b302ee47b9f2fb1b22b687333ad5da91a2bc430f436c4"
