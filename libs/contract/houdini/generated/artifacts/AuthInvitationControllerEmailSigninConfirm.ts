import type { EmailCodeInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthInvitationControllerEmailSigninConfirm",
    "kind": "HoudiniMutation",
    "hash": "93762b664d07b7ddf007446e8ffed5a0b6cccae186471c6d8fc3387d20bf55b6",
    "raw": `mutation AuthInvitationControllerEmailSigninConfirm($emailSigninConfirmInput: EmailCodeInput!) {
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

export type AuthInvitationControllerEmailSigninConfirm = {
	readonly "input": AuthInvitationControllerEmailSigninConfirm$input;
	readonly "result": AuthInvitationControllerEmailSigninConfirm$result;
};

export type AuthInvitationControllerEmailSigninConfirm$result = {
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

export type AuthInvitationControllerEmailSigninConfirm$input = {
	emailSigninConfirmInput: EmailCodeInput;
};

export type AuthInvitationControllerEmailSigninConfirm$optimistic = {
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

export type AuthInvitationControllerEmailSigninConfirm$unmasked = {
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

export type AuthInvitationControllerEmailSigninConfirm$artifact = typeof artifact

"HoudiniHash=93762b664d07b7ddf007446e8ffed5a0b6cccae186471c6d8fc3387d20bf55b6"
