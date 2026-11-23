import type { AuthenticationPurpose$options } from "$houdini/graphql/enums";
import type { PasskeyFinishInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthPasskeyAuthenticationFinish",
    "kind": "HoudiniMutation",
    "hash": "11bcadb8ab9f932b1e915b7bb2127a58b4fb94bd3999c0f1c23b7432d75bea46",
    "raw": `mutation AuthPasskeyAuthenticationFinish($passkeyAuthenticationFinishInput: PasskeyFinishInput!) {
    passkeyAuthenticationFinish(input: $passkeyAuthenticationFinishInput) {
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
            "passkeyAuthenticationFinish": {
                "type": "SessionCredentialsPayload",
                "keyRaw": "passkeyAuthenticationFinish(input: $passkeyAuthenticationFinishInput)",

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
            "passkeyAuthenticationFinishInput": "PasskeyFinishInput",
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
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type AuthPasskeyAuthenticationFinish = {
	readonly "input": AuthPasskeyAuthenticationFinish$input;
	readonly "result": AuthPasskeyAuthenticationFinish$result;
};

export type AuthPasskeyAuthenticationFinish$result = {
	readonly passkeyAuthenticationFinish: {
		readonly node: {
			readonly purpose: AuthenticationPurpose$options;
		};
	};
};

export type AuthPasskeyAuthenticationFinish$input = {
	passkeyAuthenticationFinishInput: PasskeyFinishInput;
};

export type AuthPasskeyAuthenticationFinish$optimistic = {
	readonly passkeyAuthenticationFinish?: {
		readonly node?: {
			readonly purpose?: AuthenticationPurpose$options;
		};
	};
};

export type AuthPasskeyAuthenticationFinish$unmasked = {
	readonly passkeyAuthenticationFinish: {
		readonly node: {
			readonly purpose: AuthenticationPurpose$options;
			readonly __typename: "SessionCredentials";
		};
		readonly __typename: "SessionCredentialsPayload";
	};
};

export type AuthPasskeyAuthenticationFinish$artifact = typeof artifact

"HoudiniHash=11bcadb8ab9f932b1e915b7bb2127a58b4fb94bd3999c0f1c23b7432d75bea46"
