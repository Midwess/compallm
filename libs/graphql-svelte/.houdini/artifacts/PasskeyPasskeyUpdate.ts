import type { PasskeyRegistrationInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "PasskeyPasskeyUpdate",
    "kind": "HoudiniMutation",
    "hash": "275a035178617978b66681edbca77e6b0271c1490d06a9ba9362cec935cadb2e",
    "raw": `mutation PasskeyPasskeyUpdate($passkeyUpdateId: ID!, $passkeyUpdateInput: PasskeyRegistrationInput!) {
    passkeyUpdate(id: $passkeyUpdateId, input: $passkeyUpdateInput) {
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
            "passkeyUpdate": {
                "type": "PasskeySummaryPayload",
                "keyRaw": "passkeyUpdate(id: $passkeyUpdateId, input: $passkeyUpdateInput)",

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
            "passkeyUpdateId": "ID",
            "passkeyUpdateInput": "PasskeyRegistrationInput",
        },

        "types": {
            "PasskeyRegistrationInput": {
                "name": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type PasskeyPasskeyUpdate = {
	readonly "input": PasskeyPasskeyUpdate$input;
	readonly "result": PasskeyPasskeyUpdate$result;
};

export type PasskeyPasskeyUpdate$result = {
	readonly passkeyUpdate: {
		readonly node: {
			readonly credentialId: string;
			readonly id: string;
			readonly name: string;
			readonly registeredAt: string;
			readonly lastUsedAt: string | null;
		};
	};
};

export type PasskeyPasskeyUpdate$input = {
	passkeyUpdateId: string;
	passkeyUpdateInput: PasskeyRegistrationInput;
};

export type PasskeyPasskeyUpdate$optimistic = {
	readonly passkeyUpdate?: {
		readonly node?: {
			readonly credentialId?: string;
			readonly id?: string;
			readonly name?: string;
			readonly registeredAt?: string;
			readonly lastUsedAt?: string | null;
		};
	};
};

export type PasskeyPasskeyUpdate$unmasked = {
	readonly passkeyUpdate: {
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

export type PasskeyPasskeyUpdate$artifact = typeof artifact

"HoudiniHash=275a035178617978b66681edbca77e6b0271c1490d06a9ba9362cec935cadb2e"
