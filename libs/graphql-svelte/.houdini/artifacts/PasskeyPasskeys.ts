const artifact = {
    "name": "PasskeyPasskeys",
    "kind": "HoudiniQuery",
    "hash": "14c451f69b6299c2ab83b78b2b948d50009f3f7586861ede00591102c541ac58",
    "raw": `query PasskeyPasskeys {
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
        },
    },

    "pluginData": {},
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type PasskeyPasskeys = {
	readonly "input"?: PasskeyPasskeys$input;
	readonly "result": PasskeyPasskeys$result | undefined;
};

export type PasskeyPasskeys$result = {
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
};

export type PasskeyPasskeys$input = null | undefined;

export type PasskeyPasskeys$unmasked = {
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
};

export type PasskeyPasskeys$artifact = typeof artifact

"HoudiniHash=14c451f69b6299c2ab83b78b2b948d50009f3f7586861ede00591102c541ac58"
