import type { OnboardingDestination$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "SettingsOnboardingOrganizationOnboardingSettings",
    "kind": "HoudiniQuery",
    "hash": "2c03456fd4d4d15f78ae0f29abd6d7893a0fd6d4a797979bcf094e0699d0107f",
    "raw": `query SettingsOnboardingOrganizationOnboardingSettings {
    organizationOnboardingSettings {
        node {
            welcome
            links {
                label
                href
                __typename
            }
            destination
            artworkColor
            requireFullName
            requireAvatar
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
            "organizationOnboardingSettings": {
                "type": "OrganizationOnboardingSettingsPayload",
                "keyRaw": "organizationOnboardingSettings",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "OrganizationOnboardingSettings",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "welcome": {
                                        "type": "String",
                                        "keyRaw": "welcome",
                                        "visible": true,
                                    },

                                    "links": {
                                        "type": "OnboardingLink",
                                        "keyRaw": "links",

                                        "selection": {
                                            "fields": {
                                                "label": {
                                                    "type": "String",
                                                    "keyRaw": "label",
                                                    "visible": true,
                                                },

                                                "href": {
                                                    "type": "String",
                                                    "keyRaw": "href",
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

                                    "destination": {
                                        "type": "OnboardingDestination",
                                        "keyRaw": "destination",
                                        "visible": true,
                                    },

                                    "artworkColor": {
                                        "type": "String",
                                        "keyRaw": "artworkColor",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "requireFullName": {
                                        "type": "Boolean",
                                        "keyRaw": "requireFullName",
                                        "visible": true,
                                    },

                                    "requireAvatar": {
                                        "type": "Boolean",
                                        "keyRaw": "requireAvatar",
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
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsOnboardingOrganizationOnboardingSettings = {
	readonly "input"?: SettingsOnboardingOrganizationOnboardingSettings$input;
	readonly "result": SettingsOnboardingOrganizationOnboardingSettings$result | undefined;
};

export type SettingsOnboardingOrganizationOnboardingSettings$result = {
	readonly organizationOnboardingSettings: {
		readonly node: {
			readonly welcome: string;
			readonly links: ({
				readonly label: string;
				readonly href: string;
			})[];
			readonly destination: OnboardingDestination$options;
			readonly artworkColor: string | null;
			readonly requireFullName: boolean;
			readonly requireAvatar: boolean;
		};
	};
};

export type SettingsOnboardingOrganizationOnboardingSettings$input = null | undefined;

export type SettingsOnboardingOrganizationOnboardingSettings$unmasked = {
	readonly organizationOnboardingSettings: {
		readonly node: {
			readonly welcome: string;
			readonly links: ({
				readonly label: string;
				readonly href: string;
				readonly __typename: "OnboardingLink";
			})[];
			readonly destination: OnboardingDestination$options;
			readonly artworkColor: string | null;
			readonly requireFullName: boolean;
			readonly requireAvatar: boolean;
			readonly __typename: "OrganizationOnboardingSettings";
		};
		readonly __typename: "OrganizationOnboardingSettingsPayload";
	};
};

export type SettingsOnboardingOrganizationOnboardingSettings$artifact = typeof artifact

"HoudiniHash=2c03456fd4d4d15f78ae0f29abd6d7893a0fd6d4a797979bcf094e0699d0107f"
