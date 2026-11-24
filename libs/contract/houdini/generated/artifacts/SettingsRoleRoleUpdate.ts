import type { RoleUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsRoleRoleUpdate",
    "kind": "HoudiniMutation",
    "hash": "06b728a5ecd027c59787c3aa4919ca3d018bda57954c220e957431f01b62352c",
    "raw": `mutation SettingsRoleRoleUpdate($roleUpdateId: ID!, $roleUpdateInput: RoleUpdateInput!) {
    roleUpdate(id: $roleUpdateId, input: $roleUpdateInput) {
        node {
            id
            name
            description
            color
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

export type SettingsRoleRoleUpdate = {
	readonly "input": SettingsRoleRoleUpdate$input;
	readonly "result": SettingsRoleRoleUpdate$result;
};

export type SettingsRoleRoleUpdate$result = {
	readonly roleUpdate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
		};
	};
};

export type SettingsRoleRoleUpdate$input = {
	roleUpdateId: string;
	roleUpdateInput: RoleUpdateInput;
};

export type SettingsRoleRoleUpdate$optimistic = {
	readonly roleUpdate?: {
		readonly node?: {
			readonly id?: string;
			readonly name?: string;
			readonly description?: string;
			readonly color?: string;
		};
	};
};

export type SettingsRoleRoleUpdate$unmasked = {
	readonly roleUpdate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
			readonly __typename: "Role";
		};
		readonly __typename: "RolePayload";
	};
};

export type SettingsRoleRoleUpdate$artifact = typeof artifact

"HoudiniHash=06b728a5ecd027c59787c3aa4919ca3d018bda57954c220e957431f01b62352c"
