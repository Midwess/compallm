const artifact = {
    "name": "SettingsDialogsGroupDelete",
    "kind": "HoudiniMutation",
    "hash": "b8d4e8b72edd695b71c1c18c9b673ed51c208af0937bd5459e9f64c23af766d8",
    "raw": `mutation SettingsDialogsGroupDelete($groupDeleteId: ID!) {
    groupDelete(id: $groupDeleteId) {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "groupDelete": {
                "type": "BooleanPayload",
                "keyRaw": "groupDelete(id: $groupDeleteId)",

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
            "groupDeleteId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsDialogsGroupDelete = {
	readonly "input": SettingsDialogsGroupDelete$input;
	readonly "result": SettingsDialogsGroupDelete$result;
};

export type SettingsDialogsGroupDelete$result = {
	readonly groupDelete: {
		readonly node: boolean;
	};
};

export type SettingsDialogsGroupDelete$input = {
	groupDeleteId: string;
};

export type SettingsDialogsGroupDelete$optimistic = {
	readonly groupDelete?: {
		readonly node?: boolean;
	};
};

export type SettingsDialogsGroupDelete$unmasked = {
	readonly groupDelete: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type SettingsDialogsGroupDelete$artifact = typeof artifact

"HoudiniHash=b8d4e8b72edd695b71c1c18c9b673ed51c208af0937bd5459e9f64c23af766d8"
