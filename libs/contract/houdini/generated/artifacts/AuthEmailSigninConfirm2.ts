import type { EmailCodeInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthEmailSigninConfirm2",
    "kind": "HoudiniMutation",
    "hash": "6afba1036b92d57949a55ad0a0fed35a8d0e4c3cb0d21ff6bf9ba7649fb941f3",
    "raw": `mutation AuthEmailSigninConfirm2($emailSigninConfirmInput: EmailCodeInput!) {
    emailSigninConfirm(input: $emailSigninConfirmInput) {
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
            "emailSigninConfirm": {
                "type": "SessionCredentialsPayload",
                "keyRaw": "emailSigninConfirm(input: $emailSigninConfirmInput)",

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
            "emailSigninConfirmInput": "EmailCodeInput",
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

export type AuthEmailSigninConfirm2 = {
	readonly "input": AuthEmailSigninConfirm2$input;
	readonly "result": AuthEmailSigninConfirm2$result;
};

export type AuthEmailSigninConfirm2$result = {
	/**
	 * Confirm the verified signin challenge and create a scoped session.
	 */
	readonly emailSigninConfirm: {
		readonly node: {
			readonly accessLifetime: {
				readonly expiresInSeconds: string;
			} | null;
		};
	};
};

export type AuthEmailSigninConfirm2$input = {
	emailSigninConfirmInput: EmailCodeInput;
};

export type AuthEmailSigninConfirm2$optimistic = {
	/**
	 * Confirm the verified signin challenge and create a scoped session.
	 */
	readonly emailSigninConfirm?: {
		readonly node?: {
			readonly accessLifetime?: {
				readonly expiresInSeconds?: string;
			} | null;
		};
	};
};

export type AuthEmailSigninConfirm2$unmasked = {
	/**
	 * Confirm the verified signin challenge and create a scoped session.
	 */
	readonly emailSigninConfirm: {
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

export type AuthEmailSigninConfirm2$artifact = typeof artifact

"HoudiniHash=6afba1036b92d57949a55ad0a0fed35a8d0e4c3cb0d21ff6bf9ba7649fb941f3"
