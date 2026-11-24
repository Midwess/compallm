import type { PasskeyFinishInput, PasskeyRegistrationInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "PasskeyPasskeyRegistrationFinish",
    "kind": "HoudiniMutation",
    "hash": "3f9f831e10cbbe30924c7cb9f86296bf0f7030ce668f91426544231d069a804d",
    "raw": `mutation PasskeyPasskeyRegistrationFinish($passkeyRegistrationFinishInput: PasskeyFinishInput!, $passkeyRegistrationFinishName: PasskeyRegistrationInput!) {
    passkeyRegistrationFinish(input: $passkeyRegistrationFinishInput, name: $passkeyRegistrationFinishName) {
        node {
            credentialId
            id
            name
            registeredAt
            lastUsedAt
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
            "passkeyRegistrationFinish": {
                "type": "PasskeySummaryPayload",
                "keyRaw": "passkeyRegistrationFinish(input: $passkeyRegistrationFinishInput, name: $passkeyRegistrationFinishName)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "PasskeySummary",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "credentialId": {
                                        "type": "String",
                                        "keyRaw": "credentialId",
                                        "visible": true,
                                    },

                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
                                        "visible": true,
                                    },

                                    "registeredAt": {
                                        "type": "String",
                                        "keyRaw": "registeredAt",
                                        "visible": true,
                                    },

                                    "lastUsedAt": {
                                        "type": "String",
                                        "keyRaw": "lastUsedAt",
                                        "nullable": true,
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
            "passkeyRegistrationFinishInput": "PasskeyFinishInput",
            "passkeyRegistrationFinishName": "PasskeyRegistrationInput",
        },

        "types": {
            "PasskeyFinishInput": {
                "attestationObject": "String",
                "authenticatorData": "String",
                "clientDataJson": "String",
                "delivery": "CredentialDelivery",
                "id": "String",
                "purpose": "AuthenticationPurpose",
                "rawId": "String",
                "signature": "String",
                "token": "String",
                "transports": "String",
                "userHandle": "String",
            },
            "PasskeyRegistrationInput": {
                "name": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type PasskeyPasskeyRegistrationFinish = {
	readonly "input": PasskeyPasskeyRegistrationFinish$input;
	readonly "result": PasskeyPasskeyRegistrationFinish$result;
};

export type PasskeyPasskeyRegistrationFinish$result = {
	readonly passkeyRegistrationFinish: {
		readonly node: {
			readonly credentialId: string;
			readonly id: string;
			readonly name: string;
			readonly registeredAt: string;
			readonly lastUsedAt: string | null;
		};
	};
};

export type PasskeyPasskeyRegistrationFinish$input = {
	passkeyRegistrationFinishInput: PasskeyFinishInput;
	passkeyRegistrationFinishName: PasskeyRegistrationInput;
};

export type PasskeyPasskeyRegistrationFinish$optimistic = {
	readonly passkeyRegistrationFinish?: {
		readonly node?: {
			readonly credentialId?: string;
			readonly id?: string;
			readonly name?: string;
			readonly registeredAt?: string;
			readonly lastUsedAt?: string | null;
		};
	};
};

export type PasskeyPasskeyRegistrationFinish$unmasked = {
	readonly passkeyRegistrationFinish: {
		readonly node: {
			readonly credentialId: string;
			readonly id: string;
			readonly name: string;
			readonly registeredAt: string;
			readonly lastUsedAt: string | null;
			readonly __typename: "PasskeySummary";
		};
		readonly __typename: "PasskeySummaryPayload";
	};
};

export type PasskeyPasskeyRegistrationFinish$artifact = typeof artifact

"HoudiniHash=3f9f831e10cbbe30924c7cb9f86296bf0f7030ce668f91426544231d069a804d"
