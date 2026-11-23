const artifact = {
    "name": "SettingsSessions",
    "kind": "HoudiniQuery",
    "hash": "a947c1a79274c9b00c7efa75c4fa2228612cc996889a8c5d4151c6c459453cf8",
    "raw": `query SettingsSessions($sessionsAfter: String) {
    sessions(first: 20, after: $sessionsAfter) {
        edges {
            node {
                id
                ipAddress
                userAgent
                signedInAt
                lastSeenAt
                __typename
            }
            __typename
        }
        pageInfo {
            endCursor
            hasNextPage
            __typename
        }
        __typename
    }
}
`,

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "sessions": {
                "type": "SessionConnection",
                "keyRaw": "sessions(first: 20, after: $sessionsAfter)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "SessionEdge",
                            "keyRaw": "edges",

                            "selection": {
                                "fields": {
                                    "node": {
                                        "type": "Session",
                                        "keyRaw": "node",

                                        "selection": {
                                            "fields": {
                                                "id": {
                                                    "type": "ID",
                                                    "keyRaw": "id",
                                                    "visible": true,
                                                },

                                                "ipAddress": {
                                                    "type": "String",
                                                    "keyRaw": "ipAddress",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "userAgent": {
                                                    "type": "String",
                                                    "keyRaw": "userAgent",
                                                    "nullable": true,
                                                    "visible": true,
                                                },

                                                "signedInAt": {
                                                    "type": "String",
                                                    "keyRaw": "signedInAt",
                                                    "visible": true,
                                                },

                                                "lastSeenAt": {
                                                    "type": "String",
                                                    "keyRaw": "lastSeenAt",
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

                        "pageInfo": {
                            "type": "PageInfo",
                            "keyRaw": "pageInfo",

                            "selection": {
                                "fields": {
                                    "endCursor": {
                                        "type": "String",
                                        "keyRaw": "endCursor",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "hasNextPage": {
                                        "type": "Boolean",
                                        "keyRaw": "hasNextPage",
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
            "sessionsAfter": "String",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsSessions = {
	readonly "input": SettingsSessions$input;
	readonly "result": SettingsSessions$result | undefined;
};

export type SettingsSessions$result = {
	readonly sessions: {
		readonly edges: ({
			readonly node: {
				/**
				 * Login session identity, serialized as a string without numeric precision loss.
				 */
				readonly id: string;
				/**
				 * IP observed by the API connection at sign-in; it may belong to a proxy or VPN.
				 */
				readonly ipAddress: string | null;
				/**
				 * User-Agent captured at sign-in, used only for session recognition.
				 */
				readonly userAgent: string | null;
				readonly signedInAt: string;
				readonly lastSeenAt: string;
			};
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
		};
	};
};

export type SettingsSessions$input = {
	sessionsAfter?: string | null;
};

export type SettingsSessions$unmasked = {
	readonly sessions: {
		readonly edges: ({
			readonly node: {
				/**
				 * Login session identity, serialized as a string without numeric precision loss.
				 */
				readonly id: string;
				/**
				 * IP observed by the API connection at sign-in; it may belong to a proxy or VPN.
				 */
				readonly ipAddress: string | null;
				/**
				 * User-Agent captured at sign-in, used only for session recognition.
				 */
				readonly userAgent: string | null;
				readonly signedInAt: string;
				readonly lastSeenAt: string;
				readonly __typename: "Session";
			};
			readonly __typename: "SessionEdge";
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
			readonly __typename: "PageInfo";
		};
		readonly __typename: "SessionConnection";
	};
};

export type SettingsSessions$artifact = typeof artifact

"HoudiniHash=a947c1a79274c9b00c7efa75c4fa2228612cc996889a8c5d4151c6c459453cf8"
