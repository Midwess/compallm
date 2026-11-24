const artifact = {
    "name": "SettingsMembersRoleAssign",
    "kind": "HoudiniMutation",
    "hash": "60e86290173ef42bccd15ffc4a0c73288042147d8c839c22e9df6826cffad0f6",
    "raw": `mutation SettingsMembersRoleAssign($roleAssignId: ID!, $roleAssignMemberId: ID!) {
    roleAssign(id: $roleAssignId, memberId: $roleAssignMemberId) {
        node {
            id
            name
            description
            color
            owner
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
            "roleAssign": {
                "type": "RolePayload",
                "keyRaw": "roleAssign(id: $roleAssignId, memberId: $roleAssignMemberId)",

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
            "roleAssignId": "ID",
            "roleAssignMemberId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsMembersRoleAssign = {
	readonly "input": SettingsMembersRoleAssign$input;
	readonly "result": SettingsMembersRoleAssign$result;
};

export type SettingsMembersRoleAssign$result = {
	readonly roleAssign: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
			readonly owner: boolean;
		};
	};
};

export type SettingsMembersRoleAssign$input = {
	roleAssignId: string;
	roleAssignMemberId: string;
};

export type SettingsMembersRoleAssign$optimistic = {
	readonly roleAssign?: {
		readonly node?: {
			readonly id?: string;
			readonly name?: string;
			readonly description?: string;
			readonly color?: string;
			readonly owner?: boolean;
		};
	};
};

export type SettingsMembersRoleAssign$unmasked = {
	readonly roleAssign: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly color: string;
			readonly owner: boolean;
			readonly __typename: "Role";
		};
		readonly __typename: "RolePayload";
	};
};

export type SettingsMembersRoleAssign$artifact = typeof artifact

"HoudiniHash=60e86290173ef42bccd15ffc4a0c73288042147d8c839c22e9df6826cffad0f6"
