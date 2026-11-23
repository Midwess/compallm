const artifact = {
    "name": "PasskeyPasskeys2",
    "kind": "HoudiniQuery",
    "hash": "88fcdd17f05e94bea6cc2e07317ddb56abe95b5dede862655da842449aca50af",
    "raw": `query PasskeyPasskeys2 {
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

export type PasskeyPasskeys2 = {
	readonly "input"?: PasskeyPasskeys2$input;
	readonly "result": PasskeyPasskeys2$result | undefined;
};

export type PasskeyPasskeys2$result = {
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

export type PasskeyPasskeys2$input = null | undefined;

export type PasskeyPasskeys2$unmasked = {
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

export type PasskeyPasskeys2$artifact = typeof artifact

"HoudiniHash=88fcdd17f05e94bea6cc2e07317ddb56abe95b5dede862655da842449aca50af"
