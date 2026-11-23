import type { AuthenticationPurpose$options } from "$houdini/graphql/enums";
import type { EmailCodeInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthEmailSigninConfirm",
    "kind": "HoudiniMutation",
    "hash": "95da1c3a7ae3e14ccdf793c80a0799ebd945100413ddbd9c6593ba0fae605473",
    "raw": `mutation AuthEmailSigninConfirm($emailSigninConfirmInput: EmailCodeInput!) {
    emailSigninConfirm(input: $emailSigninConfirmInput) {
        node {
            purpose
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
                                    "purpose": {
                                        "type": "AuthenticationPurpose",
                                        "keyRaw": "purpose",
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

export type AuthEmailSigninConfirm = {
	readonly "input": AuthEmailSigninConfirm$input;
	readonly "result": AuthEmailSigninConfirm$result;
};

export type AuthEmailSigninConfirm$result = {
	/**
	 * Confirm the verified signin challenge and create a scoped session.
	 */
	readonly emailSigninConfirm: {
		readonly node: {
			readonly purpose: AuthenticationPurpose$options;
		};
	};
};

export type AuthEmailSigninConfirm$input = {
	emailSigninConfirmInput: EmailCodeInput;
};

export type AuthEmailSigninConfirm$optimistic = {
	/**
	 * Confirm the verified signin challenge and create a scoped session.
	 */
	readonly emailSigninConfirm?: {
		readonly node?: {
			readonly purpose?: AuthenticationPurpose$options;
		};
	};
};

export type AuthEmailSigninConfirm$unmasked = {
	/**
	 * Confirm the verified signin challenge and create a scoped session.
	 */
	readonly emailSigninConfirm: {
		readonly node: {
			readonly purpose: AuthenticationPurpose$options;
			readonly __typename: "SessionCredentials";
		};
		readonly __typename: "SessionCredentialsPayload";
	};
};

export type AuthEmailSigninConfirm$artifact = typeof artifact

"HoudiniHash=95da1c3a7ae3e14ccdf793c80a0799ebd945100413ddbd9c6593ba0fae605473"
