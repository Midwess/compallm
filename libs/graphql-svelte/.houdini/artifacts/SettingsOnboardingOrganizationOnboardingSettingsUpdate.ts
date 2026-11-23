import type { OnboardingDestination$options } from "$houdini/graphql/enums";
import type { OrganizationOnboardingSettingsInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsOnboardingOrganizationOnboardingSettingsUpdate",
    "kind": "HoudiniMutation",
    "hash": "7f588fd1beb8d033c721a31ff86e3333b23447bfb6e54fd163fc3b09a92080cd",
    "raw": `mutation SettingsOnboardingOrganizationOnboardingSettingsUpdate($organizationOnboardingSettingsUpdateInput: OrganizationOnboardingSettingsInput!) {
    organizationOnboardingSettingsUpdate(input: $organizationOnboardingSettingsUpdateInput) {
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

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "organizationOnboardingSettingsUpdate": {
                "type": "OrganizationOnboardingSettingsPayload",
                "keyRaw": "organizationOnboardingSettingsUpdate(input: $organizationOnboardingSettingsUpdateInput)",

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

    "input": {
        "fields": {
            "organizationOnboardingSettingsUpdateInput": "OrganizationOnboardingSettingsInput",
        },

        "types": {
            "OnboardingLinkInput": {
                "href": "String",
                "label": "String",
            },
            "OrganizationOnboardingSettingsInput": {
                "artworkColor": "String",
                "destination": "OnboardingDestination",
                "links": "OnboardingLinkInput",
                "requireAvatar": "Boolean",
                "requireFullName": "Boolean",
                "welcome": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsOnboardingOrganizationOnboardingSettingsUpdate = {
	readonly "input": SettingsOnboardingOrganizationOnboardingSettingsUpdate$input;
	readonly "result": SettingsOnboardingOrganizationOnboardingSettingsUpdate$result;
};

export type SettingsOnboardingOrganizationOnboardingSettingsUpdate$result = {
	readonly organizationOnboardingSettingsUpdate: {
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

export type SettingsOnboardingOrganizationOnboardingSettingsUpdate$input = {
	organizationOnboardingSettingsUpdateInput: OrganizationOnboardingSettingsInput;
};

export type SettingsOnboardingOrganizationOnboardingSettingsUpdate$optimistic = {
	readonly organizationOnboardingSettingsUpdate?: {
		readonly node?: {
			readonly welcome?: string;
			readonly links?: ({
				readonly label?: string;
				readonly href?: string;
			})[];
			readonly destination?: OnboardingDestination$options;
			readonly artworkColor?: string | null;
			readonly requireFullName?: boolean;
			readonly requireAvatar?: boolean;
		};
	};
};

export type SettingsOnboardingOrganizationOnboardingSettingsUpdate$unmasked = {
	readonly organizationOnboardingSettingsUpdate: {
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

export type SettingsOnboardingOrganizationOnboardingSettingsUpdate$artifact = typeof artifact

"HoudiniHash=7f588fd1beb8d033c721a31ff86e3333b23447bfb6e54fd163fc3b09a92080cd"
