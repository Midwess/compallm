import type { EmailCodeInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthInvitationControllerEmailSignupConfirm",
    "kind": "HoudiniMutation",
    "hash": "21be92a4d0b74fd32cee3037d9671859363cfee322e9f3b25a03ab43a9e17f31",
    "raw": `mutation AuthInvitationControllerEmailSignupConfirm($emailSignupConfirmInput: EmailCodeInput!) {
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

export type AuthInvitationControllerEmailSignupConfirm = {
	readonly "input": AuthInvitationControllerEmailSignupConfirm$input;
	readonly "result": AuthInvitationControllerEmailSignupConfirm$result;
};

export type AuthInvitationControllerEmailSignupConfirm$result = {
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

export type AuthInvitationControllerEmailSignupConfirm$input = {
	emailSignupConfirmInput: EmailCodeInput;
};

export type AuthInvitationControllerEmailSignupConfirm$optimistic = {
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

export type AuthInvitationControllerEmailSignupConfirm$unmasked = {
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

export type AuthInvitationControllerEmailSignupConfirm$artifact = typeof artifact

"HoudiniHash=21be92a4d0b74fd32cee3037d9671859363cfee322e9f3b25a03ab43a9e17f31"
