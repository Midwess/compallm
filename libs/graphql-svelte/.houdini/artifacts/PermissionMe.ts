const artifact = {
    "name": "PermissionMe",
    "kind": "HoudiniQuery",
    "hash": "2026610c42981eb021f91ae5907dc3d87bb6f8f8a7dbe957f6a04149abf18d1a",
    "raw": `query PermissionMe {
    me {
        node {
            id
            organizationId
            cedarSource
            cedarEntities
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
            "me": {
                "type": "MePayload",
                "keyRaw": "me",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Me",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "organizationId": {
                                        "type": "ID",
                                        "keyRaw": "organizationId",
                                        "visible": true,
                                    },

                                    "cedarSource": {
                                        "type": "String",
                                        "keyRaw": "cedarSource",
                                        "visible": true,
                                    },

                                    "cedarEntities": {
                                        "type": "String",
                                        "keyRaw": "cedarEntities",
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

export type PermissionMe = {
	readonly "input"?: PermissionMe$input;
	readonly "result": PermissionMe$result | undefined;
};

export type PermissionMe$result = {
	readonly me: {
		readonly node: {
			readonly id: string;
			readonly organizationId: string;
			/**
			 * Effective policies for this verified user only. UI decisions do not grant server authority.
			 */
			readonly cedarSource: string;
			/**
			 * Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials.
			 */
			readonly cedarEntities: string;
		};
	};
};

export type PermissionMe$input = null | undefined;

export type PermissionMe$unmasked = {
	readonly me: {
		readonly node: {
			readonly id: string;
			readonly organizationId: string;
			/**
			 * Effective policies for this verified user only. UI decisions do not grant server authority.
			 */
			readonly cedarSource: string;
			/**
			 * Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials.
			 */
			readonly cedarEntities: string;
			readonly __typename: "Me";
		};
		readonly __typename: "MePayload";
	};
};

export type PermissionMe$artifact = typeof artifact

"HoudiniHash=2026610c42981eb021f91ae5907dc3d87bb6f8f8a7dbe957f6a04149abf18d1a"
