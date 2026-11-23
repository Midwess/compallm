const artifact = {
    "name": "SettingsRoleRole",
    "kind": "HoudiniQuery",
    "hash": "a4c0f3e516d1d91174c4c9a80a969fa5466107c65c06a678a206b890fc15c4e5",
    "raw": `query SettingsRoleRole($includeRoleNodeCedarSource: Boolean! = false, $roleId: ID!) {
    role(id: $roleId) {
        node {
            id
            name
            description
            color
            cedarSource @include(if: $includeRoleNodeCedarSource)
            owner
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
            "role": {
                "type": "RolePayload",
                "keyRaw": "role(id: $roleId)",
                "nullable": true,

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Role",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
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

                                    "description": {
                                        "type": "String",
                                        "keyRaw": "description",
                                        "visible": true,
                                    },

                                    "color": {
                                        "type": "String",
                                        "keyRaw": "color",
                                        "visible": true,
                                    },

                                    "cedarSource": {
                                        "type": "String",
                                        "keyRaw": "cedarSource",
                                        "nullable": true,

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeRoleNodeCedarSource",
                                                    }
                                                }
                                            }
                                        }],

                                        "visible": true,
                                    },

                                    "owner": {
                                        "type": "Boolean",
                                        "keyRaw": "owner",
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
            "roleId": "ID",
            "includeRoleNodeCedarSource": "Boolean",
        },

        "types": {},

        "defaults": {
            "includeRoleNodeCedarSource": false,
        },

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsRoleRole = {
	readonly "input": SettingsRoleRole$input;
	readonly "result": SettingsRoleRole$result | undefined;
};

export type SettingsRoleRole$result = {
	/**
	 * Find visible role metadata in the active organization.
	 */
	readonly role: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
			/**
			 * Native Cedar source; template slots bind to this role and its organization.
			 */
			readonly cedarSource: string | null;
			readonly owner: boolean;
		};
	} | null;
};

export type SettingsRoleRole$input = {
	roleId: string;
	includeRoleNodeCedarSource: boolean;
};

export type SettingsRoleRole$unmasked = {
	/**
	 * Find visible role metadata in the active organization.
	 */
	readonly role: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
			/**
			 * Native Cedar source; template slots bind to this role and its organization.
			 */
			readonly cedarSource: string | null;
			readonly owner: boolean;
			readonly __typename: "Role";
		};
		readonly __typename: "RolePayload";
	} | null;
};

export type SettingsRoleRole$artifact = typeof artifact

"HoudiniHash=a4c0f3e516d1d91174c4c9a80a969fa5466107c65c06a678a206b890fc15c4e5"
