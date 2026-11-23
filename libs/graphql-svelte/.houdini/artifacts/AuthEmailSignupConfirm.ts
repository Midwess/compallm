import type { EmailCodeInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthEmailSignupConfirm",
    "kind": "HoudiniMutation",
    "hash": "7caa7a0b30630ae0add08cbde523528863b08faee6554da7f2d3101217dc4c2e",
    "raw": `mutation AuthEmailSignupConfirm($emailSignupConfirmInput: EmailCodeInput!) {
    emailSignupConfirm(input: $emailSignupConfirmInput) {
        node {
            accessLifetime {
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
            "emailSignupConfirm": {
                "type": "SessionCredentialsPayload",
                "keyRaw": "emailSignupConfirm(input: $emailSignupConfirmInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "SessionCredentials",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "accessLifetime": {
                                        "type": "TokenLifetime",
                                        "keyRaw": "accessLifetime",
                                        "nullable": true,

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
            "emailSignupConfirmInput": "EmailCodeInput",
        },

        "types": {
            "EmailCodeInput": {
                "code": "String",
                "delivery": "CredentialDelivery",
                "purpose": "AuthenticationPurpose",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type AuthEmailSignupConfirm = {
	readonly "input": AuthEmailSignupConfirm$input;
	readonly "result": AuthEmailSignupConfirm$result;
};

export type AuthEmailSignupConfirm$result = {
	/**
	 * Confirm the verified signup challenge and create the account and session.
	 */
	readonly emailSignupConfirm: {
		readonly node: {
			readonly accessLifetime: {
				readonly expiresInSeconds: string;
			} | null;
		};
	};
};

export type AuthEmailSignupConfirm$input = {
	emailSignupConfirmInput: EmailCodeInput;
};

export type AuthEmailSignupConfirm$optimistic = {
	/**
	 * Confirm the verified signup challenge and create the account and session.
	 */
	readonly emailSignupConfirm?: {
		readonly node?: {
			readonly accessLifetime?: {
				readonly expiresInSeconds?: string;
			} | null;
		};
	};
};

export type AuthEmailSignupConfirm$unmasked = {
	/**
	 * Confirm the verified signup challenge and create the account and session.
	 */
	readonly emailSignupConfirm: {
		readonly node: {
			readonly accessLifetime: {
				readonly expiresInSeconds: string;
				readonly __typename: "TokenLifetime";
			} | null;
			readonly __typename: "SessionCredentials";
		};
		readonly __typename: "SessionCredentialsPayload";
	};
};

export type AuthEmailSignupConfirm$artifact = typeof artifact

"HoudiniHash=7caa7a0b30630ae0add08cbde523528863b08faee6554da7f2d3101217dc4c2e"
