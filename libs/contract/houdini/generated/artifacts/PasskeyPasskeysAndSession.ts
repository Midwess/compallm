const artifact = {
    "name": "PasskeyPasskeysAndSession",
    "kind": "HoudiniQuery",
    "hash": "0f19de7bfc542fe40fee767311f3057007d6e5d9c64a551e705d19c9e17c711e",
    "raw": `query PasskeyPasskeysAndSession {
    passkeys {
        edges {
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
        __typename
    }
    session {
        node {
            passkey {
                id
                __typename
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
            "passkeys": {
                "type": "PasskeySummaryConnection",
                "keyRaw": "passkeys",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "PasskeySummaryEdge",
                            "keyRaw": "edges",

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

                        "__typename": {
                            "type": "String",
                            "keyRaw": "__typename",
                        },
                    },
                },

                "visible": true,
            },

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
                                                "id": {
                                                    "type": "ID",
                                                    "keyRaw": "id",
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

export type PasskeyPasskeysAndSession = {
	readonly "input"?: PasskeyPasskeysAndSession$input;
	readonly "result": PasskeyPasskeysAndSession$result | undefined;
};

export type PasskeyPasskeysAndSession$result = {
	readonly passkeys: {
		readonly edges: ({
			readonly node: {
				readonly credentialId: string;
				readonly id: string;
				readonly name: string;
				readonly registeredAt: string;
				readonly lastUsedAt: string | null;
			};
		})[];
	};
	/**
	 * Return the verified current session identity.
	 */
	readonly session: {
		readonly node: {
			readonly passkey: {
				readonly id: string;
			} | null;
		};
	};
};

export type PasskeyPasskeysAndSession$input = null | undefined;

export type PasskeyPasskeysAndSession$unmasked = {
	readonly passkeys: {
		readonly edges: ({
			readonly node: {
				readonly credentialId: string;
				readonly id: string;
				readonly name: string;
				readonly registeredAt: string;
				readonly lastUsedAt: string | null;
				readonly __typename: "PasskeySummary";
			};
			readonly __typename: "PasskeySummaryEdge";
		})[];
		readonly __typename: "PasskeySummaryConnection";
	};
	/**
	 * Return the verified current session identity.
	 */
	readonly session: {
		readonly node: {
			readonly passkey: {
				readonly id: string;
				readonly __typename: "PasskeySummary";
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

export type PasskeyPasskeysAndSession$artifact = typeof artifact

"HoudiniHash=0f19de7bfc542fe40fee767311f3057007d6e5d9c64a551e705d19c9e17c711e"
