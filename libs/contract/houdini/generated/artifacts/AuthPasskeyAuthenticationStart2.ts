const artifact = {
    "name": "AuthPasskeyAuthenticationStart2",
    "kind": "HoudiniMutation",
    "hash": "8e928f42f1a1a16b41dd6c8235616cd415ab876d477e287df95c46c807a3b204",
    "raw": `mutation AuthPasskeyAuthenticationStart2 {
    passkeyAuthenticationStart(purpose: LOGIN) {
        node {
            token
            expiresAt
            options {
                challenge
                rpId
                rpName
                userId
                userName
                userDisplayName
                algorithms
                credentials {
                    id
                    transports
                    __typename
                }
                timeout
                userVerification
                residentKey
                extensions {
                    credProps
                    credentialProtectionPolicy
                    enforceCredentialProtectionPolicy
                    __typename
                }
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
            "passkeyAuthenticationStart": {
                "type": "PasskeyCeremonyPayload",
                "keyRaw": "passkeyAuthenticationStart(purpose: LOGIN)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "PasskeyCeremony",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "token": {
                                        "type": "String",
                                        "keyRaw": "token",
                                        "visible": true,
                                    },

                                    "expiresAt": {
                                        "type": "String",
                                        "keyRaw": "expiresAt",
                                        "visible": true,
                                    },

                                    "options": {
                                        "type": "PasskeyOptions",
                                        "keyRaw": "options",

                                        "selection": {
                                            "fields": {
                                                "challenge": {
                                                    "type": "String",
                                                    "keyRaw": "challenge",
                                                    "visible": true,
                                                },

                                                "rpId": {
                                                    "type": "String",
                                                    "keyRaw": "rpId",
                                                    "visible": true,
                                                },

                                                "rpName": {
                                                    "type": "String",
                                                    "keyRaw": "rpName",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "userId": {
                                                    "type": "String",
                                                    "keyRaw": "userId",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "userName": {
                                                    "type": "String",
                                                    "keyRaw": "userName",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "userDisplayName": {
                                                    "type": "String",
                                                    "keyRaw": "userDisplayName",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "algorithms": {
                                                    "type": "Int",
                                                    "keyRaw": "algorithms",
                                                    "visible": true,
                                                },

                                                "credentials": {
                                                    "type": "PasskeyDescriptor",
                                                    "keyRaw": "credentials",

                                                    "selection": {
                                                        "fields": {
                                                            "id": {
                                                                "type": "String",
                                                                "keyRaw": "id",
                                                                "visible": true,
                                                            },

                                                            "transports": {
                                                                "type": "String",
                                                                "keyRaw": "transports",
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

                                                "timeout": {
                                                    "type": "Int",
                                                    "keyRaw": "timeout",
                                                    "visible": true,
                                                },

                                                "userVerification": {
                                                    "type": "String",
                                                    "keyRaw": "userVerification",
                                                    "visible": true,
                                                },

                                                "residentKey": {
                                                    "type": "String",
                                                    "keyRaw": "residentKey",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "extensions": {
                                                    "type": "PasskeyExtensions",
                                                    "keyRaw": "extensions",

                                                    "selection": {
                                                        "fields": {
                                                            "credProps": {
                                                                "type": "Boolean",
                                                                "keyRaw": "credProps",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "credentialProtectionPolicy": {
                                                                "type": "String",
                                                                "keyRaw": "credentialProtectionPolicy",
                                                                "nullable": true,
                                                                "visible": true,
                                                            },

                                                            "enforceCredentialProtectionPolicy": {
                                                                "type": "Boolean",
                                                                "keyRaw": "enforceCredentialProtectionPolicy",
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
} as const

export default artifact

export type AuthPasskeyAuthenticationStart2 = {
	readonly "input"?: AuthPasskeyAuthenticationStart2$input;
	readonly "result": AuthPasskeyAuthenticationStart2$result;
};

export type AuthPasskeyAuthenticationStart2$result = {
	readonly passkeyAuthenticationStart: {
		readonly node: {
			readonly token: string;
			readonly expiresAt: string;
			readonly options: {
				readonly challenge: string;
				readonly rpId: string;
				readonly rpName: string | null;
				readonly userId: string | null;
				readonly userName: string | null;
				readonly userDisplayName: string | null;
				readonly algorithms: (number)[];
				readonly credentials: ({
					readonly id: string;
					readonly transports: (string)[];
				})[];
				readonly timeout: number;
				readonly userVerification: string;
				readonly residentKey: string | null;
				readonly extensions: {
					readonly credProps: boolean | null;
					readonly credentialProtectionPolicy: string | null;
					readonly enforceCredentialProtectionPolicy: boolean | null;
				};
			};
		};
	};
};

export type AuthPasskeyAuthenticationStart2$input = null | undefined;

export type AuthPasskeyAuthenticationStart2$optimistic = {
	readonly passkeyAuthenticationStart?: {
		readonly node?: {
			readonly token?: string;
			readonly expiresAt?: string;
			readonly options?: {
				readonly challenge?: string;
				readonly rpId?: string;
				readonly rpName?: string | null;
				readonly userId?: string | null;
				readonly userName?: string | null;
				readonly userDisplayName?: string | null;
				readonly algorithms?: (number)[];
				readonly credentials?: ({
					readonly id?: string;
					readonly transports?: (string)[];
				})[];
				readonly timeout?: number;
				readonly userVerification?: string;
				readonly residentKey?: string | null;
				readonly extensions?: {
					readonly credProps?: boolean | null;
					readonly credentialProtectionPolicy?: string | null;
					readonly enforceCredentialProtectionPolicy?: boolean | null;
				};
			};
		};
	};
};

export type AuthPasskeyAuthenticationStart2$unmasked = {
	readonly passkeyAuthenticationStart: {
		readonly node: {
			readonly token: string;
			readonly expiresAt: string;
			readonly options: {
				readonly challenge: string;
				readonly rpId: string;
				readonly rpName: string | null;
				readonly userId: string | null;
				readonly userName: string | null;
				readonly userDisplayName: string | null;
				readonly algorithms: (number)[];
				readonly credentials: ({
					readonly id: string;
					readonly transports: (string)[];
					readonly __typename: "PasskeyDescriptor";
				})[];
				readonly timeout: number;
				readonly userVerification: string;
				readonly residentKey: string | null;
				readonly extensions: {
					readonly credProps: boolean | null;
					readonly credentialProtectionPolicy: string | null;
					readonly enforceCredentialProtectionPolicy: boolean | null;
					readonly __typename: "PasskeyExtensions";
				};
				readonly __typename: "PasskeyOptions";
			};
			readonly __typename: "PasskeyCeremony";
		};
		readonly __typename: "PasskeyCeremonyPayload";
	};
};

export type AuthPasskeyAuthenticationStart2$artifact = typeof artifact

"HoudiniHash=8e928f42f1a1a16b41dd6c8235616cd415ab876d477e287df95c46c807a3b204"
