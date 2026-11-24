const artifact = {
    "name": "SettingsRoleOrganizationPolicyActions",
    "kind": "HoudiniQuery",
    "hash": "5a9d114f64e50f66175e72b6b1c8b72e336dd0e07d494d3e13e7adca154bf941",
    "raw": `query SettingsRoleOrganizationPolicyActions {
    organizationPolicyActions {
        edges {
            node {
                name
                displayName
                description
                entityType
                resourceType
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
            "organizationPolicyActions": {
                "type": "PolicyActionConnection",
                "keyRaw": "organizationPolicyActions",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "PolicyActionEdge",
                            "keyRaw": "edges",

                            "selection": {
                                "fields": {
                                    "node": {
                                        "type": "PolicyAction",
                                        "keyRaw": "node",

                                        "selection": {
                                            "fields": {
                                                "name": {
                                                    "type": "String",
                                                    "keyRaw": "name",
                                                    "visible": true,
                                                },

                                                "displayName": {
                                                    "type": "String",
                                                    "keyRaw": "displayName",
                                                    "visible": true,
                                                },

                                                "description": {
                                                    "type": "String",
                                                    "keyRaw": "description",
                                                    "visible": true,
                                                },

                                                "entityType": {
                                                    "type": "String",
                                                    "keyRaw": "entityType",
                                                    "visible": true,
                                                },

                                                "resourceType": {
                                                    "type": "String",
                                                    "keyRaw": "resourceType",
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

export type SettingsRoleOrganizationPolicyActions = {
	readonly "input"?: SettingsRoleOrganizationPolicyActions$input;
	readonly "result": SettingsRoleOrganizationPolicyActions$result | undefined;
};

export type SettingsRoleOrganizationPolicyActions$result = {
	/**
	 * Return the system-owned organization policy action catalog.
	 */
	readonly organizationPolicyActions: {
		readonly edges: ({
			readonly node: {
				readonly name: string;
				readonly displayName: string;
				readonly description: string;
				readonly entityType: string;
				readonly resourceType: string;
			};
		})[];
	};
};

export type SettingsRoleOrganizationPolicyActions$input = null | undefined;

export type SettingsRoleOrganizationPolicyActions$unmasked = {
	/**
	 * Return the system-owned organization policy action catalog.
	 */
	readonly organizationPolicyActions: {
		readonly edges: ({
			readonly node: {
				readonly name: string;
				readonly displayName: string;
				readonly description: string;
				readonly entityType: string;
				readonly resourceType: string;
				readonly __typename: "PolicyAction";
			};
			readonly __typename: "PolicyActionEdge";
		})[];
		readonly __typename: "PolicyActionConnection";
	};
};

export type SettingsRoleOrganizationPolicyActions$artifact = typeof artifact

"HoudiniHash=5a9d114f64e50f66175e72b6b1c8b72e336dd0e07d494d3e13e7adca154bf941"
