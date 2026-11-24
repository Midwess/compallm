import type { OnboardingDestination$options, OnboardingFlow$options, OnboardingStep$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "OnboardingOnboardingAdvance",
    "kind": "HoudiniMutation",
    "hash": "33fb2ff52cbf0819d988dcd089d53763011a866b48177551985fa89286c38de4",
    "raw": `mutation OnboardingOnboardingAdvance($onboardingAdvanceBack: Boolean!) {
    onboardingAdvance(back: $onboardingAdvanceBack) {
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
            "onboardingAdvance": {
                "type": "OnboardingContextPayload",
                "keyRaw": "onboardingAdvance(back: $onboardingAdvanceBack)",

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

    "input": {
        "fields": {
            "onboardingAdvanceBack": "Boolean",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type OnboardingOnboardingAdvance = {
	readonly "input": OnboardingOnboardingAdvance$input;
	readonly "result": OnboardingOnboardingAdvance$result;
};

export type OnboardingOnboardingAdvance$result = {
	readonly onboardingAdvance: {
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

export type OnboardingOnboardingAdvance$input = {
	onboardingAdvanceBack: boolean;
};

export type OnboardingOnboardingAdvance$optimistic = {
	readonly onboardingAdvance?: {
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

export type OnboardingOnboardingAdvance$unmasked = {
	readonly onboardingAdvance: {
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

export type OnboardingOnboardingAdvance$artifact = typeof artifact

"HoudiniHash=33fb2ff52cbf0819d988dcd089d53763011a866b48177551985fa89286c38de4"
