import type { AnalyticUserProfilePutInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "OnboardingAnalyticUserProfilePut",
    "kind": "HoudiniMutation",
    "hash": "e1afb9b785d66366a8e0de65308416b8157bc63895cb0762395463c157289294",
    "raw": `mutation OnboardingAnalyticUserProfilePut($analyticUserProfilePutInput: AnalyticUserProfilePutInput!) {
    analyticUserProfilePut(input: $analyticUserProfilePutInput) {
        node {
            value
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
            "analyticUserProfilePut": {
                "type": "AnalyticUserProfilePayload",
                "keyRaw": "analyticUserProfilePut(input: $analyticUserProfilePutInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "AnalyticUserProfile",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "value": {
                                        "type": "String",
                                        "keyRaw": "value",
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
            "analyticUserProfilePutInput": "AnalyticUserProfilePutInput",
        },

        "types": {
            "AnalyticUserProfilePutInput": {
                "key": "String",
                "value": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type OnboardingAnalyticUserProfilePut = {
	readonly "input": OnboardingAnalyticUserProfilePut$input;
	readonly "result": OnboardingAnalyticUserProfilePut$result;
};

export type OnboardingAnalyticUserProfilePut$result = {
	readonly analyticUserProfilePut: {
		readonly node: {
			readonly value: string;
		};
	};
};

export type OnboardingAnalyticUserProfilePut$input = {
	analyticUserProfilePutInput: AnalyticUserProfilePutInput;
};

export type OnboardingAnalyticUserProfilePut$optimistic = {
	readonly analyticUserProfilePut?: {
		readonly node?: {
			readonly value?: string;
		};
	};
};

export type OnboardingAnalyticUserProfilePut$unmasked = {
	readonly analyticUserProfilePut: {
		readonly node: {
			readonly value: string;
			readonly __typename: "AnalyticUserProfile";
		};
		readonly __typename: "AnalyticUserProfilePayload";
	};
};

export type OnboardingAnalyticUserProfilePut$artifact = typeof artifact

"HoudiniHash=e1afb9b785d66366a8e0de65308416b8157bc63895cb0762395463c157289294"
