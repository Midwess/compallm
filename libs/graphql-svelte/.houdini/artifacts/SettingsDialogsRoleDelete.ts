const artifact = {
    "name": "SettingsDialogsRoleDelete",
    "kind": "HoudiniMutation",
    "hash": "01f1403d74e61eb34b6bf72bbf92f68dcc359acfeec48c95cb7480ecf4b88dac",
    "raw": `mutation SettingsDialogsRoleDelete($roleDeleteId: ID!) {
    roleDelete(id: $roleDeleteId) {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "roleDelete": {
                "type": "BooleanPayload",
                "keyRaw": "roleDelete(id: $roleDeleteId)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Boolean",
                            "keyRaw": "node",
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
            "roleDeleteId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsDialogsRoleDelete = {
	readonly "input": SettingsDialogsRoleDelete$input;
	readonly "result": SettingsDialogsRoleDelete$result;
};

export type SettingsDialogsRoleDelete$result = {
	readonly roleDelete: {
		readonly node: boolean;
	};
};

export type SettingsDialogsRoleDelete$input = {
	roleDeleteId: string;
};

export type SettingsDialogsRoleDelete$optimistic = {
	readonly roleDelete?: {
		readonly node?: boolean;
	};
};

export type SettingsDialogsRoleDelete$unmasked = {
	readonly roleDelete: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type SettingsDialogsRoleDelete$artifact = typeof artifact

"HoudiniHash=01f1403d74e61eb34b6bf72bbf92f68dcc359acfeec48c95cb7480ecf4b88dac"
