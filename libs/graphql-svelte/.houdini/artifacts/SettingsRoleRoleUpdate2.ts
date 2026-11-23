import type { RoleUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsRoleRoleUpdate2",
    "kind": "HoudiniMutation",
    "hash": "db0f47450c4376b82eb9ddfbbef92927bba20834e1ce0bcc0283d36950669b23",
    "raw": `mutation SettingsRoleRoleUpdate2($roleUpdateId: ID!, $roleUpdateInput: RoleUpdateInput!) {
    roleUpdate(id: $roleUpdateId, input: $roleUpdateInput) {
        node {
            id
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
            "roleUpdate": {
                "type": "RolePayload",
                "keyRaw": "roleUpdate(id: $roleUpdateId, input: $roleUpdateInput)",

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
            "roleUpdateId": "ID",
            "roleUpdateInput": "RoleUpdateInput",
        },

        "types": {
            "RoleUpdateInput": {
                "cedarSource": "String",
                "description": "String",
                "name": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsRoleRoleUpdate2 = {
	readonly "input": SettingsRoleRoleUpdate2$input;
	readonly "result": SettingsRoleRoleUpdate2$result;
};

export type SettingsRoleRoleUpdate2$result = {
	readonly roleUpdate: {
		readonly node: {
			readonly id: string;
		};
	};
};

export type SettingsRoleRoleUpdate2$input = {
	roleUpdateId: string;
	roleUpdateInput: RoleUpdateInput;
};

export type SettingsRoleRoleUpdate2$optimistic = {
	readonly roleUpdate?: {
		readonly node?: {
			readonly id?: string;
		};
	};
};

export type SettingsRoleRoleUpdate2$unmasked = {
	readonly roleUpdate: {
		readonly node: {
			readonly id: string;
			readonly __typename: "Role";
		};
		readonly __typename: "RolePayload";
	};
};

export type SettingsRoleRoleUpdate2$artifact = typeof artifact

"HoudiniHash=db0f47450c4376b82eb9ddfbbef92927bba20834e1ce0bcc0283d36950669b23"
