import type { EmailInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthEmailSignupRequest",
    "kind": "HoudiniMutation",
    "hash": "5fbbb685ea954af0cf968b0f98d24a65e3fd6ecfa4e3401d808aedaa8b408c58",
    "raw": `mutation AuthEmailSignupRequest($emailSignupRequestInput: EmailInput!) {
    emailSignupRequest(input: $emailSignupRequestInput) {
        node {
            lifetime {
                expiresInSeconds
                throttleSeconds
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
            "emailSignupRequest": {
                "type": "EmailChallengePayload",
                "keyRaw": "emailSignupRequest(input: $emailSignupRequestInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "EmailChallenge",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "lifetime": {
                                        "type": "TokenLifetime",
                                        "keyRaw": "lifetime",

                                        "selection": {
                                            "fields": {
                                                "expiresInSeconds": {
                                                    "type": "String",
                                                    "keyRaw": "expiresInSeconds",
                                                    "visible": true,
                                                },

                                                "throttleSeconds": {
                                                    "type": "String",
                                                    "keyRaw": "throttleSeconds",
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
            "emailSignupRequestInput": "EmailInput",
        },

        "types": {
            "EmailInput": {
                "delivery": "CredentialDelivery",
                "email": "String",
                "invitation": "Boolean",
                "purpose": "AuthenticationPurpose",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type AuthEmailSignupRequest = {
	readonly "input": AuthEmailSignupRequest$input;
	readonly "result": AuthEmailSignupRequest$result;
};

export type AuthEmailSignupRequest$result = {
	/**
	 * Request a signup email challenge; reject existing accounts before sending.
	 */
	readonly emailSignupRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
				readonly throttleSeconds: string;
			};
		};
	};
};

export type AuthEmailSignupRequest$input = {
	emailSignupRequestInput: EmailInput;
};

export type AuthEmailSignupRequest$optimistic = {
	/**
	 * Request a signup email challenge; reject existing accounts before sending.
	 */
	readonly emailSignupRequest?: {
		readonly node?: {
			readonly lifetime?: {
				readonly expiresInSeconds?: string;
				readonly throttleSeconds?: string;
			};
		};
	};
};

export type AuthEmailSignupRequest$unmasked = {
	/**
	 * Request a signup email challenge; reject existing accounts before sending.
	 */
	readonly emailSignupRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
				readonly throttleSeconds: string;
				readonly __typename: "TokenLifetime";
			};
			readonly __typename: "EmailChallenge";
		};
		readonly __typename: "EmailChallengePayload";
	};
};

export type AuthEmailSignupRequest$artifact = typeof artifact

"HoudiniHash=5fbbb685ea954af0cf968b0f98d24a65e3fd6ecfa4e3401d808aedaa8b408c58"
