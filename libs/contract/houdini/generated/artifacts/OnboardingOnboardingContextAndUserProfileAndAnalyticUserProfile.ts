import type { OnboardingDestination$options, OnboardingFlow$options, OnboardingStep$options } from "$houdini/graphql/enums";
const artifact = {
    "name": "OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile",
    "kind": "HoudiniQuery",
    "hash": "73d7d572194e052ac5fbca9ecbbd5cba793b681fb09a1c47d4ee80326c0219d6",
    "raw": `query OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile($userProfileId: ID!) {
    onboardingContext {
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
    userProfile(id: $userProfileId) {
        node {
            displayName
            username
            email
            avatarUrl
            __typename
            id
        }
        __typename
    }
    analyticUserProfile(key: "work") {
        node {
            value
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
            "onboardingContext": {
                "type": "OnboardingContextPayload",
                "keyRaw": "onboardingContext",
                "nullable": true,

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

            "userProfile": {
                "type": "UserPayload",
                "keyRaw": "userProfile(id: $userProfileId)",
                "nullable": true,

                "selection": {
                    "fields": {
                        "node": {
                            "type": "User",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "displayName": {
                                        "type": "String",
                                        "keyRaw": "displayName",
                                        "visible": true,
                                    },

                                    "username": {
                                        "type": "String",
                                        "keyRaw": "username",
                                        "visible": true,
                                    },

                                    "email": {
                                        "type": "String",
                                        "keyRaw": "email",
                                        "visible": true,
                                    },

                                    "avatarUrl": {
                                        "type": "String",
                                        "keyRaw": "avatarUrl",
                                        "nullable": true,
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

            "analyticUserProfile": {
                "type": "AnalyticUserProfilePayload",
                "keyRaw": "analyticUserProfile(key: \"work\")",
                "nullable": true,

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
            "userProfileId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile = {
	readonly "input": OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input;
	readonly "result": OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result | undefined;
};

export type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$result = {
	readonly onboardingContext: {
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
	} | null;
	/**
	 * Read an active user profile using verified user identity.
	 */
	readonly userProfile: {
		readonly node: {
			/**
			 * Display name; this field does not establish identity or uniqueness.
			 */
			readonly displayName: string;
			readonly username: string;
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email: string;
			/**
			 * Public avatar URL; the object may not exist yet and clients must show a fallback.
			 */
			readonly avatarUrl: string | null;
		};
	} | null;
	readonly analyticUserProfile: {
		readonly node: {
			readonly value: string;
		};
	} | null;
};

export type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$input = {
	userProfileId: string;
};

export type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$unmasked = {
	readonly onboardingContext: {
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
	} | null;
	/**
	 * Read an active user profile using verified user identity.
	 */
	readonly userProfile: {
		readonly node: {
			/**
			 * Display name; this field does not establish identity or uniqueness.
			 */
			readonly displayName: string;
			readonly username: string;
			/**
			 * Email is readable by authenticated users and cannot be updated here.
			 */
			readonly email: string;
			/**
			 * Public avatar URL; the object may not exist yet and clients must show a fallback.
			 */
			readonly avatarUrl: string | null;
			readonly __typename: "User";
			/**
			 * Stable user identity, serialized as a string.
			 */
			readonly id: string;
		};
		readonly __typename: "UserPayload";
	} | null;
	readonly analyticUserProfile: {
		readonly node: {
			readonly value: string;
			readonly __typename: "AnalyticUserProfile";
		};
		readonly __typename: "AnalyticUserProfilePayload";
	} | null;
};

export type OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile$artifact = typeof artifact

"HoudiniHash=73d7d572194e052ac5fbca9ecbbd5cba793b681fb09a1c47d4ee80326c0219d6"
