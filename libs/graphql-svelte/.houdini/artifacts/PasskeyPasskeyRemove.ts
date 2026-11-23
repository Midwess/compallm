const artifact = {
    "name": "PasskeyPasskeyRemove",
    "kind": "HoudiniMutation",
    "hash": "e0be22a881492c710f7490c04a077a044658be7733daa4853b6da48ebf8b421e",
    "raw": `mutation PasskeyPasskeyRemove($passkeyRemoveId: ID!) {
    passkeyRemove(id: $passkeyRemoveId) {
        node
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "passkeyRemove": {
                "type": "IDPayload",
                "keyRaw": "passkeyRemove(id: $passkeyRemoveId)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "ID",
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
            "passkeyRemoveId": "ID",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type PasskeyPasskeyRemove = {
	readonly "input": PasskeyPasskeyRemove$input;
	readonly "result": PasskeyPasskeyRemove$result;
};

export type PasskeyPasskeyRemove$result = {
	readonly passkeyRemove: {
		readonly node: string;
	};
};

export type PasskeyPasskeyRemove$input = {
	passkeyRemoveId: string;
};

export type PasskeyPasskeyRemove$optimistic = {
	readonly passkeyRemove?: {
		readonly node?: string;
	};
};

export type PasskeyPasskeyRemove$unmasked = {
	readonly passkeyRemove: {
		readonly node: string;
		readonly __typename: "IDPayload";
	};
};

export type PasskeyPasskeyRemove$artifact = typeof artifact

"HoudiniHash=e0be22a881492c710f7490c04a077a044658be7733daa4853b6da48ebf8b421e"
