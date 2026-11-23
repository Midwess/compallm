const artifact = {
    "name": "AuthSessionRevoke",
    "kind": "HoudiniMutation",
    "hash": "071fb0a46c4811f9e37435f9e37e4d87aa9d60af41cccad8e95c8fe1f6a84063",
    "raw": `mutation AuthSessionRevoke {
    sessionRevoke {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "sessionRevoke": {
                "type": "BooleanPayload",
                "keyRaw": "sessionRevoke",

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

export type AuthSessionRevoke = {
	readonly "input"?: AuthSessionRevoke$input;
	readonly "result": AuthSessionRevoke$result;
};

export type AuthSessionRevoke$result = {
	/**
	 * Revoke the current session and clear its cookies after success.
	 */
	readonly sessionRevoke: {
		readonly node: boolean;
	};
};

export type AuthSessionRevoke$input = null | undefined;

export type AuthSessionRevoke$optimistic = {
	/**
	 * Revoke the current session and clear its cookies after success.
	 */
	readonly sessionRevoke?: {
		readonly node?: boolean;
	};
};

export type AuthSessionRevoke$unmasked = {
	/**
	 * Revoke the current session and clear its cookies after success.
	 */
	readonly sessionRevoke: {
		readonly node: boolean;
		readonly __typename: "BooleanPayload";
	};
};

export type AuthSessionRevoke$artifact = typeof artifact

"HoudiniHash=071fb0a46c4811f9e37435f9e37e4d87aa9d60af41cccad8e95c8fe1f6a84063"
