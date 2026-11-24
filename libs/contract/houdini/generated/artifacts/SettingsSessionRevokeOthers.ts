const artifact = {
    "name": "SettingsSessionRevokeOthers",
    "kind": "HoudiniMutation",
    "hash": "32e3b7bba72d4efc9f405604988ae515101828e76e3447532ad9e888b281238b",
    "raw": `mutation SettingsSessionRevokeOthers {
    sessionRevokeOthers {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "sessionRevokeOthers": {
                "type": "BooleanPayload",
                "keyRaw": "sessionRevokeOthers",

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
} as const

export default artifact

export type SettingsSessionRevokeOthers = {
	readonly "input"?: SettingsSessionRevokeOthers$input;
	readonly "result": SettingsSessionRevokeOthers$result;
};

export type SettingsSessionRevokeOthers$result = {
	/**
	 * Revoke all other account sessions, retaining this login.
	 */
	readonly sessionRevokeOthers: {
		readonly node: boolean;
	};
};

export type SettingsSessionRevokeOthers$input = null | undefined;

export type SettingsSessionRevokeOthers$optimistic = {
	/**
	 * Revoke all other account sessions, retaining this login.
	 */
	readonly sessionRevokeOthers?: {
		readonly node?: boolean;
	};
};

export type SettingsSessionRevokeOthers$unmasked = {
	/**
	 * Revoke all other account sessions, retaining this login.
	 */
	readonly sessionRevokeOthers: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type SettingsSessionRevokeOthers$artifact = typeof artifact

"HoudiniHash=32e3b7bba72d4efc9f405604988ae515101828e76e3447532ad9e888b281238b"
