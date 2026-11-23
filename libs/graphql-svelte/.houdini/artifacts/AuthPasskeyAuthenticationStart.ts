const artifact = {
    "name": "AuthPasskeyAuthenticationStart",
    "kind": "HoudiniMutation",
    "hash": "e4ef530592b890f9e50600eb20205c35b1a1eb797ff268d7a4f6fa2be971198a",
    "raw": `mutation AuthPasskeyAuthenticationStart {
    passkeyAuthenticationStart(purpose: PASSKEY_CHANGE) {
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
                "keyRaw": "passkeyAuthenticationStart(purpose: PASSKEY_CHANGE)",

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

export type AuthPasskeyAuthenticationStart = {
	readonly "input"?: AuthPasskeyAuthenticationStart$input;
	readonly "result": AuthPasskeyAuthenticationStart$result;
};

export type AuthPasskeyAuthenticationStart$result = {
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

export type AuthPasskeyAuthenticationStart$input = null | undefined;

export type AuthPasskeyAuthenticationStart$optimistic = {
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

export type AuthPasskeyAuthenticationStart$unmasked = {
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

export type AuthPasskeyAuthenticationStart$artifact = typeof artifact

"HoudiniHash=e4ef530592b890f9e50600eb20205c35b1a1eb797ff268d7a4f6fa2be971198a"
