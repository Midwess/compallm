import type { PasskeyFinishInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "AuthPasskeyAuthenticationFinish2",
    "kind": "HoudiniMutation",
    "hash": "7cb2f4872c2ef946be0565ba66a52a7e19374d37cb49343c38e1c6dbd11a8003",
    "raw": `mutation AuthPasskeyAuthenticationFinish2($passkeyAuthenticationFinishInput: PasskeyFinishInput!) {
    passkeyAuthenticationFinish(input: $passkeyAuthenticationFinishInput) {
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

export type AuthPasskeyAuthenticationFinish2 = {
	readonly "input": AuthPasskeyAuthenticationFinish2$input;
	readonly "result": AuthPasskeyAuthenticationFinish2$result;
};

export type AuthPasskeyAuthenticationFinish2$result = {
	readonly passkeyAuthenticationFinish: {
		readonly node: {
			readonly accessLifetime: {
				readonly expiresInSeconds: string;
			} | null;
		};
	};
};

export type AuthPasskeyAuthenticationFinish2$input = {
	passkeyAuthenticationFinishInput: PasskeyFinishInput;
};

export type AuthPasskeyAuthenticationFinish2$optimistic = {
	readonly passkeyAuthenticationFinish?: {
		readonly node?: {
			readonly accessLifetime?: {
				readonly expiresInSeconds?: string;
			} | null;
		};
	};
};

export type AuthPasskeyAuthenticationFinish2$unmasked = {
	readonly passkeyAuthenticationFinish: {
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

export type AuthPasskeyAuthenticationFinish2$artifact = typeof artifact

"HoudiniHash=7cb2f4872c2ef946be0565ba66a52a7e19374d37cb49343c38e1c6dbd11a8003"
