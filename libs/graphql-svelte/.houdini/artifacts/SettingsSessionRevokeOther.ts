const artifact = {
    "name": "SettingsSessionRevokeOther",
    "kind": "HoudiniMutation",
    "hash": "ce63ea641f7556f58e9baba1836f28815eec0774e76a68f3677a2e9ba73188a3",
    "raw": `mutation SettingsSessionRevokeOther($sessionRevokeOtherId: ID!) {
    sessionRevokeOther(id: $sessionRevokeOtherId) {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "sessionRevokeOther": {
                "type": "BooleanPayload",
                "keyRaw": "sessionRevokeOther(id: $sessionRevokeOtherId)",

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
            "sessionRevokeOtherId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsSessionRevokeOther = {
	readonly "input": SettingsSessionRevokeOther$input;
	readonly "result": SettingsSessionRevokeOther$result;
};

export type SettingsSessionRevokeOther$result = {
	/**
	 * Revoke one other session belonging to the authenticated account.
	 */
	readonly sessionRevokeOther: {
		readonly node: boolean;
	};
};

export type SettingsSessionRevokeOther$input = {
	sessionRevokeOtherId: string;
};

export type SettingsSessionRevokeOther$optimistic = {
	/**
	 * Revoke one other session belonging to the authenticated account.
	 */
	readonly sessionRevokeOther?: {
		readonly node?: boolean;
	};
};

export type SettingsSessionRevokeOther$unmasked = {
	/**
	 * Revoke one other session belonging to the authenticated account.
	 */
	readonly sessionRevokeOther: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type SettingsSessionRevokeOther$artifact = typeof artifact

"HoudiniHash=ce63ea641f7556f58e9baba1836f28815eec0774e76a68f3677a2e9ba73188a3"
