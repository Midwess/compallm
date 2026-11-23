import type { EmailInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthEmailSigninRequest",
    "kind": "HoudiniMutation",
    "hash": "a1f03d0d2fd710d72b53645c648d3f075ba05c2eeeb135941c9b560ba872113d",
    "raw": `mutation AuthEmailSigninRequest($emailSigninRequestInput: EmailInput!) {
    emailSigninRequest(input: $emailSigninRequestInput) {
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
            "emailSigninRequest": {
                "type": "EmailChallengePayload",
                "keyRaw": "emailSigninRequest(input: $emailSigninRequestInput)",

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
            "emailSigninRequestInput": "EmailInput",
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

export type AuthEmailSigninRequest = {
	readonly "input": AuthEmailSigninRequest$input;
	readonly "result": AuthEmailSigninRequest$result;
};

export type AuthEmailSigninRequest$result = {
	/**
	 * Request a signin email challenge for an existing active account.
	 */
	readonly emailSigninRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
				readonly throttleSeconds: string;
			};
		};
	};
};

export type AuthEmailSigninRequest$input = {
	emailSigninRequestInput: EmailInput;
};

export type AuthEmailSigninRequest$optimistic = {
	/**
	 * Request a signin email challenge for an existing active account.
	 */
	readonly emailSigninRequest?: {
		readonly node?: {
			readonly lifetime?: {
				readonly expiresInSeconds?: string;
				readonly throttleSeconds?: string;
			};
		};
	};
};

export type AuthEmailSigninRequest$unmasked = {
	/**
	 * Request a signin email challenge for an existing active account.
	 */
	readonly emailSigninRequest: {
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

export type AuthEmailSigninRequest$artifact = typeof artifact

"HoudiniHash=a1f03d0d2fd710d72b53645c648d3f075ba05c2eeeb135941c9b560ba872113d"
