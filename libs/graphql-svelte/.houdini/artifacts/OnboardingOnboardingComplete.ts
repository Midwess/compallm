import type { OnboardingDestination$options, OnboardingFlow$options, OnboardingStep$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "OnboardingOnboardingComplete",
    "kind": "HoudiniMutation",
    "hash": "223eddef0b9abb18d4e356ee1ab07abbe72ef5f0492932874badfc2850cafc7a",
    "raw": `mutation OnboardingOnboardingComplete {
    onboardingComplete {
        node {
            required
            hasBeenPresented
            flow
            step
            settings {
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
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "onboardingComplete": {
                "type": "OnboardingContextPayload",
                "keyRaw": "onboardingComplete",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "OnboardingContext",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "required": {
                                        "type": "Boolean",
                                        "keyRaw": "required",
                                        "visible": true,
                                    },

                                    "hasBeenPresented": {
                                        "type": "Boolean",
                                        "keyRaw": "hasBeenPresented",
                                        "visible": true,
                                    },

                                    "flow": {
                                        "type": "OnboardingFlow",
                                        "keyRaw": "flow",
                                        "visible": true,
                                    },

                                    "step": {
                                        "type": "OnboardingStep",
                                        "keyRaw": "step",
                                        "visible": true,
                                    },

                                    "settings": {
                                        "type": "OrganizationOnboardingSettings",
                                        "keyRaw": "settings",

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
} as const

export default artifact

export type OnboardingOnboardingComplete = {
	readonly "input"?: OnboardingOnboardingComplete$input;
	readonly "result": OnboardingOnboardingComplete$result;
};

export type OnboardingOnboardingComplete$result = {
	readonly onboardingComplete: {
		readonly node: {
			readonly required: boolean;
			readonly hasBeenPresented: boolean;
			readonly flow: OnboardingFlow$options;
			readonly step: OnboardingStep$options;
			readonly settings: {
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
};

export type OnboardingOnboardingComplete$input = null | undefined;

export type OnboardingOnboardingComplete$optimistic = {
	readonly onboardingComplete?: {
		readonly node?: {
			readonly required?: boolean;
			readonly hasBeenPresented?: boolean;
			readonly flow?: OnboardingFlow$options;
			readonly step?: OnboardingStep$options;
			readonly settings?: {
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
};

export type OnboardingOnboardingComplete$unmasked = {
	readonly onboardingComplete: {
		readonly node: {
			readonly required: boolean;
			readonly hasBeenPresented: boolean;
			readonly flow: OnboardingFlow$options;
			readonly step: OnboardingStep$options;
			readonly settings: {
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
			readonly __typename: "OnboardingContext";
		};
		readonly __typename: "OnboardingContextPayload";
	};
};

export type OnboardingOnboardingComplete$artifact = typeof artifact

"HoudiniHash=223eddef0b9abb18d4e356ee1ab07abbe72ef5f0492932874badfc2850cafc7a"
