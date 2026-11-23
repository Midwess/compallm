const artifact = {
    "name": "ScaffoldCheck",
    "kind": "HoudiniQuery",
    "hash": "0367474c91cd446150a1f296547c72e5e2c8a7325da0e84f6ccb956b241c0a41",
    "raw": `query ScaffoldCheck {
    __typename
}
`,

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "__typename": {
                "type": "String",
                "keyRaw": "__typename",
                "visible": true,
            },
        },
    },

    "pluginData": {},
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type ScaffoldCheck = {
	readonly "input"?: ScaffoldCheck$input;
	readonly "result": ScaffoldCheck$result | undefined;
};

export type ScaffoldCheck$result = {
	readonly __typename: "Query";
};

export type ScaffoldCheck$input = null | undefined;

export type ScaffoldCheck$unmasked = {
	readonly __typename: "Query";
};

export type ScaffoldCheck$artifact = typeof artifact

"HoudiniHash=0367474c91cd446150a1f296547c72e5e2c8a7325da0e84f6ccb956b241c0a41"
