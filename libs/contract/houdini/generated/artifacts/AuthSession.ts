const artifact = {
    "name": "AuthSession",
    "kind": "HoudiniQuery",
    "hash": "29ed391ed66661692ca2470cec91a073d367b2702febf6895f1fd9d429720b76",
    "raw": `query AuthSession {
    session {
        node {
            passkey {
                credentialId
                __typename
                id
            }
            __typename
            id
        }
        __typename
    }
}
`,

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "session": {
                "type": "SessionPayload",
                "keyRaw": "session",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Session",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "passkey": {
                                        "type": "PasskeySummary",
                                        "keyRaw": "passkey",
                                        "nullable": true,

                                        "selection": {
                                            "fields": {
                                                "credentialId": {
                                                    "type": "String",
                                                    "keyRaw": "credentialId",
                                                    "visible": true,
                                                },

                                                "__typename": {
                                                    "type": "String",
                                                    "keyRaw": "__typename",
                                                },

                                                "id": {
                                                    "type": "ID",
                                                    "keyRaw": "id",
                                                },
                                            },
                                        },

                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },

                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
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
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type AuthSession = {
	readonly "input"?: AuthSession$input;
	readonly "result": AuthSession$result | undefined;
};

export type AuthSession$result = {
	/**
	 * Return the verified current session identity.
	 */
	readonly session: {
		readonly node: {
			readonly passkey: {
				readonly credentialId: string;
			} | null;
		};
	};
};

export type AuthSession$input = null | undefined;

export type AuthSession$unmasked = {
	/**
	 * Return the verified current session identity.
	 */
	readonly session: {
		readonly node: {
			readonly passkey: {
				readonly credentialId: string;
				readonly __typename: "PasskeySummary";
				readonly id: string;
			} | null;
			readonly __typename: "Session";
			/**
			 * Login session identity, serialized as a string without numeric precision loss.
			 */
			readonly id: string;
		};
		readonly __typename: "SessionPayload";
	};
};

export type AuthSession$artifact = typeof artifact

"HoudiniHash=29ed391ed66661692ca2470cec91a073d367b2702febf6895f1fd9d429720b76"
