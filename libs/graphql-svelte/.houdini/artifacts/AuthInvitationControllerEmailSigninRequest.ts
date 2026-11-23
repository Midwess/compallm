import type { EmailInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthInvitationControllerEmailSigninRequest",
    "kind": "HoudiniMutation",
    "hash": "6b3b6b689f79a1f3f90549bcccc1baa883b09e6f3c47eb678ba1d4f8de039233",
    "raw": `mutation AuthInvitationControllerEmailSigninRequest($emailSigninRequestInput: EmailInput!) {
    emailSigninRequest(input: $emailSigninRequestInput) {
        node {
            lifetime {
                expiresInSeconds
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

export type AuthInvitationControllerEmailSigninRequest = {
	readonly "input": AuthInvitationControllerEmailSigninRequest$input;
	readonly "result": AuthInvitationControllerEmailSigninRequest$result;
};

export type AuthInvitationControllerEmailSigninRequest$result = {
	/**
	 * Request a signin email challenge for an existing active account.
	 */
	readonly emailSigninRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
			};
		};
	};
};

export type AuthInvitationControllerEmailSigninRequest$input = {
	emailSigninRequestInput: EmailInput;
};

export type AuthInvitationControllerEmailSigninRequest$optimistic = {
	/**
	 * Request a signin email challenge for an existing active account.
	 */
	readonly emailSigninRequest?: {
		readonly node?: {
			readonly lifetime?: {
				readonly expiresInSeconds?: string;
			};
		};
	};
};

export type AuthInvitationControllerEmailSigninRequest$unmasked = {
	/**
	 * Request a signin email challenge for an existing active account.
	 */
	readonly emailSigninRequest: {
		readonly node: {
			readonly lifetime: {
				readonly expiresInSeconds: string;
				readonly __typename: "TokenLifetime";
			};
			readonly __typename: "EmailChallenge";
		};
		readonly __typename: "EmailChallengePayload";
	};
};

export type AuthInvitationControllerEmailSigninRequest$artifact = typeof artifact

"HoudiniHash=6b3b6b689f79a1f3f90549bcccc1baa883b09e6f3c47eb678ba1d4f8de039233"
