import type { RoleCreateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsRoleCreate",
    "kind": "HoudiniMutation",
    "hash": "5b30bb64aaa6d432aace259cca7b97ad09672a358e95a95c3b7efc08b471af73",
    "raw": `mutation SettingsRoleCreate($roleCreateInput: RoleCreateInput!) {
    roleCreate(input: $roleCreateInput) {
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
            "roleCreate": {
                "type": "RolePayload",
                "keyRaw": "roleCreate(input: $roleCreateInput)",

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
            "roleCreateInput": "RoleCreateInput",
        },

        "types": {
            "RoleCreateInput": {
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

export type SettingsRoleCreate = {
	readonly "input": SettingsRoleCreate$input;
	readonly "result": SettingsRoleCreate$result;
};

export type SettingsRoleCreate$result = {
	/**
	 * Create a role in the verified active organization.
	 */
	readonly roleCreate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
		};
	};
};

export type SettingsRoleCreate$input = {
	roleCreateInput: RoleCreateInput;
};

export type SettingsRoleCreate$optimistic = {
	/**
	 * Create a role in the verified active organization.
	 */
	readonly roleCreate?: {
		readonly node?: {
			readonly id?: string;
			readonly name?: string;
			readonly description?: string;
			readonly color?: string;
		};
	};
};

export type SettingsRoleCreate$unmasked = {
	/**
	 * Create a role in the verified active organization.
	 */
	readonly roleCreate: {
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

export type SettingsRoleCreate$artifact = typeof artifact

"HoudiniHash=5b30bb64aaa6d432aace259cca7b97ad09672a358e95a95c3b7efc08b471af73"
